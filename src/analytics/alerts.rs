use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;
use crate::simulation::ExtractionMetrics;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub severity: AlertSeverity,
    pub message: String,
    pub category: AlertCategory,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlertCategory {
    ExtractionQuality,
    ParameterDeviation,
    PerformanceTrend,
    SystemHealth,
}

pub struct  AlertGenerator {
    rules: Vec<AlertRule>,
}

pub struct AlertRule {
    pub name: String,
    pub condition: Box<dyn Fn(&ExtractionMetrics) -> Option<Alert>>,
}

impl AlertGenerator {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Self::low_perfection_rate_rule(),
                Self::temperature_deviation_rule(),
                Self::pressure_instability_rule(),
            ]
        }
    }

    fn low_perfection_rate_rule() -> AlertRule {
        AlertRule {
            name: "Low Perfect Extraction Rate".to_string(),
            condition: Box::new(|metrics| {
                if metrics.perfect_extraction_rate < 0.4 {
                    Some(Alert {
                        id: Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        severity: AlertSeverity::Warning,
                        category: AlertCategory::ExtractionQuality,
                        message: "Low perfect extraction rate detected.".to_string(),
                        metadata: Some(json!({
                            "perfect_rate": metrics.perfect_extraction_rate,
                        }))
                    })
                } else {
                    None
                }
            }),
        }
    }

    fn temperature_deviation_rule() -> AlertRule {
        AlertRule {
            name: "Temperature Deviation".to_string(),
            condition: Box::new(|metrics| {
                if metrics.temperature < 90.0 || metrics.temperature > 96.0 {
                    Some(Alert {
                        id: Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        severity: AlertSeverity::Critical,
                        category: AlertCategory::ParameterDeviation,
                        message: "Temperature outside acceptable range".to_string(),
                        metadata: Some(json!({
                            "temperature": metrics.temperature,
                        }))
                    })
                } else {
                    None
                }
            }),
        }
    }

    fn pressure_instability_rule() -> AlertRule {
        AlertRule {
            name: "Pressure Instability".to_string(),
            condition: Box::new(|metrics| {
                if metrics.pressure < 8.0 || metrics.pressure > 10.0 {
                    Some(Alert {
                        id: Uuid::new_v4().to_string(),
                        timestamp: Utc::now(),
                        severity: AlertSeverity::Warning,
                        category: AlertCategory::ParameterDeviation,
                        message: "Pressure outside stable range".to_string(),
                        metadata: Some(json!({
                            "pressure": metrics.pressure,
                        }))
                    })
                } else {
                    None
                }
            }),
        }
    }

    pub fn generate_alerts(&self, metrics: &ExtractionMetrics) -> Vec<Alert> {
        self.rules
            .iter()
            .filter_map(|rule| (rule.condition)(metrics))
            .collect()
    }
}