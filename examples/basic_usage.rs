//! Basic example demonstrating how to use dioxus_use_spacetime
//!
//! This example shows:
//! - How to initialize the SpacetimeDB connection
//! - How to subscribe to tables with use_table
//! - How to display reactive data in Dioxus components
//!
//! To run this example:
//! ```bash
//! cargo run --example basic_usage
//! ```

use dioxus::prelude::*;
use dioxus_use_spacetime::*;
use serde::{Deserialize, Serialize};

// Define our data structures that match SpacetimeDB tables
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Account {
    id: u64,
    name: String,
    balance: f64,
    created_at: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Transaction {
    id: u64,
    account_id: u64,
    amount: f64,
    description: String,
    timestamp: u64,
}

fn main() {
    // Launch the Dioxus app
    dioxus::launch(App);
}

/// Root application component
fn App() -> Element {
    // Initialize the SpacetimeDB connection
    // In a real application, these would come from configuration
    let connection_status = use_spacetime_init("ws://localhost:3000", "my_database");

    rsx! {
        div {
            style: "font-family: Arial, sans-serif; padding: 20px;",
            
            h1 { "SpacetimeDB + Dioxus Example" }
            
            // Display connection status
            ConnectionStatusBanner { status: connection_status }
            
            p {
                "This example demonstrates reactive SpacetimeDB integration with Dioxus. "
                "The data below will automatically update when changes occur in SpacetimeDB."
            }
            
            // Display accounts
            AccountsSection {}
            
            // Display recent transactions
            TransactionsSection {}
        }
    }
}

/// Component that displays the connection status
#[component]
fn ConnectionStatusBanner(status: Signal<ConnectionStatus>) -> Element {
    rsx! {
        div {
            style: match status() {
                ConnectionStatus::Connecting => "padding: 15px; margin: 20px 0; background-color: #fff3cd; border: 1px solid #ffc107; border-radius: 5px; color: #856404;",
                ConnectionStatus::Connected(_) => "padding: 15px; margin: 20px 0; background-color: #d4edda; border: 1px solid #28a745; border-radius: 5px; color: #155724;",
                ConnectionStatus::Failed(_) => "padding: 15px; margin: 20px 0; background-color: #f8d7da; border: 1px solid #dc3545; border-radius: 5px; color: #721c24;",
            },
            
            match status() {
                ConnectionStatus::Connecting => rsx! {
                    div {
                        strong { "🔄 Connecting..." }
                        p { style: "margin: 5px 0 0 0;", "Attempting to connect to SpacetimeDB..." }
                    }
                },
                ConnectionStatus::Connected(ref identity) => rsx! {
                    div {
                        strong { "✅ Connected to SpacetimeDB" }
                        p { style: "margin: 5px 0 0 0;", "Identity: {identity}" }
                    }
                },
                ConnectionStatus::Failed(ref error) => rsx! {
                    div {
                        strong { "❌ Connection Failed" }
                        p { style: "margin: 5px 0 0 0;", "Error: {error}" }
                        p { style: "margin: 5px 0 0 0; font-size: 0.9em;", "Please ensure SpacetimeDB is running at ws://localhost:3000" }
                    }
                },
            }
        }
    }
}

/// Component that displays all accounts
#[component]
fn AccountsSection() -> Element {
    // Subscribe to the accounts table
    // This will automatically update when accounts change in SpacetimeDB
    let accounts = use_table::<Account>("accounts", None);

    rsx! {
        div {
            style: "margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px;",
            
            h2 { "Accounts" }
            
            if accounts.read().is_empty() {
                p { style: "color: #666;", "No accounts found. Waiting for data..." }
            } else {
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    
                    thead {
                        tr {
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #ddd;", "ID" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #ddd;", "Name" }
                            th { style: "text-align: right; padding: 8px; border-bottom: 2px solid #ddd;", "Balance" }
                        }
                    }
                    
                    tbody {
                        for account in accounts.read().iter() {
                            tr {
                                key: "{account.id}",
                                
                                td { style: "padding: 8px; border-bottom: 1px solid #ddd;", "{account.id}" }
                                td { style: "padding: 8px; border-bottom: 1px solid #ddd;", "{account.name}" }
                                td {
                                    style: "padding: 8px; border-bottom: 1px solid #ddd; text-align: right;",
                                    "${account.balance:.2}"
                                }
                            }
                        }
                    }
                }
                
                p {
                    style: "margin-top: 10px; color: #666; font-size: 0.9em;",
                    "Total accounts: {accounts.read().len()}"
                }
            }
        }
    }
}

/// Component that displays recent transactions
#[component]
fn TransactionsSection() -> Element {
    // Subscribe to the transactions table
    let transactions = use_table::<Transaction>("transactions", None);

    rsx! {
        div {
            style: "margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px;",
            
            h2 { "Recent Transactions" }
            
            if transactions.read().is_empty() {
                p { style: "color: #666;", "No transactions found. Waiting for data..." }
            } else {
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    
                    thead {
                        tr {
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #ddd;", "ID" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #ddd;", "Account" }
                            th { style: "text-align: right; padding: 8px; border-bottom: 2px solid #ddd;", "Amount" }
                            th { style: "text-align: left; padding: 8px; border-bottom: 2px solid #ddd;", "Description" }
                        }
                    }
                    
                    tbody {
                        for transaction in transactions.read().iter().take(10) {
                            tr {
                                key: "{transaction.id}",
                                
                                td { style: "padding: 8px; border-bottom: 1px solid #ddd;", "{transaction.id}" }
                                td { style: "padding: 8px; border-bottom: 1px solid #ddd;", "{transaction.account_id}" }
                                td {
                                    style: "padding: 8px; border-bottom: 1px solid #ddd; text-align: right;",
                                    "${transaction.amount:.2}"
                                }
                                td { style: "padding: 8px; border-bottom: 1px solid #ddd;", "{transaction.description}" }
                            }
                        }
                    }
                }
                
                p {
                    style: "margin-top: 10px; color: #666; font-size: 0.9em;",
                    "Showing latest 10 of {transactions.read().len()} transactions"
                }
            }
        }
    }
}

/// Example component showing filtered table subscription
#[component]
#[allow(dead_code)]
fn HighBalanceAccounts() -> Element {
    // Example of using a filter (would require server-side implementation)
    let filter = TableFilter::new().with_condition("balance_gt", "1000");
    let high_balance_accounts = use_table::<Account>("accounts", Some(filter));

    rsx! {
        div {
            h3 { "High Balance Accounts (>$1000)" }
            
            for account in high_balance_accounts.read().iter() {
                div {
                    key: "{account.id}",
                    "{account.name}: ${account.balance:.2}"
                }
            }
        }
    }
}
