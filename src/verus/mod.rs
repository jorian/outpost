pub mod pbaas;
pub mod vrsc;

use std::collections::HashMap;
use std::fmt::Debug;

use vrsc_rpc::json::vrsc::Address;
use vrsc_rpc::json::Currency;
use vrsc_rpc::{Client, RpcApi};

pub trait Chain {
    fn get_name(&self) -> String;
    fn set_name(&mut self);
    fn testnet(&self) -> bool;
    fn currencyidhex(&self) -> String;
    fn client(&self) -> &Client;
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
            .filter(|currency| [34, 40, 264].contains(&currency.currencydefinition.options))
            .collect();

        filtered_currencies.append(&mut pbaas_currencies);

        Ok(filtered_currencies)
    }

    fn get_latest_baskets(&mut self) -> Result<Vec<Basket>, ()> {
        let currencies = self.client().list_currencies(None).unwrap();
        let active_chain_id = self.client().get_blockchain_info().unwrap();

        // A bridge has 2 sides, so we need to find out which sides in order to include the reserves in our baskets.
        // A bridge is defined on the subsystem and ties to the system it was launched from.
        let active_chain_filter = |currency: &Currency| {
            currency.currencydefinition.systemid == active_chain_id.chainid || {
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
                    currencynames,
                    currencyid: currency.1.clone(),
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
    pub currency_state: vrsc_rpc::json::CurrencyState,
    pub currencynames: HashMap<Address, String>,
}

// pub fn get_latest_baskets(chain: &Box<dyn Chain>) -> Result<Vec<Basket>, ()> {}

/*use std::collections::HashMap;

use vrsc_rpc::{
    json::{vrsc::Address, Currency},
    Auth, Client, RpcApi,
};

use crate::userdata::PBaaSChain;



pub struct Verus {
    pub client: Client,
    pub testnet: bool,
    id_names: HashMap<Address, String>,
    hex_names: Option<HashMap<String, String>>,
}

impl Verus {
    pub fn new(testnet: bool, chain: Option<&PBaaSChain>) -> Self {
        let client = match testnet {
            true => {
                if let Some(chain) = chain {
                    Client::chain(true, &chain.currencyidhex, Auth::ConfigFile).unwrap()
                } else {
                    Client::vrsc(true, Auth::ConfigFile).unwrap()
                }
            }
            false => Client::vrsc(false, Auth::ConfigFile).unwrap(),
        };

        Verus {
            client,
            testnet,
            id_names: HashMap::new(),
            hex_names: None,
        }
    }


    // listcurrencies without any arguments returns only 1 264: VRSCTEST, and listcurrencies(imported) returns all the other minable pbaas currencies.
    pub fn get_latest_currencies(&mut self) -> Result<Vec<Currency>, ()> {
        let currencies = self.client.list_currencies(None).unwrap();

        let mut filtered_currencies: Vec<Currency> = currencies
            .0
            .into_iter()
            .filter(|currency| [40, 264].contains(&currency.currencydefinition.options))
            .collect();

        let currencies = self.client.list_currencies(Some("imported")).unwrap();

        let mut pbaas_currencies = currencies
            .0
            .into_iter()
            .filter(|currency| [34, 40, 264].contains(&currency.currencydefinition.options))
            .collect();

        filtered_currencies.append(&mut pbaas_currencies);

        Ok(filtered_currencies)
    }

    // TODO do a single getcurrency for all the converters to get their contents and the names?
    pub fn currency_id_to_name(&mut self, currency_id: Address) -> String {
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

    pub fn currency_id_hex_to_name(&mut self, currencyidhex: &str) -> String {
        if self.hex_names.is_none() {
            let mut currencies = self.client.list_currencies(None).unwrap().0;

            currencies.append(
                self.client
                    .list_currencies(Some("imported"))
                    .unwrap()
                    .0
                    .as_mut(),
            );

            let mut hex_map = HashMap::new();

            for currency in currencies {
                hex_map.insert(
                    currency.currencydefinition.currencyidhex.clone(),
                    currency.currencydefinition.name.clone(),
                );
            }

            self.hex_names = Some(hex_map);
        }

        match self.hex_names.as_mut().unwrap().get(currencyidhex) {
            Some(value) => return value.to_owned(),
            None => {
                panic!("no coin found in currencyidhex conversion");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        let mut verus = Verus::new(true, None);
        verus.get_latest_baskets().unwrap();
    }
}
*/
