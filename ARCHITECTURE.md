# Architecture

This document describes the internal architecture of the dioxus_use_spacetime library.

## Overview

The library provides a reactive bridge between SpacetimeDB and Dioxus applications using Dioxus's signal system and context API.

## Component Structure

```
dioxus_use_spacetime/
├── src/
│   ├── lib.rs           # Public API exports
│   ├── connection.rs    # WebSocket connection management
│   ├── context.rs       # Dioxus context provider
│   ├── hooks.rs         # Dioxus hooks (use_table, etc.)
│   ├── table.rs         # Table subscription and filtering
│   └── error.rs         # Error types
└── examples/
    └── basic_usage.rs   # Example application
```

## Core Components

### 1. SpacetimeConnection (`connection.rs`)

The `SpacetimeConnection` struct manages the WebSocket connection to a SpacetimeDB instance.

**Responsibilities:**
- Establish and maintain WebSocket connection
- Handle connection state (connected/disconnected)
- Subscribe/unsubscribe to tables
- Send messages to SpacetimeDB

**Key Methods:**
- `new(url, database)` - Create new connection instance
- `connect()` - Establish connection to SpacetimeDB
- `disconnect()` - Close connection
- `subscribe_table(name)` - Subscribe to a table
- `unsubscribe_table(name)` - Unsubscribe from a table

### 2. SpacetimeContext (`context.rs`)

The `SpacetimeContext` is a Dioxus context that wraps the connection and manages subscription lifecycle.

**Responsibilities:**
- Provide connection to all child components
- Track active subscriptions with reference counting
- Clean up subscriptions when no longer needed

**Key Methods:**
- `new(url, database)` - Create context
- `initialize()` - Initialize connection
- `subscribe(table_name)` - Subscribe to table (ref counted)
- `unsubscribe(table_name)` - Unsubscribe from table (ref counted)
- `is_subscribed(table_name)` - Check if subscribed

**Reference Counting:**
Multiple components can subscribe to the same table. The context maintains a reference count and only unsubscribes when the count reaches zero.

### 3. Table Subscription (`table.rs`)

The `TableSubscription` struct represents an active subscription to a SpacetimeDB table.

**Components:**
- `TableFilter` - Define filtering conditions
- `TableSubscription<T>` - Hold table data and metadata

**Key Methods:**
- `new(table_name, filter)` - Create subscription
- `get_data()` - Get current data snapshot
- `update_data(data)` - Update with new data
- `add_row(row)` - Add single row
- `clear()` - Clear all data

### 4. Hooks (`hooks.rs`)

Dioxus hooks that provide the user-facing API.

#### `use_spacetime_init(url, database)`

Initializes the SpacetimeDB context. Should be called once at the application root.

**Lifecycle:**
1. Creates `SpacetimeContext`
2. Provides context to child components
3. Asynchronously connects to SpacetimeDB

#### `use_table<T>(table_name, filter) -> Signal<Vec<T>>`

Subscribes to a table and returns a reactive signal.

**Lifecycle:**
1. Gets context from parent
2. Creates signal to hold data
3. Subscribes to table on mount
4. Returns signal for reactive access
5. (Future) Unsubscribes on unmount

**Type Parameters:**
- `T: Clone + 'static` - The row type (typically with `Deserialize`)

#### `use_table_row<T>(table_name, id_field, id_value) -> Signal<Vec<T>>`

Convenience hook for subscribing to a single row by ID.

**Implementation:**
Wraps `use_table` with a filter.

## Data Flow

```
User Component
     ↓ (calls use_table)
Hooks Layer
     ↓ (accesses context)
SpacetimeContext
     ↓ (manages connection)
SpacetimeConnection
     ↓ (WebSocket)
SpacetimeDB Server
     ↓ (updates)
Signal Updates
     ↓ (triggers)
UI Re-render
```

### Subscription Flow

1. Component calls `use_table("accounts", None)`
2. Hook gets `SpacetimeContext` from Dioxus context
3. Hook creates `Signal<Vec<Account>>`
4. Hook subscribes to table via context
5. Context increments reference count for "accounts"
6. If first subscription, connection subscribes to table
7. Signal is returned to component
8. Component reads signal with `.read()`
9. When SpacetimeDB sends updates (future):
   - Connection receives update
   - Updates propagate to signal
   - Signal triggers component re-render

### Cleanup Flow (Future Implementation)

1. Component unmounts
2. Cleanup function calls `context.unsubscribe("accounts")`
3. Context decrements reference count
4. If count reaches 0, connection unsubscribes from table

## Error Handling

The library uses a custom `SpacetimeError` enum with variants for:
- `ConnectionError` - Connection issues
- `SubscriptionError` - Subscription failures
- `ContextNotFound` - Missing context (user error)
- `SerializationError` - Data serialization issues
- `Other` - General errors

All fallible operations return `Result<T, SpacetimeError>`.

## Future Enhancements

### Real-time Updates

Currently, the implementation provides the structure but doesn't wire up real-time updates from SpacetimeDB. To add this:

1. Set up SpacetimeDB SDK callbacks in `SpacetimeConnection`
2. When table updates arrive, call `signal.set(new_data)`
3. Dioxus automatically re-renders components

### Mutations

Add support for calling SpacetimeDB reducers:

```rust
context.call_reducer("transfer", TransferArgs { from, to, amount }).await?;
```

### Advanced Filtering

Implement server-side filtering:

```rust
let filter = TableFilter::new()
    .where_eq("status", "active")
    .where_gt("balance", 100);
```

### Reconnection Logic

Add automatic reconnection on connection loss:

```rust
impl SpacetimeConnection {
    async fn reconnect(&self) -> Result<()> {
        // Implement exponential backoff
        // Re-subscribe to all tables
    }
}
```

## Testing Strategy

- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test hooks with mock SpacetimeDB
- **Doc Tests**: Ensure example code compiles
- **Example Apps**: Demonstrate real-world usage

## Performance Considerations

- **Reference Counting**: Prevents duplicate subscriptions
- **Async Operations**: Non-blocking connection and subscription
- **Efficient Updates**: Only re-render affected components
- **Memory Management**: Automatic cleanup via Drop trait (future)

## Thread Safety

- Uses `Arc<RwLock<T>>` for shared state
- All async operations use Tokio runtime
- Safe to use across Dioxus's async boundaries
