use chrono::Utc;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Color;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = msg.timestamp;

    let back = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| e.title("Loading...").color(Color::DARK_ORANGE))
        })
        .await?;

    let stop = back.timestamp;

    back.delete(&ctx.http).await?;

    info!("datetime start: {:?}", start);
    info!("datetime stop: {:?}", stop);
    let ping = stop - start;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("pong!")
                    .description(format!("⌛ {}ms", ping.num_milliseconds()))
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
