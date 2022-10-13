use vrsc_rpc::{Auth, Client, RpcApi};

#[derive(Debug)]
pub struct Basket {
    name: String,
    _currency_state: vrsc_rpc::json::CurrencyState,
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

impl ToString for Basket {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&self.name);

        for reserve_currency in &self._currency_state.reservecurrencies {
            s.push_str(&format!(
                "\n-- {}: {}",
                reserve_currency.currencyid,
                reserve_currency.reserves.as_vrsc()
            ));
        }

        s
    }
}

pub fn get_latest_baskets() -> Result<Vec<Basket>, ()> {
    let client = Client::chain("vrsctest", Auth::ConfigFile, None).unwrap();

    let currencies = client.list_currencies(None).unwrap();

    // options:33 for fractional baskets
    let filtered_currencies: Vec<String> = currencies
        .0
        .into_iter()
        .filter(|currency| currency.currencydefinition.options == 33)
        .map(|currency| currency.currencydefinition.name)
        .collect();

    // get_currency_state for bridges (options:545) to get the very latest reserve definition
    // let mut last_currency_states = vec![];
    let mut last_currency_states = vec![];

    for currency in &filtered_currencies {
        if let Some(currency_state_result) = client.get_currency_state(&currency).unwrap().first() {
            last_currency_states.push(Basket {
                name: currency.to_string(),
                _currency_state: currency_state_result.currencystate.clone(),
            });
        }
    }

    dbg!(&last_currency_states);

    Ok(last_currency_states)

    // Ok(filtered_currencies)
    // Ok(vec![])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        let _ = get_latest_baskets();
    }
}
