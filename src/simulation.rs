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

// Coffee Types could be defined in a separate module
#[derive(Serialize, Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum CoffeeType {
    Arabica,
    Robusta,
    Blend,
    SingleOrigin,
}

impl Default for CoffeeType {
    fn default() -> Self {
        CoffeeType::Arabica
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RoastLevel {
    Light,
    Medium,
    Dark,
    ExtraDark,
}

impl Default for RoastLevel {
    fn default() -> Self {
        RoastLevel::Medium
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum GrindSize {
    Coarse,
    Medium,
    Fine,
}

impl Default for GrindSize {
    fn default() -> Self {
        GrindSize::Medium
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ExtractionMetrics {
    pub timestamp: u64,
    pub temperature: f64,
    pub pressure: f64,
    pub time_seconds: u64,
    pub water_volume_oz: f64,
    pub coffee_type: CoffeeType,
    pub roast_level: RoastLevel,
    pub grind_size: GrindSize,
    pub result: String,
    pub extraction_time: f64,
    pub perfect_extraction_rate: f64,
    pub quality_score: u8,
    pub recommendations: Vec<String>,
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

    pub(crate) fn is_good(&self) -> bool {
        self.is_perfect()
            || ((PERFECT_TEMP_MIN..=PERFECT_TEMP_MAX).contains(&self.temperature)
                && (PERFECT_PRESS_MIN..=PERFECT_PRESS_MAX).contains(&self.pressure))
            || ((PERFECT_TEMP_MIN..=PERFECT_TEMP_MAX).contains(&self.temperature)
                && (PERFECT_PRESS_MIN..=PERFECT_PRESS_MAX).contains(&self.pressure)
                && (PERFECT_TIME_MIN..=PERFECT_TIME_MAX).contains(&self.time_seconds))
    }

    // New algorithm for quality score
    pub fn calculate_quality_score(&self) -> u8 {
        let temp_score = if (PERFECT_TEMP_MIN..=PERFECT_TEMP_MAX).contains(&self.temperature) {
            30
        } else if self.temperature < PERFECT_TEMP_MIN {
            (30.0 * (1.0 - (self.temperature / PERFECT_TEMP_MIN))) as u8
        } else {
            (30.0 * ((PERFECT_TEMP_MAX - self.temperature) / (PERFECT_TEMP_MAX - PERFECT_TEMP_MIN)))
                as u8
        };

        let press_score = if (PERFECT_PRESS_MIN..=PERFECT_PRESS_MAX).contains(&self.pressure) {
            30
        } else if self.pressure < PERFECT_PRESS_MIN {
            (30.0 * (1.0 - (self.pressure / PERFECT_PRESS_MIN))) as u8
        } else {
            (30.0 * ((PERFECT_PRESS_MAX - self.pressure) / (PERFECT_PRESS_MAX - PERFECT_PRESS_MIN)))
                as u8
        };

        let time_score = if (PERFECT_TIME_MIN..=PERFECT_TIME_MAX).contains(&self.time_seconds) {
            30
        } else if self.time_seconds < PERFECT_TIME_MIN {
            (30.0 * (1.0 - (self.time_seconds as f64 / PERFECT_TIME_MIN as f64))) as u8
        } else {
            (30.0
                * ((PERFECT_TIME_MAX as f64 - self.time_seconds as f64)
                    / (PERFECT_TIME_MAX as f64 - PERFECT_TIME_MIN as f64))) as u8
        };

        // Add bonus points for perfect extraction
        let mut bonus = 0;

        // Bonus for coffee type and roast level optimization
        match (&self.coffee_type, &self.roast_level) {
            (CoffeeType::Arabica, RoastLevel::Medium) => bonus += 5,
            (CoffeeType::Robusta, RoastLevel::Dark) => bonus += 5,
            (CoffeeType::Blend, RoastLevel::Medium) => bonus += 5,
            (CoffeeType::SingleOrigin, RoastLevel::Light) => bonus += 5,
            _ => {}
        }

        // Bonus for grind size optimization
        match (&self.grind_size, self.time_seconds) {
            (GrindSize::Coarse, t) if t < 20 => bonus += 5,
            (GrindSize::Medium, t) if t >= 20 && t <= 30 => bonus += 5,
            (GrindSize::Fine, t) if t > 30 => bonus += 5,
            _ => {}
        }

        let final_score = (temp_score + press_score + time_score) + bonus;
        final_score
    }

    // Recommendations Generator based on quality score
    pub fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Recommendations based on temperature
        if self.temperature < PERFECT_TEMP_MIN {
            recommendations.push(format!(
                "Increase temperature to {} degrees",
                PERFECT_TEMP_MIN
            ));
        } else if self.temperature > PERFECT_TEMP_MAX {
            recommendations.push(format!(
                "Decrease temperature to {} degrees",
                PERFECT_TEMP_MAX
            ));
        }

        // Recommendations based on pressure
        if self.pressure < PERFECT_PRESS_MIN {
            recommendations.push(format!("Increase pressure to {} psi", PERFECT_PRESS_MIN));
        } else if self.pressure > PERFECT_PRESS_MAX {
            recommendations.push(format!("Decrease pressure to {} psi", PERFECT_PRESS_MAX));
        }

        // Recommendations based on time
        if self.time_seconds < PERFECT_TIME_MIN {
            recommendations.push(format!(
                "Increase extraction time to {} seconds",
                PERFECT_TIME_MIN
            ));
        } else if self.time_seconds > PERFECT_TIME_MAX {
            recommendations.push(format!(
                "Decrease extraction time to {} seconds",
                PERFECT_TIME_MAX
            ));
        }

        // Recommendations based on grind size and time
        match self.grind_size {
            GrindSize::Fine if self.time_seconds > 30 => {
                recommendations
                    .push("Consider using a coarser grind size for better extraction".to_string());
            }
            GrindSize::Coarse if self.time_seconds < 20 => {
                recommendations
                    .push("Consider using a finer grind size for better extraction".to_string());
            }
            _ => {}
        }

        // Recommendations based on coffee type and roast level
        match (&self.coffee_type, &self.roast_level) {
            (CoffeeType::Arabica, RoastLevel::Medium) => {
                recommendations
                    .push("Consider using a lighter roast for a more delicate flavor".to_string());
            }
            (CoffeeType::Robusta, RoastLevel::Dark) => {
                recommendations
                    .push("Consider using a medium roast for a balanced flavor".to_string());
            }
            (CoffeeType::Blend, RoastLevel::Medium) => {
                recommendations.push("Consider using a dark roast for a bolder flavor".to_string());
            }
            (CoffeeType::SingleOrigin, RoastLevel::Light) => {
                recommendations
                    .push("Consider using a medium roast for a more balanced flavor".to_string());
            }
            _ => {}
        }

        recommendations
    }
}

#[instrument]
pub fn simulate_extraction(
    temperature: Option<f64>,
    pressure: Option<f64>,
    time_seconds: Option<u64>,
    coffee_type: Option<CoffeeType>,
    roast_level: Option<RoastLevel>,
    grind_size: Option<GrindSize>,
) -> ExtractionMetrics {
    let temp = temperature.unwrap_or(98.6);
    let press = pressure.unwrap_or(1013.25);
    let time = time_seconds.unwrap_or(60);
    let coffee = coffee_type.unwrap_or(CoffeeType::default());
    let roast = roast_level.unwrap_or(RoastLevel::default());
    let grind = grind_size.unwrap_or(GrindSize::default());

    debug!(
        temperature = temp,
        pressure = press,
        time = time,
        coffee_type = ?coffee,
        roast_level = ?roast,
        grind_size = ?grind,
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
        extraction_time: 0.0,
        perfect_extraction_rate: 0.0,
        coffee_type: coffee,
        roast_level: roast,
        grind_size: grind,
        quality_score: 0,
        recommendations: Vec::new(),
    };

    let is_perfect = metrics.is_perfect();
    let quality_score = metrics.calculate_quality_score();
    let recommendations = metrics.generate_recommendations();

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
            extraction_time: 0.0,
            perfect_extraction_rate: 1.0,
            coffee_type: CoffeeType::Arabica,
            roast_level: RoastLevel::Medium,
            grind_size: GrindSize::Medium,
            quality_score: 0,
            recommendations: Vec::new(),
        };
        assert!(perfect_metrics.is_perfect());
    }

    #[test]
    fn test_simulate_extraction() {
        let metrics = simulate_extraction(Some(95.0), Some(9.0), Some(25), None, None, None);
        assert_eq!(metrics.result, "Perfect Extraction");

        let metrics = simulate_extraction(Some(100.0), Some(9.0), Some(25), None, None, None);
        assert_eq!(metrics.result, "Suboptimal Extraction");

        let metrics = simulate_extraction(Some(92.0), Some(12.0), Some(25), None, None, None);
        assert_eq!(metrics.result, "Suboptimal Extraction");

        let metrics = simulate_extraction(Some(92.0), Some(9.0), Some(15), None, None, None);
        assert_eq!(metrics.result, "Suboptimal Extraction");
        let metrics = simulate_extraction(Some(92.0), Some(9.0), Some(35), None, None, None);
        assert_eq!(metrics.result, "Suboptimal Extraction");
    }

    #[test]
    fn test_quality_score() {
        let perfect_metrics = ExtractionMetrics {
            timestamp: 0,
            temperature: 92.0,
            pressure: 9.0,
            time_seconds: 25,
            water_volume_oz: 8.0,
            coffee_type: CoffeeType::Arabica,
            roast_level: RoastLevel::Medium,
            grind_size: GrindSize::Medium,
            result: String::new(),
            extraction_time: 0.0,
            perfect_extraction_rate: 1.0,
            quality_score: 0,
            recommendations: Vec::new(),
        };

        let score = perfect_metrics.calculate_quality_score();
        assert_eq!(score, 95); // 30 + 30 + 30 + 5 (bonus por arabica + medium)
    }

    #[test]
    fn test_recommendations() {
        let suboptimal_metrics = ExtractionMetrics {
            timestamp: 0,
            temperature: 88.0,
            pressure: 7.0,
            time_seconds: 35,
            water_volume_oz: 8.0,
            coffee_type: CoffeeType::Robusta,
            roast_level: RoastLevel::Light,
            grind_size: GrindSize::Coarse,
            result: String::new(),
            extraction_time: 0.0,
            perfect_extraction_rate: 0.0,
            quality_score: 0,
            recommendations: Vec::new(),
        };

        let recommendations = suboptimal_metrics.generate_recommendations();
        assert!(recommendations.len() >= 3);
    }
}
