//! Agent Templates for Rig
//!
//! This file contains various agent patterns for common use cases.

use rig::providers::openai;
use rig::completion::Prompt;
use anyhow::Result;

// =============================================================================
// BASIC AGENT
// =============================================================================

/// Simple agent for basic Q&A
pub async fn basic_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();
    
    let response = agent.prompt("Hello!").await?;
    println!("{}", response);
    
    Ok(())
}

// =============================================================================
// CONFIGURED AGENT
// =============================================================================

/// Agent with temperature and token limit
pub async fn configured_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a creative assistant.")
        .temperature(0.8)  // Higher for creativity
        .max_tokens(1000)  // Limit response length
        .build();
    
    let response = agent
        .prompt("Write a haiku about Rust programming.")
        .await?;
    
    println!("{}", response);
    Ok(())
}

// =============================================================================
// SPECIALIZED AGENTS
// =============================================================================

/// Code review agent
pub async fn code_review_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble(r#"
            You are an expert code reviewer specializing in Rust.
            Review code for:
            - Correctness and safety
            - Idiomatic Rust patterns
            - Performance considerations
            - Error handling
            Provide specific, actionable feedback.
        "#)
        .temperature(0.3)  // Lower for consistency
        .build();
    
    let code = r#"
        fn process(data: Vec<String>) -> Vec<String> {
            let mut result = vec![];
            for item in data {
                result.push(item.to_uppercase());
            }
            result
        }
    "#;
    
    let response = agent
        .prompt(&format!("Review this Rust code:\n```rust\n{}\n```", code))
        .await?;
    
    println!("{}", response);
    Ok(())
}

/// SQL generation agent
pub async fn sql_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble(r#"
            You are a SQL expert. Generate PostgreSQL queries.
            - Use parameterized queries with $1, $2, etc. for user input
            - Prefer JOINs over subqueries when possible
            - Include appropriate indexes suggestions
            - Return only the SQL, no explanations
        "#)
        .build();
    
    let response = agent
        .prompt("Find all users who made purchases in the last 30 days")
        .await?;
    
    println!("{}", response);
    Ok(())
}

// =============================================================================
// MULTI-TURN CONVERSATION
// =============================================================================

use rig::completion::{Chat, Message};

/// Multi-turn chat agent
pub async fn chat_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a helpful coding tutor.")
        .build();
    
    // First turn
    let messages = vec![
        Message::user("What is ownership in Rust?"),
    ];
    let response1 = agent.chat(messages.clone()).await?;
    println!("Assistant: {}", response1);
    
    // Second turn (include history)
    let mut messages = messages;
    messages.push(Message::assistant(&response1));
    messages.push(Message::user("Can you give an example?"));
    
    let response2 = agent.chat(messages).await?;
    println!("Assistant: {}", response2);
    
    Ok(())
}

// =============================================================================
// STREAMING AGENT
// =============================================================================

use futures::StreamExt;

/// Streaming response agent
pub async fn streaming_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a storyteller.")
        .build();
    
    let mut stream = agent
        .stream_prompt("Tell me a short story about a Rustacean.")
        .await?;
    
    print!("Story: ");
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(text) => print!("{}", text),
            Err(e) => eprintln!("\nError: {}", e),
        }
    }
    println!();
    
    Ok(())
}

// =============================================================================
// FALLBACK PATTERN
// =============================================================================

/// Agent with fallback to cheaper model
pub async fn fallback_agent() -> Result<()> {
    let client = openai::Client::from_env();
    
    let primary = client
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();
    
    let fallback = client
        .agent("gpt-4o-mini")
        .preamble("You are a helpful assistant.")
        .build();
    
    let query = "What is Rust?";
    
    let response = match primary.prompt(query).await {
        Ok(r) => r,
        Err(_) => {
            println!("Primary failed, using fallback...");
            fallback.prompt(query).await?
        }
    };
    
    println!("{}", response);
    Ok(())
}
