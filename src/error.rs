//! Error types for the dioxus_use_spacetime library

use std::fmt;

/// Result type for SpacetimeDB operations
pub type Result<T> = std::result::Result<T, SpacetimeError>;

/// Errors that can occur when working with SpacetimeDB
#[derive(Debug, Clone)]
pub enum SpacetimeError {
    /// Connection error
    ConnectionError(String),
    
    /// Table subscription error
    SubscriptionError(String),
    
    /// Context not found error
    ContextNotFound,
    
    /// Serialization/Deserialization error
    SerializationError(String),
    
    /// General error
    Other(String),
}

impl fmt::Display for SpacetimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpacetimeError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            SpacetimeError::SubscriptionError(msg) => write!(f, "Subscription error: {}", msg),
            SpacetimeError::ContextNotFound => write!(f, "SpacetimeDB context not found. Did you call use_spacetime_init?"),
            SpacetimeError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            SpacetimeError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SpacetimeError {}

impl From<String> for SpacetimeError {
    fn from(s: String) -> Self {
        SpacetimeError::Other(s)
    }
}

impl From<&str> for SpacetimeError {
    fn from(s: &str) -> Self {
        SpacetimeError::Other(s.to_string())
    }
}
