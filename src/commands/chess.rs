use crate::converter::{ConnectionPool, ShessManager};
use crate::types::{Game, User};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::utils::{Color, MessageBuilder};
use shess::{defaults::normal::Default8x8, discord::Discord, Backend, Game as ShessGame, Mode};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[command]
pub async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let players = Vec::<i32>::new();
    let uuid = Uuid::new_v4();
    let channel_id = msg.channel_id.0.to_string();
    {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        let rec = sqlx::query!(
            r#"
                SELECT * FROM shess_games where channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&pool)
        .await;

        if rec.is_ok() {
            msg.reply(
                &ctx.http,
                format!("This channel already has a running game"),
            )
            .await?;
            return Ok(());
        }

        sqlx::query!(
            r#"
                INSERT INTO shess_games VALUES($1, $2, $3, $4)
            "#,
            uuid,
            channel_id,
            &players,
            false
        )
        .execute(&pool)
        .await?;
    }

    msg.reply(&ctx.http, format!("Successfully crated game"))
        .await?;

    Ok(())
}

#[command]
pub async fn stop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id.0.to_string();
    {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        let rec = sqlx::query!(
            r#"
                SELECT * FROM shess_games where channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&pool)
        .await;

        if rec.is_err() {
            msg.reply(&ctx.http, format!("This channel has no running game"))
                .await?;
            return Ok(());
        }

        sqlx::query!(
            r#"
                DELETE FROM shess_games WHERE channel_id = $1
            "#,
            channel_id
        )
        .execute(&pool)
        .await?;
    }

    msg.reply(&ctx.http, format!("Successfully stopped game"))
        .await?;

    Ok(())
}

#[command]
pub async fn join(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id.0.to_string();
    let player = msg.author.id.0.to_string();
    {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        let user_id = get_or_create_user_id(player, &pool).await;

        let rec = sqlx::query!(
            r#"
                SELECT * FROM shess_games where channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&pool)
        .await;

        if rec.is_err() {
            msg.reply(
                &ctx.http,
                format!("This channel has no game in this channel"),
            )
            .await?;
            return Ok(());
        }

        let rec = rec.unwrap();

        let mut game = Game {
            id: rec.id,
            channel_id: rec.channel_id,
            players: rec.players,
            running: rec.running,
        };

        if game.running {
            msg.reply(&ctx.http, format!("You can't join a running game"))
                .await?;
            return Ok(());
        }

        if game.players.contains(&user_id) {
            msg.reply(&ctx.http, format!("You already joined this game"))
                .await?;
            return Ok(());
        }

        game.players.push(user_id);

        sqlx::query!(
            r#"
                UPDATE shess_games SET players = $1
                WHERE id = $2
            "#,
            &game.players,
            game.id
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    msg.reply(&ctx.http, format!("Successfully joined game"))
        .await?;

    Ok(())
}

#[command]
pub async fn start(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id.0.to_string();
    {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        let shessy = ctx
            .data
            .read()
            .await
            .get::<ShessManager>()
            .cloned()
            .unwrap();

        let mut lock = shessy.lock().await;

        let rec = sqlx::query!(
            r#"
                SELECT * FROM shess_games where channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&pool)
        .await;

        if rec.is_err() {
            msg.reply(&ctx.http, format!("This channel has no running game"))
                .await?;
            return Ok(());
        }

        let rec = rec.unwrap();

        let mut game = Game {
            id: rec.id,
            channel_id: rec.channel_id,
            players: rec.players,
            running: rec.running,
        };

        if game.running {
            msg.reply(&ctx.http, format!("Game already running"))
                .await?;
            return Ok(());
        }

        if game.players.len() != 2 {
            msg.reply(
                &ctx.http,
                format!("Too many or not enough players, you need 2"),
            )
            .await?;
            return Ok(());
        }

        let mut shess_game = ShessGame::<Default8x8, Discord>::new();

        shess_game.backend.set_player(0, game.players[0]);
        shess_game.backend.set_player(1, game.players[1]);

        let mut board = shess_game.mode.rendered_board();

        lock.default_games.insert(game.id, shess_game);

        sqlx::query!(
            r#"
                UPDATE shess_games SET running = $1
                WHERE id = $2
            "#,
            true,
            game.id
        )
        .execute(&pool)
        .await
        .unwrap();

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Shess")
                        .description(format!("{}", board))
                        .color(Color::ORANGE)
                })
            })
            .await?;
    }

    msg.reply(&ctx.http, format!("Successfully started game"))
        .await?;

    Ok(())
}

#[command]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id.0.to_string();
    {
        let pool = ctx
            .data
            .read()
            .await
            .get::<ConnectionPool>()
            .cloned()
            .unwrap();

        let shessy = ctx
            .data
            .read()
            .await
            .get::<ShessManager>()
            .cloned()
            .unwrap();

        let mut lock = shessy.lock().await;

        let rec = sqlx::query!(
            r#"
                SELECT * FROM shess_games where channel_id = $1
            "#,
            channel_id
        )
        .fetch_one(&pool)
        .await;

        if rec.is_err() {
            msg.reply(&ctx.http, format!("This channel has no running game"))
                .await?;
            return Ok(());
        }

        let rec = rec.unwrap();

        let mut game = Game {
            id: rec.id,
            channel_id: rec.channel_id,
            players: rec.players,
            running: rec.running,
        };

        if !game.running {
            msg.reply(&ctx.http, format!("Game is not running")).await?;
            return Ok(());
        }

        let mut shess_game = None;

        for (key, value) in &mut lock.default_games {
            if *key == game.id {
                shess_game = Some(value);
            }
        }
        if shess_game.is_none() {
            msg.reply(&ctx.http, format!("Game is not running")).await?;
            return Ok(());
        }

        let mut shess_game = shess_game.unwrap();
        let current_player = shess_game.current_player.0;
        let player_id = shess_game.backend.player_to_backend(current_player);

        let user_id = get_or_create_user_id(msg.author.id.0.to_string(), &pool).await;
        if player_id != user_id {
            msg.reply(&ctx.http, format!("This is not your turn"))
                .await?;
            return Ok(());
        }

        let input = args.message().to_string();

        shess_game.backend.send(input).unwrap();
        shess_game.next_move();

        let board = shess_game.mode.rendered_board();

        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Shess")
                        .description(format!("{}", board))
                        .color(Color::ORANGE)
                })
            })
            .await?;
    }

    Ok(())
}

async fn get_or_create_user_id(discord_id: String, pool: &PgPool) -> i32 {
    let rec = sqlx::query!(
        r#"
                SELECT * FROM users where discord_id = $1
            "#,
        discord_id,
    )
    .fetch_one(pool)
    .await;

    match rec {
        Ok(user) => user.id,
        Err(_) => {
            let mut tx = pool.begin().await.unwrap();
            let user = sqlx::query(
                r#"
                INSERT INTO users (discord_id) VALUES ($1)
                RETURNING id, discord_id
            "#,
            )
            .bind(discord_id)
            .map(|row: PgRow| User {
                id: row.get(0),
                discord_id: row.get(1),
            })
            .fetch_one(&mut tx)
            .await
            .unwrap();

            tx.commit().await.unwrap();
            user.id
        }
    }
}
