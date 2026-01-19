---
name: rig-rust
description: This skill should be used when working with Rig, a Rust framework for building LLM-powered applications. Use this when implementing AI features, creating agents with tools, configuring LLM providers (OpenAI, Anthropic, Gemini, Cohere), building RAG pipelines, or working with embeddings and vector stores in Rust applications.
---

# Rig Expert

## Overview

Rig is a Rust library for building LLM-powered applications with a focus on **ergonomics, modularity, and type-safety**. It provides a unified API for working with multiple LLM providers and makes it easy to build agents with tools, RAG systems, and complex AI workflows.

This skill provides comprehensive guidance on:
- Creating agents with custom tools
- Configuring multiple LLM providers
- Building RAG pipelines with vector stores
- Working with embeddings
- Implementing reliable extraction patterns
- Production deployment best practices

## Core Capabilities

### 1. Basic Completion

Create simple LLM completions with type-safe responses.

**When to use**: Basic text generation, simple Q&A, completions without complex tooling.

**Quick reference**:
```rust
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai = openai::Client::from_env();
    
    let agent = openai
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();
    
    let response = agent
        .prompt("What is the capital of France?")
        .await?;
    
    println!("{}", response);
    Ok(())
}
```

**Templates**: See `assets/agent-template.rs` for comprehensive examples.

**Best practices**:
- Use appropriate model for task complexity
- Set clear preamble for consistent behavior
- Handle errors with `anyhow` or custom error types

**Full documentation**: See `references/core-concepts.md` for details.

### 2. Structured Extraction

Extract structured data from text using deriving `schemars::JsonSchema`.

**When to use**: Parsing unstructured text into typed Rust structures.

**Quick reference**:
```rust
use rig::{completion::Prompt, providers::openai};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct Person {
    /// The person's full name
    name: String,
    /// The person's age in years
    age: u32,
    /// The person's email address
    email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai = openai::Client::from_env();
    
    let extractor = openai
        .extractor::<Person>("gpt-4o")
        .build();
    
    let person: Person = extractor
        .extract("John Doe is 30 years old. His email is john@example.com")
        .await?;
    
    println!("{:?}", person);
    Ok(())
}
```

**Best practices**:
- Use doc comments on struct fields for better extraction
- Make optional fields `Option<T>`
- Validate extracted data after extraction

**Full documentation**: See `references/extraction.md`.

### 3. Agents with Tools

Build agents that can use custom tools to perform actions.

**When to use**: Complex tasks requiring external actions, API calls, calculations.

**Quick reference**:
```rust
use rig::{
    completion::ToolDefinition,
    providers::openai,
    tool::Tool,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct CalculatorInput {
    operation: String,
    a: f64,
    b: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("Calculator error: {0}")]
struct CalculatorError(String);

struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    
    type Args = CalculatorInput;
    type Output = f64;
    type Error = CalculatorError;
    
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Perform mathematical calculations".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"]
                    },
                    "a": { "type": "number" },
                    "b": { "type": "number" }
                },
                "required": ["operation", "a", "b"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.operation.as_str() {
            "add" => Ok(args.a + args.b),
            "subtract" => Ok(args.a - args.b),
            "multiply" => Ok(args.a * args.b),
            "divide" => {
                if args.b == 0.0 {
                    Err(CalculatorError("Division by zero".into()))
                } else {
                    Ok(args.a / args.b)
                }
            }
            _ => Err(CalculatorError("Unknown operation".into())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai = openai::Client::from_env();
    
    let agent = openai
        .agent("gpt-4o")
        .preamble("You are a calculator assistant.")
        .tool(Calculator)
        .build();
    
    let response = agent
        .prompt("What is 42 multiplied by 17?")
        .await?;
    
    println!("{}", response);
    Ok(())
}
```

**Templates**: See `assets/tool-template.rs` for more examples.

**Full documentation**: See `references/tools.md`.

### 4. LLM Provider Configuration

Support for OpenAI, Anthropic Claude, Google Gemini, Cohere, and local models.

**Quick configuration examples**:
```rust
use rig::providers::{openai, anthropic, gemini, cohere};

// OpenAI
let openai = openai::Client::from_env();
// Uses OPENAI_API_KEY environment variable

// Anthropic Claude
let anthropic = anthropic::Client::from_env();
// Uses ANTHROPIC_API_KEY environment variable

// Google Gemini
let gemini = gemini::Client::from_env();
// Uses GEMINI_API_KEY environment variable

// Cohere
let cohere = cohere::Client::from_env();
// Uses COHERE_API_KEY environment variable

// With explicit API key
let openai = openai::Client::new("sk-...");
```

**Provider compatibility matrix**:

| Feature | OpenAI | Anthropic | Gemini | Cohere |
|---------|--------|-----------|--------|--------|
| Completion | ✅ | ✅ | ✅ | ✅ |
| Streaming | ✅ | ✅ | ✅ | ✅ |
| Tool Calling | ✅ | ✅ | ✅ | ✅ |
| Embeddings | ✅ | ✅ | ✅ | ✅ |
| Vision | ✅ | ✅ | ✅ | ❌ |

**Templates**: See `assets/config-template.rs` for comprehensive examples.

**Full documentation**: See `references/providers.md`.

### 5. RAG (Retrieval-Augmented Generation)

Build RAG pipelines with vector stores and embeddings.

**When to use**: Question answering over documents, knowledge bases, semantic search.

**Quick reference**:
```rust
use rig::{
    embeddings::EmbeddingsBuilder,
    providers::openai,
    vector_store::in_memory_store::InMemoryVectorStore,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let openai = openai::Client::from_env();
    
    // Create embedding model
    let embedding_model = openai.embedding_model("text-embedding-3-small");
    
    // Create vector store with documents
    let mut vector_store = InMemoryVectorStore::default();
    
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc1", "Paris is the capital of France.")
        .simple_document("doc2", "London is the capital of England.")
        .simple_document("doc3", "Berlin is the capital of Germany.")
        .build()
        .await?;
    
    vector_store.add_documents(embeddings).await?;
    
    // Create RAG agent
    let rag_agent = openai
        .agent("gpt-4o")
        .preamble("Answer questions based on the provided context.")
        .dynamic_context(2, vector_store.index(embedding_model))
        .build();
    
    let response = rag_agent
        .prompt("What is the capital of France?")
        .await?;
    
    println!("{}", response);
    Ok(())
}
```

**Full documentation**: See `references/rag.md`.

### 6. Embeddings

Create and work with embeddings for semantic similarity.

**Quick reference**:
```rust
use rig::{embeddings::EmbeddingsBuilder, providers::openai};

let openai = openai::Client::from_env();
let model = openai.embedding_model("text-embedding-3-small");

// Single embedding
let embedding = model.embed_text("Hello, world!").await?;

// Batch embeddings
let embeddings = EmbeddingsBuilder::new(model)
    .simple_document("id1", "First document")
    .simple_document("id2", "Second document")
    .build()
    .await?;
```

**Full documentation**: See `references/embeddings.md`.

### 7. Streaming Responses

Handle streaming responses for real-time output.

**Quick reference**:
```rust
use rig::providers::openai;
use futures::StreamExt;

let openai = openai::Client::from_env();

let agent = openai
    .agent("gpt-4o")
    .build();

let mut stream = agent
    .stream_prompt("Write a poem about Rust")
    .await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(text) => print!("{}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Quick Start Workflow

### For New Projects

1. **Add Rig to Cargo.toml**:
```toml
[dependencies]
rig-core = "0.4"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = "0.8"
```

2. **Set up environment variables**:
```bash
export OPENAI_API_KEY="sk-..."
# or
export ANTHROPIC_API_KEY="sk-ant-..."
```

3. **Create basic agent**:
```rust
use rig::providers::openai;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();
    
    let response = agent.prompt("Hello!").await?;
    println!("{}", response);
    Ok(())
}
```

4. **Add tools as needed** (see `assets/tool-template.rs`)

5. **Add RAG for knowledge-based Q&A** (see `references/rag.md`)

## Common Patterns

### Pattern: Multi-Tool Agent

```rust
let agent = openai
    .agent("gpt-4o")
    .preamble("You are a helpful assistant with multiple capabilities.")
    .tool(Calculator)
    .tool(WebSearch)
    .tool(DatabaseQuery)
    .build();
```

### Pattern: Fallback Models

```rust
let response = primary_agent
    .prompt(query)
    .await
    .or_else(|_| fallback_agent.prompt(query))
    .await?;
```

### Pattern: Retry with Backoff

```rust
use tokio::time::{sleep, Duration};

async fn with_retry<F, T, E>(f: F, max_retries: u32) -> Result<T, E>
where
    F: Fn() -> futures::future::BoxFuture<'static, Result<T, E>>,
{
    let mut retries = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if retries < max_retries => {
                retries += 1;
                sleep(Duration::from_millis(100 * 2u64.pow(retries))).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### Pattern: Context Window Management

```rust
// Use dynamic context with limited results
let agent = openai
    .agent("gpt-4o")
    .dynamic_context(5, vector_store.index(embedding_model)) // Top 5 results
    .build();
```

## Resources

This skill includes comprehensive reference materials and templates:

### References (load as needed for detailed information)

- [core-concepts.md](./references/core-concepts.md): Complete guide to agents, completions, and basic usage
- [tools.md](./references/tools.md): Building custom tools, tool patterns, error handling
- [rag.md](./references/rag.md): RAG pipelines, vector stores, document processing
- [providers.md](./references/providers.md): All LLM provider configurations and features

### Assets (templates for quick starts)

- [agent-template.rs](./assets/agent-template.rs): Agent examples from basic to advanced
- [tool-template.rs](./assets/tool-template.rs): Tool implementation patterns
- [config-template.rs](./assets/config-template.rs): Configuration patterns for all providers

## When to Use This Skill

Trigger this skill when:
- Building LLM-powered features in Rust applications
- Creating agents with custom tools
- Setting up or troubleshooting LLM providers in Rust
- Building RAG systems with vector stores
- Working with embeddings in Rust
- Implementing structured extraction
- Debugging Rig code or configuration issues
