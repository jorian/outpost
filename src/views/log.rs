use std::{collections::VecDeque, sync::mpsc};

use cursive::{Printer, Vec2, View};
use vrsc_rpc::json::vrsc::Amount;

pub struct LogView {
    buffer: VecDeque<LogMessage>,
    rx: mpsc::Receiver<LogMessage>,
}

impl LogView {
    pub fn new(rx: mpsc::Receiver<LogMessage>) -> Self {
        let buffer = VecDeque::new();

        LogView { buffer, rx }
    }

    fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(message) = self.rx.try_recv() {
            self.buffer.push_back(message);

            if self.buffer.len() > 500 {
                self.buffer.pop_front();
            }
        }
    }
}

impl View for LogView {
    fn layout(&mut self, _: Vec2) {
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        let mut counter = 0;
        for message in self.buffer.iter().rev() {
            counter += message.height();

            let mut linenum = 1;
            if let Some(amount_out) = message.amount_out {
                printer.print(
                    (0, printer.size.y - 1 - linenum - counter + message.height()),
                    &format!("{}", amount_out),
                );
                linenum += 1;
            }

            printer.print(
                (0, printer.size.y - 1 - linenum - counter + message.height()),
                &format!("{}", message.amount_in),
            );
            linenum += 1;

            printer.print(
                (0, printer.size.y - 1 - linenum - counter + message.height()),
                &format!(
                    "{} transfer initiated for {} ",
                    message.time, message.reserve
                ),
            );

            linenum += 1;
            printer.print(
                (0, printer.size.y - 1 - linenum - counter + message.height()),
                "-------------------",
            )

            // match message._type {
            //     MessageType::Initiate => {
            //         printer.print(
            //             (0, printer.size.y - 1 - i),
            //             &format!(
            //                 "{} transfer initiated: {} | amount in: {} {}",
            //                 message.time,
            //                 message.reserve.as_str().trim_matches('"'),
            //                 message.amount_in.as_vrsc(),
            //                 message.amount_currency
            //             ),
            //         );
            //     }
            //     MessageType::Settle => {
            //         printer.print(
            //             (0, printer.size.y - 1 - i),
            //             &format!(
            //                 "{} transfer settled: {} | amount in: {}",
            //                 message.time,
            //                 message.reserve.as_str().trim_matches('"'),
            //                 message.amount_in
            //             ),
            //         );
            //     }
            // }
        }
    }
}

#[derive(Clone)]
pub struct LogMessage {
    pub time: String,
    pub _type: MessageType,
    pub reserve: String,
    pub amount_currency: String,
    pub amount_in: Amount,
    pub amount_out: Option<f64>,
}

impl LogMessage {
    pub fn height(&self) -> usize {
        if self.amount_out.is_some() {
            5
        } else {
            4
        }
    }
}

#[derive(Clone)]
pub enum MessageType {
    Initiate,
    Settle,
}

// impl View for LogMessage {}
