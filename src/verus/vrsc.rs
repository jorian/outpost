use std::collections::HashMap;

use vrsc_rpc::{json::vrsc::Address, Client, RpcApi};

use super::Chain;

pub struct VerusChain {
    testnet: bool,
    name: String,
    currencyidhex: String,
    client: Client,
    id_names: HashMap<Address, String>,
}

impl Chain for VerusChain {
    fn testnet(&self) -> bool {
        self.testnet
    }

    fn currencyidhex(&self) -> String {
        self.currencyidhex.clone()
    }

    fn set_name(&mut self) {}

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn currency_id_to_name(&mut self, currency_id: vrsc_rpc::json::vrsc::Address) -> String {
        match self.id_names.get(&currency_id) {
            Some(value) => return value.to_owned(),
            None => {
                let value = self
                    .client()
                    .get_currency(&currency_id.to_string())
                    .unwrap()
                    .fullyqualifiedname;

                self.id_names.insert(currency_id.clone(), value);
                self.id_names.get(&currency_id).unwrap().to_owned()
            }
        }
    }
}

impl VerusChain {
    pub fn new(testnet: bool) -> Self {
        if !testnet {
            unimplemented!()
        }

        let client = Client::vrsc(testnet, vrsc_rpc::Auth::ConfigFile).unwrap();

        VerusChain {
            testnet,
            name: "vrsctest".to_string(),
            currencyidhex: "2d4eb6919e9fdb2934ff2481325e6335a29eefa6".to_string(),
            client,
            id_names: HashMap::new(),
        }
    }
}
