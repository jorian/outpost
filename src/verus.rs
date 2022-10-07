use vrsc_rpc::{Auth, Client};

pub struct Basket {
    name: String,
}

pub trait Currency: Send {
    fn name(&self) -> String;
}

pub struct Reserve {
    name: String,
}

impl Currency for Reserve {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Currency for Basket {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn get_currencies() -> Vec<Box<dyn Currency>> {
    let client = Client::chain("vrsctest", Auth::ConfigFile, None).unwrap();

    // client.

    vec![]
}
