pub mod pbaas;
pub mod vrsc;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::mpsc;

use tracing::{debug, error, info};
use vrsc_rpc::json::vrsc::Address;
use vrsc_rpc::json::Currency;
use vrsc_rpc::{Client, RpcApi};

use crate::controller::ControllerMessage;

pub trait Chain {
    fn get_name(&self) -> String;
    fn set_name(&mut self);
    fn get_config_dir(&self) -> PathBuf;
    fn get_config_file(&self) -> HashMap<String, String>;
    fn testnet(&self) -> bool;
    fn currencyidhex(&self) -> String;
    fn client(&self) -> &Client;

    fn start_zmq_tx_listener(&self, c_tx: mpsc::Sender<ControllerMessage>) {
        let config_file = self.get_config_file();
        if let Some(zmqpubhashtx) = config_file.get("zmqpubhashtx") {
            if let Ok(zmqpubhashtxurl) = url::Url::from_str(zmqpubhashtx) {
                if let Some(port) = zmqpubhashtxurl.port() {
                    let context = zmq::Context::new();

                    let socket = context.socket(zmq::SUB).expect("a new zmq socket");
                    let socket = socket;
                    socket
                        .connect(&format!("tcp://127.0.0.1:{}", port))
                        .expect("a connection to the zmq socket");
                    socket
                        .set_subscribe(b"hash")
                        .expect("failed subscribing to zmq");

                    info!(
                        "ZMQ listening for transactions on {} using port {}",
                        &self.get_name(),
                        port
                    );

                    let c_tx_clone = c_tx.clone();
                    let name = self.get_name().clone();

                    std::thread::spawn(move || loop {
                        let data = socket.recv_multipart(0).unwrap();
                        let hex = data[1]
                            .iter()
                            .map(|b| format!("{:02x}", *b))
                            .collect::<Vec<_>>()
                            .join("");

                        debug!("new tx: {}", &hex);

                        c_tx_clone
                            .send(ControllerMessage::NewTransaction(name.clone(), hex))
                            .unwrap();
                    });
                } else {
                    error!("zmqpubhashtx port missing in config file")
                }
            } else {
                error!("zmqpubhashtx missing in config file")
            }
        }
    }

    fn start_zmq_block_listener(&self, c_tx: mpsc::Sender<ControllerMessage>) {
        let config_file = self.get_config_file();
        if let Some(zmqpubhashblock) = config_file.get("zmqpubhashblock") {
            if let Ok(zmqpubhashblockurl) = url::Url::from_str(zmqpubhashblock) {
                if let Some(port) = zmqpubhashblockurl.port() {
                    let context = zmq::Context::new();

                    let socket = context.socket(zmq::SUB).expect("a new zmq socket");
                    let socket = socket;
                    socket
                        .connect(&format!("tcp://127.0.0.1:{}", port))
                        .expect("a connection to the zmq socket");
                    socket
                        .set_subscribe(b"hash")
                        .expect("failed subscribing to zmq");

                    info!(
                        "ZMQ listening for blocks on {} using port {}",
                        &self.get_name(),
                        port
                    );

                    let c_tx_clone = c_tx.clone();
                    let name = self.get_name().clone();

                    std::thread::spawn(move || loop {
                        let data = socket.recv_multipart(0).unwrap();
                        let hex = data[1]
                            .iter()
                            .map(|b| format!("{:02x}", *b))
                            .collect::<Vec<_>>()
                            .join("");

                        debug!("new block: {}", &hex);

                        c_tx_clone
                            .send(ControllerMessage::NewBlock(name.clone(), hex))
                            .unwrap();
                    });
                } else {
                    error!("zmqpubhashblock port missing in config file")
                }
            } else {
                error!("zmqpubhashblock missing in config file")
            }
        }
    }

    fn currency_id_to_name(&mut self, currency_id: Address) -> String;

    fn get_latest_currencies(&self) -> Result<Vec<Currency>, ()> {
        let currencies = self.client().list_currencies(None).unwrap();

        let mut filtered_currencies: Vec<Currency> = currencies
            .0
            .into_iter()
            .filter(|currency| [40, 264].contains(&currency.currencydefinition.options))
            .collect();

        let currencies = self.client().list_currencies(Some("imported")).unwrap();

        let mut pbaas_currencies = currencies
            .0
            .into_iter()
            .filter(|currency| [34, 40, 136, 264].contains(&currency.currencydefinition.options))
            .collect();

        filtered_currencies.append(&mut pbaas_currencies);
        let filtered_currencies = filtered_currencies
            .into_iter()
            .filter(|c| c.currencydefinition.currencyidhex != self.currencyidhex())
            .collect();

        Ok(filtered_currencies)
    }

    fn get_latest_baskets(&mut self) -> Result<Vec<Basket>, ()> {
        let currencies = self.client().list_currencies(None).unwrap();
        let active_chain_id = self.client().get_blockchain_info().unwrap();

        // A bridge has 2 sides, so we need to find out which sides in order to include the reserves in our baskets.
        // A bridge is defined on the subsystem and ties to the system it was launched from.
        let active_chain_filter = |currency: &Currency| {
            &currency.currencydefinition.systemid == &active_chain_id.chainid || {
                if let Some(launchsystemid) = currency.currencydefinition.launchsystemid.as_ref() {
                    *launchsystemid == active_chain_id.chainid
                } else {
                    true
                }
            }
        };

        let mut filtered_currencies: Vec<(String, Address)> = currencies
            .0
            .into_iter()
            .filter(|currency| [33, 35, 545].contains(&currency.currencydefinition.options))
            .filter(active_chain_filter)
            .map(|currency| {
                (
                    currency.currencydefinition.fullyqualifiedname,
                    currency.currencydefinition.currencyid,
                )
            })
            .collect();

        let imported_currencies = self.client().list_currencies(Some("imported")).unwrap();

        filtered_currencies.append(
            &mut imported_currencies
                .0
                .into_iter()
                .filter(|currency| currency.currencydefinition.options == 545)
                .filter(active_chain_filter)
                .map(|currency| {
                    (
                        currency.currencydefinition.fullyqualifiedname,
                        currency.currencydefinition.currencyid,
                    )
                })
                .collect::<Vec<_>>(),
        );

        filtered_currencies.sort_unstable();
        filtered_currencies.dedup();

        let mut last_currency_states = vec![];

        for currency in &filtered_currencies {
            if let Some(currency_state_result) = self
                .client()
                .get_currency_state(&currency.1.to_string())
                .unwrap()
                .first()
            {
                let currencynames = currency_state_result
                    .currencystate
                    .reservecurrencies
                    .iter()
                    .map(|rc| {
                        (
                            rc.currencyid.clone(),
                            self.currency_id_to_name(rc.currencyid.clone()),
                        )
                    })
                    .collect();

                last_currency_states.push(Basket {
                    name: self.currency_id_to_name(currency.1.clone()),
                    currencyid: currency.1.clone(),
                    active_chain_id: active_chain_id.chainid.clone(),
                    currencynames,
                    currency_state: currency_state_result.currencystate.clone(),
                });
            }
        }

        Ok(last_currency_states)
    }
}

impl Debug for dyn Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", &self.currencyidhex(), self.get_name())
    }
}

#[derive(Debug, Clone)]
pub struct Basket {
    pub name: String,
    pub currencyid: Address,
    pub active_chain_id: Address,
    pub currency_state: vrsc_rpc::json::CurrencyState,
    pub currencynames: HashMap<Address, String>,
}

fn read_config_contents(path: &Path) -> HashMap<String, String> {
    let contents = fs::read_to_string(path.to_str().unwrap()).unwrap();

    let map: HashMap<String, String> = contents
        .as_str()
        .split('\n')
        .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
        .filter(|vec| vec.len() == 2)
        .map(|vec| (vec[0].to_string(), vec[1].to_string()))
        .collect::<HashMap<String, String>>();

    map
}
