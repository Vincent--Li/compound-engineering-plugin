//! Tool Templates for Rig

use rig::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::Deserialize;

// Calculator Tool
#[derive(Deserialize, JsonSchema)]
pub struct CalculatorInput {
    expression: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Calculation error: {0}")]
pub struct CalculatorError(String);

pub struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    type Args = CalculatorInput;
    type Output = f64;
    type Error = CalculatorError;
    
    async fn definition(&self, _: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Evaluate math expressions".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": { "type": "string" }
                },
                "required": ["expression"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<f64, CalculatorError> {
        meval::eval_str(&args.expression)
            .map_err(|e| CalculatorError(e.to_string()))
    }
}

// HTTP Client Tool
#[derive(Deserialize, JsonSchema)]
pub struct HttpInput {
    url: String,
    #[serde(default)]
    method: String,
}

#[derive(Debug, thiserror::Error)]
#[error("HTTP error: {0}")]
pub struct HttpError(String);

pub struct HttpClient {
    client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self { client: reqwest::Client::new() }
    }
}

impl Tool for HttpClient {
    const NAME: &'static str = "http_request";
    type Args = HttpInput;
    type Output = String;
    type Error = HttpError;
    
    async fn definition(&self, _: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Make HTTP requests".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "method": { "type": "string", "default": "GET" }
                },
                "required": ["url"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<String, HttpError> {
        let method = if args.method.is_empty() { "GET" } else { &args.method };
        let method: reqwest::Method = method.parse()
            .map_err(|_| HttpError("Invalid method".into()))?;
        
        self.client.request(method, &args.url)
            .send().await
            .map_err(|e| HttpError(e.to_string()))?
            .text().await
            .map_err(|e| HttpError(e.to_string()))
    }
}
