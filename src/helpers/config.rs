use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub db_address: String,
    pub connection_limit: Option<u32>,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let limit = match dotenv::var("DB_LIMIT") {
            Ok(value) => Some(value.parse::<u32>().unwrap()),
            Err(_) => None,
        };
        Self {
            token: dotenv::var("DISCORD_TOKEN").expect("Token must be supplied!"),
            db_address: dotenv::var("DATABASE_URL").expect("DB Address must be supplied!"),
            connection_limit: limit,
        }
    }
}
