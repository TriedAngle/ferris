use crate::code_execution::api::Channel;
use crate::code_execution::{api, CodeBlock, Language};
use serenity::framework::standard::Args;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(mut args: Args) -> (api::CommandFlags, CodeBlock, String) {
    let mut errors = String::new();

    let mut flags = api::CommandFlags {
        channel: api::Channel::Nightly,
        mode: api::Mode::Debug,
        edition: api::Edition::E2018,
        warn: false,
    };

    let mut message_split = Vec::new();
    let mut arguments = HashMap::new();

    while let Ok(arg) = args.single::<String>() {
        message_split.push(arg);
    }

    for part in &message_split {
        if part.contains("=") {
            let split = part.split('=').collect::<Vec<&str>>();
            arguments.insert(split[0], split[1]);
        }
        if part.contains("`") {
            break;
        }
    }

    if let Some(mode) = arguments.get("mode") {
        match mode.parse() {
            Ok(val) => flags.mode = val,
            Err(e) => errors += &format!("{}\n", e),
        }
    }

    if let Some(channel) = arguments.get("channel") {
        match channel.parse() {
            Ok(val) => flags.channel = val,
            Err(e) => errors += &format!("{}\n", e),
        }
    }

    if let Some(edition) = arguments.get("edition") {
        match edition.parse() {
            Ok(val) => flags.edition = val,
            Err(e) => errors += &format!("{}\n", e),
        }
    }

    if let Some(warn) = arguments.get("warn") {
        match warn.parse() {
            Ok(val) => flags.warn = val,
            Err(e) => errors += &format!("{}\n", e),
        }
    }

    let mut code_block = CodeBlock {
        content: "".to_string(),
        language: Language::Rust,
        full: false,
    };

    let info = message_split[arguments.len()].clone();

    let tricks = if info.contains("```") {
        code_block.full = true;
        3
    } else {
        1
    };

    let start_pos = if tricks == 3 {
        args.message()
            .find(&message_split[arguments.len()])
            .unwrap()
            + info.len()
            + 1
    } else {
        args.message()
            .find(&message_split[arguments.len()])
            .unwrap()
            + 1
    };

    code_block.content = args.message()[start_pos..args.message().len() - tricks].to_string();

    if !code_block.full {
        code_block.full = true;
        code_block.content.insert_str(0, "fn main() {\n");
        code_block.content.push_str("\n}");
    }

    (flags, code_block, errors)
}
