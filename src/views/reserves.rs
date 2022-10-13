use cursive::{
    theme::{BaseColor, Color},
    utils::markup::StyledString,
    view::ViewWrapper,
    views::{LinearLayout, Panel},
    View,
};
use tracing::debug;

use crate::verus::{self, Basket};

pub struct Reserves {
    view: Panel<LinearLayout>,
    baskets: Vec<Basket>,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: Panel::new(LinearLayout::horizontal()).title("Reserves"),
            baskets: vec![],
        }
    }

    pub fn update(&mut self) {
        let new_baskets = verus::get_latest_baskets();

        if let Ok(_baskets) = new_baskets {
            debug!("{:?}", _baskets);
            // get filters from selector
            // show basket in overview
        }
    }
}

impl ViewWrapper for Reserves {
    cursive::wrap_impl!(self.view: Panel<LinearLayout>);
}

fn styled_string(text: &str, color: Color) -> StyledString {
    let mut s = StyledString::new();
    s.append(StyledString::styled(format!("{}", text), color));

    s
}
