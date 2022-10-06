use std::sync::mpsc;

use cursive::{menu::Tree, CursiveRunnable, CursiveRunner};

use crate::{
    controller::{ControllerMessage, CurrencyMode},
    verus::Basket,
};

pub type UIReceiver = mpsc::Receiver<UIMessage>;
pub type UISender = mpsc::Sender<UIMessage>;

pub struct UI {
    pub siv: CursiveRunner<CursiveRunnable>,
    ui_rx: UIReceiver,
    pub ui_tx: UISender,
}

impl UI {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();
        let mut siv = cursive::ncurses().into_runner();

        siv.menubar().add_subtree(
            "Currency mode",
            Tree::new()
                .leaf("Reserve", {
                    let c_tx_clone = c_tx.clone();
                    move |_| {
                        c_tx_clone
                            .send(ControllerMessage::CurrencyModeChange(CurrencyMode::Reserve))
                            .unwrap();
                    }
                })
                .leaf("Basket", {
                    let c_tx_clone = c_tx.clone();
                    move |_| {
                        c_tx_clone
                            .send(ControllerMessage::CurrencyModeChange(CurrencyMode::Basket))
                            .unwrap();
                    }
                }),
        );

        siv.set_autohide_menu(false);

        // 2 modes:
        // - reserve currency mode
        // - basket mode

        // |--------------------------------------------------------------------------------------------------|
        // | ______menubar___________________________________________________________________________________ |
        // |              |                                                                                   |
        // |  [ ] VRSC    |   VRSC-ETH                                                                        |
        // |  [ ] BTC     |   -> VRSC                                                       1.23456789        |
        // |  [x] vETH    |   -> vETH                                                       0.12345678        |
        // |  [ ] USDc    |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |              |                                                                                   |
        // |--------------------------------------------------------------------------------------------------|

        UI { siv, ui_rx, ui_tx }
    }

    pub fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UIMessage::UpdateReserveOverview(_baskets) => {
                    // Need to show:
                    // - name of the basket
                    // - amount of basket currency in circulation
                    // - names of the reserves that were selected
                    // - amounts of the reserves in circulation

                    // clicking on the name of the basket should open up a layer with all the information of the basket and all its currencies
                    // the selection should just be a filter of the baskets
                }
            }
        }

        self.siv.run();

        true
    }
}

pub enum UIMessage {
    UpdateReserveOverview(Vec<Basket>),
}
