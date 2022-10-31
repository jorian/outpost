use cursive::{view::Resizable, Vec2, View, XY};
use tracing::debug;

pub struct ReserveTable {
    pub reserve_name: String,
}

impl View for ReserveTable {
    fn draw(&self, printer: &cursive::Printer) {
        // debug!("{:?}", printer.output_size);

        debug!("{:?}", printer.output_size);

        let eofp = dbg!((printer.output_size.x / 2) - (&self.reserve_name.len() / 2));
        let bolp = dbg!(eofp + &self.reserve_name.len());

        for i in 0..(eofp - 1) {
            printer.print((i, 0), "-");
        }

        printer.print((eofp - 1, 0), &format!(" {} ", &self.reserve_name));

        for i in (bolp + 1)..printer.output_size.x {
            printer.print((i, 0), "-");
        }

        printer.print((0, 1), "0");
    }

    fn required_size(&mut self, constraint: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(3, 3)
    }

    fn needs_relayout(&self) -> bool {
        false
    }
}

impl ReserveTable {
    pub fn new(name: String) -> Self {
        ReserveTable { reserve_name: name }
    }
}
