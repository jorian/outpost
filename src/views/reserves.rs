use cursive::{
    theme::{BaseColor, Color},
    utils::markup::StyledString,
    view::ViewWrapper,
    views::{LinearLayout, Panel},
    View,
};

pub struct Reserves {
    view: Panel<LinearLayout>,
}

impl Reserves {
    pub fn new() -> impl View {
        Reserves {
            view: Panel::new(LinearLayout::horizontal()).title("Reserves"),
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
