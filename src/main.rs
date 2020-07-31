use dotenv;
use std::{sync::Arc};
use serenity::client::{Client, Context, bridge::voice::ClientVoiceManager};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Mutex, TypeMapKey};

mod math;
mod memes;
mod utils;
mod language_filter;
mod voice;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        language_filter::check(&ctx, &msg);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Token must be supplied!");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

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
            .group(&math::MATH_GROUP)
            .group(&voice::MUSIC_GROUP),
    );

    if let Err(e) = client.start() {
        println!("Client error: {:?}", e);
    }
}
