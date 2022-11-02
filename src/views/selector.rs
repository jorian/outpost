use cursive::{view::ViewWrapper, views::*, View};
use tracing::debug;

use crate::verus::Basket;

pub struct Selector {
    view: LinearLayout,
    pub mode: SelectorMode,
}

impl Selector {
    pub fn new() -> impl View {
        Selector {
            view: LinearLayout::horizontal(),
            mode: SelectorMode::Baskets,
        }
    }

    pub fn update(&mut self, _reserve_currencies: Vec<Basket>) {
        debug!("update selector overview");
        // let new_baskets = verus::get_latest_baskets();

        // if let Ok(_baskets) = new_baskets {
        // debug!("{:#?}", reserve_currencies);

        // self.view.clear();
        // self.view.add_child(ScrollView::new(TextView::new(format!(
        //     "{:#?}",
        //     reserve_currencies
        // ))));

        // get filters from selector
        // show basket in overview
        // }
    }
}

impl ViewWrapper for Selector {
    cursive::wrap_impl!(self.view: LinearLayout);
}

#[derive(Clone, Copy)]
pub enum SelectorMode {
    Reserves,
    Baskets,
}
