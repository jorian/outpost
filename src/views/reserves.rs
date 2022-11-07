use cursive::{view::ViewWrapper, views::*, View};
use tracing::info;
use vrsc_rpc::json::Currency;

use crate::{verus::Basket, views::reservetable::ReserveTable};

pub struct Reserves {
    view: LinearLayout,
    baskets: Vec<Basket>,
    checked_currencies: Vec<Currency>,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal(),
            baskets: vec![],
            checked_currencies: vec![],
        }
    }

    pub fn update_baskets(&mut self, baskets: Vec<Basket>) {
        info!("{} baskets retrieved", baskets.len());
        self.baskets = baskets;
    }

    pub fn update_checked_currencies(&mut self, checked_currencies: Vec<Currency>) {
        self.checked_currencies = checked_currencies;
    }

    pub fn update_view(&mut self) {
        self.view.clear();
        self.view.add_child(
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
            }), // .full_width(),
        );
    }
}

impl ViewWrapper for Reserves {
    cursive::wrap_impl!(self.view: LinearLayout);
}
