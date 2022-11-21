use std::sync::mpsc;

use cursive::{
    align::HAlign,
    view::ViewWrapper,
    views::{Dialog, SelectView},
    wrap_impl,
};
use tracing::debug;

use crate::controller::ControllerMessage;

pub struct PbaasDialog {
    view: Dialog,
}

impl PbaasDialog {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>, data: Vec<String>) -> Self {
        let mut view = Dialog::new();
        let mut sv = SelectView::new();
        for chain in data.iter() {
            sv.add_item(chain, chain.to_string());
        }

        let c_tx_clone = c_tx.clone();

        sv.set_on_submit(move |siv, item: &str| {
            debug!("selected {:?}", &item);
            c_tx_clone
                .send(ControllerMessage::ChainChange(item.to_string()))
                .unwrap();
            siv.pop_layer();
        });

        view.set_content(sv.h_align(HAlign::Left));

        PbaasDialog { view }
    }
}

impl ViewWrapper for PbaasDialog {
    wrap_impl!(self.view: Dialog);
}
