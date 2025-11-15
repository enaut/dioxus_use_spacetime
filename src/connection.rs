//! SpacetimeDB connection management

use crate::error::{Result, SpacetimeError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Represents a connection to a SpacetimeDB instance
#[derive(Clone)]
pub struct SpacetimeConnection {
    url: String,
    database: String,
    connected: Arc<RwLock<bool>>,
    identity: Arc<RwLock<Option<String>>>,
}

impl SpacetimeConnection {
    /// Create a new SpacetimeDB connection
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket URL of the SpacetimeDB instance (e.g., "ws://localhost:3000")
    /// * `database` - The name of the database to connect to
    pub fn new(url: impl Into<String>, database: impl Into<String>) -> Self {
        let url = url.into();
        let database = database.into();
        
        info!("Creating SpacetimeDB connection to {} (database: {})", url, database);
        
        Self {
            url,
            database,
            connected: Arc::new(RwLock::new(false)),
            identity: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the connection URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the database name
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Check if the connection is active
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    /// Get the identity of the current connection
    /// Returns None if not connected or identity not available
    pub async fn identity(&self) -> Option<String> {
        self.identity.read().await.clone()
    }

    /// Connect to the SpacetimeDB instance
    ///
    /// Attempts to establish a real WebSocket connection to SpacetimeDB
    pub async fn connect(&self) -> Result<()> {
        debug!("Connecting to SpacetimeDB at {}", self.url);
        
        // Try to establish a WebSocket connection to test if SpacetimeDB is running
        use tokio_tungstenite::connect_async;
        
        match connect_async(&self.url).await {
            Ok((mut ws_stream, _)) => {
                info!("WebSocket connection established to SpacetimeDB");
                
                // For now, we'll just verify the connection works
                // In a real implementation with generated bindings, you would:
                // 1. Send proper authentication/connection init message
                // 2. Wait for IdentityToken response
                // 3. Store the identity
                
                // Generate a connection-test identity for now
                // In production, this would come from the IdentityToken message
                use spacetimedb_lib::Identity as StdbIdentity;
                let test_identity = StdbIdentity::from_claims("local", "guest");
                let identity_str = test_identity.to_hex().to_string();
                
                // Close the test connection
                let _ = ws_stream.close(None).await;
                
                let mut identity = self.identity.write().await;
                *identity = Some(identity_str);
                
                let mut connected = self.connected.write().await;
                *connected = true;
                
                info!("Successfully connected to SpacetimeDB");
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to SpacetimeDB: {}", e);
                Err(SpacetimeError::ConnectionError(
                    format!("Cannot connect to SpacetimeDB at {}. Is SpacetimeDB running? Error: {}", self.url, e)
                ))
            }
        }
    }

    /// Disconnect from the SpacetimeDB instance
    pub async fn disconnect(&self) -> Result<()> {
        debug!("Disconnecting from SpacetimeDB");
        
        let mut connected = self.connected.write().await;
        *connected = false;
        
        let mut identity = self.identity.write().await;
        *identity = None;
        
        info!("Disconnected from SpacetimeDB");
        Ok(())
    }

    /// Subscribe to a table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to subscribe to
    pub async fn subscribe_table(&self, table_name: &str) -> Result<()> {
        if !self.is_connected().await {
            return Err(SpacetimeError::ConnectionError(
                "Not connected to SpacetimeDB".to_string(),
            ));
        }

        debug!("Subscribing to table: {}", table_name);
        
        // In a real implementation, this would:
        // 1. Send subscription request to SpacetimeDB
        // 2. Set up listeners for table updates
        // 3. Return subscription handle
        
        Ok(())
    }

    /// Unsubscribe from a table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to unsubscribe from
    pub async fn unsubscribe_table(&self, table_name: &str) -> Result<()> {
        debug!("Unsubscribing from table: {}", table_name);
        
        // In a real implementation, this would:
        // 1. Send unsubscription request to SpacetimeDB
        // 2. Clean up listeners
        
        Ok(())
    }
}

impl std::fmt::Debug for SpacetimeConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpacetimeConnection")
            .field("url", &self.url)
            .field("database", &self.database)
            .finish()
    }
}
