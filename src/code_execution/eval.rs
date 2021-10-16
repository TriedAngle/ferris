use crate::code_execution::api::{
    Channel, CommandFlags, CrateType, Edition, PlayResult, PlaygroundRequest,
};
use crate::code_execution::CodeBlock;
use crate::converter::ReqwestClient;
use eyre::Result;
use serenity::client::Context;

// play and eval work similarly, so this function abstracts over the two
pub async fn eval(
    ctx: &Context,
    mut flags: CommandFlags,
    code: CodeBlock,
    force_warnings: bool, // If true, force enable warnings regardless of flags
) -> Result<PlayResult> {
    let code = code.content;

    if force_warnings {
        flags.warn = true;
    }

    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestClient>()
        .cloned()
        .unwrap();
    let mut result = client
        .post("https://play.rust-lang.org/execute")
        .json(&PlaygroundRequest {
            code: &code,
            channel: flags.channel,
            crate_type: if code.contains("fn main") {
                CrateType::Binary
            } else {
                CrateType::Library
            },
            edition: flags.edition,
            mode: flags.mode,
            tests: false,
        })
        .send()
        .await?
        .json::<PlayResult>()
        .await?;

    Ok(result)
}
