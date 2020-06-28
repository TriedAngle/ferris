use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{CommandResult, Args, macros::{
    command,
    group,
}};

#[group]
#[prefix = "math!"]
#[commands(add, sub, mul, div, custom)]
struct Math;

#[command]
fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first + second;

    if let Err(e) = msg.channel_id.say(&ctx.http, &res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, e);
    }

    Ok(())
}

#[command]
fn sub(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first - second;

    if let Err(e) = msg.channel_id.say(&ctx.http, &res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, e);
    }

    Ok(())
}

#[command]
fn mul(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first * second;

    if let Err(e) = msg.channel_id.say(&ctx.http, &res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, e);
    }

    Ok(())
}

#[command]
fn div(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first / second;

    if let Err(e) = msg.channel_id.say(&ctx.http, &res.to_string()) {
        println!("Err sending product of {} and {}: {:?}", first, second, e);
    }

    Ok(())
}

#[command]
fn custom(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let res = &msg.content;
    if let Err(e) = msg.channel_id.say(&ctx.http,
                                       format!("{} \n {:?}", &res.to_string(), &args.message()),
    ) {
        println!("{:?}", e);
    }

    Ok(())
}