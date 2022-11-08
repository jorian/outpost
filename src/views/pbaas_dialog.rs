use std::sync::mpsc;

use cursive::{
    align::HAlign,
    view::ViewWrapper,
    views::{Dialog, LinearLayout, SelectView, TextView},
    wrap_impl, Cursive, View,
};
use tracing::debug;

use crate::{controller::ControllerMessage, userdata::Chain};

pub struct PbaasDialog {
    c_tx: mpsc::Sender<ControllerMessage>,
    view: Dialog,
}

impl PbaasDialog {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>, data: Vec<Chain>) -> Self {
        let mut view = Dialog::new();
        let mut sv = SelectView::new();
        for chain in data.into_iter() {
            sv.add_item(&chain.name, chain.clone());
        }

        let c_tx_clone = c_tx.clone();

        sv.set_on_submit(move |siv, item: &Chain| {
            debug!("selected {}", &item.name);
            c_tx_clone
                .send(ControllerMessage::ChainChange(item.clone()))
                .unwrap();
            siv.pop_layer();
        });

        view.set_content(sv.h_align(HAlign::Left));

        PbaasDialog { c_tx, view }
    }
}

impl ViewWrapper for PbaasDialog {
    wrap_impl!(self.view: Dialog);
}
