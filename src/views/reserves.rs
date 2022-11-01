use std::sync::Arc;

use cursive::{
    theme::{Color, Style},
    utils::markup::StyledString,
    view::{Resizable, ViewWrapper},
    views::*,
    View,
};
use tracing::{debug, info};

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

    pub fn update(&mut self, baskets: Arc<Vec<Basket>>) {
        info!("{} baskets retrieved", baskets.len());
        debug!("{:?}", baskets);

        self.view.get_inner_mut().clear();
        self.view.get_inner_mut().add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                for basket in baskets.iter() {
                    ll.add_child(ReserveTable::new(
                        basket.name.clone(),
                        basket.currency_state.reservecurrencies.clone(),
                    ));
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

fn to_styled_string(s: &str) -> StyledString {
    let sum: u32 = s[0..2].bytes().fold(0, |acc, sum| acc + sum as u32);
    let mut ss = StyledString::new();
    ss.append_styled(
        s,
        Style::from(Color::from_256colors(233 + (sum % 15) as u8)),
    );

    ss
}
