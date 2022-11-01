use cursive::{theme::Color, Vec2, View};
use vrsc_rpc::json::ReserveCurrency;

pub struct ReserveTable {
    pub reserve_name: String,
    pub reserve_currencies: Vec<ReserveCurrency>,
}

// ----------------------- VRSC-GBP ----------------------- Price ----------- Weight
// iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq                001.00000000 | 0200000.00273192
impl View for ReserveTable {
    fn draw(&self, printer: &cursive::Printer) {
        // title draw:
        let eofp = dbg!(((printer.output_size.x.saturating_sub(26)) / 2)
            .saturating_sub(&self.reserve_name.len() / 2));
        let bolp = dbg!(eofp + &self.reserve_name.len());

        for i in 0..(eofp.saturating_sub(1)) {
            printer.print((i, 0), "-");
        }

        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print(
                (eofp.saturating_sub(1), 0),
                &format!(" {} ", &self.reserve_name),
            );
        });

        for i in (bolp + 1)..(printer.output_size.x.saturating_sub(26)) {
            printer.print((i, 0), "-");
        }
        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print((printer.output_size.x.saturating_sub(25), 0), "Price");
        });

        printer.print(
            (printer.output_size.x.saturating_sub(20), 0),
            " ----------- ",
        );

        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print((printer.output_size.x.saturating_sub(7), 0), "Weight");
        });

        let vrsc = self
            .reserve_currencies
            .iter()
            .find(|b| &b.currencyid.to_string() == "iJhCezBExJHvtyH3fGhNnt2NhU4Ztkf2yq")
            .unwrap();

        for (i, rc) in self.reserve_currencies.iter().enumerate() {
            printer.print((0, i + 1), &rc.currencyid.to_string());
            printer.print(
                // 32 should be calcualted
                (printer.output_size.x.saturating_sub(32), i + 1),
                &format!(
                    "{:012.8} | {:016.8}",
                    rc.reserves.as_vrsc() / vrsc.reserves.as_vrsc(),
                    rc.reserves.as_vrsc()
                ),
            );
        }
    }

    // when drawing this table, i need to know how many currencies to show in order to calculate the required height of the view.
    // that means that this table needs to have state on which currencies it should show.
    // that means that the initiation of this table should accept a list of currencies.
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        // account for filter?
        Vec2::new(3, self.reserve_currencies.len() + 2) // 1 for title, 1 for blank space below
    }

    fn needs_relayout(&self) -> bool {
        false
    }
}

impl ReserveTable {
    pub fn new(name: String, reserve_currencies: Vec<ReserveCurrency>) -> Self {
        ReserveTable {
            reserve_name: name,
            reserve_currencies,
        }
    }
}
