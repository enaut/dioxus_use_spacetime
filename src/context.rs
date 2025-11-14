//! Context management for SpacetimeDB connection

use crate::connection::SpacetimeConnection;
use crate::error::Result;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

/// The SpacetimeDB context that manages the connection and subscriptions
///
/// This context should be initialized once at the application level and then
/// accessed from child components using `use_spacetime_context`.
#[derive(Clone)]
pub struct SpacetimeContext {
    /// The underlying connection to SpacetimeDB
    connection: SpacetimeConnection,
    
    /// Active table subscriptions
    /// Maps table names to their subscription metadata
    subscriptions: Arc<RwLock<HashMap<String, SubscriptionInfo>>>,
}

/// Information about an active subscription
#[derive(Clone)]
struct SubscriptionInfo {
    /// Number of components subscribed to this table
    ref_count: usize,
}

impl SpacetimeContext {
    /// Create a new SpacetimeDB context
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket URL of the SpacetimeDB instance
    /// * `database` - The name of the database to connect to
    pub fn new(url: impl Into<String>, database: impl Into<String>) -> Self {
        let connection = SpacetimeConnection::new(url, database);
        
        Self {
            connection,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the underlying connection
    pub fn connection(&self) -> &SpacetimeConnection {
        &self.connection
    }

    /// Initialize the connection
    ///
    /// This should be called once when the context is created
    pub async fn initialize(&self) -> Result<()> {
        self.connection.connect().await
    }

    /// Subscribe to a table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to subscribe to
    ///
    /// Returns true if this is a new subscription, false if already subscribed
    pub async fn subscribe(&self, table_name: &str) -> Result<bool> {
        let mut subs = self.subscriptions.write().await;
        
        if let Some(info) = subs.get_mut(table_name) {
            // Already subscribed, just increment ref count
            info.ref_count += 1;
            debug!("Incremented subscription ref count for table '{}' to {}", table_name, info.ref_count);
            Ok(false)
        } else {
            // New subscription
            self.connection.subscribe_table(table_name).await?;
            subs.insert(
                table_name.to_string(),
                SubscriptionInfo { ref_count: 1 },
            );
            debug!("Created new subscription for table '{}'", table_name);
            Ok(true)
        }
    }

    /// Unsubscribe from a table
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to unsubscribe from
    ///
    /// Returns true if the subscription was removed, false if there are still references
    pub async fn unsubscribe(&self, table_name: &str) -> Result<bool> {
        let mut subs = self.subscriptions.write().await;
        
        if let Some(info) = subs.get_mut(table_name) {
            info.ref_count -= 1;
            
            if info.ref_count == 0 {
                // Last reference, remove subscription
                subs.remove(table_name);
                self.connection.unsubscribe_table(table_name).await?;
                debug!("Removed subscription for table '{}'", table_name);
                Ok(true)
            } else {
                debug!("Decremented subscription ref count for table '{}' to {}", table_name, info.ref_count);
                Ok(false)
            }
        } else {
            // Not subscribed
            Ok(false)
        }
    }

    /// Check if currently subscribed to a table
    pub async fn is_subscribed(&self, table_name: &str) -> bool {
        self.subscriptions.read().await.contains_key(table_name)
    }

    /// Get the number of active subscriptions
    pub async fn subscription_count(&self) -> usize {
        self.subscriptions.read().await.len()
    }
}

impl std::fmt::Debug for SpacetimeContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpacetimeContext")
            .field("connection", &self.connection)
            .finish()
    }
}

/// Hook to access the SpacetimeDB context
///
/// This hook provides access to the global SpacetimeDB context that was
/// initialized with `use_spacetime_init`.
///
/// # Panics
///
/// Panics if the context has not been initialized. Make sure to call
/// `use_spacetime_init` in a parent component before using this hook.
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_use_spacetime::*;
///
/// fn MyComponent() -> Element {
///     let context = use_spacetime_context();
///     
///     rsx! {
///         div { "Connected to: {context.connection().database()}" }
///     }
/// }
/// ```
pub fn use_spacetime_context() -> SpacetimeContext {
    match try_consume_context::<SpacetimeContext>() {
        Some(ctx) => ctx,
        None => panic!("SpacetimeDB context not found. Did you call use_spacetime_init in a parent component?"),
    }
}
