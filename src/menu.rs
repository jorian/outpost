use std::sync::mpsc;

use cursive::{menu::Tree, views::Dialog, Cursive};

use crate::controller::ControllerMessage;

pub fn set_menubar(siv: &mut Cursive, c_tx: mpsc::Sender<ControllerMessage>) {
    siv.menubar()
        .add_subtree(
            "File",
            Tree::new().leaf("Quit", |s| {
                s.add_layer(
                    Dialog::text("Do you really want to quit?")
                        .button("Yes", |s| s.quit())
                        .dismiss_button("No"),
                )
            }),
        )
        .add_subtree(
            "Edit",
            Tree::new().subtree(
                "Basket mode",
                Tree::new()
                    .leaf("Select", {
                        let c_tx = c_tx.clone();
                        move |_| {
                            let _ =
                                c_tx.send(ControllerMessage::BasketModeChange(BasketMode::Select));
                        }
                    })
                    .leaf("Full", {
                        let c_tx = c_tx.clone();
                        move |_| {
                            let _ =
                                c_tx.send(ControllerMessage::BasketModeChange(BasketMode::Full));
                        }
                    })
                    .leaf("Complete", |_| {}),
            ),
        );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasketMode {
    Select,
    Full,
}
