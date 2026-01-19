# RAG (Retrieval-Augmented Generation)

## Overview

RAG combines retrieval from a knowledge base with LLM generation for grounded responses.

## Basic RAG Pipeline

```rust
use rig::{
    embeddings::EmbeddingsBuilder,
    providers::openai,
    vector_store::in_memory_store::InMemoryVectorStore,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let openai = openai::Client::from_env();
    let embedding_model = openai.embedding_model("text-embedding-3-small");
    
    // 1. Create vector store
    let mut vector_store = InMemoryVectorStore::default();
    
    // 2. Add documents with embeddings
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .simple_document("doc1", "Rust is a systems programming language.")
        .simple_document("doc2", "Python is known for its simplicity.")
        .simple_document("doc3", "JavaScript runs in web browsers.")
        .build()
        .await?;
    
    vector_store.add_documents(embeddings).await?;
    
    // 3. Create RAG agent
    let agent = openai
        .agent("gpt-4o")
        .preamble("Answer based on the provided context. If unsure, say so.")
        .dynamic_context(3, vector_store.index(embedding_model))
        .build();
    
    // 4. Query
    let response = agent.prompt("What is Rust?").await?;
    println!("{}", response);
    
    Ok(())
}
```

## Document Processing

### Simple Documents

```rust
let embeddings = EmbeddingsBuilder::new(model)
    .simple_document("id1", "Document content here.")
    .simple_document("id2", "Another document.")
    .build()
    .await?;
```

### Documents with Metadata

```rust
use rig::embeddings::DocumentEmbeddings;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct DocMetadata {
    title: String,
    source: String,
    date: String,
}

let doc = DocumentEmbeddings::new(
    "doc1",
    "Document content...",
    DocMetadata {
        title: "Guide".into(),
        source: "manual".into(),
        date: "2024-01-01".into(),
    },
);
```

### Chunking Large Documents

```rust
fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut i = 0;
    
    while i < words.len() {
        let end = (i + chunk_size).min(words.len());
        let chunk = words[i..end].join(" ");
        chunks.push(chunk);
        i += chunk_size - overlap;
    }
    
    chunks
}

// Use in embeddings
let chunks = chunk_text(&large_document, 500, 50);
let mut builder = EmbeddingsBuilder::new(model);

for (i, chunk) in chunks.iter().enumerate() {
    builder = builder.simple_document(
        format!("doc_chunk_{}", i),
        chunk.clone(),
    );
}

let embeddings = builder.build().await?;
```

## Vector Stores

### In-Memory Store (Built-in)

```rust
use rig::vector_store::in_memory_store::InMemoryVectorStore;

let mut store = InMemoryVectorStore::default();
store.add_documents(embeddings).await?;
```

### Integration with External Stores

Rig supports integration with various vector databases:

```rust
// MongoDB Atlas
use rig_mongodb::MongoDbVectorStore;

// Qdrant
use rig_qdrant::QdrantVectorStore;

// LanceDB
use rig_lancedb::LanceDbVectorStore;
```

### Similarity Search

```rust
let index = vector_store.index(embedding_model);

// Search for similar documents
let results = index
    .top_n::<String>("What is Rust?", 5)
    .await?;

for (score, doc) in results {
    println!("Score: {:.4}, Content: {}", score, doc);
}
```

## Advanced RAG Patterns

### Hybrid Search

Combine semantic and keyword search:

```rust
async fn hybrid_search(
    query: &str,
    semantic_store: &VectorStore,
    keyword_index: &KeywordIndex,
    model: &EmbeddingModel,
) -> Vec<Document> {
    let semantic_results = semantic_store
        .index(model.clone())
        .top_n::<String>(query, 10)
        .await?;
    
    let keyword_results = keyword_index.search(query, 10);
    
    // Combine and deduplicate
    merge_results(semantic_results, keyword_results)
}
```

### Query Expansion

```rust
async fn expand_query(agent: &Agent, query: &str) -> Vec<String> {
    let expanded = agent
        .prompt(&format!(
            "Generate 3 alternative phrasings of this question: {}",
            query
        ))
        .await?;
    
    expanded.lines().map(String::from).collect()
}
```

### Reranking

```rust
async fn rerank(
    agent: &Agent,
    query: &str,
    candidates: Vec<String>,
) -> Vec<String> {
    let prompt = format!(
        "Rank these documents by relevance to: {}\n\nDocuments:\n{}",
        query,
        candidates.iter().enumerate()
            .map(|(i, d)| format!("{}. {}", i + 1, d))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    // Parse ranking from response
    // ...
}
```

## RAG Best Practices

1. **Chunk appropriately**: Balance between context and specificity
2. **Use overlap**: Prevent context loss at chunk boundaries
3. **Include metadata**: Add source, date, and category for filtering
4. **Test retrieval**: Verify relevant documents are returned
5. **Limit context**: Don't overwhelm the model with too many chunks
6. **Handle no results**: Gracefully respond when no relevant docs found

```rust
let agent = openai
    .agent("gpt-4o")
    .preamble(r#"
        Answer based on the provided context.
        If the context doesn't contain relevant information, say:
        "I don't have enough information to answer that question."
    "#)
    .dynamic_context(3, vector_store.index(model))
    .build();
```
