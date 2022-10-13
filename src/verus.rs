use vrsc_rpc::{json::vrsc::Address, Auth, Client, RpcApi};

#[derive(Debug)]
pub struct Basket {
    pub name: String,
    pub currency_state: vrsc_rpc::json::CurrencyState,
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

pub fn get_latest_baskets() -> Result<Vec<Basket>, ()> {
    let client = Client::chain("vrsctest", Auth::ConfigFile, None).unwrap();

    let currencies = client.list_currencies(None).unwrap();

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

    let imported_currencies = client.list_currencies(Some("imported")).unwrap();

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

    // get_currency_state for bridges (options:545) to get the very latest reserve definition
    // let mut last_currency_states = vec![];
    let mut last_currency_states = vec![];

    for currency in &filtered_currencies {
        if let Some(currency_state_result) = client
            .get_currency_state(&currency.1.to_string())
            .unwrap()
            .first()
        {
            last_currency_states.push(Basket {
                name: currency.0.to_string(),
                currency_state: currency_state_result.currencystate.clone(),
            });
        }
    }

    Ok(last_currency_states)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        let _ = get_latest_baskets();
    }
}
