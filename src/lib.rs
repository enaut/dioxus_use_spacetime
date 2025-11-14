//! # Dioxus SpacetimeDB Integration
//!
//! A reactive SpacetimeDB client library for Dioxus applications.
//!
//! This library provides Dioxus hooks and context for seamlessly integrating
//! SpacetimeDB into your Dioxus applications with reactive, signal-based updates.
//!
//! ## Features
//!
//! - **Reactive Table Subscriptions**: Use `use_table` to subscribe to SpacetimeDB tables
//! - **Context-based Connection Management**: Initialize once, use everywhere
//! - **Signal-like API**: Familiar Dioxus signal patterns for database operations
//!
//! ## Example
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use dioxus_use_spacetime::*;
//!
//! fn app() -> Element {
//!     // Initialize the SpacetimeDB connection
//!     use_spacetime_init("ws://localhost:3000", "my_database");
//!     
//!     rsx! {
//!         AccountList {}
//!     }
//! }
//!
//! fn AccountList() -> Element {
//!     // Subscribe to the accounts table
//!     let accounts = use_table::<Account>("accounts", None);
//!     
//!     rsx! {
//!         div {
//!             for account in accounts.read().iter() {
//!                 div { "{account.name}" }
//!             }
//!         }
//!     }
//! }
//! # #[derive(Clone)]
//! # struct Account { name: String }
//! ```

mod connection;
mod context;
mod error;
mod hooks;
mod table;

pub use connection::SpacetimeConnection;
pub use context::{use_spacetime_context, SpacetimeContext};
pub use error::{SpacetimeError, Result};
pub use hooks::{use_spacetime_init, use_table, use_table_row};
pub use table::{TableFilter, TableSubscription};

// Re-export commonly used types from spacetimedb-sdk
pub use spacetimedb_sdk;

#[cfg(test)]
mod tests {
    #[test]
    fn test_library_compiles() {
        // Basic compilation test
        assert!(true);
    }
}
