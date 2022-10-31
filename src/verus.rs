use std::collections::HashMap;

use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

#[derive(Debug)]
pub struct Basket {
    pub name: String,
    pub currencyid: Address,
    pub currency_state: vrsc_rpc::json::CurrencyState,
    pub currencynames: HashMap<Address, String>,
}

pub trait Currency: Send {
    fn name(&self) -> String;
}

pub struct Reserve {
    name: String,
}

impl Currency for Reserve {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Currency for Basket {
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Verus {
    client: Client,
    id_names: HashMap<Address, String>,
}

impl Verus {
    pub fn new() -> Self {
        Verus {
            client: Client::chain("vrsctest", Auth::ConfigFile, None).unwrap(),
            id_names: HashMap::new(),
        }
    }
    pub fn get_latest_baskets(&mut self) -> Result<Vec<Basket>, ()> {
        let currencies = self.client.list_currencies(None).unwrap();

        // options:33 for fractional baskets
        let mut filtered_currencies: Vec<(String, Address)> = currencies
            .0
            .into_iter()
            .filter(|currency| [33, 545].contains(&currency.currencydefinition.options))
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
                .map(|currency| {
                    (
                        currency.currencydefinition.fullyqualifiedname,
                        currency.currencydefinition.currencyid,
                    )
                })
                .collect::<Vec<_>>(),
        );

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        let mut verus = Verus::new();
        verus.get_latest_baskets().unwrap();
    }
}
