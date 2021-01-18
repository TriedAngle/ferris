use crate::converter::{ConnectionPool, ReqwestClient, ShardManagerContainer};
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::Client;
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

mod commands;
mod converter;
mod helpers;
mod requests;

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, &help_options, groups, owners).await;
    Ok(())
}

struct Handler {
    run_loop: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {
    //     language_filter::check(&ctx, &msg);
    // }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let guild_id = reaction.guild_id.unwrap();
        let user_id = reaction.user_id.unwrap();
        if reaction.message_id == 800732039665942559 {
            let emoji_name = match reaction.emoji {
                ReactionType::Custom { name, .. } => name.unwrap(),
                ReactionType::Unicode(name) => name,
                _ => String::new(),
            };
            if emoji_name == "lol" {
                let mut user = guild_id.member(&ctx, &user_id).await.unwrap();
                user.add_role(&ctx, 800734215297040404).await.unwrap();
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let guild_id = reaction.guild_id.unwrap();
        let user_id = reaction.user_id.unwrap();
        if reaction.message_id == 800732039665942559 {
            let emoji_name = match reaction.emoji {
                ReactionType::Custom { name, .. } => name.unwrap(),
                ReactionType::Unicode(name) => name,
                _ => String::new(),
            };
            if emoji_name == "lol" {
                let mut user = guild_id.member(&ctx, &user_id).await.unwrap();
                user.remove_role(&ctx, 800734215297040404).await.unwrap();
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let config = helpers::config::Config::new();

    let http = Http::new_with_token(&config.token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(err) => panic!("Could not access application info: {:?}", err),
    };

    let pool = helpers::database::get_db_pool(config.db_address.clone()).await?;

    let reqwest_client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0")
        .build()?;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefixes(vec!["praise!", "!"])
                .on_mention(Some(bot_id))
                .delimiters(vec![", ", ","])
        })
        .help(&MY_HELP)
        .group(&commands::utils::UTILS_GROUP)
        .group(&commands::math::MATH_GROUP);

    let mut client = Client::builder(&config.token)
        .event_handler(Handler {
            run_loop: AtomicBool::new(true),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ConnectionPool>(pool);
        data.insert::<ReqwestClient>(Arc::new(reqwest_client));
    }

    if let Err(e) = client.start_autosharded().await {
        println!("Client error: {:?}", e);
    }

    Ok(())
}
