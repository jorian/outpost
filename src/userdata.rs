use std::{
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

use os_info::Type as OSType;
use tracing::{error, info};

pub struct UserData {
    pub pbaas_chains: Vec<PBaaSChain>,
}

impl UserData {
    pub fn new(testnet: bool) -> Self {
        let mut pbaas_chains = get_local_pbaas_chains();

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
    pub fn new<S: Into<String>>(name: Option<S>, currencyidhex: Option<S>) -> Self {
        PBaaSChain {
            name: name.map(|n| n.into()),
            currencyidhex: currencyidhex.map(|c| c.into()),
            zmqhashblock: None,
            zmqhashtx: None,
        }
    }
}

fn functional_pbaas_chains() -> Vec<PBaaSChain> {
    pbaas_dir_entries()
        .filter_map(|d| d.ok())
        .map_while(|pbaas_dir| {
            // pbaas_dir is the dir we need the name of to compare it later on with the .conf file
            std::fs::read_dir(pbaas_dir.path())
                .ok()
                .and_then(|mut pbaas_dir_files| {
                    pbaas_dir_files.find_map(|file_result| {
                        file_result.ok().and_then(|file| {
                            dbg!(&file.file_name());
                            if file.file_name() == pbaas_dir.file_name() {
                                Some(PBaaSChain::new(
                                    None, // the fullyqualifiedname of this chain is not retrievable at this point.
                                    // TODO consider using currencyidhex as leading name and add real name during runtime where there is unicode support.
                                    PathBuf::new()
                                        .file_stem()
                                        .map(|name| name.to_string_lossy().into_owned()), // VRSC makes an OS safe string called currencyidhex, so this never fails.
                                ))
                            } else {
                                None
                            }
                        })
                    })
                })
        })
        .collect()
}

fn get_local_pbaas_chains() -> Vec<PBaaSChain> {
    let mut installed_chains = vec![];

    let dir = pbaas_dir_entries();

    dir.into_iter()
        .filter_map(|d| d.ok())
        .for_each(|pbaas_dir| {
            for pbaas_dir_file in std::fs::read_dir(pbaas_dir.path()) {
                // pbaas_dir_file
                //     .filter_map(|i| {
                //         i.ok().and_then(|file| {
                //             if let Some(filename) = file.file_name().to_str() {
                //                 if filename
                //                     == format!("{}.conf", pbaas_dir.file_name().to_str().unwrap())
                //                 {
                //                     let pathbuf = PathBuf::from(&file.path());

                //                     if pathbuf.exists() {
                //                         // installed_chains.push(
                //                         Some(PBaaSChain::new(
                //                             None, // the fullyqualifiedname of this chain is not retrievable at this point.
                //                             // TODO consider using currencyidhex as leading name and add real name during runtime where there is unicode support.
                //                             pathbuf
                //                                 .file_stem()
                //                                 .map(|name| name.to_string_lossy().into_owned()), // VRSC makes an OS safe string called currencyidhex, so this never fails.
                //                         ))
                //                         // )
                //                     } else {
                //                         None
                //                     }
                //                 } else {
                //                     None
                //                 }
                //             } else {
                //                 None
                //             }
                //         })
                //     })
                // .collect()

                for i in pbaas_dir_file.into_iter() {
                    // go over all the contents of each folder
                    if let Ok(direntry) = i {
                        match direntry.file_name().to_str() {
                            Some(filename) => {
                                if filename
                                    == format!(
                                        "{}.conf",
                                        &pbaas_dir.file_name().into_string().unwrap()
                                    )
                                    .as_str()
                                {
                                    let pathbuf = PathBuf::from(&direntry.path());

                                    if pathbuf.exists() {
                                        installed_chains.push(PBaaSChain::new(
                                            None, // the fullyqualifiedname of this chain is not retrievable at this point.
                                            // TODO consider using currencyidhex as leading name and add real name during runtime where there is unicode support.
                                            pathbuf
                                                .file_stem()
                                                .map(|name| name.to_string_lossy().into_owned()), // VRSC makes an OS safe string called currencyidhex, so this never fails.
                                        ));
                                    }
                                }
                            }
                            None => {} // currencyidhex will always be OS safe strings
                        }
                    }
                }
            }
        });
    // .collect::<Vec<PBaaSChain>>();

    installed_chains
}

fn pbaas_dir_entries() -> ReadDir {
    if let Some(verustest_path) = pbaas_dir_location(true) {
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
        dbg!(functional_pbaas_chains());
    }
}
