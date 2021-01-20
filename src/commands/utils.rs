use chrono::{Duration, Utc};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Color;
use std::time::{Instant, SystemTime};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let sent = msg.timestamp;
    let ping = Utc::now().timestamp_millis() - sent.timestamp_millis();
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("pong!")
                    .description(format!("⌛ {:?}", ping))
                    .color(Color::DARK_GREEN)
            })
        })
        .await?;
    Ok(())
}

#[command]
pub async fn ferris(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Thank you for praising Ferris and being a cool Rustacean!")
                    .description("further enjoy writing in the best language on earth :)!")
                    .color(Color::DARK_GOLD)
                    .image("https://rustacean.net/assets/rustacean-flat-happy.png")
            })
        })
        .await?;

    Ok(())
}
