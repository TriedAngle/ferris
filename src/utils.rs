use serenity::model::channel::{Message, Embed};
use serenity::prelude::{Context};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::utils::MessageBuilder;

#[group]
#[commands(ping, ferris)]
struct Utils;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

#[command]
fn ferris(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Thank you for praising Ferris and being a cool Rustacean, who enjoys writing in the best language on earth!").image("https://rustacean.net/assets/rustacean-flat-happy.png")
        })
    }) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}