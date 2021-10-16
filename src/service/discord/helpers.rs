use serenity::model::id::{ChannelId, GuildId, MessageId};

pub fn get_custom_emoji(emoji: String, name: String, animated: bool) -> String {
    let mut emoji_string = String::new();

    if animated {
        emoji_string.push_str("<a:");
    } else {
        emoji_string.push_str("<:");
    }

    emoji_string.push_str(&format!("{}:{}>", name, emoji));

    emoji_string
}

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "https://discordapp.com/channels/{}/{}/{}",
        guild_id.0, channel_id.0, message_id.0
    )
}
