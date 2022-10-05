pub mod controller;
pub mod ui;
pub mod verus;

use controller::Controller;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

fn main() {
    logging_setup();

    let data = Arc::new(());

    let mut controller = Controller::new(Arc::clone(&data));

    controller.start();
}

fn logging_setup() {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    let _ = color_eyre::install();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
