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

        self.view.get_inner_mut().set_content(format!(
            "{}",
            currencies
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
