use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const PERFECT_TEMP_MIN: f64 = 90.0;
const PERFECT_TEMP_MAX: f64 = 96.0;
const PERFECT_PRESS_MIN: f64 = 8.0;
const PERFECT_PRESS_MAX: f64 = 10.0;
const PERFECT_TIME_MIN: u64 = 20;
const PERFECT_TIME_MAX: u64 = 30;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ExtractionMetrics {
    pub timestamp: u64,
    pub temperature: f64,
    pub pressure: f64,
    pub time_seconds: u64,
    pub water_volume_oz: f64,
    pub result: String,
}
impl ExtractionMetrics {
    fn is_perfect(&self) -> bool {
        (PERFECT_TEMP_MIN..=PERFECT_TEMP_MAX).contains(&self.temperature)
            && (PERFECT_PRESS_MIN..=PERFECT_PRESS_MAX).contains(&self.pressure)
            && (PERFECT_TIME_MIN..=PERFECT_TIME_MAX).contains(&self.time_seconds)
    }
}

pub fn simulate_extraction(
    temperature: Option<f64>,
    pressure: Option<f64>,
    time_seconds: Option<u64>,
) -> ExtractionMetrics {
    let metrics = ExtractionMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        temperature: temperature.unwrap_or(98.6),
        pressure: pressure.unwrap_or(1013.25),
        time_seconds: time_seconds.unwrap_or(60),
        water_volume_oz: 8.0,
        result: String::new(),
    };

    ExtractionMetrics {
        result: if metrics.is_perfect() {
            "Perfect Extraction".to_string()
        } else {
            "Suboptimal Extraction".to_string()
        },
        ..metrics
    }
}

// Test the simulation function
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_metrics_is_perfect() {
        let perfect_metrics = ExtractionMetrics {
            timestamp: 0,
            temperature: 92.0,
            pressure: 9.0,
            time_seconds: 25,
            water_volume_oz: 8.0,
            result: String::new(),
        };
        assert!(perfect_metrics.is_perfect());
    }
    #[test]
    fn test_simulate_extraction() {
        let metrics = simulate_extraction(Some(95.0), Some(9.0), Some(25));
        assert_eq!(metrics.result, "Perfect Extraction");

        let metrics = simulate_extraction(Some(100.0), Some(9.0), Some(25));
        assert_eq!(metrics.result, "Suboptimal Extraction");

        let metrics = simulate_extraction(Some(92.0), Some(12.0), Some(25));
        assert_eq!(metrics.result, "Suboptimal Extraction");

        let metrics = simulate_extraction(Some(92.0), Some(9.0), Some(15));
        assert_eq!(metrics.result, "Suboptimal Extraction");
    }
}