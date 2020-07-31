use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::Result as SerenityResult;
use serenity::model::channel::Message;
use serenity::prelude::{Context};

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

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
