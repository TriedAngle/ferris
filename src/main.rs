use crate::converter::{ConnectionPool, ReqwestClient, ShardManagerContainer};
use serenity::async_trait;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use serenity::model::guild::{Guild, GuildUnavailable};
use serenity::model::id::{GuildId, UserId};
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
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("ready");
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        if let Err(e) = helpers::database::add_guild(&pool, guild.id, is_new).await {
            eprintln!("Error in guild creation! (ID {}): {}", guild.id.0, e);
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, _full: Option<Guild>) {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        if let Err(e) = helpers::database::delete_guild(&pool, incomplete.id).await {
            eprintln!("Error in guild creation! (ID {}): {}", incomplete.id.0, e);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = helpers::reaction_roles::dispatch_event(&ctx, &reaction, false).await {
            eprintln!(
                "Error in reaction dispatch! (ID {}): {}",
                reaction.guild_id.unwrap().0,
                e
            );

            let _ = reaction.channel_id.say(ctx, concat!("Looks like there was an error when you reacted! \n",
            "Please make sure you have the `Add Reactions` permission enabled for both the channel and the bot role!")).await;
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        println!("called");
        if let Err(e) = helpers::reaction_roles::dispatch_event(&ctx, &reaction, true).await {
            eprintln!(
                "Error in reaction dispatch! (ID {}): {}",
                reaction.guild_id.unwrap().0,
                e
            );

            let _ = reaction.channel_id.say(ctx, concat!("Looks like there was an error when you reacted! \n",
            "Please make sure you have the `Add Reactions` permission enabled for both the channel and the bot role!")).await;
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

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
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
                .owners(owners)
        })
        .help(&MY_HELP)
        .group(&commands::GENERAL_GROUP)
        .group(&commands::UTILITY_GROUP)
        .group(&commands::ADMIN_GROUP);

    let mut client = Client::builder(&config.token)
        .framework(framework)
        .event_handler(Handler {
            run_loop: AtomicBool::new(true),
        })
        .intents({
            let mut intents = GatewayIntents::all();
            intents.remove(GatewayIntents::DIRECT_MESSAGES);
            intents.remove(GatewayIntents::DIRECT_MESSAGE_REACTIONS);
            intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);
            intents
        })
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
