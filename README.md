# dioxus_use_spacetime

A reactive [SpacetimeDB](https://spacetimedb.com/) client library for [Dioxus](https://dioxuslabs.com/) applications.

This library provides utilities for integrating SpacetimeDB into your Dioxus applications. It works seamlessly with SpacetimeDB's generated client SDK.

## Features

- 🔄 **Real SpacetimeDB Connection**: Actual WebSocket connection to SpacetimeDB instances
- 📊 **Connection Status Tracking**: Visual feedback for connection state (connecting/connected/failed)
- 🎣 **Works with Generated SDK**: Use SpacetimeDB's code-generated client bindings
- 🔌 **Connection Management**: Utilities for managing SpacetimeDB connections
- 🎯 **Type-safe**: Leverage Rust's type system with generated types from your module
- ⚡ **Generic Library**: No dependencies on specific modules - works with any SpacetimeDB module

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus = "0.7"
dioxus_use_spacetime = "0.1"
spacetimedb-sdk = "1.0"
```

## Quick Start

This library is designed to work with SpacetimeDB's generated client SDK. Follow these steps:

### 1. Create a SpacetimeDB Module

Create your SpacetimeDB module with tables and reducers:

```rust
// In your SpacetimeDB module
use spacetimedb::*;

#[spacetimedb(table)]
pub struct Person {
    name: String,
}

#[spacetimedb(reducer)]
pub fn add(name: String) {
    Person::insert(Person { name });
}
```

### 2. Generate the Client SDK

```bash
spacetime generate --lang rust --out-dir ./src/module_bindings
```

### 3. Use in Your Dioxus App

```rust
use dioxus::prelude::*;
use dioxus_use_spacetime::ConnectionStatus;
use spacetimedb_sdk::{Table, DbContext};

// Include your generated module bindings
mod module_bindings;
use module_bindings::{DbConnection, Person, PersonTableAccess, add};

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut conn = use_signal(|| None::<DbConnection>);
    let mut connection_status = use_signal(|| ConnectionStatus::Connecting);
    
    // Connect to SpacetimeDB
    use_effect(move || {
        spawn(async move {
            match DbConnection::builder()
                .with_uri("ws://localhost:3000")
                .with_module_name("person_module")
                .build()
            {
                Ok(db_conn) => {
                    let identity = db_conn.try_identity()
                        .map(|id| id.to_hex().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    connection_status.set(ConnectionStatus::Connected(identity));
                    conn.set(Some(db_conn));
                }
                Err(e) => {
                    connection_status.set(ConnectionStatus::Failed(format!("{}", e)));
                }
            }
        });
    });
    
    rsx! {
        h1 { "My SpacetimeDB App" }
        // Display connection status
        match connection_status() {
            ConnectionStatus::Connected(id) => rsx! { p { "Connected: {id}" } },
            ConnectionStatus::Failed(err) => rsx! { p { "Failed: {err}" } },
            _ => rsx! { p { "Connecting..." } }
        }
        // Use your connection...
    }
}
```

## Examples

### Full Example

See `examples/person_manager.rs` for a complete working example that:
- Connects to SpacetimeDB
- Displays connection status
- Subscribes to a `person` table
- Shows all persons in the UI
- Provides an input field and button to add new persons
- Calls reducers to modify data

To run the example:

```bash
# 1. Start SpacetimeDB
spacetime start

# 2. Publish the example module
cd examples/spacetime_module
spacetime publish person_module

# 3. Run the example
cargo run --example person_manager
```

## Library Design Philosophy

This library is intentionally **generic** and **minimal**:

- ✅ **No module-specific dependencies**: The library doesn't depend on any particular SpacetimeDB module
- ✅ **Works with any generated SDK**: Use with any SpacetimeDB module you create
- ✅ **Thin wrapper**: Provides connection utilities and status tracking
- ✅ **Direct SDK access**: You use the generated SDK directly for table access and reducers

### What This Library Provides

1. **ConnectionStatus enum**: Track connection state (Connecting/Connected/Failed)
2. **Connection utilities**: Helper patterns for managing SpacetimeDB connections in Dioxus
3. **Examples**: Reference implementations showing best practices

### What You Use From SpacetimeDB SDK

1. **DbConnection**: From your generated module bindings
2. **Table access**: Via the generated table traits (e.g., `PersonTableAccess`)
3. **Reducers**: Call reducers through the generated reducer traits (e.g., `add`)
4. **Subscriptions**: Use `subscription_builder()` from the SDK

## Usage Patterns

### Subscribe to Tables

```rust
fn MyComponent(conn: Signal<Option<DbConnection>>) -> Element {
    let mut persons = use_signal(|| Vec::<Person>::new());
    
    use_effect(move || {
        if let Some(ref db_conn) = *conn.read() {
            // Subscribe to table
            let _sub = db_conn.subscription_builder()
                .on_applied(move |_ctx| {
                    // Subscription applied
                })
                .subscribe("SELECT * FROM person");
            
            // Poll for updates
            spawn(async move {
                loop {
                    if let Some(ref db_conn) = *conn.read() {
                        let list: Vec<Person> = db_conn.db.person().iter().collect();
                        persons.set(list);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            });
        }
    });
    
    rsx! {
        for person in persons.read().iter() {
            div { "{person.name}" }
        }
    }
}
```

### Filtering (Coming Soon)

Subscribe to filtered data:

```rust
fn ActiveUsers() -> Element {
    let filter = TableFilter::new()
        .with_condition("status", "active");
    
    let active_users = use_table::<User>("users", Some(filter));
    
    rsx! {
        for user in active_users.read().iter() {
            div { "{user.name}" }
        }
    }
}
```

### Access Context

Access the SpacetimeDB context directly if needed:

```rust
fn MyComponent() -> Element {
    let context = use_spacetime_context();
    
    // Use context.connection() for advanced operations
    
    rsx! { /* ... */ }
}
```

## Examples

Check out the [examples](examples/) directory for complete working examples:

- `basic_usage.rs` - Complete example showing accounts and transactions

Run an example with:

```bash
cargo run --example basic_usage
```

## Architecture

The library consists of several key components:

- **`SpacetimeConnection`**: Manages the WebSocket connection to SpacetimeDB
- **`SpacetimeContext`**: Dioxus context that holds the connection and manages subscriptions
- **`use_spacetime_init`**: Hook to initialize the connection at the app root
- **`use_table`**: Hook to subscribe to tables with reactive updates
- **`TableFilter`**: Type-safe filtering for table subscriptions

## How It Works

1. **Initialization**: `use_spacetime_init` creates a `SpacetimeContext` and provides it to all child components
2. **Subscription**: `use_table` subscribes to a table and returns a `Signal<Vec<T>>`
3. **Updates**: When data changes in SpacetimeDB, the signal automatically updates, triggering re-renders
4. **Cleanup**: Subscriptions are automatically cleaned up when components unmount

## Roadmap

- [x] Basic connection management
- [x] Table subscriptions with reactive signals
- [x] Context-based API
- [ ] Full SpacetimeDB SDK integration
- [ ] Server-side filtering
- [ ] Mutations and reducers
- [ ] Error handling improvements
- [ ] Reconnection logic
- [ ] Advanced query support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- [SpacetimeDB](https://spacetimedb.com/) - The database platform
- [Dioxus](https://dioxuslabs.com/) - The Rust UI framework
