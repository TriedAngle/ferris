use crate::code_execution::eval;
use crate::code_execution::util::parse;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn code(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (flags, code_block, errors) = parse(args.clone());
    info!(
        "flags: {:?}\ncode_block: {:?}\nerrors: {:?}",
        flags, code_block, errors
    );
    let result = eval::eval(ctx, flags, code_block, false).await.unwrap();

    let result_string = if result.stderr.is_empty() {
        result.stdout
    } else if result.stdout.is_empty() {
        result.stderr
    } else {
        format!("{}\n{}", result.stderr, result.stdout)
    };

    msg.reply(&ctx.http, format!("```\n{}\n```", result_string))
        .await?;

    Ok(())
}
