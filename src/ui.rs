use std::sync::mpsc;

use cursive::{
    views::{Dialog, TextView},
    CursiveRunnable, CursiveRunner,
};

pub type UIReceiver = mpsc::Receiver<UIMessage>;
pub type UISender = mpsc::Sender<UIMessage>;

pub struct UI {
    pub siv: CursiveRunner<CursiveRunnable>,
    ui_rx: UIReceiver,
    pub ui_tx: UISender,
}

impl UI {
    pub fn new() -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UIMessage>();
        let mut siv = cursive::ncurses().into_runner();

        siv.add_layer(
            Dialog::new()
                .title("Hello")
                .content(TextView::new("Hello, world!"))
                .button("Quit", |siv| siv.quit()),
        );

        UI { siv, ui_rx, ui_tx }
    }

    pub fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UIMessage::UpdateReserveOverview => {}
            }
        }

        self.siv.run();

        true
    }
}

pub enum UIMessage {
    UpdateReserveOverview,
}
