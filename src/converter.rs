use reqwest::Client;
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use serenity::client::bridge::gateway::ShardManager;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ConnectionPool;

pub struct ReqwestClient;

pub struct ShardManagerContainer;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Client>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}