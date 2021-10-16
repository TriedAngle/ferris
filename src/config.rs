use crate::service::db::DataService;
use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub host_address: String,
    pub host_port: u32,
    pub mongodb_address: String,
    pub mongodb_port: u32,
    pub mongodb_admin_name: String,
    pub mongodb_admin_password: String,
    pub redis_address: String,
    pub redis_port: u32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Configuration");

        let mut config = config::Config::new();
        config.merge(config::Environment::default())?;
        config
            .try_into()
            .context("Unable to load config from environment")
    }

    pub async fn create_data_service(&self) -> Result<DataService> {
        info!("Creating DataService");
        let mongodb_url = format!(
            "mongodb://{}:{}@{}:{}",
            self.mongodb_admin_name,
            self.mongodb_admin_password,
            self.mongodb_address,
            self.mongodb_port
        );

        let redis_url = format!("redis://{}:{}", self.redis_address, self.redis_port);

        let client = mongodb::Client::with_uri_str(&mongodb_url).await?;
        let db = client.database("track");
        let redis = redis::Client::open(redis_url)?;

        let service = DataService { client, db, redis };

        Ok(service)
    }
}
