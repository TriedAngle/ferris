use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};

use serenity::model::channel::Message;
use serenity::prelude::Context;

#[group("math")]
#[commands(math)]
struct Math;

#[command]
async fn math(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = meval::eval_str(&args.message());
    let answer = match result {
        Ok(result) => result.to_string(),
        Err(e) => format!("Error: {:?}", e),
    };

    msg.reply(
        &ctx.http,
        format!("```\n{} = {}\n```", answer, &args.message()),
    )
    .await?;

    Ok(())
}
