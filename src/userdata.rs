use std::path::PathBuf;

use os_info::Type as OSType;
use tracing::info;

pub struct UserData {
    pub pbaas_chains: Vec<Chain>,
}

impl UserData {
    pub fn new(testnet: bool) -> Self {
        let mut pbaas_chains = get_local_pbaas_chains();

        if testnet {
            pbaas_chains.push(Chain {
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
pub struct Chain {
    pub name: Option<String>,
    pub currencyidhex: Option<String>,
    pub zmqhashblock: Option<u16>,
    pub zmqhashtx: Option<u16>,
}

impl Chain {
    pub fn new<S: Into<String>>(name: Option<S>, currencyidhex: Option<S>) -> Self {
        Chain {
            name: name.map(|n| n.into()),
            currencyidhex: currencyidhex.map(|c| c.into()),
            zmqhashblock: None,
            zmqhashtx: None,
        }
    }
}

fn get_local_pbaas_chains() -> Vec<Chain> {
    let mut installed_chains = vec![];

    //TODO refactor:
    if let Some(verustest_path) = pbaas_dir_location(true) {
        if let Ok(dir) = verustest_path.read_dir() {
            for entry in dir {
                if let Ok(entry) = entry {
                    if let Ok(pbaasentry) = PathBuf::from(&entry.path()).read_dir() {
                        for i in pbaasentry.into_iter() {
                            if let Ok(direntry) = i {
                                match direntry.file_name().to_str() {
                                    Some(filename) => {
                                        if filename
                                            == format!(
                                                "{}.conf",
                                                &entry.file_name().into_string().unwrap()
                                            )
                                            .as_str()
                                        {
                                            let pathbuf = PathBuf::from(&direntry.path());

                                            if pathbuf.exists() {
                                                installed_chains.push(Chain::new(
                                                    None, // the fullyqualifiedname of this chain is not retrievable at this point.
                                                    // TODO consider using currencyidhex as leading name and add real name during runtime where there is unicode support.
                                                    pathbuf.file_stem().map(|name| {
                                                        name.to_string_lossy().into_owned()
                                                    }), // VRSC makes an OS safe string called currencyidhex, so this never fails.
                                                ));
                                            }
                                        }
                                    }
                                    None => {} // currencyidhex will always be OS safe strings
                                }
                            }
                        }
                    }
                }
            }
        } else {
            info!("no pbaas directory found")
        }
    } else {
        info!("no .verustest directory found")
    }

    installed_chains
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
    use super::get_local_pbaas_chains;

    #[test]
    fn it_works() {
        dbg!(get_local_pbaas_chains());
    }
}
