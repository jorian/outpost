use std::str::FromStr;

use vrsc_rpc::json::vrsc::Address;

pub struct UserData {
    pub pbaas_chains: Vec<Chain>,
}

impl UserData {
    pub fn new() -> Self {
        let pbaas_chains = vec![
            Chain::new("vrsctest", None),
            Chain::new("v2", Some("62008467889825894230843329f6ce9d17c3944e")),
            Chain::new("🎃", Some("db2b60428ce720b27302e39b31d0804f61b92c29")),
        ];

        UserData { pbaas_chains }
    }
}

#[derive(Clone)]
pub struct Chain {
    pub name: String,
    pub currencyidhex: Option<String>,
}

impl Chain {
    pub fn new<S: Into<String>>(name: S, currencyidhex: Option<S>) -> Self {
        Chain {
            name: name.into(),
            currencyidhex: currencyidhex.map(|c| c.into()),
        }
    }
}
