use crate::converter::ConnectionPool;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Reaction, ReactionType};
use serenity::model::id::{ChannelId, GuildId, MessageId, RoleId, UserId};
use serenity::static_assertions::_core::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Default)]
struct ReactionInfo<'a> {
    guild_id: GuildId,
    user_id: UserId,
    message_id: MessageId,
    channel_id: ChannelId,
    emoji: &'a str,
}

pub async fn dispatch_event(ctx: &Context, reaction: &Reaction, remove: bool) -> CommandResult {
    let mut reaction_info = ReactionInfo::default();

    reaction_info.guild_id = reaction.guild_id.unwrap();
    reaction_info.user_id = reaction.user_id.unwrap();
    reaction_info.channel_id = reaction.channel_id;
    reaction_info.message_id = reaction.message_id;

    match &reaction.emoji {
        ReactionType::Unicode(name) => {
            reaction_info.emoji = name;

            handle_role(ctx, remove, reaction_info).await?;
        }
        ReactionType::Custom { id, .. } => {
            let id = &id.as_u64().to_string();
            reaction_info.emoji = id;

            handle_role(ctx, remove, reaction_info).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn handle_role(ctx: &Context, remove: bool, rec_info: ReactionInfo<'_>) -> CommandResult {
    let pool = ctx
        .data
        .read()
        .await
        .get::<ConnectionPool>()
        .cloned()
        .unwrap();

    let reaction_data = sqlx::query!(
        "SELECT role_id FROM reaction_roles WHERE message_id = $1 AND guild_id = $2 AND emoji = $3",
        rec_info.message_id.0 as i64,
        rec_info.guild_id.0 as i64,
        rec_info.emoji
    )
    .fetch_all(&pool)
    .await?;

    if !reaction_data.is_empty() {
        for data in reaction_data {
            let role_id = RoleId::from(data.role_id as u64);

            if remove {
                if ctx
                    .http
                    .remove_member_role(rec_info.guild_id.0, rec_info.user_id.0, role_id.0)
                    .await
                    .is_err()
                {
                    let err_msg = rec_info
                        .channel_id
                        .say(
                            ctx,
                            concat!("Role removal unsuccessful. \
                    Please make sure the bot's role is above the one you want to assign! \n",
                    "This message will delete itself in 10 seconds. Please report this to the \
                    moderators/administrators."),
                        )
                        .await?;

                    sleep(Duration::from_secs(10)).await;

                    err_msg.delete(ctx).await?;
                }
            } else if ctx
                .http
                .add_member_role(rec_info.guild_id.0, rec_info.user_id.0, role_id.0)
                .await
                .is_err()
            {
                let err_msg = rec_info
                    .channel_id
                    .say(
                        ctx,
                        concat!("Role assignment unsuccessful. \
                Please make sure the bot's role is above the one you want to assign! \n",
                "This message will delete itself in 10 seconds. Please report this to the \
                moderators/administrators."),
                    )
                    .await?;

                sleep(Duration::from_secs(10)).await;

                err_msg.delete(ctx).await?;
            };
        }
    }

    Ok(())
}
