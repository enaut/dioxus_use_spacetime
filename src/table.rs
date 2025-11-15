//! Table subscription and filtering functionality

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A filter for table queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFilter {
    /// Field-value pairs to filter by
    pub conditions: HashMap<String, String>,
}

impl TableFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self {
            conditions: HashMap::new(),
        }
    }

    /// Add a condition to the filter
    pub fn with_condition(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.insert(field.into(), value.into());
        self
    }

    /// Check if a filter is empty
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }
}

impl Default for TableFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a subscription to a SpacetimeDB table
///
/// This maintains the current state of the table data and provides
/// methods to query and update it.
pub struct TableSubscription<T> {
    /// The name of the table
    table_name: String,
    
    /// The current data in the table
    data: Arc<RwLock<Vec<T>>>,
    
    /// Optional filter applied to the subscription
    filter: Option<TableFilter>,
}

impl<T> TableSubscription<T>
where
    T: Clone,
{
    /// Create a new table subscription
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to subscribe to
    /// * `filter` - Optional filter to apply to the subscription
    pub fn new(table_name: impl Into<String>, filter: Option<TableFilter>) -> Self {
        Self {
            table_name: table_name.into(),
            data: Arc::new(RwLock::new(Vec::new())),
            filter,
        }
    }

    /// Get the table name
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Get the current filter
    pub fn filter(&self) -> Option<&TableFilter> {
        self.filter.as_ref()
    }

    /// Get the current data (read-only)
    pub async fn get_data(&self) -> Vec<T> {
        self.data.read().await.clone()
    }

    /// Update the table data
    ///
    /// This would be called when updates are received from SpacetimeDB
    pub async fn update_data(&self, new_data: Vec<T>) {
        let mut data = self.data.write().await;
        *data = new_data;
    }

    /// Add a single row to the table
    pub async fn add_row(&self, row: T) {
        let mut data = self.data.write().await;
        data.push(row);
    }

    /// Clear all data from the table
    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
    }

    /// Get the number of rows
    pub async fn len(&self) -> usize {
        self.data.read().await.len()
    }

    /// Check if the table is empty
    pub async fn is_empty(&self) -> bool {
        self.data.read().await.is_empty()
    }
}

impl<T> Clone for TableSubscription<T> {
    fn clone(&self) -> Self {
        Self {
            table_name: self.table_name.clone(),
            data: Arc::clone(&self.data),
            filter: self.filter.clone(),
        }
    }
}

impl<T> std::fmt::Debug for TableSubscription<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableSubscription")
            .field("table_name", &self.table_name)
            .field("filter", &self.filter)
            .finish()
    }
}
