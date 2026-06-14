use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use shared_common::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Starting Orbis Server Binary Mode");

    // 2. Load Configuration
    let cfg = Config::load();

    // 3. Start Server from library
    api_gateway::start_server(&cfg.database_url, cfg.port).await?;

    Ok(())
}
