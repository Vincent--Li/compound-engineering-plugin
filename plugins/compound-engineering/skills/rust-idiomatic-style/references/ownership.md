# Ownership & Lifetimes

## Core Ownership Rules

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. You can have multiple immutable borrows OR one mutable borrow

## Borrowing Patterns

### Prefer Borrowing Over Cloning

```rust
// ❌ Avoid: unnecessary clone
fn process(data: Vec<String>) {
    let copy = data.clone();
    // ...
}

// ✅ Prefer: borrow when you don't need ownership
fn process(data: &[String]) {
    // ...
}
```

### Use Slices for Flexibility

```rust
// ❌ Avoid: requires exact type
fn sum(numbers: &Vec<i32>) -> i32 { ... }

// ✅ Prefer: accepts arrays, vectors, slices
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
```

### Cow for Flexibility

```rust
use std::borrow::Cow;

fn process_name(name: Cow<str>) -> Cow<str> {
    if name.contains(' ') {
        Cow::Owned(name.replace(' ', "_"))
    } else {
        name
    }
}
```

## Lifetime Annotations

### When to Add Lifetimes

```rust
// Compiler infers: fn first(s: &str) -> &str
fn first(s: &str) -> &str {
    &s[..1]
}

// Must annotate when multiple references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Struct Lifetimes

```rust
// When struct holds references
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
}
```

### Static Lifetime

```rust
// Lives for entire program duration
const CONFIG: &'static str = "config";

// String literals are always 'static
let s: &'static str = "hello";
```

## Smart Pointers

### Box<T> - Heap Allocation

```rust
// For recursive types
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// For large data to avoid stack overflow
let large_data = Box::new([0u8; 1_000_000]);
```

### Rc<T> - Reference Counting

```rust
use std::rc::Rc;

// Shared ownership (single-threaded)
let shared = Rc::new(ExpensiveData::new());
let clone1 = Rc::clone(&shared);
let clone2 = Rc::clone(&shared);
```

### Arc<T> - Atomic Reference Counting

```rust
use std::sync::Arc;

// Shared ownership (thread-safe)
let shared = Arc::new(data);
let handle = thread::spawn({
    let shared = Arc::clone(&shared);
    move || {
        // use shared
    }
});
```

## Interior Mutability

### Cell<T> - Copy Types

```rust
use std::cell::Cell;

struct Counter {
    value: Cell<i32>,
}

impl Counter {
    fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }
}
```

### RefCell<T> - Runtime Borrow Checking

```rust
use std::cell::RefCell;

struct Cache {
    data: RefCell<HashMap<String, String>>,
}

impl Cache {
    fn get_or_insert(&self, key: &str) -> String {
        let mut data = self.data.borrow_mut();
        data.entry(key.to_string())
            .or_insert_with(|| compute_value(key))
            .clone()
    }
}
```

### Mutex<T> / RwLock<T> - Thread-Safe

```rust
use std::sync::{Mutex, RwLock};

// Single writer or reader
let counter = Mutex::new(0);
*counter.lock().unwrap() += 1;

// Multiple readers or single writer
let data = RwLock::new(HashMap::new());
let read = data.read().unwrap();
let mut write = data.write().unwrap();
```

## Common Patterns

### Taking Ownership Efficiently

```rust
// Take by value when you need ownership
fn consume(s: String) { ... }

// Return ownership back
fn transform(s: String) -> String {
    s.to_uppercase()
}

// Or borrow if you don't need it
fn analyze(s: &str) -> usize {
    s.len()
}
```

### Entry API for Maps

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Efficient insert-or-update
map.entry("key".to_string())
    .and_modify(|v| *v += 1)
    .or_insert(1);
```
