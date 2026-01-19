//! Configuration Templates for Rig

use rig::providers::{openai, anthropic, gemini, cohere};

// OpenAI Configuration
pub fn openai_client() -> openai::Client {
    // Uses OPENAI_API_KEY env var
    openai::Client::from_env()
}

// Anthropic Configuration
pub fn anthropic_client() -> anthropic::Client {
    // Uses ANTHROPIC_API_KEY env var
    anthropic::Client::from_env()
}

// Gemini Configuration  
pub fn gemini_client() -> gemini::Client {
    // Uses GEMINI_API_KEY env var
    gemini::Client::from_env()
}

// Cohere Configuration
pub fn cohere_client() -> cohere::Client {
    // Uses COHERE_API_KEY env var
    cohere::Client::from_env()
}

// Multi-provider setup
pub struct Providers {
    pub openai: openai::Client,
    pub anthropic: anthropic::Client,
}

impl Providers {
    pub fn from_env() -> Self {
        Self {
            openai: openai::Client::from_env(),
            anthropic: anthropic::Client::from_env(),
        }
    }
}
