use std::{fs::ReadDir, path::PathBuf};

use os_info::Type as OSType;
use tracing::{error, info};

pub struct UserData {
    pub pbaas_chains: Vec<PBaaSChain>,
}

impl UserData {
    pub fn new(testnet: bool) -> Self {
        let mut pbaas_chains = local_pbaas_chains(testnet);

        if testnet {
            pbaas_chains.push(PBaaSChain {
                name: Some("VRSCTEST".to_string()),
                currencyidhex: None,
                zmqhashblock: None, // todo find on the fly
                zmqhashtx: None,
            });
        } else {
            unimplemented!()
        }

        UserData { pbaas_chains }
    }
}

#[derive(Debug, Clone)]
pub struct PBaaSChain {
    pub name: Option<String>,
    pub currencyidhex: Option<String>,
    pub zmqhashblock: Option<u16>,
    pub zmqhashtx: Option<u16>,
}

impl PBaaSChain {
    pub fn new<S: Into<String>>(currencyidhex: S) -> Self {
        PBaaSChain {
            name: None,
            currencyidhex: Some(currencyidhex.into()),
            zmqhashblock: None,
            zmqhashtx: None,
        }
    }
}

/// Gets all the locally installed PBaaS chains.
/// Some assumptions have been made:
/// - the .verustest/VerusTest directory has not been edited by a user. It assumes that all the directories that are found in .verustest are
/// PBaaS chains. No guarantees can be given about each directory being an actual PBaaS chain.
fn local_pbaas_chains(testnet: bool) -> Vec<PBaaSChain> {
    pbaas_dir_entries(testnet)
        .filter_map(|d| d.ok())
        .map(|dir| {
            let currencyidhex = dir.file_name();
            PBaaSChain::new(currencyidhex.to_string_lossy().to_owned())
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

// Finds and returns the location of the `pbaas` folder in which installations of pbaas chains are configured
// returns None if that directory or the hidden .verustest directory doesn't exist
//
// TODO add mainnet pbaas location
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(local_pbaas_chains(true));
    }
}
