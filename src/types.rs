use serde::{Deserialize, Serialize};
use shess::defaults::normal::Default8x8;
use shess::discord::Discord;
use shess::{Game as ShessGame, Mode};
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Shessy {
    pub default_games: HashMap<Uuid, ShessGame<Default8x8, Discord>>,
}

impl Shessy {
    pub fn new() -> Self {
        Self {
            default_games: HashMap::new(),
        }
    }

    pub fn default_by_uuid(&mut self, uuid: Uuid) -> &mut ShessGame<Default8x8, Discord> {
        self.default_games.get_mut(&uuid).unwrap()
    }

    pub fn delete_default_by_uuid(&mut self, uuid: &Uuid) -> bool {
        if self.default_games.contains_key(uuid) {
            self.default_games.remove(uuid);
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub discord_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Game {
    pub id: Uuid,
    pub channel_id: String,
    pub players: Vec<i32>,
    pub running: bool,
}
