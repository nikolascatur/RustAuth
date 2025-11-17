use dotenv::dotenv;

use crate::cli_config::run_cli;

mod cli_config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_cli().await;
}
