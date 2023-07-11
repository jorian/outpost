use cursive::{
    view::{Resizable, ViewWrapper},
    views::*,
    View,
};
use tracing::{debug, info};
use vrsc_rpc::json::Currency;

use crate::{menu::BasketMode, verus::Basket, views::reservetable::ReserveTable};

pub struct Reserves {
    view: ResizedView<LinearLayout>,
    baskets: Vec<Basket>,
    checked_currencies: Vec<Currency>,
    basket_mode: BasketMode,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal().min_width(100),
            baskets: vec![],
            checked_currencies: vec![],
            basket_mode: BasketMode::All,
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

    pub fn update_basket_mode(&mut self, b: BasketMode) {
        debug!("updating basket mode to {:?}", b);
        self.basket_mode = b;
    }

    pub fn update_view(&mut self) {
        self.view.get_inner_mut().clear();
        self.view.get_inner_mut().add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                debug!("basket mode: {:?}", self.basket_mode);

                match self.basket_mode {
                    BasketMode::All => {
                        let filtered_baskets = self.baskets.iter().filter(|basket| {
                            basket.currency_state.reservecurrencies.iter().any(|rc| {
                                self.checked_currencies
                                    .iter()
                                    .map(|c| &c.currencydefinition.currencyid)
                                    .cloned()
                                    .collect::<Vec<_>>()
                                    .contains(&rc.currencyid)
                            })
                        });

                        debug!("filtered_baskets: {:?}", filtered_baskets);

                        filtered_baskets.for_each(|b| ll.add_child(ReserveTable::new(b.clone())));
                    }
                    BasketMode::Selected => {
                        for mut basket in self.baskets.clone().into_iter() {
                            // apply the filter from selector
                            basket.currency_state.reservecurrencies.retain(|rc| {
                                self.checked_currencies.iter().any(|checked_currency| {
                                    checked_currency.currencydefinition.currencyid == rc.currencyid
                                        || rc.currencyid == basket.active_chain_id
                                })
                            });

                            debug!(
                                "basket.currency_state.reservecurrencies: {:?}",
                                basket.currency_state.reservecurrencies
                            );

                            if basket.currency_state.reservecurrencies.len() > 1 {
                                ll.add_child(ReserveTable::new(basket));
                            }
                        }
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
