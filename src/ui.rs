use std::sync::mpsc;

use cursive::{
    view::{Nameable, Resizable},
    views::{LinearLayout, Panel},
    CbSink, CursiveRunnable, CursiveRunner,
};
use tracing::debug;
use vrsc_rpc::json::Currency;

use crate::{
    controller::ControllerMessage,
    userdata::UserData,
    verus::Basket,
    views::{
        filterbox::FilterBox,
        log::{LogMessage, LogView},
        pbaas_dialog::PbaasDialog,
        reserves::Reserves,
        selector::Selector,
    },
};

pub type UIReceiver = mpsc::Receiver<UIMessage>;
pub type UISender = mpsc::Sender<UIMessage>;

pub struct UI {
    pub siv: CursiveRunner<CursiveRunnable>,
    ui_rx: UIReceiver,
    pub ui_tx: UISender,
}

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

fn gather_pbaas_chains(cb_sink: CbSink) {
    std::thread::spawn(move || {
        let data = UserData::new();

        cb_sink.send(Box::new(|siv| siv.set_user_data(data)));
    });
}

impl UI {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>, l_rx: mpsc::Receiver<LogMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();
        let mut siv = cursive::ncurses().into_runner();
        siv.update_theme(|theme| theme.shadow = false);

        gather_pbaas_chains(siv.cb_sink().clone());
        let c_tx_clone = c_tx.clone();

        siv.add_global_callback('p', move |s| {
            if let Some(chains) = s.with_user_data(|data: &mut UserData| data.pbaas_chains.clone())
            {
                s.add_layer(PbaasDialog::new(c_tx_clone.clone(), chains));
            }
        });

        let main_view = LinearLayout::horizontal()
            .child(
                Panel::new(
                    LinearLayout::vertical().child(
                        Selector::new(c_tx.clone())
                            .with_name("SELECTOR")
                            .full_height(),
                    ),
                )
                .title("Selector")
                .fixed_width(30),
            )
            .child(
                Panel::new(Reserves::new().with_name("RESERVES"))
                    .title("Reserves")
                    .full_width(),
            )
            .child(Panel::new(LogView::new(l_rx)).title("Log").min_width(75));

        siv.add_fullscreen_layer(main_view);

        UI { siv, ui_rx, ui_tx }
    }

    pub fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UIMessage::UpdateReserveOverview(baskets) => {
                    debug!("update reserve overview");

                    let cb_sink = self.siv.cb_sink().clone();
                    std::thread::spawn(move || {
                        cb_sink
                            .send(Box::new(move |s| {
                                s.call_on_name("RESERVES", |reserves_view: &mut Reserves| {
                                    reserves_view.update_baskets(baskets);

                                    reserves_view.update_view();
                                });
                            }))
                            .unwrap();
                    });
                }
                UIMessage::UpdateSelectorCurrencies(vec) => {
                    self.siv
                        .call_on_name("SELECTOR", |selector_view: &mut Selector| {
                            selector_view.update(vec);
                        });
                }
                UIMessage::ApplyFilter => {
                    let mut checked_currencies = vec![];

                    let cb_sink = self.siv.cb_sink().clone();
                    std::thread::spawn(move || {
                        cb_sink.send(Box::new(move |s| {
                            s.call_on_all_named("filterbox", |filterbox: &mut FilterBox| {
                                if filterbox.checkbox.is_checked() {
                                    debug!("{}", &filterbox.currency.currencydefinition.name);
                                    checked_currencies.push(filterbox.currency.clone());
                                }
                            });

                            debug!("{:?}", &checked_currencies);

                            s.call_on_name("RESERVES", |reserves_view: &mut Reserves| {
                                reserves_view.update_checked_currencies(checked_currencies);

                                reserves_view.update_view();
                            });
                        }))
                    });
                }
                UIMessage::NewLog(message) => {
                    debug!("let's put a message in the log for {}", message);
                }
            }
        }

        self.siv.step();

        true
    }
}

pub enum UIMessage {
    UpdateReserveOverview(Vec<Basket>),
    UpdateSelectorCurrencies(Vec<Currency>),
    ApplyFilter,
    NewLog(String),
}
