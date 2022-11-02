use std::sync::mpsc::Sender;

use cursive::{view::ViewWrapper, views::Checkbox, wrap_impl, View};
use vrsc_rpc::json::ReserveCurrency;

use crate::controller::ControllerMessage;

pub struct FilterBox {
    pub reserve_currency: ReserveCurrency,
    pub checkbox: Checkbox,
}

impl FilterBox {
    pub fn new(reserve_currency: ReserveCurrency, c_tx: Sender<ControllerMessage>) -> Self {
        FilterBox {
            reserve_currency,
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
        printer.print((4, 0), &self.reserve_currency.currencyid.to_string());
    }
}
