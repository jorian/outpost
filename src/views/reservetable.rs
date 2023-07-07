use cursive::{theme::Color, Vec2, View};
use tracing::debug;
use vrsc_rpc::json::vrsc::Denomination;

use crate::verus::Basket;

pub struct ReserveTable {
    pub basket: Basket,
}

// -- papa.v2 -------------------------------------------------------------------------------------------- Price ------------ Weight
// VRSCTEST                                                                                  1756739166.70826087 |      202.02656542
// v2                                                                                       11110314722.06983757 | 44443705.11234567
impl View for ReserveTable {
    fn draw(&self, printer: &cursive::Printer) {
        let biggest_number_weight = self
            .basket
            .currency_state
            .reservecurrencies
            .iter()
            .map(|rc| {
                rc.reserves
                    .as_sat()
                    .to_string()
                    .chars()
                    .collect::<Vec<_>>()
                    .len()
            })
            .max()
            .unwrap()
            + 1;

        debug!("{:#?}", self.basket.currency_state.reservecurrencies);
        debug!(
            "biggest number weight {}: {}",
            self.basket.name, biggest_number_weight
        );

        let biggest_number_price = self
            .basket
            .currency_state
            .reservecurrencies
            .iter()
            .map(|rc| {
                rc.priceinreserve
                    .as_sat()
                    .to_string()
                    .chars()
                    .collect::<Vec<_>>()
                    .len()
            })
            .max()
            .unwrap()
            + 1;

        debug!("biggest number price: {}", biggest_number_price);

        // title draw:
        // two dashes:
        // printer.print((0, 0), "     ");

        let supply = &self.basket.currency_state.supply;
        let str_supply = supply.to_string_in(Denomination::Verus);

        let bolp = &self.basket.name.len() + str_supply.len() + 8;

        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print(
                (0, 0),
                &format!(" -- {} ({})", &self.basket.name, str_supply),
            );
        });

        for i in (bolp)
            ..(printer
                .output_size
                .x
                .saturating_sub(biggest_number_weight + 10))
        {
            printer.print((i, 0), "-");
        }
        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print(
                (
                    printer
                        .output_size
                        .x
                        .saturating_sub(biggest_number_weight + 9),
                    0,
                ),
                "Price",
            );
        });

        for i in printer
            .output_size
            .x
            .saturating_sub(biggest_number_weight + 3)
            ..printer.output_size.x.saturating_sub(7)
        {
            printer.print((i, 0), "-");
        }

        printer.with_color(Color::from_256colors(32).into(), |printer| {
            printer.print((printer.output_size.x.saturating_sub(8), 0), " Amount ");
        });

        for (i, rc) in self
            .basket
            .currency_state
            .reservecurrencies
            .iter()
            .enumerate()
        {
            printer.print(
                (0, i + 1),
                &format!(
                    " {}",
                    self.basket
                        .currencynames
                        .get(&rc.currencyid)
                        .unwrap_or(&rc.currencyid.to_string()),
                ),
            );

            printer.print(
                (
                    printer
                        .output_size
                        .x
                        .saturating_sub(biggest_number_weight + 4 + biggest_number_price),
                    i + 1,
                ),
                &format!(
                    "{number:prec$.8}",
                    prec = biggest_number_price,
                    number = rc.priceinreserve.as_vrsc()
                ),
            );

            printer.print(
                (
                    printer
                        .output_size
                        .x
                        .saturating_sub(biggest_number_weight + 4),
                    i + 1,
                ),
                &format!(
                    " | {number:prec$.8}",
                    prec = biggest_number_weight,
                    number = rc.reserves.as_vrsc()
                ),
            );
        }
    }
    // when drawing this table, i need to know how many currencies to show in order to calculate the required height of the view.
    // that means that this table needs to have state on which currencies it should show.
    // that means that the initiation of this table should accept a list of currencies.
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        // account for filter?
        Vec2::new(3, self.basket.currency_state.reservecurrencies.len() + 2) // 1 for title, 1 for blank space below
    }

    fn needs_relayout(&self) -> bool {
        false
    }
}

impl ReserveTable {
    pub fn new(basket: Basket) -> Self {
        ReserveTable { basket }
    }
}
