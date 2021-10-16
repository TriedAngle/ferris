use crate::service::db::DataService;
use reqwest::Client;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database;

pub struct ReqwestClient;

pub struct ShardManagerContainer;

pub struct ShessManager;

impl TypeMapKey for Database {
    type Value = DataService;
}

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Client>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
