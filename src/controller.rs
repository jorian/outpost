use std::{str::FromStr, sync::mpsc};

use tracing::{debug, error, info};
use vrsc_rpc::{
    bitcoin::{hashes::sha256d::Hash, Txid},
    RpcApi,
};

use crate::{
    ui::{UIMessage, UI},
    util::zmq::*,
    verus::Verus,
};

pub struct Controller {
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub ui: UI,
    pub verus: Verus,
}

impl Controller {
    pub fn new(testnet: bool) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        zmq_block_notify(c_tx.clone());
        zmq_tx_notify(c_tx.clone());

        Controller {
            c_rx,
            ui: UI::new(c_tx.clone()),
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

                        info!("new tx arrived: {}", txid);

                        let hash = Hash::from_str(&txid).unwrap();
                        let txid = Txid::from_hash(hash);
                        let raw_tx = self.verus.client.get_raw_transaction_verbose(&txid);
                        debug!("{:#?}", &raw_tx);
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
