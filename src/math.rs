use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult, Args, macros::{
    command,
    group,
}};

#[group]
#[commands(math)]
struct Math;

#[command]
fn math(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let result = meval::eval_str(&args.message());

    let answer = match result {
        Ok(result) => result.to_string(),
        Err(e) => format!("Error: {:?}", e),
    };

    if let Err(e) = msg.channel_id.say(&ctx.http, answer) {
        println!("{:?}", e);
    }

    Ok(())
}