# Testing

## Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_with_result() -> Result<(), Error> {
        let result = fallible_function()?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        divide(1, 0);
    }

    #[test]
    #[ignore = "slow test, run manually"]
    fn expensive_test() {
        // Long-running test
    }
}
```

## Integration Tests

```rust
// tests/integration_test.rs
use my_crate::{Config, App};

#[test]
fn test_full_workflow() {
    let config = Config::default();
    let app = App::new(config);
    
    let result = app.process("input");
    
    assert!(result.is_ok());
}
```

## Doctests

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # Panics
///
/// Panics if the result overflows.
///
/// ```should_panic
/// # use my_crate::add;
/// add(i32::MAX, 1);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a.checked_add(b).expect("overflow")
}
```

### Hiding Lines in Doctests

```rust
/// ```
/// # use my_crate::Config;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::load("config.toml")?;
/// assert!(config.is_valid());
/// # Ok(())
/// # }
/// ```
```

## Async Tests

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_fetch().await;
    assert!(result.is_ok());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_concurrent() {
    // Test with multi-threaded runtime
}
```

## Test Organization

### Test Helpers

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test fixtures
    fn sample_user() -> User {
        User {
            id: 1,
            name: "Test User".into(),
            email: "test@example.com".into(),
        }
    }

    // Common setup
    fn setup() -> TestContext {
        TestContext::new()
    }

    #[test]
    fn test_user_creation() {
        let user = sample_user();
        assert_eq!(user.name, "Test User");
    }
}
```

### Shared Test Utilities

```rust
// tests/common/mod.rs
pub fn setup_database() -> TestDatabase {
    // Setup code
}

pub struct TestFixture {
    pub db: TestDatabase,
    pub config: Config,
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            db: setup_database(),
            config: Config::test(),
        }
    }
}

// tests/api_test.rs
mod common;

#[test]
fn test_api() {
    let fixture = common::TestFixture::new();
    // Use fixture
}
```

## Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parse_round_trip(s in "\\PC*") {
        let parsed = parse(&s);
        if let Ok(value) = parsed {
            let serialized = serialize(&value);
            assert_eq!(parse(&serialized), Ok(value));
        }
    }

    #[test]
    fn test_addition_commutative(a in 0i32..1000, b in 0i32..1000) {
        assert_eq!(add(a, b), add(b, a));
    }
}
```

## Mocking

### Manual Mocking with Traits

```rust
trait Database {
    fn get_user(&self, id: i64) -> Option<User>;
}

struct RealDatabase { /* ... */ }
impl Database for RealDatabase { /* ... */ }

#[cfg(test)]
struct MockDatabase {
    users: HashMap<i64, User>,
}

#[cfg(test)]
impl Database for MockDatabase {
    fn get_user(&self, id: i64) -> Option<User> {
        self.users.get(&id).cloned()
    }
}

#[test]
fn test_with_mock() {
    let mut mock = MockDatabase { users: HashMap::new() };
    mock.users.insert(1, User { id: 1, name: "Test".into() });
    
    let service = UserService::new(mock);
    assert!(service.find_user(1).is_some());
}
```

### Using mockall

```rust
use mockall::{automock, predicate::*};

#[automock]
trait Database {
    fn get_user(&self, id: i64) -> Option<User>;
}

#[test]
fn test_with_mockall() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(1))
        .times(1)
        .returning(|_| Some(User { id: 1, name: "Test".into() }));
    
    let service = UserService::new(mock);
    assert!(service.find_user(1).is_some());
}
```

## Test Coverage

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Run with coverage
cargo llvm-cov --html

# View report
open target/llvm-cov/html/index.html
```

## Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Tests in module
cargo test module::

# With output
cargo test -- --nocapture

# Include ignored tests
cargo test -- --ignored

# Run doctests only
cargo test --doc

# Run integration tests only
cargo test --test integration_test
```
