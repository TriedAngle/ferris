use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::Result as SerenityResult;

#[group]
#[commands(ping, ferris)]
struct Utils;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn ferris(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Thank you for praising Ferris and being a cool Rustacean, who enjoys writing in the best language on earth!").image("https://rustacean.net/assets/rustacean-flat-happy.png")
        })
    }).await?;

    Ok(())
}
