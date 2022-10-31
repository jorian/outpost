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
    view: LinearLayout,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: LinearLayout::horizontal(),
        }
    }

    pub fn update(&mut self, baskets: Arc<Vec<Basket>>) {
        info!("{} baskets retrieved", baskets.len());
        debug!("{:?}", baskets);

        self.view.clear();
        self.view.add_child(
            ScrollView::new({
                let mut ll = LinearLayout::vertical();

                for _basket in baskets.iter() {
                    ll.add_child(ReserveTable::new(_basket.name.clone()))
                    // let vrsc = basket
                    //     .currency_state
                    //     .reservecurrencies
                    //     .iter()
                    //     .find(|b| &b.currencyid.to_string() == "iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq")
                    //     .unwrap(); // all currency baskets have VRSC in its reserves.

                    // ll.add_child(TextView::new({
                    //     let mut ss = StyledString::new();
                    //     ss.append_styled(
                    //         String::from(&basket.name),
                    //         Style::from(Color::from_256colors(32)),
                    //     );

                    //     ss
                    // }));
                    // for reserve_currency in &basket.currency_state.reservecurrencies {
                    //     ll.add_child(TextView::new(to_styled_string(&format!(
                    //         "{}: {:012.8} | {:016.8}",
                    //         reserve_currency.currencyid,
                    //         reserve_currency.reserves.as_vrsc() / vrsc.reserves.as_vrsc(),
                    //         reserve_currency.reserves.as_vrsc()
                    //     ))));
                    // }
                    // ll.add_child(DummyView {}.fixed_height(1));
                }

                ll
            })
            .full_width(),
        );
    }
}

impl ViewWrapper for Reserves {
    cursive::wrap_impl!(self.view: LinearLayout);
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
