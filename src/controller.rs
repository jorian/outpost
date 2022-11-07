use std::{str::FromStr, sync::mpsc};

use chrono::Local;
use tracing::{debug, error, info};
use vrsc_rpc::{
    bitcoin::{hashes::sha256d::Hash, Txid},
    RpcApi,
};

use crate::{
    ui::{UIMessage, UI},
    util::zmq::*,
    verus::Verus,
    views::log::LogMessage,
};

pub struct Controller {
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub l_tx: mpsc::Sender<LogMessage>,
    pub ui: UI,
    pub verus: Verus,
}

impl Controller {
    pub fn new(testnet: bool) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        let (l_tx, l_rx) = mpsc::channel::<LogMessage>();
        zmq_block_notify(c_tx.clone());
        zmq_tx_notify(c_tx.clone());

        Controller {
            c_rx,
            l_tx,
            ui: UI::new(c_tx.clone(), l_rx),
            verus: Verus::new(testnet),
        }
    }

    pub fn start(&mut self) {
        self.ui.siv.set_autorefresh(false);

        self.update_selection_screen();
        self.update_baskets();

        while self.ui.step() {
            if let Some(message) = self.c_rx.try_iter().next() {
                match message {
                    ControllerMessage::CurrencyModeChange => {}
                    ControllerMessage::CurrencySelectionChange => {
                        info!("Filter changed");

                        if let Err(e) = self.ui.ui_tx.send(UIMessage::ApplyFilter) {
                            error!("{:?}", e)
                        }
                    }
                    ControllerMessage::NewBlock(blockhash) => {
                        info!("new block arrived: {}", blockhash);

                        self.update_baskets();
                    }
                    ControllerMessage::NewTransaction(txid) => {
                        // 1d878bf932c406647374cafa9019ee5b00c581309e01f772d6e147f34b6bc601 = reservetransfer > spenttxid
                        // 0b80f3f5b0932f47c6d75f67085979cf5067b60077f3196f080fa788f078804d
                        // 6c070618db343c1ba288f7da713729540058c4e54ea63b5ac0c5757fc5166d76

                        // 59f9b15870491c4112b6b892f5d3a54a8ad301503071c27a885429ce4df2a86d VRSCTEST -> VRSC-EUR

                        // 1b04030001011504af02625e74df9de1cf78921e0690ab94b2d6c603cc3604030901011504af02625e74df9de1cf78921e0690ab94b2d6c6031a0176f89c6dc26d4d775b3dceef7ad4f1d3efd35a0380e9aacb0d75
                        // 1b04030001011504af02625e74df9de1cf78921e0690ab94b2d6c603cc3604030901011504af02625e74df9de1cf78921e0690ab94b2d6c6031a0176f89c6dc26d4d775b3dceef7ad4f1d3efd35a0380e9c8bf0775

                        let hash = Hash::from_str(&txid).unwrap();
                        let txid = Txid::from_hash(hash);
                        match self.verus.client.get_raw_transaction_verbose(&txid) {
                            Ok(raw_tx) => {
                                if raw_tx.confirmations.is_none() {
                                    for vout in &raw_tx.vout {
                                        if let Some(reserve_transfer) =
                                            &vout.script_pubkey.reservetransfer
                                        {
                                            info!("a transfer was initiated: {}", raw_tx.txid);
                                            info!("{:#?}", reserve_transfer);

                                            let currencyname = self.verus.currency_id_to_name(
                                                reserve_transfer.destinationcurrencyid.clone(),
                                            );

                                            let amount_in_currency =
                                                self.verus.currency_id_to_name(
                                                    reserve_transfer
                                                        .currencyvalues
                                                        .keys()
                                                        .last()
                                                        .unwrap()
                                                        .to_owned(),
                                                );

                                            self.l_tx
                                                .send(LogMessage {
                                                    time: format!(
                                                        "{}",
                                                        Local::now().format("%H:%M:%S")
                                                    ),
                                                    _type: crate::views::log::MessageType::Initiate,
                                                    reserve: currencyname,
                                                    amount_currency: amount_in_currency,
                                                    amount_in: vout.value,
                                                    amount_out: None,
                                                })
                                                .unwrap();
                                        }
                                    }
                                }
                                if raw_tx.confirmations.is_some() {
                                    for vout in &raw_tx.vout {
                                        // let value =
                                        //     serde_json::to_value(&vout.script_pubkey).unwrap();

                                        if let Some(crosschain_import) =
                                            &vout.script_pubkey.crosschainimport
                                        {
                                            info!("a transfer was settled: {}", raw_tx.txid);
                                            info!("crosschainimport {:#?}", crosschain_import);

                                            // let currencyname = self.verus.currency_id_to_name(
                                            //     crosschain_import.importcurrencyid.clone(),
                                            // );

                                            // self.l_tx
                                            //     .send(LogMessage {
                                            //         time: format!(
                                            //             "{}",
                                            //             Local::now().format("%H:%M:%S")
                                            //         ),
                                            //         _type: crate::views::log::MessageType::Initiate,
                                            //         reserve: currencyname,
                                            //         amount_in: crosschain_import
                                            //             .valuein
                                            //             .as_f64()
                                            //             .unwrap(), //.as_vrsc(),
                                            //         amount_out: None,
                                            //     })
                                            //     .unwrap();
                                        }
                                        // if let Some(object) = value["reserveoutput"].as_object() {
                                        //     info!("reserveoutput {:#?}", object);
                                        // }
                                    }
                                }
                            }
                            Err(e) => error!("{:?}", e),
                        }

                        let cb_sink = self.ui.siv.cb_sink().clone();
                        cb_sink
                            .send(Box::new(move |siv| {
                                siv.noop();
                            }))
                            .unwrap();
                    }
                }
            }
        }
    }

    pub fn update_selection_screen(&mut self) {
        if let Ok(currencies) = self.verus.get_latest_currencies() {
            if let Err(e) = self
                .ui
                .ui_tx
                .send(UIMessage::UpdateSelectorCurrencies(currencies))
            {
                error!("UIMessage send error: {:?}", e);
            }
        }
    }

    pub fn update_baskets(&mut self) {
        if let Ok(baskets) = self.verus.get_latest_baskets() {
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

pub enum ControllerMessage {
    NewBlock(String),
    NewTransaction(String),
    CurrencySelectionChange,
    CurrencyModeChange,
}
