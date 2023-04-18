use std::{
    collections::HashMap,
    rc::Rc,
    str::FromStr,
    sync::{mpsc, RwLock},
};

use chrono::Local;
use serde_json::Value;
use tracing::{debug, error, info};
use vrsc_rpc::{
    bitcoin::{hashes::sha256d::Hash, Txid},
    json::{vrsc::Amount, GetRawTransactionResultVerbose},
    Client, RpcApi,
};

use crate::{
    ui::{UIMessage, UI},
    verus::pbaas::local_pbaas_chains,
    verus::{vrsc::VerusChain, Chain},
    views::log::LogMessage,
};

pub type IdNames = Rc<RwLock<HashMap<String, String>>>;

pub struct Controller {
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub l_tx: mpsc::Sender<LogMessage>,
    pub ui: UI,
    pbaas_chains: Vec<Rc<RwLock<Box<dyn Chain>>>>,
    active_chain: Rc<RwLock<Box<dyn Chain>>>,
    id_names: IdNames,
}

impl Controller {
    pub fn new(testnet: bool) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();

        let (l_tx, l_rx) = mpsc::channel::<LogMessage>();

        let id_names = Rc::new(RwLock::new(HashMap::new()));

        let pbaas_chains = get_running_chains(testnet, Rc::clone(&id_names));
        let first = Rc::clone(pbaas_chains.first().unwrap());

        for chain in pbaas_chains.iter() {
            if let Ok(read) = chain.read() {
                read.start_zmq_tx_listener(c_tx.clone());
                read.start_zmq_block_listener(c_tx.clone())
            }
        }

        let controller = Controller {
            c_rx,
            l_tx,
            ui: UI::new(c_tx.clone(), l_rx),
            pbaas_chains: pbaas_chains,
            active_chain: first,
            id_names,
        };

        controller
    }

    pub fn start(&mut self) {
        self.ui.siv.set_autorefresh(false);

        self.update_selection_screen();
        self.update_baskets();
        let _ = self.ui.ui_tx.send(UIMessage::UpdateTLV(get_tlv()));

        while self.ui.step() {
            if let Some(message) = self.c_rx.try_iter().next() {
                match message {
                    ControllerMessage::CurrencySelectionChange => {
                        info!("Filter changed");

                        if let Err(e) = self.ui.ui_tx.send(UIMessage::ApplyFilter) {
                            error!("{:?}", e)
                        }
                    }
                    ControllerMessage::NewBlock(chain_name, blockhash) => {
                        let active_chain_name = self.active_chain.read().unwrap().get_name();
                        if active_chain_name == chain_name {
                            info!("new block arrived on {}: {}", chain_name, blockhash);

                            self.update_baskets();
                        }

                        self.ui.ui_tx.send(UIMessage::UpdateTLV(get_tlv())).unwrap();
                    }
                    ControllerMessage::NewTransaction(chain_name, txid) => {
                        let active_chain_name = self.active_chain.read().unwrap().get_name();
                        if active_chain_name == chain_name {
                            debug!("process this tx: {}", txid);

                            let hash = Hash::from_str(&txid).unwrap();
                            let txid = Txid::from_hash(hash);

                            if let Ok(raw_tx) = self
                                .active_chain
                                .read()
                                .unwrap()
                                .client()
                                .get_raw_transaction_verbose(&txid)
                            {
                                process_transaction(
                                    raw_tx,
                                    Rc::clone(&self.id_names),
                                    Rc::clone(&self.active_chain),
                                    self.l_tx.clone(),
                                )
                            }
                        }

                        let cb_sink = self.ui.siv.cb_sink().clone();
                        cb_sink
                            .send(Box::new(move |siv| {
                                siv.noop();
                            }))
                            .unwrap();
                    }
                    ControllerMessage::ChainChange(chain) => {
                        debug!("change the chain to {:?}", &chain);

                        self.active_chain = Rc::clone(
                            self.pbaas_chains
                                .iter()
                                .find(|c| c.read().unwrap().get_name() == chain)
                                .unwrap(),
                        );

                        self.update_selection_screen();
                        self.update_baskets();
                    }
                    ControllerMessage::PBaaSDialog(c_tx) => {
                        let labels = self
                            .pbaas_chains
                            .iter()
                            .map(|c| c.read().unwrap().get_name())
                            .collect();

                        self.ui
                            .ui_tx
                            .send(UIMessage::PBaasDialog(c_tx, labels))
                            .unwrap();
                    }
                }
            }
        }

        // 1d878bf932c406647374cafa9019ee5b00c581309e01f772d6e147f34b6bc601 = reservetransfer > spenttxid
        // 0b80f3f5b0932f47c6d75f67085979cf5067b60077f3196f080fa788f078804d
        // 6c070618db343c1ba288f7da713729540058c4e54ea63b5ac0c5757fc5166d76

        // 59f9b15870491c4112b6b892f5d3a54a8ad301503071c27a885429ce4df2a86d VRSCTEST -> VRSC-EUR

        // 1b04030001011504af02625e74df9de1cf78921e0690ab94b2d6c603cc3604030901011504af02625e74df9de1cf78921e0690ab94b2d6c6031a0176f89c6dc26d4d775b3dceef7ad4f1d3efd35a0380e9aacb0d75
        // 1b04030001011504af02625e74df9de1cf78921e0690ab94b2d6c603cc3604030901011504af02625e74df9de1cf78921e0690ab94b2d6c6031a0176f89c6dc26d4d775b3dceef7ad4f1d3efd35a0380e9c8bf0775
        fn process_transaction(
            raw_tx: GetRawTransactionResultVerbose,
            id_names: IdNames,
            active_chain: Rc<RwLock<Box<dyn Chain>>>,
            l_tx: mpsc::Sender<LogMessage>,
        ) {
            if raw_tx.confirmations.is_none() {
                for vout in &raw_tx.vout {
                    if let Some(reserve_transfer) = &vout.script_pubkey.reservetransfer {
                        debug!("a transfer was initiated: {}", raw_tx.txid);

                        if let Ok(mut write) = id_names.write() {
                            let currencyname = write
                                .entry(reserve_transfer.destinationcurrencyid.to_string())
                                .or_insert_with(|| {
                                    active_chain
                                        .read()
                                        .unwrap()
                                        .client()
                                        .get_currency(
                                            &reserve_transfer.destinationcurrencyid.to_string(),
                                        )
                                        .unwrap()
                                        .fullyqualifiedname
                                })
                                .clone();

                            debug!("currencyname: {}", &currencyname);

                            let amount_in_currency = write
                                .entry(
                                    reserve_transfer
                                        .currencyvalues
                                        .keys()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .or_insert_with(|| {
                                    active_chain
                                        .read()
                                        .unwrap()
                                        .client()
                                        .get_currency(
                                            &reserve_transfer.destinationcurrencyid.to_string(),
                                        )
                                        .unwrap()
                                        .fullyqualifiedname
                                })
                                .clone();

                            l_tx.send(LogMessage {
                                time: format!("{}", Local::now().format("%H:%M:%S")),
                                _type: crate::views::log::MessageType::Initiate,
                                reserve: currencyname,
                                amount_in_currency: amount_in_currency,
                                amount_in: vout.value,
                                amount_out: None,
                            })
                            .unwrap();
                        }
                    }
                }
            }

            if raw_tx.confirmations.is_some() {
                for vout in &raw_tx.vout {
                    if let Some(crosschain_import) = &vout.script_pubkey.crosschainimport {
                        info!("a transfer was settled: {}", raw_tx.txid);
                        info!("crosschainimport {:#?}", crosschain_import);
                        if let Ok(mut write) = id_names.write() {
                            let currencyname = write
                                .entry(crosschain_import.importcurrencyid.to_string())
                                .or_insert_with(|| {
                                    active_chain
                                        .read()
                                        .unwrap()
                                        .client()
                                        .get_currency(
                                            &crosschain_import.importcurrencyid.to_string(),
                                        )
                                        .unwrap()
                                        .fullyqualifiedname
                                })
                                .clone();

                            l_tx.send(LogMessage {
                                time: format!("{}", Local::now().format("%H:%M:%S")),
                                _type: crate::views::log::MessageType::Initiate,
                                reserve: currencyname,
                                amount_in_currency: String::new(),
                                amount_in: vout.value,
                                amount_out: None,
                            })
                            .unwrap();
                        }
                    }
                    // if let Some(object) = value["reserveoutput"].as_object() {
                    //     info!("reserveoutput {:#?}", object);
                    // }
                }
            }
        }
    }

    pub fn update_selection_screen(&mut self) {
        if let Ok(write) = self.active_chain.write() {
            if let Ok(currencies) = write.get_latest_currencies() {
                if let Err(e) = self
                    .ui
                    .ui_tx
                    .send(UIMessage::UpdateSelectorCurrencies(currencies))
                {
                    error!("UIMessage send error: {:?}", e);
                }

                if let Err(e) = self.ui.ui_tx.send(UIMessage::ApplyFilter) {
                    error!("UIMessage send error: {:?}", e);
                }
            }
        }
    }

    pub fn update_baskets(&mut self) {
        if let Ok(mut write) = self.active_chain.write() {
            if let Ok(baskets) = write.get_latest_baskets() {
                if let Err(e) = self
                    .ui
                    .ui_tx
                    .send(UIMessage::UpdateReserveOverview(baskets))
                {
                    error!("{:?}", e)
                }
            }
        }
    }
}

fn get_tlv() -> Amount {
    let client = Client::vrsc(true, vrsc_rpc::Auth::ConfigFile).unwrap();
    let resp: Value = client
        .call("getcurrencyconverters", &["vrsctest".into()])
        .unwrap();

    let mut total_vrsc = Amount::ZERO;

    for obj in resp.as_array().unwrap().iter() {
        let obj = obj.as_object().unwrap();
        let last_nota = obj["lastnotarization"].as_object().unwrap();
        let currency_state = last_nota["currencystate"].as_object().unwrap();
        let reserve_currencies = currency_state["reservecurrencies"].as_array().unwrap();
        for currency in reserve_currencies.iter() {
            if currency
                .as_object()
                .unwrap()
                .get("currencyid")
                .unwrap()
                .as_str()
                .unwrap()
                == "iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq"
            {
                total_vrsc += Amount::from_vrsc(
                    currency
                        .as_object()
                        .unwrap()
                        .get("reserves")
                        .unwrap()
                        .as_f64()
                        .unwrap(),
                )
                .unwrap();
            }
        }
    }

    dbg!(total_vrsc.as_vrsc());

    total_vrsc
}

pub fn get_running_chains(testnet: bool, id_names: IdNames) -> Vec<Rc<RwLock<Box<dyn Chain>>>> {
    let mut all_chains: Vec<Rc<RwLock<Box<dyn Chain>>>> = vec![];

    let v_chain: Rc<RwLock<Box<dyn Chain>>> = Rc::new(RwLock::new(Box::new(VerusChain::new(
        testnet,
        Rc::clone(&id_names),
    ))));
    all_chains.push(v_chain);

    let local_chains = local_pbaas_chains(testnet, Rc::clone(&id_names));
    local_chains.into_iter().for_each(|mut c| {
        c.set_name();
        all_chains.push(Rc::new(RwLock::new(Box::new(c))));
    });

    all_chains
        .into_iter()
        .filter(|chain| chain.read().unwrap().client().ping().is_ok())
        .collect()
}

pub enum ControllerMessage {
    NewBlock(String, String),
    NewTransaction(String, String),
    CurrencySelectionChange,
    ChainChange(String),
    PBaaSDialog(mpsc::Sender<ControllerMessage>),
}
