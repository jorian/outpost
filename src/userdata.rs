use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    str::FromStr,
};

use os_info::Type as OSType;
use tracing::error;
use url::Url;

// pub struct UserData {
//     pub pbaas_chains: Vec<PBaaSChain>,
// }

// impl UserData {
//     pub fn new(testnet: bool) -> Self {
//         UserData { pbaas_chains }
//     }
// }

/// Every PBaaS chain has at least a currencyidhex. It is the only identifier to determine locally installed chains,
/// as it is used to denote the data directory for that PBaaS chain.
///
/// With currencyidhex we are able to find the necessary information.
#[derive(Debug, Clone)]
pub struct PBaaSChain {
    testnet: bool,
    pub name: Option<String>,
    pub currencyidhex: String,
    config_dir: PathBuf,
    pub zmqhashblock: Option<u16>,
    pub zmqhashtx: Option<u16>,
}

impl PBaaSChain {
    pub fn new<S: Into<String>>(currencyidhex: S, testnet: bool) -> Self {
        let currencyidhex = currencyidhex.into();

        let config_dir = get_config_dir(&currencyidhex);

        PBaaSChain {
            testnet,
            name: None,
            currencyidhex: currencyidhex,
            config_dir,
            zmqhashblock: None,
            zmqhashtx: None,
        }
    }

    pub fn set_zmq_ports(&mut self) -> io::Result<()> {
        match self.testnet {
            true => {
                if let Some(mut pbaas_dir) = pbaas_dir_location(true) {
                    pbaas_dir.push(&self.currencyidhex);
                    pbaas_dir.push(&format!("{}.conf", &self.currencyidhex));
                    if pbaas_dir.exists() {
                        // fetch zmq from configfile
                        let contents = dbg!(fs::read_to_string(pbaas_dir.to_str().unwrap())?);

                        let map: HashMap<String, String> = contents
                            .as_str()
                            .split('\n')
                            .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
                            .filter(|vec| vec.len() == 2)
                            .map(|vec| (vec[0].to_string(), vec[1].to_string()))
                            .collect::<HashMap<String, String>>();

                        dbg!(&map);

                        let zmqhashblock = map.get("zmqpubhashblock").ok_or(ErrorKind::NotFound)?;
                        let zmqhashtx = map.get("zmqpubhashtx").ok_or(ErrorKind::NotFound)?;

                        if let Ok(zmqhashblockurl) = Url::from_str(zmqhashblock) {
                            self.zmqhashblock = zmqhashblockurl.port();
                        } else {
                            return Err(ErrorKind::NotFound.into());
                        }

                        if let Ok(zmqhashtxurl) = Url::from_str(zmqhashtx) {
                            self.zmqhashtx = zmqhashtxurl.port();
                        } else {
                            return Err(ErrorKind::NotFound.into());
                        }
                    } else {
                        return Err(ErrorKind::NotFound.into());
                    }
                }

                dbg!(&self);

                Ok(())
            }
            false => {
                unimplemented!()
            }
        }
    }
}

fn get_config_dir(currencyidhex: &str) -> PathBuf {
    if let Some(mut pbaas_dir) = pbaas_dir_location(true) {
        pbaas_dir.push(&currencyidhex);

        return pbaas_dir;
    } else {
        panic!("no config dir found");
    }
}

fn get_config_file(currencyidhex: &str) -> PathBuf {
    let mut config_dir = get_config_dir(currencyidhex);
    config_dir.push(&format!("{}.conf", &currencyidhex));

    config_dir
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
            PBaaSChain::new(currencyidhex.to_string_lossy().to_owned(), testnet)
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

    #[test]
    fn zmq_port_retrieval() {
        let mut local_chains = local_pbaas_chains(true);
        local_chains.iter_mut().for_each(|chain| {
            if let Err(_) = chain.set_zmq_ports() {
                error!("could not set zmq ports {}", chain.currencyidhex)
            }
        })
    }
}
