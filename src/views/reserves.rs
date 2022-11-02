use cursive::{
    reexports::log::debug,
    view::{Resizable, ViewWrapper},
    views::*,
    View,
};
use tracing::info;
use vrsc_rpc::json::Currency;

use crate::{verus::Basket, views::reservetable::ReserveTable};

pub struct Reserves {
    view: ResizedView<LinearLayout>,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal().min_width(200),
        }
    }

    pub fn update(&mut self, baskets: Vec<Basket>, checked_currencies: Vec<Currency>) {
        info!("{} baskets retrieved", baskets.len());

        self.view.get_inner_mut().clear();
        self.view.get_inner_mut().add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                // apply the filter:
                for mut basket in baskets.into_iter() {
                    debug!("{:?}", &checked_currencies);
                    basket.currency_state.reservecurrencies.retain(|rc| {
                        checked_currencies
                            .iter()
                            .any(|cur| cur.currencydefinition.currencyid == rc.currencyid)
                            || rc.currencyid.to_string()
                                == "iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq".to_string()
                    });
                    // debug!("{:?}", &basket);
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
