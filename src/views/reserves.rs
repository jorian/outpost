use std::sync::Arc;

use cursive::{
    view::{Resizable, ViewWrapper},
    views::*,
    View,
};
use tracing::debug;

use crate::verus::Basket;

pub struct Reserves {
    view: LinearLayout,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal(),
        }
    }

    pub fn update(&mut self, baskets: Arc<Vec<Basket>>) {
        debug!("update reserves overview");
        // let new_baskets = verus::get_latest_baskets();

        // if let Ok(_baskets) = new_baskets {
        debug!("{:#?}", baskets);

        self.view.clear();
        self.view.add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();
                for basket in baskets.iter() {
                    ll.add_child(TextView::new(basket.to_string()));
                }

                ll
            })
            .full_width(),
        );

        // get filters from selector
        // show basket in overview
        // }
    }
}

impl ViewWrapper for Reserves {
    cursive::wrap_impl!(self.view: LinearLayout);
}
