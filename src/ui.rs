use std::sync::mpsc;

use cursive::{
    view::{Nameable, Resizable},
    views::{LinearLayout, Panel, ResizedView},
    CursiveRunnable, CursiveRunner,
};
use tracing::debug;
use vrsc_rpc::json::Currency;

use crate::{
    controller::ControllerMessage,
    verus::Basket,
    views::{filterbox::FilterBox, reserves::Reserves, selector::Selector},
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

impl UI {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();
        let mut siv = cursive::ncurses().into_runner();
        siv.update_theme(|theme| theme.shadow = false);

        let main_view = LinearLayout::horizontal()
            .child(
                Panel::new(
                    LinearLayout::vertical()
                        // .child(DummyView {}.fixed_height(1))
                        .child(
                            Selector::new(c_tx.clone())
                                .with_name("SELECTOR")
                                .full_height(),
                        ),
                )
                .title("Selector")
                .fixed_width(30),
            )
            .child(
                Panel::new(ResizedView::with_full_screen(
                    Reserves::new().with_name("RESERVES"),
                ))
                .title("Reserves"),
            );

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
}
