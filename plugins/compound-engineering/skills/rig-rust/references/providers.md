# Providers

## OpenAI

```rust
use rig::providers::openai;

// From environment
let client = openai::Client::from_env();
// Uses OPENAI_API_KEY

// Explicit API key
let client = openai::Client::new("sk-...");

// Available models
let gpt4 = client.agent("gpt-4o").build();
let gpt4_mini = client.agent("gpt-4o-mini").build();
let gpt35 = client.agent("gpt-3.5-turbo").build();

// Embeddings
let embeddings = client.embedding_model("text-embedding-3-small");
let large_embeddings = client.embedding_model("text-embedding-3-large");
```

### OpenAI-Compatible APIs

```rust
// Azure OpenAI
let client = openai::Client::new("...")
    .with_base_url("https://your-resource.openai.azure.com/openai/deployments/your-deployment");

// Local Ollama with OpenAI-compatible API
let client = openai::Client::new("ollama")
    .with_base_url("http://localhost:11434/v1");
```

## Anthropic

```rust
use rig::providers::anthropic;

// From environment
let client = anthropic::Client::from_env();
// Uses ANTHROPIC_API_KEY

// Available models
let claude35 = client.agent("claude-3-5-sonnet-20241022").build();
let claude3_opus = client.agent("claude-3-opus-20240229").build();
let claude3_haiku = client.agent("claude-3-haiku-20240307").build();
```

### Anthropic-Specific Features

```rust
// Extended thinking (Claude 3.5)
let agent = client
    .agent("claude-3-5-sonnet-20241022")
    .preamble("Think step by step.")
    .build();
```

## Google Gemini

```rust
use rig::providers::gemini;

// From environment
let client = gemini::Client::from_env();
// Uses GEMINI_API_KEY

// Available models
let gemini_pro = client.agent("gemini-1.5-pro").build();
let gemini_flash = client.agent("gemini-1.5-flash").build();
```

## Cohere

```rust
use rig::providers::cohere;

// From environment
let client = cohere::Client::from_env();
// Uses COHERE_API_KEY

// Completion
let command = client.agent("command-r-plus").build();

// Embeddings
let embed = client.embedding_model("embed-english-v3.0");
```

## Provider Feature Comparison

| Feature | OpenAI | Anthropic | Gemini | Cohere |
|---------|--------|-----------|--------|--------|
| Completion | ✅ | ✅ | ✅ | ✅ |
| Streaming | ✅ | ✅ | ✅ | ✅ |
| Tool Calling | ✅ | ✅ | ✅ | ✅ |
| Embeddings | ✅ | ✅ | ✅ | ✅ |
| Vision | ✅ | ✅ | ✅ | ❌ |
| JSON Mode | ✅ | ✅ | ✅ | ✅ |

## Model Selection Guide

### By Task Complexity

| Task | Recommended Model |
|------|-------------------|
| Simple Q&A | gpt-4o-mini, claude-3-haiku |
| Complex reasoning | gpt-4o, claude-3-5-sonnet |
| Code generation | gpt-4o, claude-3-5-sonnet |
| Long context | gemini-1.5-pro (1M tokens) |
| Fast responses | gpt-4o-mini, gemini-1.5-flash |

### By Cost (approximate)

| Tier | Models |
|------|--------|
| Budget | gpt-4o-mini, claude-3-haiku, gemini-1.5-flash |
| Standard | gpt-4o, claude-3-5-sonnet, gemini-1.5-pro |
| Premium | claude-3-opus |

## Error Handling by Provider

```rust
match agent.prompt("Hello").await {
    Ok(response) => println!("{}", response),
    Err(e) => {
        let error_str = e.to_string();
        
        // Rate limiting
        if error_str.contains("rate limit") {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
        
        // Context too long
        if error_str.contains("maximum context length") {
            // Truncate or summarize input
        }
        
        // API key issues
        if error_str.contains("authentication") {
            // Check API key
        }
    }
}
```

## Multi-Provider Setup

```rust
use rig::providers::{openai, anthropic, gemini};

struct MultiProvider {
    openai: openai::Client,
    anthropic: anthropic::Client,
    gemini: gemini::Client,
}

impl MultiProvider {
    fn new() -> Self {
        Self {
            openai: openai::Client::from_env(),
            anthropic: anthropic::Client::from_env(),
            gemini: gemini::Client::from_env(),
        }
    }
    
    fn fast_agent(&self) -> impl rig::completion::Prompt {
        self.openai.agent("gpt-4o-mini").build()
    }
    
    fn smart_agent(&self) -> impl rig::completion::Prompt {
        self.anthropic.agent("claude-3-5-sonnet-20241022").build()
    }
    
    fn long_context_agent(&self) -> impl rig::completion::Prompt {
        self.gemini.agent("gemini-1.5-pro").build()
    }
}
```
