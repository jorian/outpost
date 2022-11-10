use cursive::{
    view::{Resizable, ViewWrapper},
    views::*,
    View,
};
use tracing::{debug, info};
use vrsc_rpc::json::Currency;

use crate::{verus::Basket, views::reservetable::ReserveTable};

pub struct Reserves {
    view: ResizedView<LinearLayout>,
    baskets: Vec<Basket>,
    checked_currencies: Vec<Currency>,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal().min_width(100),
            baskets: vec![],
            checked_currencies: vec![],
        }
    }

    // #[instrument(level = "debug", skip(self))]
    pub fn update_baskets(&mut self, baskets: Vec<Basket>) {
        info!("{} baskets retrieved", baskets.len());
        self.baskets = baskets;
    }

    // #[instrument(level = "debug", skip(self))]
    pub fn update_checked_currencies(&mut self, checked_currencies: Vec<Currency>) {
        debug!("checked_currencies: {:?}", &checked_currencies);
        self.checked_currencies = checked_currencies;
    }

    pub fn update_view(&mut self) {
        self.view.get_inner_mut().clear();
        self.view.get_inner_mut().add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                for mut basket in self.baskets.clone().into_iter() {
                    basket.currency_state.reservecurrencies.retain(|rc| {
                        self.checked_currencies
                            .iter()
                            .any(|cur| cur.currencydefinition.currencyid == rc.currencyid)
                            || rc.currencyid.to_string()
                                == "iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq".to_string()
                    });
                    if basket.currency_state.reservecurrencies.len() > 1 {
                        ll.add_child(ReserveTable::new(basket));
                    }
                }

                ll
            })
            .full_width(),
        );
    }
}

impl ViewWrapper for Reserves {
    cursive::wrap_impl!(self.view: ResizedView<LinearLayout>);
}
