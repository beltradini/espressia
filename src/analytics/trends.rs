use crate::simulation::ExtractionMetrics;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractionTrends {
    pub period: TrendPeriod,
    pub perfect_extraction_rate: f64,
    pub avg_metrics: AverageMetrics,
    pub trend_direction: TrendDirection,
    pub quality_distribution: QualityDistribution,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TrendPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AverageMetrics {
    pub temperature: f64,
    pub pressure: f64,
    pub extraction_time: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualityDistribution {
    pub perfect: u32,
    pub good: u32,
    pub suboptimal: u32,
}
impl ExtractionTrends {
    pub fn calculate(metrics: &[ExtractionMetrics], period: TrendPeriod) -> Self {
        Self {
            period,
            perfect_extraction_rate: Self::calculate_perfect_extraction_rate(metrics),
            avg_metrics: Self::calculate_average_metrics(metrics),
            trend_direction: Self::calculate_trend_direction(metrics),
            quality_distribution: Self::calculate_quality_distribution(metrics),
        }
    }

    fn calculate_perfect_extraction_rate(metrics: &[ExtractionMetrics]) -> f64 {
        let perfect_count = metrics.iter().filter(|m| m.is_perfect()).count();
        (perfect_count as f64 / metrics.len() as f64) * 100.0
    }

    fn calculate_average_metrics(metrics: &[ExtractionMetrics]) -> AverageMetrics {
        let total = metrics.len() as f64;
        if total == 0.0 {
            
            return AverageMetrics {
                temperature: 0.0,
                pressure: 0.0,
                extraction_time: 0.0,
            };
        }
        
        let sum_temperature: f64 = metrics.iter().map(|m| m.temperature).sum();
        let sum_pressure: f64 = metrics.iter().map(|m| m.pressure).sum();
        let sum_extraction_time: f64 = metrics.iter().map(|m| m.extraction_time).sum();
        
        AverageMetrics {
            temperature: sum_temperature / total,
            pressure: sum_pressure / total,
            extraction_time: sum_extraction_time / total,
        }
    }

    fn calculate_trend_direction(metrics: &[ExtractionMetrics]) -> TrendDirection {
        let perfect_rate = Self::calculate_perfect_extraction_rate(metrics);
        match perfect_rate {
            rate if rate > 75.0 => TrendDirection::Improving,
            rate if rate > 50.0 => TrendDirection::Stable,
            _ => TrendDirection::Declining,
        }
    }

    fn calculate_quality_distribution(metrics: &[ExtractionMetrics]) -> QualityDistribution {
        QualityDistribution {
            perfect: metrics.iter().filter(|m| m.is_perfect()).count() as u32,
            good: metrics.iter().filter(|m| m.is_good()).count() as u32,
            suboptimal: metrics.iter().filter(|m| !m.is_perfect() && !m.is_good()).count() as u32,
        }
    }
}