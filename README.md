# UPDATE: Added comments to main.rs for Rust noobs. 
([Arc.fun Rig Agent RAG Code Walkthru](https://github.com/unicornlaunching/rig-rag-system-example/blob/main/The%20Syndicate%20-%20Making%20Agents%20with%20Rig%20on%20Arc.fun%20with%20Rust.mp3?raw=true))

# PDF-Based RAG System with Rig

A Retrieval-Augmented Generation (RAG) system built in Rust using the Rig framework. This system processes PDF documents, generates embeddings, and enables interactive Q&A based on the document content.

## Features

- PDF document processing with automatic chunking
- OpenAI embeddings generation
- In-memory vector store for document retrieval
- Interactive CLI interface for Q&A
- Context-aware responses using RAG

## Prerequisites

- Rust (latest stable version)
- An OpenAI API key
- PDF documents in the `documents` directory

## Setup

1. Create a new Rust project:
```bash
cargo new rag_system
cd rag_system
```

2. Add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
rig-core = { version = "0.5.0", features = ["pdf", "derive"] }
tokio = { version = "1.34.0", features = ["full"] }
anyhow = "1.0.75"
serde = { version = "1.0", features = ["derive"] }
```

3. Set up your OpenAI API key:
```bash
export OPENAI_API_KEY=your-api-key-here
```

4. Create a `documents` directory and add your PDF files:
```bash
mkdir documents
# Add your PDF files to the documents directory
```

## Project Structure

```
rag_system/
├── Cargo.toml
├── Cargo.lock
├── documents/
│   ├── document1.pdf
│   └── document2.pdf
└── src/
    └── main.rs
```

## Code Overview

The system consists of several key components:

### Document Structure
```rust
#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Document {
    id: String,
    #[embed]
    content: String,
}
```
Represents a document chunk with a unique ID and content.

### PDF Processing
The `load_pdf` function:
- Loads PDF content using Rig's built-in PDF loader
- Splits content into manageable chunks (2000 characters each)
- Maintains word boundaries while chunking
- Handles errors gracefully

### RAG Pipeline
The main pipeline:
1. Loads and chunks PDF documents
2. Generates embeddings using OpenAI's text-embedding-ada-002 model
3. Stores embeddings in an in-memory vector store
4. Creates a RAG agent with dynamic context retrieval
5. Provides an interactive CLI interface

## Usage

1. Build and run the project:
```bash
cargo run
```

2. Interact with the system through the CLI:
```
Welcome to the chatbot! Type 'exit' to quit.
> Tell me about the main themes in the documents
```

3. Type 'exit' to quit the chatbot.

## Customization

### Chunk Size
Adjust the chunk size by modifying the `chunk_size` variable in the `load_pdf` function:
```rust
let chunk_size = 2000;
```

### Context Window
Change the number of chunks used for context by modifying the `dynamic_context` parameter:
```rust
.dynamic_context(4, index)
```

### Model Selection
Change the OpenAI model by modifying the model selection:
```rust
.agent("gpt-4") 
```

## Error Handling

The system includes comprehensive error handling:
- PDF loading errors
- OpenAI API errors
- Document processing errors
- Embedding generation errors

Errors are handled using the `anyhow` crate and include context for better debugging.

## Advanced Features

### Document Chunking
The system implements smart document chunking:
- Preserves word boundaries
- Prevents token limit issues
- Enables processing of large documents

### Dynamic Context
The RAG agent:
- Retrieves relevant chunks based on query similarity
- Synthesizes information from multiple chunks
- Maintains context across questions

