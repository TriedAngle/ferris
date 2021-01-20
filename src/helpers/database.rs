use serenity::framework::standard::CommandResult;
use serenity::model::id::GuildId;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn get_db_pool(db_address: String) -> CommandResult<PgPool> {
    let connection_string = &db_address;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await?;

    Ok(pool)
}

pub async fn add_guild(pool: &PgPool, guild_id: GuildId, is_new: bool) -> CommandResult {
    let exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM guild_info WHERE guild_id = $1)",
        guild_id.0 as i64
    )
    .fetch_one(pool)
    .await?;

    if is_new || !exists.exists.unwrap() {
        sqlx::query!(
            "INSERT INTO guild_info VALUES($1) ON CONFLICT DO NOTHING",
            guild_id.0 as i64
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn delete_guild(pool: &PgPool, guild_id: GuildId) -> CommandResult {
    sqlx::query!(
        "DELETE FROM guild_info WHERE guild_id = $1",
        guild_id.0 as i64
    )
    .execute(pool)
    .await?;

    Ok(())
}
