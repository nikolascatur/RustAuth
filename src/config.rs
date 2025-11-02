use std::sync::Arc;

use anyhow::Ok;
use sqlx::{
    PgPool,
    postgres::{PgPoolOptions, any},
};

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    return Ok(pool);
}

pub async fn postgress_setup() -> anyhow::Result<PgPool> {
    let load_database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./db.sqlite".to_string());
    println!("nikoo {}", load_database_url);
    return init_db_pool(&load_database_url).await;
}

pub async fn setup_db() -> anyhow::Result<()> {
    let load_database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./db.sqlite".to_string());
    let db = init_db_pool(&load_database_url).await?;
    let shared_pool = Arc::new(db);
    // let ret = Ok(db);
    Ok(())
}
