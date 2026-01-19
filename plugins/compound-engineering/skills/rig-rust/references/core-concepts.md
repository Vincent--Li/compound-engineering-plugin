# Core Concepts

## Agents

Agents are the primary way to interact with LLMs in Rig. They encapsulate a model, preamble (system prompt), tools, and context.

### Basic Agent

```rust
use rig::providers::openai;

let client = openai::Client::from_env();

let agent = client
    .agent("gpt-4o")
    .preamble("You are a helpful assistant.")
    .build();

let response = agent.prompt("Hello!").await?;
```

### Agent with Temperature

```rust
let agent = client
    .agent("gpt-4o")
    .preamble("You are a creative writer.")
    .temperature(0.9)
    .build();
```

### Agent with Max Tokens

```rust
let agent = client
    .agent("gpt-4o")
    .preamble("Be concise.")
    .max_tokens(100)
    .build();
```

## Completions

The completion system provides a unified interface across providers.

### Simple Completion

```rust
let response = agent.prompt("What is 2+2?").await?;
```

### Completion with Context

```rust
let response = agent
    .context("The user is asking about mathematics.")
    .prompt("What is 2+2?")
    .await?;
```

### Chat-style Completion

```rust
use rig::completion::{Chat, Message};

let messages = vec![
    Message::user("Hello!"),
    Message::assistant("Hi there! How can I help?"),
    Message::user("What's the weather?"),
];

let response = agent.chat(messages).await?;
```

## Extractors

Extractors parse unstructured text into typed Rust structures.

### Basic Extraction

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Contact {
    name: String,
    email: String,
    phone: Option<String>,
}

let extractor = client
    .extractor::<Contact>("gpt-4o")
    .build();

let contact: Contact = extractor
    .extract("John Doe, john@example.com, 555-1234")
    .await?;
```

### Extraction with Instructions

```rust
let extractor = client
    .extractor::<Contact>("gpt-4o")
    .preamble("Extract contact information. Be strict about format.")
    .build();
```

## Models

Rig supports multiple model types for different use cases.

### Completion Models

```rust
// GPT-4
let gpt4 = client.completion_model("gpt-4o");

// GPT-3.5
let gpt35 = client.completion_model("gpt-3.5-turbo");

// Claude
let claude = anthropic.completion_model("claude-3-5-sonnet-20241022");
```

### Embedding Models

```rust
// OpenAI embeddings
let embeddings = client.embedding_model("text-embedding-3-small");

// Large embedding model
let large_embeddings = client.embedding_model("text-embedding-3-large");
```

## Error Handling

Rig uses standard Rust error handling patterns.

### Using anyhow

```rust
use anyhow::Result;

async fn process() -> Result<String> {
    let response = agent.prompt("Hello").await?;
    Ok(response)
}
```

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("LLM error: {0}")]
    Llm(#[from] rig::completion::CompletionError),
    
    #[error("Configuration error: {0}")]
    Config(String),
}
```

### Handling Specific Errors

```rust
match agent.prompt("Hello").await {
    Ok(response) => println!("{}", response),
    Err(e) => {
        if e.to_string().contains("rate limit") {
            // Retry logic
        } else {
            return Err(e.into());
        }
    }
}
```

## Async Runtime

Rig is async-first and works best with Tokio.

### Main Function

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Your code here
    Ok(())
}
```

### Spawning Tasks

```rust
let handle = tokio::spawn(async move {
    agent.prompt("Background task").await
});

let result = handle.await??;
```

## Configuration Best Practices

### Environment Variables

```rust
// Prefer from_env() for security
let client = openai::Client::from_env();

// Set in environment:
// OPENAI_API_KEY=sk-...
// ANTHROPIC_API_KEY=sk-ant-...
```

### Multi-Model Setup

```rust
let openai = openai::Client::from_env();
let anthropic = anthropic::Client::from_env();

// Use different models for different tasks
let fast_agent = openai.agent("gpt-4o-mini").build();
let smart_agent = anthropic.agent("claude-3-5-sonnet-20241022").build();
```
