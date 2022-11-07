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
    // fn required_size(&mut self, constraint: Vec2) -> Vec2 {}

    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    // this simply prints the buffer every new draw, no append or something
    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        for (i, message) in self.buffer.iter().rev().take(printer.size.y).enumerate() {
            match message._type {
                MessageType::Initiate => {
                    printer.print(
                        (0, printer.size.y - 1 - i),
                        &format!(
                            "{} transfer initiated: {} | amount in: {} {}",
                            message.time,
                            message.reserve.as_str().trim_matches('"'),
                            message.amount_in.as_vrsc(),
                            message.amount_currency
                        ),
                    );
                }
                MessageType::Settle => {
                    printer.print(
                        (0, printer.size.y - 1 - i),
                        &format!(
                            "{} transfer settled: {} | amount in: {}",
                            message.time,
                            message.reserve.as_str().trim_matches('"'),
                            message.amount_in
                        ),
                    );
                }
            }
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

#[derive(Clone)]
pub enum MessageType {
    Initiate,
    Settle,
}

// impl View for LogMessage {}
