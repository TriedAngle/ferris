use dotenv;
use serenity::client::Client;
use serenity::prelude::{EventHandler};
use serenity::framework::standard::{
    StandardFramework,
};

mod math;
mod utils;

struct Handler;

impl EventHandler for Handler { }

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Token must be supplied!");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefixes(vec!["praise!", "!"])
            .with_whitespace(true)
        )
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
        .group(&utils::UTILS_GROUP)
        .group(&math::MATH_GROUP)
    );


    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
