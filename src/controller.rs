use std::sync::mpsc;

use tracing::{error, info};

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
    pub fn new() -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        zmq_block_notify(c_tx.clone());

        Controller {
            c_rx,
            ui: UI::new(c_tx.clone()),
            verus: Verus::new(),
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
                        info!("new tx arrived: {}", txid);
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
