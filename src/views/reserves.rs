use cursive::{
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

    pub fn update(&mut self, baskets: Vec<Basket>, _checked_currencies: Vec<Currency>) {
        info!("{} baskets retrieved", baskets.len());

        self.view.get_inner_mut().clear();
        self.view.get_inner_mut().add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                for basket in baskets.into_iter() {
                    ll.add_child(ReserveTable::new(basket));
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
