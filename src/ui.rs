use std::{collections::BTreeMap, sync::mpsc};

use cursive::{
    view::{Nameable, Resizable},
    views::{LinearLayout, Panel},
    CursiveRunnable, CursiveRunner,
};
use tracing::debug;
use vrsc_rpc::json::Currency;

use crate::{
    controller::ControllerMessage,
    menu::BasketMode,
    verus::Basket,
    views::{
        filterbox::FilterBox,
        log::{LogMessage, LogView},
        pbaas_dialog::PbaasDialog,
        reserves::Reserves,
        selector::Selector,
        tvl::TVL,
    },
};

pub type UIReceiver = mpsc::Receiver<UIMessage>;
pub type UISender = mpsc::Sender<UIMessage>;

pub struct UI {
    pub siv: CursiveRunner<CursiveRunnable>,
    ui_rx: UIReceiver,
    pub ui_tx: UISender,
}

impl UI {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>, l_rx: mpsc::Receiver<LogMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();
        let mut siv = cursive::ncurses().into_runner();
        siv.update_theme(|theme| theme.shadow = false);

        let c_tx_clone = c_tx.clone();

        siv.add_global_callback('p', move |s| {
            debug!("p was pressed");
            if s.find_name::<PbaasDialog>("pbaas_dialog").is_some() {
                debug!("pbaas dialog found, ignore");
            } else {
                c_tx_clone
                    .send(ControllerMessage::PBaaSDialog(c_tx_clone.clone()))
                    .unwrap();
            }
        });

        crate::menu::set_menubar(&mut siv, c_tx.clone());
        siv.set_autohide_menu(false);

        let main_view = LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(
                        Panel::new(
                            LinearLayout::vertical().child(
                                Selector::new(c_tx.clone())
                                    .with_name("SELECTOR")
                                    .full_height(),
                            ),
                        )
                        .title("Selector")
                        .full_width(),
                    )
                    .child(
                        Panel::new(LogView::new(l_rx))
                            .title("Log")
                            .full_height()
                            .max_height(20),
                    ),
            )
            .child(
                Panel::new(Reserves::new().with_name("RESERVES"))
                    .title("Reserves")
                    .full_width(),
            )
            .child(Panel::new(TVL::new().with_name("TVL")).title("TVL"));

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
                    debug!("{:#?}", &baskets);

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
                    debug!("{:?}", &vec);
                    self.siv
                        .call_on_name("SELECTOR", |selector_view: &mut Selector| {
                            selector_view.update(vec);
                        });
                }
                UIMessage::UpdateTLV(hm) => {
                    let cb_sink = self.siv.cb_sink().clone();
                    cb_sink
                        .send(Box::new(|s| {
                            s.call_on_name("TVL", |tvl_view: &mut TVL| tvl_view.update(hm));
                        }))
                        .unwrap();
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

                            // debug!("{:?}", &checked_currencies);

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
                UIMessage::PBaasDialog(c_tx, labels) => {
                    let cb_sink = self.siv.cb_sink().clone();
                    cb_sink
                        .send(Box::new(|s| {
                            s.add_layer(PbaasDialog::new(c_tx, labels).with_name("pbaas_dialog"));
                        }))
                        .unwrap();
                }
                UIMessage::BasketModeChange(basket_mode) => {
                    let cb_sink = self.siv.cb_sink().clone();
                    cb_sink
                        .send(Box::new(|s| {
                            s.call_on_name("RESERVES", |view: &mut Reserves| {
                                view.update_basket_mode(basket_mode);

                                view.update_view();
                            });
                        }))
                        .unwrap();
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
    UpdateTLV(BTreeMap<String, f64>),
    ApplyFilter,
    NewLog(String),
    PBaasDialog(mpsc::Sender<ControllerMessage>, Vec<String>),
    BasketModeChange(BasketMode),
}
