use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping)]
struct Utils;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}