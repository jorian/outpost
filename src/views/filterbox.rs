use std::sync::mpsc::Sender;

use cursive::{view::ViewWrapper, views::Checkbox, wrap_impl, Vec2, View};
use vrsc_rpc::json::Currency;

use crate::controller::ControllerMessage;

pub struct FilterBox {
    pub currency: Currency,
    pub checkbox: Checkbox,
}

impl FilterBox {
    pub fn new(currency: Currency, c_tx: Sender<ControllerMessage>) -> Self {
        FilterBox {
            currency,
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
        printer.print((4, 0), &self.currency.currencydefinition.name);
    }

    fn wrap_required_size(&mut self, req: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(4 + self.currency.currencydefinition.name.len(), 1)
    }
}
