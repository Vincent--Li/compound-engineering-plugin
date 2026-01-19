# Modules & Crates

## Module Structure

### Modern Style (Rust 2018+)

```
src/
├── lib.rs          # Crate root for library
├── main.rs         # Crate root for binary
├── config.rs       # mod config
├── database/
│   ├── mod.rs      # mod database
│   ├── postgres.rs # database::postgres
│   └── sqlite.rs   # database::sqlite
└── api/
    ├── mod.rs
    └── handlers.rs
```

### lib.rs

```rust
pub mod config;
pub mod database;
pub mod api;

pub use config::Config;
pub use database::Database;
```

### Module Files

```rust
// src/database/mod.rs
mod postgres;
mod sqlite;

pub use postgres::PostgresConnection;
pub use sqlite::SqliteConnection;

pub trait Database {
    fn connect(&self) -> Result<(), Error>;
}
```

## Visibility

```rust
// Public to all
pub fn public_function() {}

// Private (default)
fn private_function() {}

// Public within crate only
pub(crate) fn crate_only() {}

// Public to parent module
pub(super) fn parent_only() {}

// Public to specific path
pub(in crate::api) fn api_only() {}
```

### Re-exports

```rust
// lib.rs
pub use self::config::Config;
pub use self::error::{Error, Result};

// Flatten module hierarchy for users
mod internal {
    pub mod deeply_nested {
        pub struct ImportantType;
    }
}
pub use internal::deeply_nested::ImportantType;
```

## Workspaces

### Cargo.toml (Workspace Root)

```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
license = "MIT"

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### Member Cargo.toml

```toml
[package]
name = "my-api"
version.workspace = true
edition.workspace = true

[dependencies]
tokio.workspace = true
serde.workspace = true
my-core = { path = "../core" }
```

## Project Layouts

### Library

```
my-lib/
├── Cargo.toml
├── src/
│   └── lib.rs
├── examples/
│   └── basic.rs
├── tests/
│   └── integration_test.rs
└── benches/
    └── benchmark.rs
```

### Binary with Library

```
my-app/
├── Cargo.toml
├── src/
│   ├── lib.rs      # Library code
│   ├── main.rs     # Binary entry
│   └── bin/
│       └── tool.rs # Additional binary
```

### Cargo.toml

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_app"
path = "src/lib.rs"

[[bin]]
name = "my-app"
path = "src/main.rs"

[[bin]]
name = "my-tool"
path = "src/bin/tool.rs"
```

## Best Practices

### Prelude Pattern

```rust
// src/prelude.rs
pub use crate::Error;
pub use crate::Result;
pub use crate::Config;
pub use crate::traits::*;

// In other modules
use crate::prelude::*;
```

### Feature Flags

```toml
[features]
default = ["json"]
json = ["serde_json"]
full = ["json", "async", "cli"]
async = ["tokio"]
cli = ["clap"]
```

```rust
#[cfg(feature = "json")]
pub mod json {
    // JSON functionality
}

#[cfg(feature = "async")]
pub async fn async_function() {
    // Async implementation
}
```

### Conditional Compilation

```rust
#[cfg(test)]
mod tests {
    // Test-only code
}

#[cfg(target_os = "windows")]
fn platform_specific() {}

#[cfg(all(feature = "async", not(feature = "blocking")))]
fn async_only() {}
```

### Inline Modules (for small modules)

```rust
// Instead of separate file for tiny modules
mod small {
    pub const VALUE: i32 = 42;
}
```

## Organizing Large Crates

```rust
// lib.rs - Keep minimal, just re-exports
pub mod error;
pub mod config;
pub mod domain;
pub mod services;
pub mod api;

// Public API
pub use error::{Error, Result};
pub use config::Config;
pub use domain::{User, Document};
```

### Internal Modules

```rust
// Keep implementation details private
mod internal {
    pub(crate) fn helper() {}
}

// Expose only what users need
pub mod api {
    use crate::internal::helper;
    
    pub fn public_function() {
        helper();
    }
}
```
