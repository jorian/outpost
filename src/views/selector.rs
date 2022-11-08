use std::sync::mpsc::Sender;

use cursive::{
    view::{Nameable, ViewWrapper},
    views::*,
    View,
};
use tracing::debug;
use vrsc_rpc::json::Currency;

use crate::controller::ControllerMessage;

use super::filterbox::FilterBox;

pub struct Selector {
    view: LinearLayout,
    c_tx: Sender<ControllerMessage>,
}

impl Selector {
    pub fn new(c_tx: Sender<ControllerMessage>) -> impl View {
        Selector {
            view: LinearLayout::vertical(),
            c_tx,
        }
    }

    pub fn update(&mut self, reserve_currencies: Vec<Currency>) {
        debug!("update selector overview");

        self.view.clear();
        for rc in reserve_currencies {
            self.view
                .add_child(FilterBox::new(rc, self.c_tx.clone()).with_name("filterbox"));
        }
    }
}

impl ViewWrapper for Selector {
    cursive::wrap_impl!(self.view: LinearLayout);
}
