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
    view: ScrollView<LinearLayout>,
    c_tx: Sender<ControllerMessage>,
}

impl Selector {
    pub fn new(c_tx: Sender<ControllerMessage>) -> impl View {
        Selector {
            view: ScrollView::new(LinearLayout::vertical()),
            c_tx,
        }
    }

    pub fn update(&mut self, reserve_currencies: Vec<Currency>) {
        debug!("update selector overview");

        if !self.view.get_inner().is_empty() {
            self.view.get_inner_mut().set_focus_index(0).unwrap();
        }

        self.view.get_inner_mut().clear();
        for rc in reserve_currencies {
            self.view
                .get_inner_mut()
                .add_child(FilterBox::new(rc, self.c_tx.clone()).with_name("filterbox"));
        }
    }
}

impl ViewWrapper for Selector {
    cursive::wrap_impl!(self.view: ScrollView<LinearLayout>);
}
