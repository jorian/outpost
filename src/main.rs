pub mod controller;
pub mod ui;
pub mod util;
pub mod verus;
pub mod views;

use controller::Controller;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

fn main() {
    logging_setup();

    let data = Arc::new(SessionData { testnet: true });

    let mut controller = Controller::new(Arc::clone(&data));

    controller.start();
}

fn logging_setup() {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    let _ = color_eyre::install();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "cursive=info,outpost=debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub struct SessionData {
    pub testnet: bool,
}
