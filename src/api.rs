use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use serde_json::from_str;
use tokio::{fs, net::TcpListener, sync::Mutex as AsyncMutex};
use crate::simulation::{ExtractionMetrics, simulate_extraction};

#[derive(Debug, Serialize)]
struct ApiError {
    message: String,
    status: u16,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    metrics: MetricsStore,
}

type Result<T> = std::result::Result<T, ApiError>;
type MetricsStore = Arc<AsyncMutex<Vec<ExtractionMetrics>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExtractionParams {
    #[serde(default = "default_temperature")]
    temperature: f64,
    #[serde(default = "default_pressure")]
    pressure: f64,
    #[serde(default = "default_time_seconds")]
    time_seconds: u64,
}

fn default_temperature() -> f64 { 93.0 }
fn default_pressure() -> f64 { 9.0 }
fn default_time_seconds() -> u64 { 25 }

impl ExtractionParams {
    fn validate(&self) -> Result<()> {
        if !(90.0..=96.0).contains(&self.temperature) {
            return Err(ApiError {
                message: "Temperature must be between 90.0 and 96.0".to_string(),
                status: 400,
            });
        }
        if !(8.0..=10.0).contains(&self.pressure) {
            return Err(ApiError {
                message: "Pressure must be between 8.0 and 10.0".to_string(),
                status: 400,
            });
        }
        if !(20..=30).contains(&self.time_seconds) {
            return Err(ApiError {
                message: "Time must be between 20 and 30 seconds".to_string(),
                status: 400,
            });
        }
        Ok(())
    }
}

pub async fn start_extraction(
    State(state): State<AppState>,
    Query(params): Query<ExtractionParams>,
) -> Result<Json<ExtractionMetrics>> {
    params.validate()?;

    let metrics = simulate_extraction(
        Some(params.temperature),
        Some(params.pressure),
        Some(params.time_seconds),
    );

    let mut metrics_store = state.metrics.lock().await;
    metrics_store.push(metrics.clone());
    save_metrics_to_json(&metrics_store).await.map_err(|e| ApiError {
        message: format!("Failed to save metrics: {}", e),
        status: 500,
    })?;

    Ok(Json(metrics))
}

pub async fn get_metrics(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExtractionMetrics>>> {
    let metrics_store = state.metrics.lock().await;
    if metrics_store.is_empty() {
        return Err(ApiError {
            message: "No metrics available".to_string(),
            status: 404,
        });
    }
    Ok(Json(metrics_store.to_vec()))
}

async fn save_metrics_to_json(metrics: &[ExtractionMetrics]) -> std::io::Result<()> {
    const METRICS_FILE: &str = "metrics.json";
    let json = serde_json::to_string_pretty(metrics)?;
    fs::write(METRICS_FILE, json).await?;
    println!("Metrics saved to {METRICS_FILE}");
    Ok(())
}

impl AppState {
    pub async fn load_metrics() -> Self {
        let metrics = match fs::read_to_string("metrics.json").await {
            Ok(content) => from_str::<Vec<ExtractionMetrics>>(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        Self {
            metrics: Arc::new(AsyncMutex::new(metrics)),
        }
    }
}

pub async fn setup_server(app_state: AppState) -> std::io::Result<()> {
    let app = Router::new()
        .route("/start", post(start_extraction))
        .route("/metrics", get(get_metrics))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Espressia running on {addr}");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::load_metrics().await;
    setup_server(app_state).await?;
    Ok(())
}