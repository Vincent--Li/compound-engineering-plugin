# Performance

## Zero-Cost Abstractions

### Iterators

```rust
// Iterators are as fast as manual loops
let sum: i32 = (0..1000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// Equivalent performance to:
let mut sum = 0;
for x in 0..1000 {
    if x % 2 == 0 {
        sum += x * 2;
    }
}
```

### Generic Monomorphization

```rust
// Generics compile to specialized code
fn process<T: Display>(value: T) {
    println!("{}", value);
}

// Compiled as if you wrote:
fn process_i32(value: i32) { println!("{}", value); }
fn process_string(value: String) { println!("{}", value); }
```

## Memory Optimization

### Stack vs Heap

```rust
// Stack allocated (fast)
let point = Point { x: 1.0, y: 2.0 };

// Heap allocated (when necessary)
let boxed = Box::new(LargeStruct::new());

// Choose appropriate collection
let vec: Vec<i32> = vec![1, 2, 3];        // Heap
let array: [i32; 3] = [1, 2, 3];           // Stack
let slice: &[i32] = &array;                 // Reference
```

### Small String Optimization

```rust
use compact_str::CompactString;
use smallvec::SmallVec;

// Inline small strings
let small: CompactString = "hello".into();

// Inline small vectors
let small_vec: SmallVec<[i32; 8]> = smallvec![1, 2, 3];
```

### Avoid Allocations

```rust
// ❌ Allocates new String
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ✅ Return reference when possible
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// ✅ Use Cow for conditional allocation
use std::borrow::Cow;
fn normalize(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}
```

## Benchmarking

### Criterion

```rust
// Cargo.toml
// [dev-dependencies]
// criterion = "0.5"
// [[bench]]
// name = "benchmarks"
// harness = false

// benches/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### Running Benchmarks

```bash
cargo bench

# Compare against baseline
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

## Profiling

### CPU Profiling

```bash
# Using flamegraph
cargo install flamegraph
cargo flamegraph --bin myapp

# Using perf
perf record --call-graph dwarf cargo run --release
perf report
```

### Memory Profiling

```bash
# Using heaptrack
heaptrack cargo run --release
heaptrack_gui heaptrack.myapp.*.gz

# Using valgrind
valgrind --tool=massif cargo run --release
ms_print massif.out.*
```

## Common Optimizations

### Use Release Build

```bash
cargo build --release
cargo run --release
```

### Profile-Guided Optimization

```bash
# Build with instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo" cargo build --release

# Run workload
./target/release/myapp < workload.txt

# Build with PGO data
RUSTFLAGS="-Cprofile-use=/tmp/pgo" cargo build --release
```

### Link-Time Optimization

```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
```

### Cache Computation

```rust
use std::cell::OnceCell;
use std::sync::OnceLock;

// Thread-local lazy initialization
thread_local! {
    static CACHE: OnceCell<ExpensiveData> = OnceCell::new();
}

// Global lazy initialization (thread-safe)
static GLOBAL_CACHE: OnceLock<ExpensiveData> = OnceLock::new();

fn get_data() -> &'static ExpensiveData {
    GLOBAL_CACHE.get_or_init(|| ExpensiveData::compute())
}
```

### Parallel Processing

```rust
use rayon::prelude::*;

// Parallel iteration
let sum: i32 = data.par_iter()
    .map(|x| x * 2)
    .sum();

// Parallel sorting
data.par_sort();
```

## Best Practices

1. **Measure first** - Profile before optimizing
2. **Use release builds** - Debug builds are slow
3. **Prefer stack allocation** - Heap is slower
4. **Avoid unnecessary clones** - Borrow instead
5. **Use iterators** - They're zero-cost
6. **Consider data locality** - Keep related data together
7. **Use appropriate data structures** - HashMap vs BTreeMap, Vec vs VecDeque
8. **Batch allocations** - Reserve capacity upfront

```rust
// Reserve capacity
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);  // No reallocations
}
```
