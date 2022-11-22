pub mod configuration;
pub mod controller;
pub mod ui;
pub mod verus;
pub mod views;

use configuration::get_configuration;
use controller::Controller;
use tracing_subscriber::EnvFilter;

fn main() {
    let config = get_configuration().expect("failed to read configuration");

    logging_setup(); // TODO add RUST_LOG env to config

    let mut controller = Controller::new(config.testnet);

    controller.start();
}

fn logging_setup() {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    let _ = color_eyre::install();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "cursive=info,outpost=debug,vrsc-rpc=debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
