use dotenv;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

impl EventHandler for Handler { }

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    Ok(())
}

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Token must be supplied!");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .before(|_ctx, msg, command_name| {
            println!("==> IN '{}' FROM '{}'", command_name, msg.author.name);
            true
        })
        .after(|_ctx, msg, command_name, error| {
            match error {
                Ok(()) => println!("<== OUT '{}' FROM '{}'", command_name, msg.author.name),
                Err(e) => println!("<=/= OUT '{}' ERROR: {:?}", command_name, e),
            }
        })
        .group(&GENERAL_GROUP));


    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
