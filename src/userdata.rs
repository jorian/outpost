pub struct UserData {
    pub pbaas_chains: Vec<String>,
}

impl UserData {
    pub fn new() -> Self {
        UserData {
            pbaas_chains: vec![
                "v2".to_string(),
                "quantum".to_string(),
                "gravity".to_string(),
                "🎃".to_string(),
            ],
        }
    }
}
