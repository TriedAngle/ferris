pub mod helpers;
mod permission;
mod reaction_roles;
pub mod types;

use crate::service::discord::reaction_roles::FerrisGuildReactionRoleBoards;
use crate::service::discord::types::{FerrisGuild, FerrisGuildId};
use mongodb::Collection;

pub struct DiscordService {
    pub guilds: Collection<FerrisGuild>,
    // temporary will move to guilds later
    pub guild_ids: Collection<FerrisGuildId>,
    pub reaction_role_boards: Collection<FerrisGuildReactionRoleBoards>,
}

impl DiscordService {
    pub fn new(db: mongodb::Database) -> Self {
        Self {
            guilds: db.collection::<FerrisGuild>("guilds"),
            guild_ids: db.collection::<FerrisGuildId>("guild_ids"),
            reaction_role_boards: db
                .collection::<FerrisGuildReactionRoleBoards>("reaction_role_boards"),
        }
    }
}
