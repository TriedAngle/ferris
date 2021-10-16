use crate::service::discord::types::{
    FerrisChannelId, FerrisGuildId, FerrisMessageId, FerrisReactionEmoji, FerrisRoleId,
};

pub struct FerrisGuildReactionRoleBoards {
    pub guild: FerrisGuildId,
    pub boards: Vec<FerrisReactionRoleBoard>,
}

#[derive(Debug, Default, Clone)]
pub struct FerrisReactionRoleBoard {
    pub channel: FerrisChannelId,
    pub msg: FerrisMessageId,
    pub reaction_roles: Vec<FerrisReactionRole>,
}

#[derive(Debug, Default, Clone)]
pub struct FerrisReactionRole {
    pub role: FerrisRoleId,
    pub emoji: FerrisReactionEmoji,
}
