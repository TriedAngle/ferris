use crate::types::Shessy;
use reqwest::Client;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ConnectionPool;

pub struct ReqwestClient;

pub struct ShardManagerContainer;

pub struct ShessManager;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Client>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for ShessManager {
    type Value = Arc<Mutex<Shessy>>;
}
