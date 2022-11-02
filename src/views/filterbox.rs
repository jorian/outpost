use cursive::{view::ViewWrapper, views::Checkbox, wrap_impl, View};

pub struct FilterBox {
    pub name: String,
    pub checkbox: Checkbox,
}

impl FilterBox {
    pub fn new(label: String) -> Self {
        FilterBox {
            name: label,
            checkbox: Checkbox::new(),
        }
    }
}

impl ViewWrapper for FilterBox {
    wrap_impl!(self.checkbox: Checkbox);

    fn wrap_draw(&self, printer: &cursive::Printer) {
        self.checkbox.draw(printer);
        printer.print((4, 0), &self.name);
    }
}
