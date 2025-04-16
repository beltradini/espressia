use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, instrument};

// Constants could be made public for use in validation
pub const PERFECT_TEMP_MIN: f64 = 90.0;
pub const PERFECT_TEMP_MAX: f64 = 96.0;
pub const PERFECT_PRESS_MIN: f64 = 8.0;
pub const PERFECT_PRESS_MAX: f64 = 10.0;
pub const PERFECT_TIME_MIN: u64 = 20;
pub const PERFECT_TIME_MAX: u64 = 30;

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
    pub fn is_perfect(&self) -> bool {
        let is_perfect = (PERFECT_TEMP_MIN..=PERFECT_TEMP_MAX).contains(&self.temperature)
            && (PERFECT_PRESS_MIN..=PERFECT_PRESS_MAX).contains(&self.pressure)
            && (PERFECT_TIME_MIN..=PERFECT_TIME_MAX).contains(&self.time_seconds);

        debug!(
            temp = self.temperature,
            pressure = self.pressure,
            time = self.time_seconds,
            perfect = is_perfect,
            "Evaluated extraction quality"
        );

        is_perfect
    }
}

#[instrument]
pub fn simulate_extraction(
    temperature: Option<f64>,
    pressure: Option<f64>,
    time_seconds: Option<u64>,
) -> ExtractionMetrics {
    let temp = temperature.unwrap_or(98.6);
    let press = pressure.unwrap_or(1013.25);
    let time = time_seconds.unwrap_or(60);

    debug!(
        temperature = temp,
        pressure = press,
        time = time,
        "Simulating extraction with parameters"
    );

    let metrics = ExtractionMetrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        temperature: temp,
        pressure: press,
        time_seconds: time,
        water_volume_oz: 8.0,
        result: String::new(),
    };

    let is_perfect = metrics.is_perfect();

    ExtractionMetrics {
        result: if is_perfect {
            "Perfect Extraction".to_string()
        } else {
            "Suboptimal Extraction".to_string()
        },
        ..metrics
    }
}

// The tests remain unchanged
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