use os_info::Type as OSType;
use std::{collections::HashMap, fs::ReadDir, path::PathBuf, rc::Rc};
use tracing::{debug, instrument, warn};
use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

use crate::controller::IdNames;

use super::{read_config_contents, Chain};

#[derive(Debug)]
pub struct PBaaSChain {
    testnet: bool,
    name: Option<String>, // the name of a PBaaSChain can only be retrieved by querying a verus daemon at runtime
    currencyidhex: String,
    client: Client,
    id_names: IdNames,
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
        if let Ok(mut write) = self.id_names.write() {
            write
                .entry(currency_id.to_string())
                .or_insert_with(|| {
                    self.client()
                        .get_currency(&currency_id.to_string())
                        .unwrap()
                        .fullyqualifiedname
                })
                .clone()
        } else {
            String::new()
        }
    }
}

impl PBaaSChain {
    #[instrument]
    pub fn new(testnet: bool, currencyidhex: String, id_names: IdNames) -> Self {
        dbg!(&currencyidhex);
        let client = Client::chain(testnet, &currencyidhex, Auth::ConfigFile).unwrap();
        // unwrap: we can unwrap this because a pbaas chain instance is only created when it is locally found.

        PBaaSChain {
            testnet,
            name: None,
            currencyidhex,
            client,
            id_names,
        }
    }
}

fn pbaas_dir_location(testnet: bool) -> Option<PathBuf> {
    debug!("{:?}", os_info::get().os_type());
    match os_info::get().os_type() {
        OSType::Ubuntu | OSType::Linux | OSType::Debian => {
            if let Some(homedir) = dirs::home_dir() {
                if testnet {
                    Some(PathBuf::from(&format!(
                        "{}/.verustest/pbaas",
                        &homedir.to_str().unwrap()
                    )))
                } else {
                    Some(PathBuf::from(&format!(
                        "{}/.verus/pbaas",
                        &homedir.to_str().unwrap()
                    )))
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
                    Some(PathBuf::from(&format!(
                        "{}/Verus/pbaas",
                        &config_dir.to_str().unwrap()
                    )))
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
/// - the PBaaS directory has not been edited by a user. It assumes that all the directories that are found in PBaaS directory are
/// PBaaS chains. No guarantees can be given about each directory being an actual PBaaS chain.
pub fn local_pbaas_chains(testnet: bool, id_names: IdNames) -> Vec<PBaaSChain> {
    if let Some(entries) = pbaas_dir_entries(testnet) {
        entries
            .filter_map(|d| d.ok())
            .map(|dir| {
                let currencyidhex = dir.file_name();
                PBaaSChain::new(
                    testnet,
                    currencyidhex.to_string_lossy().to_string(),
                    Rc::clone(&id_names),
                )
            })
            .collect()
    } else {
        vec![]
    }
}

fn pbaas_dir_entries(testnet: bool) -> Option<ReadDir> {
    if let Some(pbaas_path) = pbaas_dir_location(testnet) {
        pbaas_path.read_dir().ok()
    } else {
        warn!("Could not determine the PBaaS directory location for this operating system.");

        None
    }
}
