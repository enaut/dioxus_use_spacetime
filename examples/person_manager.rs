//! Example demonstrating integration with a SpacetimeDB module
//!
//! This example uses the generated SDK for a simple person management module.
//! It shows how to:
//! - Connect to SpacetimeDB using the generated module SDK
//! - Subscribe to tables and display data reactively
//! - Call reducers to add new data
//!
//! To run this example:
//! 1. Make sure SpacetimeDB is running: `spacetime start`
//! 2. Publish the module: `cd examples/spacetime_module && spacetime publish person_module`
//! 3. Run the example: `cargo run --example person_manager`

use dioxus::prelude::*;
use dioxus_use_spacetime::ConnectionStatus;
use spacetimedb_sdk::{Table, DbContext};

// Include the generated SDK
#[path = "example_spacetime_module_sdk/mod.rs"]
mod example_spacetime_module_sdk;

use example_spacetime_module_sdk::{
    DbConnection, Person, PersonTableAccess, add,
};

fn main() {
    dioxus::launch(App);
}

/// Root application component
fn App() -> Element {
    // Store the connection in a signal so we can access it from child components
    let mut conn = use_signal(|| None::<DbConnection>);
    let mut connection_status = use_signal(|| ConnectionStatus::Connecting);
    let mut error_msg = use_signal(|| String::new());
    
    // Initialize connection on mount
    use_effect(move || {
        spawn(async move {
            match DbConnection::builder()
                .with_uri("ws://localhost:3000")
                .with_module_name("person_module")
                .build()
            {
                Ok(db_conn) => {
                    // Get identity
                    let identity = db_conn.try_identity()
                        .map(|id| id.to_hex().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    
                    connection_status.set(ConnectionStatus::Connected(identity));
                    conn.set(Some(db_conn));
                }
                Err(e) => {
                    let error = format!("Failed to connect to SpacetimeDB: {}. Make sure SpacetimeDB is running at ws://localhost:3000 and the person_module is published.", e);
                    error_msg.set(error.clone());
                    connection_status.set(ConnectionStatus::Failed(error));
                }
            }
        });
    });
    
    // Advance messages periodically if connected
    use_effect(move || {
        spawn(async move {
            loop {
                if let Some(ref db_conn) = *conn.read() {
                    if let Err(_) = db_conn.frame_tick() {
                        break;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
            }
        });
    });

    rsx! {
        div {
            style: "font-family: Arial, sans-serif; padding: 20px;",
            
            h1 { "Person Manager - SpacetimeDB Example" }
            
            // Display connection status
            ConnectionStatusBanner { status: connection_status }
            
            // Only show content if connected
            if conn.read().is_some() {
                PersonManager { conn: conn }
            } else {
                p { 
                    style: "color: #666; margin: 20px 0;",
                    "Waiting for connection..."
                }
            }
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
                        p { style: "margin: 5px 0 0 0;", "{error}" }
                    }
                },
            }
        }
    }
}

/// Component for managing persons
#[component]
fn PersonManager(conn: Signal<Option<DbConnection>>) -> Element {
    let mut persons = use_signal(|| Vec::<Person>::new());
    let mut new_name = use_signal(|| String::new());
    let mut status_message = use_signal(|| String::new());
    
    // Subscribe to person table and update signal when data changes
    use_effect(move || {
        if let Some(ref db_conn) = *conn.read() {
            // Set up subscription
            let _sub = db_conn.subscription_builder()
                .on_applied(move |_ctx| {
                    // Subscription applied - data will be available via iter()
                })
                .subscribe("SELECT * FROM person");
            
            // Spawn a task to periodically read from the connection
            spawn(async move {
                if let Some(ref db_conn) = *conn.read() {
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        let person_list: Vec<Person> = db_conn.db.person().iter().collect();
                        persons.set(person_list);
                    }
                }
            });
        }
    });

    let add_person = move |_evt: Event<MouseData>| {
        let name = new_name.read().clone();
        if name.is_empty() {
            status_message.set("Please enter a name".to_string());
            return;
        }
        
        if let Some(ref db_conn) = *conn.read() {
            match db_conn.reducers.add(name.clone()) {
                Ok(_) => {
                    status_message.set(format!("Added person: {}", name));
                    new_name.set(String::new());
                }
                Err(e) => {
                    status_message.set(format!("Error adding person: {}", e));
                }
            }
        }
    };

    rsx! {
        div {
            style: "margin: 20px 0;",
            
            // Add person section
            div {
                style: "margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; background-color: #f8f9fa;",
                
                h2 { "Add New Person" }
                
                div {
                    style: "display: flex; gap: 10px; align-items: center;",
                    
                    input {
                        style: "flex: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px;",
                        r#type: "text",
                        placeholder: "Enter name",
                        value: "{new_name}",
                        oninput: move |evt| new_name.set(evt.value().clone()),
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter {
                                let name = new_name.read().clone();
                                if !name.is_empty() {
                                    if let Some(ref db_conn) = *conn.read() {
                                        match db_conn.reducers.add(name.clone()) {
                                            Ok(_) => {
                                                status_message.set(format!("Added person: {}", name));
                                                new_name.set(String::new());
                                            }
                                            Err(e) => {
                                                status_message.set(format!("Error adding person: {}", e));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    button {
                        style: "padding: 8px 16px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: add_person,
                        "Add Person"
                    }
                }
                
                if !status_message.read().is_empty() {
                    p {
                        style: "margin: 10px 0 0 0; color: #666;",
                        "{status_message}"
                    }
                }
            }
            
            // Person list section
            div {
                style: "margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px;",
                
                h2 { "People ({persons.read().len()})" }
                
                if persons.read().is_empty() {
                    p { 
                        style: "color: #666;",
                        "No people yet. Add someone above!" 
                    }
                } else {
                    ul {
                        style: "list-style: none; padding: 0;",
                        
                        for person in persons.read().iter() {
                            li {
                                key: "{person.name}",
                                style: "padding: 10px; margin: 5px 0; background-color: #f8f9fa; border-radius: 4px; border-left: 3px solid #007bff;",
                                "{person.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}
