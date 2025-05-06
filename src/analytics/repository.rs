use std::sync::Arc;
use chrono::Utc;
use sled::Db;
// use tracing_subscriber::fmt::format;
use crate::analytics::alerts::Alert;
use crate::analytics::trends::ExtractionTrends;
use crate::analytics::errors::RepositoryError;

pub struct AnalyticsRepository {
    db: Arc<Db>,
}

impl AnalyticsRepository {
    pub(crate) fn new(p0: Arc<Db>) -> Self {
        Self { db: p0 }
    }
}

impl AnalyticsRepository {
    pub fn store_trends(&self, trends: &ExtractionTrends) -> Result<(), RepositoryError> {
        let key = format!("alert_{}", Utc::now().timestamp_millis());
        let serialized = serde_json::to_vec(trends)?;
        self.db.insert(key, serialized)?;
        Ok(())
    }
    
    pub fn store_alerts(&self, alerts: &[Alert]) -> Result<(), RepositoryError> {
        for alert in alerts {
            let key = format!("alert_{}", Utc::now().timestamp_millis());
            let serialized = serde_json::to_vec(alert)?;
            self.db.insert(key, serialized)?;
}
        Ok(())
    }
    
    pub fn get_alerts(&self) -> Result<Vec<Alert>, RepositoryError> {
        let mut alerts = Vec::new();
        for entry in self.db.iter() {
            let (_key, value) = entry?;
            let alert: Alert = serde_json::from_slice(&value)?;
            alerts.push(alert);
        }
        Ok(alerts)
    }
    
    pub fn get_trends(&self) -> Result<Vec<ExtractionTrends>, RepositoryError> {
        let mut trends = Vec::new();
        for entry in self.db.iter() {
            let (_key, value) = entry?;
            let trend: ExtractionTrends = serde_json::from_slice(&value)?;
            trends.push(trend);
        }
        Ok(trends)
    }
    
    pub fn retrieve_trends(&self, key: &str) -> Result<ExtractionTrends, RepositoryError> {
        let value = self.db.get(key)?;
        if let Some(value) = value {
            let trend: ExtractionTrends = serde_json::from_slice(&value)?;
            Ok(trend)
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    
    pub fn retrieve_alerts(&self, key: &str) -> Result<Alert, RepositoryError> {
        let value = self.db.get(key)?;
        if let Some(value) = value {
            let alert: Alert = serde_json::from_slice(&value)?;
            Ok(alert)
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}