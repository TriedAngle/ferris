use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::model::channel::Message;
use serenity::prelude::Context;

#[command]
pub async fn math(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
