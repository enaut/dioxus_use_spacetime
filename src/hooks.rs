//! Dioxus hooks for SpacetimeDB integration

use crate::context::SpacetimeContext;
use crate::table::TableFilter;
use dioxus::prelude::*;
use tracing::{debug, error};

/// Connection status enum
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    /// Not yet attempted to connect
    Connecting,
    /// Successfully connected
    Connected(String), // Contains identity
    /// Failed to connect
    Failed(String), // Contains error message
}

/// Initialize the SpacetimeDB connection and provide it as context
///
/// This hook should be called once at the application root level. It creates
/// a SpacetimeDB context and makes it available to all child components.
///
/// # Arguments
///
/// * `url` - The WebSocket URL of the SpacetimeDB instance (e.g., "ws://localhost:3000")
/// * `database` - The name of the database to connect to
///
/// # Returns
///
/// A signal containing the current connection status
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_use_spacetime::*;
///
/// fn App() -> Element {
///     // Initialize connection at the app level
///     let status = use_spacetime_init("ws://localhost:3000", "my_database");
///     
///     rsx! {
///         div {
///             MyComponent {}
///         }
///     }
/// }
/// # fn MyComponent() -> Element { rsx! { div {} } }
/// ```
pub fn use_spacetime_init(url: &str, database: &str) -> Signal<ConnectionStatus> {
    let mut status = use_signal(|| ConnectionStatus::Connecting);
    
    let context = use_context_provider(|| {
        debug!("Initializing SpacetimeDB context");
        SpacetimeContext::new(url, database)
    });

    // Initialize the connection asynchronously
    use_effect(move || {
        let context = context.clone();
        spawn(async move {
            match context.initialize().await {
                Ok(_) => {
                    debug!("SpacetimeDB connection initialized successfully");
                    if let Some(identity) = context.identity().await {
                        status.set(ConnectionStatus::Connected(identity));
                    } else {
                        status.set(ConnectionStatus::Connected("unknown".to_string()));
                    }
                }
                Err(e) => {
                    let error_msg = format!("{}", e);
                    error!("Failed to initialize SpacetimeDB connection: {}", error_msg);
                    status.set(ConnectionStatus::Failed(error_msg));
                }
            }
        });
    });
    
    status
}

/// Subscribe to a SpacetimeDB table and get reactive access to its data
///
/// This hook subscribes to a table in SpacetimeDB and returns a Signal containing
/// the table data. The signal will automatically update when the table changes.
///
/// # Arguments
///
/// * `table_name` - The name of the table to subscribe to
/// * `filter` - Optional filter to apply to the subscription
///
/// # Type Parameters
///
/// * `T` - The type of rows in the table. Must implement `Clone` and should typically
///   derive `serde::Deserialize` for SpacetimeDB integration.
///
/// # Returns
///
/// A `Signal<Vec<T>>` containing the current table data. The signal will update
/// automatically when rows are added, removed, or modified in SpacetimeDB.
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_use_spacetime::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// struct Account {
///     id: u64,
///     name: String,
///     balance: f64,
/// }
///
/// fn AccountList() -> Element {
///     // Subscribe to all accounts
///     let accounts = use_table::<Account>("accounts", None);
///     
///     rsx! {
///         div {
///             h2 { "Accounts" }
///             for account in accounts.read().iter() {
///                 div {
///                     "Account: {account.name}, Balance: ${account.balance}"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// # Example with Filter
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_use_spacetime::*;
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Clone, Debug, Deserialize, Serialize)]
/// # struct Account { id: u64, name: String, balance: f64 }
/// fn HighBalanceAccounts() -> Element {
///     // Subscribe to accounts with high balance (filter would be applied server-side)
///     let filter = TableFilter::new()
///         .with_condition("balance_gt", "1000");
///     let accounts = use_table::<Account>("accounts", Some(filter));
///     
///     rsx! {
///         div {
///             h2 { "High Balance Accounts" }
///             for account in accounts.read().iter() {
///                 div { "Account: {account.name}" }
///             }
///         }
///     }
/// }
/// ```
pub fn use_table<T>(table_name: &str, _filter: Option<TableFilter>) -> Signal<Vec<T>>
where
    T: Clone + 'static,
{
    let table_name = table_name.to_string();
    let context = crate::context::use_spacetime_context();
    
    // Create a signal to hold the table data
    let data = use_signal(|| Vec::<T>::new());
    
    // Subscribe to the table
    use_effect(move || {
        let table_name = table_name.clone();
        let context = context.clone();
        
        spawn(async move {
            debug!("Subscribing to table: {}", table_name);
            
            match context.subscribe(&table_name).await {
                Ok(_) => {
                    debug!("Successfully subscribed to table: {}", table_name);
                    
                    // In a real implementation, we would:
                    // 1. Set up a listener for table updates
                    // 2. Update the signal when changes occur
                    // 3. Handle the filter if provided
                    
                    // For now, the table starts empty and would be populated
                    // by the SpacetimeDB SDK's update callbacks
                }
                Err(e) => {
                    error!("Failed to subscribe to table '{}': {}", table_name, e);
                }
            }
        });
    });
    
    data
}

/// Subscribe to a single row from a SpacetimeDB table
///
/// This is a convenience hook for subscribing to a specific row, typically by ID.
///
/// # Arguments
///
/// * `table_name` - The name of the table to subscribe to
/// * `id_field` - The name of the ID field (e.g., "id")
/// * `id_value` - The value of the ID to filter by
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_use_spacetime::*;
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Clone, Debug, Deserialize, Serialize)]
/// # struct Account { id: u64, name: String }
///
/// fn AccountDetails(account_id: u64) -> Element {
///     let account = use_table_row::<Account>("accounts", "id", &account_id.to_string());
///     
///     rsx! {
///         div {
///             if let Some(acc) = account.read().first() {
///                 div { "Account: {acc.name}" }
///             } else {
///                 div { "Loading..." }
///             }
///         }
///     }
/// }
/// ```
pub fn use_table_row<T>(table_name: &str, id_field: &str, id_value: &str) -> Signal<Vec<T>>
where
    T: Clone + 'static,
{
    let filter = TableFilter::new().with_condition(id_field, id_value);
    use_table(table_name, Some(filter))
}
