# Error Handling

## Core Types

### Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## The ? Operator

```rust
fn read_username() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

## Custom Error Types

### Using thiserror (for libraries)

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
}
```

### Using anyhow (for applications)

```rust
use anyhow::{Context, Result, bail, ensure};

fn process_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;
    
    ensure!(config.port > 0, "Port must be positive");
    
    if config.host.is_empty() {
        bail!("Host cannot be empty");
    }
    
    Ok(config)
}
```

## Error Handling Patterns

### Map and Map_err

```rust
// Transform the Ok value
let result = "42".parse::<i32>()
    .map(|n| n * 2);

// Transform the Err value
let result = File::open("file.txt")
    .map_err(|e| AppError::Io(e));
```

### And_then for Chaining

```rust
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
        .and_then(|n| Ok(n.checked_mul(2).ok_or_else(|| /* error */)?))
}
```

### Unwrap vs Expect

```rust
// ❌ Avoid in production code
let value = result.unwrap();

// ✅ Better: provides context
let value = result.expect("Config should be valid after validation");

// ✅ Best: handle the error
let value = result?;
```

### Ok_or and Ok_or_else

```rust
// Convert Option to Result
let user = users.get(&id)
    .ok_or(AppError::NotFound { resource: "user".into() })?;

// Lazy error construction
let user = users.get(&id)
    .ok_or_else(|| AppError::NotFound { resource: format!("user {}", id) })?;
```

## Option Combinators

```rust
let value = Some(5);

// Transform
value.map(|x| x * 2);           // Some(10)

// Filter
value.filter(|x| *x > 10);      // None

// Flatten
Some(Some(5)).flatten();        // Some(5)

// Provide default
value.unwrap_or(0);             // 5
None.unwrap_or(0);              // 0

// Lazy default
value.unwrap_or_else(|| expensive_default());

// Convert to Result
value.ok_or("missing value")?;
```

## Best Practices

### Define Error at Crate Level

```rust
// src/error.rs
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ...
}
```

### Error Context

```rust
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let path = get_config_path()?;
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config from {:?}", path))?;
    // ...
}
```

### Recoverable Errors

```rust
match operation() {
    Ok(value) => process(value),
    Err(AppError::NotFound { .. }) => use_default(),
    Err(AppError::RateLimit { retry_after }) => {
        sleep(retry_after);
        operation()?
    }
    Err(e) => return Err(e),
}
```

### Early Return Pattern

```rust
fn process(data: &Data) -> Result<Output> {
    if !data.is_valid() {
        return Err(AppError::InvalidInput);
    }
    
    let intermediate = transform(data)?;
    
    if intermediate.is_empty() {
        return Ok(Output::default());
    }
    
    finalize(intermediate)
}
```
