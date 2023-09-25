use config::config::ConfigManager;
use tcp_server::stream::Server;

mod config;
pub mod error;
mod event_handler;
mod tcp_server;
pub mod types;
pub mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = ConfigManager::initialize_or_create().await.unwrap();
    Server::create(config.endpoint)?;

    Ok(())
}
