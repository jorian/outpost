use std::collections::BTreeMap;

use cursive::{view::ViewWrapper, views::*, View};
use tracing::debug;
pub struct TVL {
    view: ScrollView<TextView>,
}

impl TVL {
    pub fn new() -> impl View {
        Self {
            view: ScrollView::new(TextView::new("0.0 VRSC")),
        }
    }

    pub fn update(&mut self, currencies: BTreeMap<String, f64>) {
        debug!("update TVL: {:#?}", currencies);

        let mut sorted_currencies = Vec::from_iter(currencies);
        sorted_currencies.sort_by(|currency_a, currency_b| {
            currency_a
                .0
                .to_lowercase()
                .cmp(&currency_b.0.to_lowercase())
        });

        self.view.get_inner_mut().set_content(format!(
            "{}",
            sorted_currencies
                .iter()
                .map(|(k, v)| format!(
                    " {:<max_name_len$}: {value:>max$.*} \n",
                    k,
                    8,
                    max_name_len = 17,
                    value = v,
                    max = 17
                ))
                .collect::<Vec<String>>()
                .join("")
        ));
    }
}

impl ViewWrapper for TVL {
    cursive::wrap_impl!(self.view: ScrollView<TextView>);
}
