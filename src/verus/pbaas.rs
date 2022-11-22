use os_info::Type as OSType;
use std::{collections::HashMap, fs::ReadDir, path::PathBuf};
use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

use super::{read_config_contents, Chain};

pub struct PBaaSChain {
    testnet: bool,
    name: Option<String>, // the name of a PBaaSChain can only be retrieved by querying a verus daemon at runtime
    currencyidhex: String,
    client: Client,
    id_names: HashMap<Address, String>,
}

impl Chain for PBaaSChain {
    fn get_config_dir(&self) -> PathBuf {
        if let Some(mut pbaas_dir) = pbaas_dir_location(true) {
            pbaas_dir.push(&self.currencyidhex);

            return pbaas_dir;
        } else {
            panic!("no config dir found");
        }
    }

    fn get_config_file(&self) -> HashMap<String, String> {
        let mut config_dir = self.get_config_dir();
        config_dir.push(&format!("{}.conf", &self.currencyidhex));

        read_config_contents(&config_dir)
    }

    fn testnet(&self) -> bool {
        self.testnet
    }

    fn currencyidhex(&self) -> String {
        self.currencyidhex.clone()
    }

    fn set_name(&mut self) {
        if let Ok(response) = self.client.get_blockchain_info() {
            self.name = Some(response.name);
        }
    }

    fn get_name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => String::new(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn currency_id_to_name(&mut self, currency_id: Address) -> String {
        match self.id_names.get(&currency_id) {
            Some(value) => return value.to_owned(),
            None => {
                let value = self
                    .client
                    .get_currency(&currency_id.to_string())
                    .unwrap()
                    .fullyqualifiedname;

                self.id_names.insert(currency_id.clone(), value);
                self.id_names.get(&currency_id).unwrap().to_owned()
            }
        }
    }
}

impl PBaaSChain {
    pub fn new(testnet: bool, currencyidhex: String) -> Self {
        let client = Client::chain(testnet, &currencyidhex, Auth::ConfigFile).unwrap();
        // unwrap: we can unwrap this because a pbaas chain instance is only created when it is locally found.

        PBaaSChain {
            testnet,
            name: None,
            currencyidhex,
            client,
            id_names: HashMap::new(),
        }
    }
}

fn pbaas_dir_location(testnet: bool) -> Option<PathBuf> {
    match os_info::get().os_type() {
        OSType::Ubuntu | OSType::Linux => {
            if let Some(homedir) = dirs::home_dir() {
                if testnet {
                    Some(PathBuf::from(&format!(
                        "{}/.verustest/pbaas",
                        &homedir.to_str().unwrap()
                    )))
                } else {
                    unimplemented!()
                }
            } else {
                panic!("unsupported OS (home directory could not be found)")
            }
        }
        OSType::Macos | OSType::Windows => {
            if let Some(config_dir) = dirs::config_dir() {
                if testnet {
                    Some(PathBuf::from(&format!(
                        "{}/VerusTest/pbaas",
                        &config_dir.to_str().unwrap()
                    )))
                } else {
                    unimplemented!()
                }
            } else {
                panic!("unsupported OS (config directory could not be found")
            }
        }
        _ => None,
    }
}

/// Gets all the locally installed PBaaS chains.
/// Some assumptions have been made:
/// - the .verustest/VerusTest directory has not been edited by a user. It assumes that all the directories that are found in .verustest are
/// PBaaS chains. No guarantees can be given about each directory being an actual PBaaS chain.
pub fn local_pbaas_chains(testnet: bool) -> Vec<PBaaSChain> {
    pbaas_dir_entries(testnet)
        .filter_map(|d| d.ok())
        .map(|dir| {
            let currencyidhex = dir.file_name();
            PBaaSChain::new(testnet, currencyidhex.to_string_lossy().to_string())
        })
        .collect()
}

fn pbaas_dir_entries(testnet: bool) -> ReadDir {
    if let Some(verustest_path) = pbaas_dir_location(testnet) {
        if let Ok(dir) = verustest_path.read_dir() {
            return dir;
        } else {
            panic!("a .verustest directory was not found on this machine. Are there any PBaaS chains installed?");
        }
    } else {
        panic!("Could not determine the .verustest location for this operating system. Are you running a weird version of Shubuntu?")
    }
}
