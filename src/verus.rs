use std::{borrow::Borrow, collections::HashMap};

use tracing::{debug, instrument};
use vrsc_rpc::{
    json::{vrsc::Address, Currency},
    Auth, Client, RpcApi,
};

use crate::userdata::Chain;

#[derive(Debug, Clone)]
pub struct Basket {
    pub name: String,
    pub currencyid: Address,
    pub currency_state: vrsc_rpc::json::CurrencyState,
    pub currencynames: HashMap<Address, String>,
}

pub struct Verus {
    pub client: Client,
    id_names: HashMap<Address, String>,
}

impl Verus {
    pub fn new(testnet: bool, chain: Option<&Chain>) -> Self {
        let client = match testnet {
            true => {
                if let Some(chain) = chain {
                    Client::chain(
                        true,
                        chain.currencyidhex.as_deref().unwrap(),
                        Auth::ConfigFile,
                    )
                    .unwrap()
                } else {
                    debug!("We should be getting here");
                    Client::vrsc(true, Auth::ConfigFile).unwrap()
                }
            }
            false => Client::vrsc(false, Auth::ConfigFile).unwrap(),
        };

        Verus {
            client,
            id_names: HashMap::new(),
        }
    }
    pub fn get_latest_baskets(&mut self) -> Result<Vec<Basket>, ()> {
        let currencies = self.client.list_currencies(None).unwrap();
        let active_chain_id = self.client.get_blockchain_info().unwrap();

        // options:33 for fractional baskets
        let mut filtered_currencies: Vec<(String, Address)> = currencies
            .0
            .into_iter()
            .filter(|currency| [33, 35, 545].contains(&currency.currencydefinition.options))
            // only include baskets that were defined on the chain that is currently active
            .filter(|currency| currency.currencydefinition.systemid == active_chain_id.chainid)
            .map(|currency| {
                (
                    currency.currencydefinition.fullyqualifiedname,
                    currency.currencydefinition.currencyid,
                )
            })
            .collect();

        let imported_currencies = self.client.list_currencies(Some("imported")).unwrap();

        filtered_currencies.append(
            &mut imported_currencies
                .0
                .into_iter()
                .filter(|currency| currency.currencydefinition.options == 545)
                .filter(|currency| currency.currencydefinition.systemid == active_chain_id.chainid)
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
                .client
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

    // listcurrencies without any arguments returns only 1 264 for VRSCTEST, and listcurrencies(imported) returns all the other minable pbaas currencies.
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
            .inspect(|cur| {
                debug!("{:#?}", &cur);
            })
            .filter(|currency| [34, 40, 264].contains(&currency.currencydefinition.options))
            .collect();

        filtered_currencies.append(&mut pbaas_currencies);

        Ok(filtered_currencies)
    }

    // TODO do a single getcurrency for all the converters to get their contents and the names?
    pub fn currency_id_to_name(&mut self, currency_id: Address) -> String {
        dbg!(&currency_id);
        match self.id_names.get(&currency_id) {
            Some(value) => return value.to_owned(),
            None => {
                let value = self
                    .client
                    .get_currency(&currency_id.to_string())
                    .unwrap()
                    .fullyqualifiedname;

                // let value = match self.client.get_currency(&currency_id.to_string()) {
                //     Ok(result) => result.fullyqualifiedname,
                //     Err(_) => currency_id.to_string(),
                // };
                self.id_names.insert(currency_id.clone(), value);
                self.id_names.get(&currency_id).unwrap().to_owned()
            }
        }
    }

    // pub fn currency_id_hex_to_name(&mut self, currencyidhex: String) {}
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
