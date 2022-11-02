use std::sync::mpsc::Sender;

use cursive::{view::ViewWrapper, views::Checkbox, wrap_impl, View};

use crate::{controller::ControllerMessage, verus::Reserve};

use super::reserves::Reserves;

pub struct FilterBox {
    pub name: String,
    pub checkbox: Checkbox,
}

impl FilterBox {
    pub fn new(label: String, c_tx: Sender<ControllerMessage>) -> Self {
        FilterBox {
            name: label,
            checkbox: Checkbox::new().on_change(move |_, _| {
                c_tx.send(ControllerMessage::CurrencySelectionChange)
                    .unwrap();
            }),
        }
    }
}

impl ViewWrapper for FilterBox {
    wrap_impl!(self.checkbox: Checkbox);

    fn wrap_draw(&self, printer: &cursive::Printer) {
        self.checkbox.draw(printer);
        printer.print((4, 0), &self.name);
    }
}
