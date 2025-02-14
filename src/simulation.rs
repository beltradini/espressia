use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug, Clone)]
pub struct ExtractionMetrics {
    pub timestamp: u64,
    pub temperature: f64,
    pub pressure: f64,
    pub time_seconds: u64,
    pub water_volume_oz: f64,
    pub result: String,
}

// Simulation of metrics extraction personalized for the simulation
pub fn simulate_extraction(
    temperature: Option<f64>,
    pressure: Option<f64>,
    time_seconds: Option<u64>,
) -> ExtractionMetrics {
    let temp = temperature.unwrap_or(98.6); // Default temperature
    let press = pressure.unwrap_or(1013.25); // Default pressure
    let time = time_seconds.unwrap_or(60); // Default time
    let water_volume = 8.0; // Default water volume

    let result = if temp >= 90.0 && temp <= 96.0 && press >= 8.0 && press <= 10.0 && time >= 20 && time <= 30 {
        "Perfect Extraction".to_string()
    } else {
        "Suboptimal Extraction".to_string()
    };

    ExtractionMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        temperature: temp,
        pressure: press,
        time_seconds: time,
        water_volume_oz: water_volume,
        result,
    }
}