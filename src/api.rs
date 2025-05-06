use axum::{
    routing::{get, post},
    Router,
    extract::State as AxumState,
    Json,
};
use serde::{Serialize, Deserialize};
use tracing::{info, error, debug};
use chrono::Utc;
use sled::Db;
use crate::analytics::trends::{ExtractionTrends, TrendPeriod};
use crate::analytics::alerts::Alert;
use crate::analytics::repository::AnalyticsRepository;
use crate::simulation::{ExtractionMetrics, simulate_extraction};
use axum::{
    extract::{Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    //routing::{get, post},
    //Json,
    //Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[derive(Debug, Serialize)]
pub struct ApiError {
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
    db: Arc<Db>,
}

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtractionParams {
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_pressure")]
    pub pressure: f64,
    #[serde(default = "default_time_seconds")]
    pub time_seconds: u64,
}

fn default_temperature() -> f64 { 93.0 }
fn default_pressure() -> f64 { 9.0 }
fn default_time_seconds() -> u64 { 25 }

impl ExtractionParams {
    pub fn validate(&self) -> Result<()> {
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
    AxumState(state): AxumState<AppState>,
    Query(params): Query<ExtractionParams>,
) -> Result<Json<ExtractionMetrics>> {
    debug!("Received extraction request: {:?}", params);

    params.validate()?;

    let metrics = simulate_extraction(
        Some(params.temperature),
        Some(params.pressure),
        Some(params.time_seconds),
    );

    info!("Simulated extraction with temp={}, pressure={}, time={}",
        params.temperature, params.pressure, params.time_seconds);

    let metrics_bytes = serde_json::to_vec(&metrics).map_err(|e| {
        error!("Failed to serialize metrics: {}", e);
        ApiError {
            message: format!("Failed to serialize metrics: {}", e),
            status: 500,
        }
    })?;

    let key = format!("metric_{}", Utc::now().timestamp_millis());
    state.db.insert(key.as_bytes(), metrics_bytes).map_err(|e| {
        error!("Failed to store metrics in sled: {}", e);
        ApiError {
            message: format!("Failed to store metrics: {}", e),
            status: 500,
        }
    })?;
    debug!("Stored metrics with key: {}", key);

    Ok(Json(metrics))
}

pub async fn get_metrics(
    AxumState(state): AxumState<AppState>,
) -> Result<Json<Vec<ExtractionMetrics>>> {
    let mut metrics = Vec::new();
    for entry in state.db.iter() {
        let (_key, value) = entry.map_err(|e| {
            error!("Failed to read from sled: {}", e);
            ApiError {
                message: format!("Failed to read metrics: {}", e),
                status: 500,
            }
        })?;
        let metric: ExtractionMetrics = serde_json::from_slice(&value).map_err(|e| {
            error!("Failed to deserialize metric: {}", e);
            ApiError {
                message: format!("Failed to deserialize metric: {}", e),
                status: 500,
            }
        })?;
        metrics.push(metric);
    }

    if metrics.is_empty() {
        info!("No metrics available in sled");
        return Err(ApiError {
            message: "No metrics available".to_string(),
            status: 404,
        });
    }

    info!("Returning {} stored metrics", metrics.len());
    Ok(Json(metrics))
}

// Trends endpoint
pub async fn get_trends(
    AxumState(state): AxumState<AppState>,
    Query(_period): Query<TrendPeriod>
) -> Result<Json<Vec<ExtractionTrends>>> {
    let repository = AnalyticsRepository::new(state.db.clone());
    let trends = repository.get_trends()
        .map_err(|e| {
        error!("Error fetching trends: {:?}", e);
        ApiError {
            message:"Error fetching trends".to_string(),
            status: 500,
        }
    })?;
    Ok(Json(trends))
}

// Alerts endpoint
pub async fn get_alerts(
    AxumState(state): AxumState<AppState>
) -> Result<Json<Vec<Alert>>> {
    let repository = AnalyticsRepository::new(state.db.clone());
    let alerts = repository.get_alerts()
        .map_err(|e|{
        error!("Error fetching alerts: {:?}", e);
        ApiError {
            message: "Error fetching alerts".to_string(),
            status: 500,
        }
    })?;
    Ok(Json(alerts))
}

impl AppState {
    pub fn new() -> Self {
        let db_config = sled::Config::new()
            .path("espressia_metrics_db")
            .use_compression(true)
            .mode(sled::Mode::HighThroughput);

        let db = db_config.open().expect("Failed to open sled database");
        info!("Initialized sled database at espressia_metrics_db");
        Self { db: Arc::new(db) }
    }
}

pub async fn setup_server(app_state: AppState) -> std::io::Result<()> {
    let app = Router::new()
        .route("/start", post(start_extraction))
        .route("/metrics", get(get_metrics))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Espressia running on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}