use serenity::model::id::{ChannelId, GuildId, MessageId, RoleId};

#[derive(Debug, Default, Clone)]
pub struct FerrisGuild {}

#[derive(Debug, Default, Clone)]
pub struct FerrisReactionEmoji {
    pub name: Option<String>,
    pub emoji: Option<String>,
    pub animated: Option<bool>,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct FerrisGuildId(i64);
#[derive(Debug, Default, Copy, Clone)]
pub struct FerrisChannelId(i64);
#[derive(Debug, Default, Copy, Clone)]
pub struct FerrisMessageId(i64);
#[derive(Debug, Default, Copy, Clone)]
pub struct FerrisRoleId(i64);
#[derive(Debug, Default, Clone)]
pub struct FerrisEmoji(String);

impl From<String> for FerrisEmoji {
    fn from(emoji: String) -> Self {
        FerrisEmoji(emoji)
    }
}

impl From<GuildId> for FerrisGuildId {
    fn from(id: GuildId) -> Self {
        Self(id.0 as i64)
    }
}

impl From<ChannelId> for FerrisChannelId {
    fn from(id: ChannelId) -> Self {
        Self(id.0 as i64)
    }
}

impl From<MessageId> for FerrisMessageId {
    fn from(id: MessageId) -> Self {
        Self(id.0 as i64)
    }
}

impl From<RoleId> for FerrisRoleId {
    fn from(id: RoleId) -> Self {
        Self(id.0 as i64)
    }
}
