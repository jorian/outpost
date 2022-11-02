use std::sync::mpsc;

use tracing::{error, info};
use vrsc_rpc::json::Currency;

use crate::{
    ui::{UIMessage, UI},
    util::zmq::*,
    verus::{Basket, Verus},
};

pub struct Controller {
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub ui: UI,
    pub baskets: Vec<Basket>,
    pub currencies: Vec<Currency>,
    pub verus: Verus,
}

impl Controller {
    pub fn new() -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        zmq_block_notify(c_tx.clone());

        Controller {
            c_rx,
            ui: UI::new(c_tx.clone()),
            baskets: vec![],
            currencies: vec![],
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

                        self.update_baskets();
                    }
                    ControllerMessage::NewBlock(blockhash) => {
                        info!("new block arrived: {}", blockhash);

                        self.update_baskets();

                        // create a Selector view with data that i can query using `self.ui.siv.call_on("SELECTOR_VIEW")`
                        // self.client.get_currency_converters();

                        // call a update method on the reserves view.
                        // - send UIMessage to update Reserves
                        // - Reserves queries RPC
                        // - Reserves calls siv to get latest selector data
                        // - Reserves uses the selector data to filter

                        // how do i know that a specific basket was selected?
                        // - can we have multiple baskets at the same time? why not? (maybe v2)

                        // need to get all the relevant baskets
                        // do the filtering and send it to the UI

                        // at this point, i need to start querying the daemon for
                        // the latest currency state (getcurrencystate 'currency'), based on
                        // - some selected currency or currencies that exist in the reserve.
                        //   where should i store selected currencies? it's the resulting list of currencies for `getcurrencyconverters`
                        // - a specific basket itself
                        // and then i need to send a message to the UI thread to update the view (table).
                        // the message needs to contain:
                        // - amount of x in reserve
                        // - price in reserve, relative to the basket currency

                        // - update a log view (v2)
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
            self.currencies = currencies;

            if let Err(e) = self
                .ui
                .ui_tx
                .send(UIMessage::UpdateSelectorCurrencies(self.currencies.clone()))
            {
                error!("UIMessage send error: {:?}", e);
            }
        }
    }

    pub fn update_baskets(&mut self) {
        if let Ok(baskets) = self.verus.get_latest_baskets() {
            self.baskets = baskets;

            if let Err(e) = self
                .ui
                .ui_tx
                .send(UIMessage::UpdateReserveOverview(self.baskets.clone()))
            {
                error!("UIMessage send error: {:?}", e);
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
