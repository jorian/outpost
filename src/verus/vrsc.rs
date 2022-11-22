use os_info::Type as OSType;
use std::{collections::HashMap, path::PathBuf};
use vrsc_rpc::{json::vrsc::Address, Client, RpcApi};

use super::{read_config_contents, Chain};

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

    fn get_config_dir(&self) -> PathBuf {
        let mut path;
        match self.testnet {
            true => {
                path = get_komodo_installation_folder();
                path.push("vrsctest");
            }
            false => {
                path = get_komodo_installation_folder();
                path.push("VRSC");
            }
        }

        path
    }

    fn get_config_file(&self) -> HashMap<String, String> {
        let config_file_path = match self.testnet {
            true => {
                let mut path = self.get_config_dir();
                path.push("vrsctest.conf");

                path
            }
            false => {
                let mut path = self.get_config_dir();
                path.push("VRSC.conf");

                path
            }
        };

        read_config_contents(&config_file_path)
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

fn get_komodo_installation_folder() -> PathBuf {
    let mut full_path = PathBuf::new();
    match os_info::get().os_type() {
        OSType::Ubuntu | OSType::Linux | OSType::Debian | OSType::OracleLinux => {
            if let Some(path) = dirs::home_dir() {
                full_path.push(path);
                full_path.push(".komodo");
            }
        }
        OSType::Macos | OSType::Windows => {
            if let Some(path) = dirs::data_local_dir() {
                full_path.push(path);
                full_path.push("Komodo")
            }
        }
        _ => panic!("OS not supported"),
    }

    if !full_path.is_dir() {
        panic!("config dir is not correct");
    }

    full_path
}
