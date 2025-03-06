mod simulation;
mod api;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Espressia Simulation Server...");

    // Initialize app state with loaded metrics from file
    let app_state = api::AppState::load_metrics().await;

    // Print starting message
    println!("Starting Espressia Simulation Server...");

    // Setup and run server
    api::setup_server(app_state).await?;

    Ok(())
}