mod simulation;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize app state with loaded metrics from file
    let app_state = api::AppState::load_metrics().await;

    // Print starting message
    println!("Starting Mastrena 3.0 Simulation Server...");

    // Setup and run server
    api::setup_server(app_state).await?;

    Ok(())
}