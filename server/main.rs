use std::sync::Arc;

use crate::{cli_config::run_cli, config::init_db_pool};
use dotenv::dotenv;

mod cli_config;
mod config;
mod model;
mod repository;
mod service;
mod util;

#[tokio::main]
async fn main() {
    // let load_database_url =
    //     std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./db.sqlite".to_string());
    // let pool = init_db_pool(&load_database_url);
    // let shared_pool = Arc::new(pool);
    dotenv().ok();
    run_cli().await;
}
