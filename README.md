# dioxus_use_spacetime

A reactive [SpacetimeDB](https://spacetimedb.com/) client library for [Dioxus](https://dioxuslabs.com/) applications.

This library provides Dioxus hooks and context for seamlessly integrating SpacetimeDB into your Dioxus applications with reactive, signal-based updates.

## Features

- 🔄 **Reactive Table Subscriptions**: Subscribe to SpacetimeDB tables with automatic UI updates
- 🎣 **Dioxus Hooks**: Familiar hook-based API (`use_table`, `use_spacetime_init`)
- 🔌 **Context-based Connection**: Initialize once, use everywhere in your component tree
- 🎯 **Type-safe**: Leverage Rust's type system for compile-time safety
- 🔍 **Filtering Support**: Subscribe to filtered subsets of tables

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus = "0.6"
dioxus_use_spacetime = "0.1"
spacetimedb-sdk = "1.0"
```

## Quick Start

Here's a minimal example to get you started:

```rust
use dioxus::prelude::*;
use dioxus_use_spacetime::*;
use serde::{Deserialize, Serialize};

// Define your table structure
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Account {
    id: u64,
    name: String,
    balance: f64,
}

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    // Initialize the SpacetimeDB connection
    use_spacetime_init("ws://localhost:3000", "my_database");
    
    rsx! {
        AccountList {}
    }
}

fn AccountList() -> Element {
    // Subscribe to the accounts table
    let accounts = use_table::<Account>("accounts", None);
    
    rsx! {
        div {
            h1 { "Accounts" }
            for account in accounts.read().iter() {
                div { "{account.name}: ${account.balance}" }
            }
        }
    }
}
```

## Usage

### Initialize Connection

Use `use_spacetime_init` at the root of your application to establish the connection:

```rust
fn App() -> Element {
    use_spacetime_init("ws://localhost:3000", "my_database");
    
    rsx! {
        // Your app components
    }
}
```

### Subscribe to Tables

Use the `use_table` hook to subscribe to a table:

```rust
fn MyComponent() -> Element {
    // Subscribe to all rows
    let users = use_table::<User>("users", None);
    
    rsx! {
        for user in users.read().iter() {
            div { "{user.name}" }
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
