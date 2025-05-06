use std::error::Error;
use std::{fmt, io};
use std::fmt::Formatter;

// Trail auxiliary imports
trait ErrorMessage {
    fn error_message(&self) -> String;
}

// Implementation of ErrorMessage trait for RepositoryError
impl ErrorMessage for RepositoryError {
    fn error_message(&self) -> String {
        match self {
            RepositoryError::DatabaseError(err) => err.to_string(),
            RepositoryError::SerializationError(err) => err.to_string(),
            RepositoryError::NotFound => "Item not found".to_string(),
        }
    }
}

// Implementation of ErrorMessage trait for NotificationError
impl ErrorMessage for NotificationError {
    fn error_message(&self) -> String {
        match self {
            NotificationError::DatabaseError(err) => err.to_string(),
            NotificationError::SerializationError(err) => err.to_string(),
            NotificationError::NotFound => "Notification not found".to_string(),
            _ => {
                "Network error occurred".to_string()
            }
        }
    }
}

// Implementation of ErrorMessage trait for NetworkError
impl ErrorMessage for NetworkError {
    fn error_message(&self) -> String {
        match self { 
            NetworkError::ConnectionError(err) => err.to_string(),
            NetworkError::TimeoutError(err) => err.to_string(),
            NetworkError::ResponseError(err) => err.to_string(),
        }
    }
}


// Repository Error 
#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(sled::Error),
    SerializationError(serde_json::Error),
    NotFound,
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message())
    }
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RepositoryError::DatabaseError(err) => Some(err),
            RepositoryError::SerializationError(err) => Some(err),
            RepositoryError::NotFound => None,
        }
    }
}

impl From<sled::Error> for RepositoryError {
    fn from(err: sled::Error) -> Self {
        RepositoryError::DatabaseError(err)
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        RepositoryError::SerializationError(err)
    }
}

// Notifications Error 
#[derive(Debug)]
pub enum NotificationError {
    DatabaseError(sled::Error),
    SerializationError(serde_json::Error),
    NotFound,
    NetworkError(String),
}

impl fmt::Display for NotificationError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        write!(_f, "{}", self.error_message())       
    }
}

impl Error for NotificationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NotificationError::DatabaseError(err) => Some(err),
            NotificationError::SerializationError(err) => Some(err),
            NotificationError::NotFound => None,
            _ => {
                None
            }
        }
    }
}

// Network Error 
#[derive(Debug)]

pub enum NetworkError {
    ConnectionError(io::Error),
    TimeoutError(io::Error),
    ResponseError(io::Error),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message())
    }
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetworkError::ConnectionError(err) => Some(err),
            NetworkError::TimeoutError(err) => Some(err),
            NetworkError::ResponseError(err) => Some(err),
        }
    }
}

