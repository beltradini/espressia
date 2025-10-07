use crate::analytics::alerts::Alert;
use crate::analytics::errors::NotificationError;
use axum::response::Response;

pub trait Notifier {
    fn send_alert(&self, alert: &Alert) -> Result<(), NotificationError>;
}

// NetworkError 
enum NetworkError {
    ConnectionError(String),
    TimeoutError(String),
    ResponseError(String),
}

// Config, review the implementation 
pub struct SmtpConfig {
    server: String,
    port: u16,
    username: String,
    password: String,
}

pub struct  EmailNotifier {
    smtp_config: SmtpConfig,
}

pub struct SlackNotifier {
    webhook_url: String,
}

impl Notifier for EmailNotifier {
    fn send_alert(&self, _alert: &Alert) -> Result<(), NotificationError> {
        match self .smtp_config {
            SmtpConfig::Connect => Ok(()),
            SmtpConfig::Disconnect => Ok(())
        }
    }
}

struct Client();
impl Client {
    fn new() -> Self {
        Client()
    }
    
    fn post(&self, _url: &str) -> Self {
        Client()
    }
    
    fn json(&self, _payload: &serde_json::Value) -> Self {
        Client()

    }
    
    pub fn send(&self) -> Result<Response, NotificationError> {
        let response = Response::builder()
            .status(200)
            .body("OK".into())
            .map_err(|_| NotificationError::NetworkError("Failed to send request".to_string()))?;
        
        Ok(response)
    }
}

impl Notifier for SlackNotifier {
    fn send_alert(&self, _alert: &Alert) -> Result<(), NotificationError> {
        let client = Client::new();
        let payload = serde_json::json!({
            "text": "Alert: Extraction quality issue detected!",
        });
        
        let res = client.post(&self.webhook_url)
            .json(&payload)
            .send()
            .map_err(|e| NotificationError::NetworkError(e.to_string()))?;
        
        if res.status().is_success() {
            Ok(())
        } else {
            Err(NotificationError::NetworkError(format!(
                "Failed to send alert: {}",
                res.status()
            )))
        }
    }
}

pub struct  NotificationOrchestrator {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl NotificationOrchestrator {
    pub fn notify(&self, alert: &Alert) -> Result<(), NotificationError> {
        for notifier in &self.notifiers {
            notifier.send_alert(alert)?;
        }
        Ok(())
    }
}