mod simulation;
mod api;

use tracing::{info, Level};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting Espressia v1.0.0");
    let app_state = api::AppState::new();
    api::setup_server(app_state).await?;
    Ok(())
}