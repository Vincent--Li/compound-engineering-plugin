---
name: rust-idiomatic-style
description: This skill should be used when writing Rust code following idiomatic Rust conventions and best practices. It applies when writing Rust code, creating libraries or binaries, working with async code, error handling, or any Rust file. Triggers on Rust code generation, refactoring requests, code review, or when the user mentions Rust idioms, Clippy, Rustacean style, or zero-cost abstractions. Embodies ownership, borrowing, lifetimes, trait-based design, and the "fearless concurrency" philosophy.
---

<objective>
Apply idiomatic Rust conventions to Rust code. This skill provides comprehensive domain expertise based on the Rust API Guidelines, Clippy lints, and patterns from successful Rust projects.
</objective>

<essential_principles>
## Core Philosophy

"Make illegal states unrepresentable. The compiler is your friend."

**Idiomatic Rust means:**
- Use the type system to prevent bugs at compile time
- Prefer static dispatch over dynamic dispatch when possible
- Use iterators instead of manual loops
- Leverage pattern matching for exhaustive handling
- Make data transformations explicit through method chains
- Zero-cost abstractions: abstractions that impose no runtime overhead

**What to deliberately avoid:**
- Excessive `clone()` calls - prefer borrowing
- `unwrap()` in non-prototype code - use proper error handling
- `Rc<RefCell<T>>` when ownership can be restructured
- Stringly-typed APIs - use enums and strong types
- Manual memory management patterns from C/C++
- Overusing `dyn Trait` when generics suffice

**Development Philosophy:**
- Correctness over convenience
- Compile-time guarantees over runtime checks
- Explicit over implicit
- Composition over inheritance (traits over classical OOP)
</essential_principles>

<intake>
What are you working on?

1. **Ownership & Lifetimes** - Borrowing patterns, lifetime annotations, smart pointers
2. **Error Handling** - Result, Option, custom errors, the `?` operator
3. **Traits & Generics** - Trait design, bounds, associated types, impl Trait
4. **Async/Await** - Tokio/async-std patterns, futures, streams
5. **Modules & Crates** - Project structure, visibility, pub(crate), workspaces
6. **Testing** - Unit tests, integration tests, doctests, property testing
7. **Performance** - Zero-cost abstractions, benchmarking, profiling
8. **Code Review** - Review code against Rust idioms
9. **General Guidance** - Philosophy and conventions

**Specify a number or describe your task.**
</intake>

<routing>
| Response | Reference to Read |
|----------|-------------------|
| 1, "ownership", "lifetime", "borrow" | [ownership.md](./references/ownership.md) |
| 2, "error", "result", "option" | [error-handling.md](./references/error-handling.md) |
| 3, "trait", "generic", "impl" | [traits.md](./references/traits.md) |
| 4, "async", "tokio", "future" | [async.md](./references/async.md) |
| 5, "module", "crate", "workspace" | [modules.md](./references/modules.md) |
| 6, "test", "testing", "doctest" | [testing.md](./references/testing.md) |
| 7, "performance", "benchmark", "optimize" | [performance.md](./references/performance.md) |
| 8, "review" | Read all references, then review code |
| 9, general task | Read relevant references based on context |

**After reading relevant references, apply patterns to the user's code.**
</routing>

<quick_reference>
## Naming Conventions

**Types:** `PascalCase` for structs, enums, traits, type aliases
**Functions/Methods:** `snake_case` for functions, methods, variables
**Constants:** `SCREAMING_SNAKE_CASE` for constants and statics
**Lifetimes:** Short lowercase letters (`'a`, `'b`, `'de`)
**Crates:** `kebab-case` for crate names, `snake_case` for imports

**Conversion Naming:**
- `as_*` - Cheap reference-to-reference conversion
- `to_*` - Expensive conversion, may allocate
- `into_*` - Consumes self, returns owned value
- `from_*` - Associated function for construction
- `try_*` - Fallible version of the operation

**Predicate Naming:**
- `is_*` - Returns bool for state queries
- `has_*` - Returns bool for containment

## Common Patterns

**Builder Pattern:**
```rust
let config = Config::builder()
    .host("localhost")
    .port(8080)
    .build()?;
```

**Newtype Pattern:**
```rust
struct UserId(u64);
struct Email(String);
```

**Error Handling:**
```rust
// Use thiserror for library errors
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },
}
```

**Iterator Chains:**
```rust
let sum: i32 = items
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.value())
    .sum();
```

**Pattern Matching:**
```rust
match result {
    Ok(value) if value > 0 => process(value),
    Ok(_) => handle_zero_or_negative(),
    Err(e) => return Err(e.into()),
}
```

## Rust Syntax Preferences

```rust
// Prefer if let for single-pattern matching
if let Some(value) = optional {
    process(value);
}

// Use match for exhaustive handling
match status {
    Status::Active => activate(),
    Status::Inactive => deactivate(),
    Status::Pending { since } => check_timeout(since),
}

// Use ? for error propagation
fn read_config() -> Result<Config, Error> {
    let file = File::open("config.toml")?;
    let config = parse_config(file)?;
    Ok(config)
}

// Prefer impl Trait for return types
fn create_iterator() -> impl Iterator<Item = i32> {
    (0..100).filter(|x| x % 2 == 0)
}
```

## Key Traits to Implement

| Trait | When to Implement |
|-------|-------------------|
| `Debug` | Always (use derive) |
| `Clone` | When copying makes sense |
| `Default` | When a sensible default exists |
| `Display` | For user-facing output |
| `From/Into` | For type conversions |
| `AsRef/AsMut` | For cheap borrows |
| `Deref` | For smart pointer types |
| `Iterator` | For iterable collections |
| `Send + Sync` | For thread safety (usually auto-derived) |
</quick_reference>

<reference_index>
## Domain Knowledge

All detailed patterns in `references/`:

| File | Topics |
|------|--------|
| [ownership.md](./references/ownership.md) | Borrowing, lifetimes, smart pointers, interior mutability |
| [error-handling.md](./references/error-handling.md) | Result, Option, custom errors, anyhow, thiserror |
| [traits.md](./references/traits.md) | Trait design, bounds, associated types, blanket impls |
| [async.md](./references/async.md) | Tokio patterns, futures, streams, cancellation |
| [modules.md](./references/modules.md) | Project structure, visibility, workspaces, pub(crate) |
| [testing.md](./references/testing.md) | Unit tests, integration tests, doctests, mocking |
| [performance.md](./references/performance.md) | Zero-cost abstractions, benchmarking, profiling |
</reference_index>

<success_criteria>
Code follows idiomatic Rust when:
- The type system prevents invalid states at compile time
- Ownership and borrowing are used correctly (no unnecessary clones)
- Errors are handled explicitly with Result/Option
- Pattern matching is exhaustive
- Traits are used for abstraction and polymorphism
- Iterator methods replace manual loops where appropriate
- Public APIs follow Rust naming conventions
- Unsafe code is minimized and well-documented
- Dependencies are appropriate (not over- or under-using crates)
- Code passes `cargo clippy` without warnings
</success_criteria>

<credits>
Based on:
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- Patterns from successful Rust projects (tokio, serde, axum, etc.)
</credits>
