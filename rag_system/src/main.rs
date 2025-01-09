// Import error handling utilities from the anyhow crate
use anyhow::{Context, Result};
// Import various components from the rig crate (used for AI/ML operations)
use rig::{
    embeddings::EmbeddingsBuilder,        // For creating text embeddings
    loaders::PdfFileLoader,              // For loading PDF files
    providers::openai::{self, TEXT_EMBEDDING_ADA_002}, // For OpenAI client
    vector_store::in_memory_store::InMemoryVectorStore, // For in-memory vector store
    Embed,
};
use serde::{Deserialize, Serialize};        // For serialization and deserialization
use std::path::PathBuf;                     // For handling file paths
use dotenv::dotenv;

#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]        // Define a struct for documents with embedding capabilities
struct Document {
    id: String,
    #[embed]
    content: String,
}

fn load_pdf(path: PathBuf) -> Result<Vec<String>> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let chunk_size = 2000; // Approximately 2000 characters per chunk
    
    for entry in PdfFileLoader::with_glob(path.to_str().unwrap())?.read() {
        let content = entry?;
        
        // Split content into words
        let words: Vec<&str> = content.split_whitespace().collect();
        
        for word in words {
            if current_chunk.len() + word.len() + 1 > chunk_size {
                // If adding the next word would exceed chunk size,
                // save current chunk and start a new one
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk.clear();
                }
            }
            current_chunk.push_str(word);
            current_chunk.push(' ');
        }
    }
    
    // last chunk
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    if chunks.is_empty() {
        anyhow::bail!("No content found in PDF file: {:?}", path);
    }
    
    Ok(chunks)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load environment variables from .env file
    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();
    
    // Load PDFs using Rig's built-in PDF loader
    let documents_dir = std::env::current_dir()?.join("documents");
    
    let moores_law_chunks = load_pdf(documents_dir.join("01.pdf"))
        .context("Failed to load 01.pdf")?;
    let last_question_chunks = load_pdf(documents_dir.join("02.pdf"))
        .context("Failed to load 02.pdf")?;

    println!("Successfully loaded and chunked PDF documents");

    // Create embedding model
    let model = openai_client.embedding_model(TEXT_EMBEDDING_ADA_002);

    // Create embeddings builder
    let mut builder = EmbeddingsBuilder::new(model.clone());

    // Add chunks from Moore's Law
    for (i, chunk) in moores_law_chunks.into_iter().enumerate() {
        builder = builder.document(Document {
            id: format!("moores_law_{}", i),
            content: chunk,
        })?;
    }

    // Add chunks from The Last Question
    for (i, chunk) in last_question_chunks.into_iter().enumerate() {
        builder = builder.document(Document {
            id: format!("last_question_{}", i),
            content: chunk,
        })?;
    }

    // Build embeddings
    let embeddings = builder.build().await?;

    println!("Successfully generated embeddings");

    // Create vector store and index
    let vector_store = InMemoryVectorStore::from_documents(embeddings);
    let index = vector_store.index(model);

    println!("Successfully created vector store and index");

    // Create RAG agent
    let rag_agent = openai_client
        .agent("gpt-4")
        .preamble("You are a helpful assistant that answers questions based on the provided document context. When answering questions, try to synthesize information from multiple chunks if they're related.")
        .dynamic_context(4, index) // Increased to 4 since we have chunks now
        .build();

    println!("Starting CLI chatbot...");

    // Start interactive CLI
    rig::cli_chatbot::cli_chatbot(rag_agent).await?;

    Ok(())
}
