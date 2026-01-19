# Tools

Tools allow agents to perform actions beyond text generation.

## Defining a Tool

```rust
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
struct SearchInput {
    /// The search query
    query: String,
    /// Maximum number of results
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize { 10 }

#[derive(Debug, thiserror::Error)]
#[error("Search error: {0}")]
struct SearchError(String);

struct WebSearch {
    api_key: String,
}

impl Tool for WebSearch {
    const NAME: &'static str = "web_search";
    
    type Args = SearchInput;
    type Output = Vec<SearchResult>;
    type Error = SearchError;
    
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for information".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum results",
                        "default": 10
                    }
                },
                "required": ["query"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Perform actual search
        let results = search_api(&args.query, args.limit).await
            .map_err(|e| SearchError(e.to_string()))?;
        Ok(results)
    }
}
```

## Adding Tools to Agents

```rust
let agent = openai
    .agent("gpt-4o")
    .preamble("You are a research assistant.")
    .tool(WebSearch { api_key: "...".into() })
    .tool(Calculator)
    .tool(DatabaseQuery::new(db_pool))
    .build();
```

## Common Tool Patterns

### Calculator Tool

```rust
#[derive(Deserialize, JsonSchema)]
struct CalcInput {
    expression: String,
}

struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    type Args = CalcInput;
    type Output = f64;
    type Error = CalcError;
    
    async fn definition(&self, _: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Evaluate mathematical expressions".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Math expression to evaluate"
                    }
                },
                "required": ["expression"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<f64, CalcError> {
        meval::eval_str(&args.expression)
            .map_err(|e| CalcError(e.to_string()))
    }
}
```

### HTTP Request Tool

```rust
#[derive(Deserialize, JsonSchema)]
struct HttpInput {
    url: String,
    method: String,
    body: Option<String>,
}

struct HttpClient {
    client: reqwest::Client,
}

impl Tool for HttpClient {
    const NAME: &'static str = "http_request";
    type Args = HttpInput;
    type Output = String;
    type Error = HttpError;
    
    async fn call(&self, args: Self::Args) -> Result<String, HttpError> {
        let method = args.method.parse()
            .map_err(|_| HttpError("Invalid method".into()))?;
        
        let mut req = self.client.request(method, &args.url);
        
        if let Some(body) = args.body {
            req = req.body(body);
        }
        
        let response = req.send().await
            .map_err(|e| HttpError(e.to_string()))?;
        
        response.text().await
            .map_err(|e| HttpError(e.to_string()))
    }
}
```

### Database Query Tool

```rust
use sqlx::PgPool;

#[derive(Deserialize, JsonSchema)]
struct QueryInput {
    /// SQL query to execute (SELECT only)
    query: String,
}

struct DatabaseQuery {
    pool: PgPool,
}

impl Tool for DatabaseQuery {
    const NAME: &'static str = "database_query";
    type Args = QueryInput;
    type Output = serde_json::Value;
    type Error = DbError;
    
    async fn call(&self, args: Self::Args) -> Result<serde_json::Value, DbError> {
        // Safety: Only allow SELECT queries
        if !args.query.trim().to_uppercase().starts_with("SELECT") {
            return Err(DbError("Only SELECT queries allowed".into()));
        }
        
        let rows: Vec<serde_json::Value> = sqlx::query_as(&args.query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError(e.to_string()))?;
        
        Ok(serde_json::to_value(rows)?)
    }
}
```

## Tool Error Handling

### Custom Error Types

```rust
#[derive(Debug, thiserror::Error)]
enum ToolError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),
}
```

### Graceful Degradation

```rust
async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
    match primary_service(&args).await {
        Ok(result) => Ok(result),
        Err(_) => {
            // Fall back to secondary service
            fallback_service(&args).await
        }
    }
}
```

## Tool Best Practices

1. **Clear descriptions**: Write detailed descriptions for the LLM
2. **Validate inputs**: Check arguments before processing
3. **Handle errors gracefully**: Provide meaningful error messages
4. **Limit scope**: Tools should do one thing well
5. **Security**: Sanitize inputs, especially for SQL/shell commands
6. **Async-safe**: Use async-compatible libraries
