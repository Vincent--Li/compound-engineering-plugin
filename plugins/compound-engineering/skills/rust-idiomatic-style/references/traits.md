# Traits & Generics

## Defining Traits

```rust
pub trait Summary {
    // Required method
    fn summarize(&self) -> String;
    
    // Provided method with default implementation
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..50])
    }
}
```

## Implementing Traits

```rust
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, &self.content[..100])
    }
}
```

## Trait Bounds

### Where Clauses

```rust
// Simple bound
fn print<T: Display>(item: T) {
    println!("{}", item);
}

// Multiple bounds
fn process<T: Clone + Debug>(item: T) { ... }

// Where clause for readability
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    // ...
}
```

### impl Trait

```rust
// In argument position (static dispatch)
fn process(iter: impl Iterator<Item = i32>) -> i32 {
    iter.sum()
}

// In return position
fn create_iterator() -> impl Iterator<Item = i32> {
    (0..100).filter(|x| x % 2 == 0)
}
```

### dyn Trait (Dynamic Dispatch)

```rust
// Trait objects for heterogeneous collections
fn process_all(items: &[Box<dyn Summary>]) {
    for item in items {
        println!("{}", item.summarize());
    }
}
```

## Associated Types

```rust
trait Container {
    type Item;
    
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

impl<T> Container for Vec<T> {
    type Item = T;
    
    fn get(&self, index: usize) -> Option<&T> {
        <[T]>::get(self, index)
    }
    
    fn len(&self) -> usize {
        self.len()
    }
}
```

## Associated Constants

```rust
trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

impl Bounded for i32 {
    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;
}
```

## Supertraits

```rust
trait Animal {
    fn name(&self) -> &str;
}

// Dog must also implement Animal
trait Dog: Animal {
    fn bark(&self);
}
```

## Blanket Implementations

```rust
// Implement for all types that satisfy bounds
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}
```

## Generic Structs & Enums

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Specialized implementation
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## Common Derived Traits

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Config {
    name: String,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

## Trait Object Safety

A trait is object-safe if:
- All methods have `Self: Sized` or
- All methods satisfy:
  - Don't return `Self`
  - Don't use `Self` in generic type parameters
  - Don't have generic type parameters

```rust
// Object-safe
trait Draw {
    fn draw(&self);
}

// NOT object-safe (returns Self)
trait Clone {
    fn clone(&self) -> Self;
}
```

## Marker Traits

```rust
// Send: can be transferred between threads
// Sync: can be shared between threads via reference

// Automatically implemented for most types
// Opt out with negative impls:
impl !Send for MyType {}
```

## Extension Traits

```rust
// Add methods to existing types
trait StringExt {
    fn is_blank(&self) -> bool;
}

impl StringExt for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
}

// Usage
"  ".is_blank() // true
```
