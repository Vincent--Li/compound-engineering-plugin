# Async/Await Patterns

## Basic Async

```rust
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

## Runtime Setup (Tokio)

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = fetch_data("https://example.com").await?;
    println!("{}", result);
    Ok(())
}

// Or with configuration
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() { ... }

// Single-threaded runtime
#[tokio::main(flavor = "current_thread")]
async fn main() { ... }
```

## Spawning Tasks

```rust
use tokio::task;

// Spawn a task
let handle = task::spawn(async {
    // async work
    42
});

let result = handle.await?;

// Spawn blocking work
let result = task::spawn_blocking(|| {
    // CPU-intensive or blocking I/O
    compute_heavy_work()
}).await?;
```

## Concurrent Execution

### Join Multiple Futures

```rust
use tokio::try_join;

async fn fetch_all() -> Result<(A, B, C), Error> {
    let (a, b, c) = try_join!(
        fetch_a(),
        fetch_b(),
        fetch_c()
    )?;
    Ok((a, b, c))
}

// Or with join! for non-Result futures
use tokio::join;
let (a, b) = join!(future_a, future_b);
```

### Select First to Complete

```rust
use tokio::select;

select! {
    result = fetch_data() => {
        println!("Got data: {:?}", result);
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("Timeout!");
    }
}
```

## Streams

```rust
use futures::stream::{self, StreamExt};

// Create a stream
let stream = stream::iter(vec![1, 2, 3, 4, 5]);

// Process stream
let results: Vec<_> = stream
    .map(|x| async move { x * 2 })
    .buffer_unordered(4)  // Concurrent processing
    .collect()
    .await;
```

## Channels

### MPSC (Multi-producer, single-consumer)

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send("hello").await.unwrap();
});

while let Some(msg) = rx.recv().await {
    println!("Got: {}", msg);
}
```

### Broadcast

```rust
use tokio::sync::broadcast;

let (tx, _rx) = broadcast::channel(16);

// Each subscriber gets all messages
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tx.send("hello").unwrap();
```

### Oneshot

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    tx.send("result").unwrap();
});

let value = rx.await?;
```

## Synchronization

### Mutex

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

let data = Arc::new(Mutex::new(0));

let data_clone = Arc::clone(&data);
tokio::spawn(async move {
    let mut lock = data_clone.lock().await;
    *lock += 1;
});
```

### RwLock

```rust
use tokio::sync::RwLock;

let data = RwLock::new(HashMap::new());

// Multiple readers
let read = data.read().await;

// Single writer
let mut write = data.write().await;
```

### Semaphore

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

let semaphore = Arc::new(Semaphore::new(10));

let permit = semaphore.acquire().await?;
// Do work with limited concurrency
drop(permit);
```

## Timeouts and Delays

```rust
use tokio::time::{timeout, sleep, Duration};

// Add timeout to operation
match timeout(Duration::from_secs(5), some_operation()).await {
    Ok(result) => println!("Got result: {:?}", result),
    Err(_) => println!("Operation timed out"),
}

// Delay
sleep(Duration::from_millis(100)).await;
```

## Graceful Shutdown

```rust
use tokio::signal;
use tokio::sync::broadcast;

let (shutdown_tx, _) = broadcast::channel(1);

// Listen for shutdown
let mut shutdown_rx = shutdown_tx.subscribe();
tokio::spawn(async move {
    loop {
        select! {
            _ = shutdown_rx.recv() => {
                println!("Shutting down...");
                break;
            }
            result = do_work() => {
                // process result
            }
        }
    }
});

// Trigger shutdown
signal::ctrl_c().await?;
shutdown_tx.send(())?;
```

## Async Traits

```rust
use async_trait::async_trait;

#[async_trait]
trait Repository {
    async fn find(&self, id: i64) -> Option<Entity>;
    async fn save(&self, entity: &Entity) -> Result<(), Error>;
}

#[async_trait]
impl Repository for PostgresRepo {
    async fn find(&self, id: i64) -> Option<Entity> {
        // async implementation
    }
    
    async fn save(&self, entity: &Entity) -> Result<(), Error> {
        // async implementation
    }
}
```

## Best Practices

1. **Avoid blocking in async code** - Use `spawn_blocking` for CPU-heavy work
2. **Use appropriate channel types** - mpsc for work queues, broadcast for pub/sub
3. **Handle cancellation** - Design for graceful shutdown
4. **Limit concurrency** - Use semaphores to prevent resource exhaustion
5. **Prefer `try_join!` over `join!`** - For proper error propagation
