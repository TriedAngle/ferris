use serenity::framework::standard::CommandResult;
use sqlx::postgres::{PgPoolOptions, PgPool};

pub async fn get_db_pool(db_address: String) -> CommandResult<PgPool> {
    let connection_string = &db_address;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await?;

    Ok(pool)
}
