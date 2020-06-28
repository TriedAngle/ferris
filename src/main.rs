use dotenv;
use serenity::client::{Client, Context};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::EventHandler;

mod math;
mod memes;
mod utils;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "lol no generics" {
            if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("stop swearing! (sometimes even the truth may be swearing)")
                        .image("https://www.dropbox.com/s/hz23pzn8ur1mao6/mad-rustacean.png?dl=1")
                })
            }) {
                println!("Error sending message: {:?}", e);
            }
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Token must be supplied!");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefixes(vec!["praise!", "!"]).with_whitespace(true))
            .before(|_ctx, msg, command_name| {
                println!("==> IN '{}' FROM '{}'", command_name, msg.author.name);
                true
            })
            .after(|_ctx, msg, command_name, error| match error {
                Ok(()) => println!("<== OUT '{}' FROM '{}'", command_name, msg.author.name),
                Err(e) => println!("<=/= OUT '{}' ERROR: {:?}", command_name, e),
            })
            .group(&utils::UTILS_GROUP)
            .group(&math::MATH_GROUP),
    );

    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
