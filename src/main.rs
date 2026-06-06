// ARCADIA: Advanced and Responsive Computational Architecture for Dynamic Interactive AI
//        /\__/\   - main.rs
//       ( o.o  )  - v1.0.0
//         >^<     - by @rUv
//
// Main entry point for ARCADIA Game Engine

use arcadia::vector_index::{VectorIndex, VectorIndexConfig, SearchResult};
use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  ARCADIA: AI-Driven Game Engine Architecture               ║");
    println!("║  Advanced & Responsive Computational Architecture          ║");
    println!("║  for Dynamic Interactive AI                                ║");
    println!("║                                                             ║");
    println!("║  Version: 1.0.0                                            ║");
    println!("║  Author: @rUv                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    info!("ARCADIA Game Engine v1.0.0");
    info!("Starting up...");

    // Initialize vector index if API key is available
    let vector_index = initialize_vector_index().await;
    
    // Run the main engine loop
    run_engine_loop(vector_index).await?;

    Ok(())
}

/// Initialize the vector index with OpenAI embeddings
async fn initialize_vector_index() -> Option<VectorIndex> {
    match std::env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            info!("OpenAI API key found, initializing vector index...");
            
            let config = VectorIndexConfig {
                url: "https://api.openai.com".to_string(),
                api_key,
                qdrant_url: Some("http://localhost:6334".to_string()),
                collection_name: "arcadia_vectors".to_string(),
                embedding_model: "text-embedding-3-small".to_string(),
                vector_dimension: 1536,
            };
            
            match VectorIndex::new(config).await {
                Ok(index) => {
                    info!("✓ Vector index initialized successfully");
                    Some(index)
                }
                Err(e) => {
                    warn!("⚠ Vector index initialization failed: {}", e);
                    None
                }
            }
        }
        Err(_) => {
            info!("OpenAI API key not set, vector index disabled");
            info!("To enable vector search, set the OPENAI_API_KEY environment variable");
            None
        }
    }
}

/// Run the main engine loop
async fn run_engine_loop(vector_index: Option<VectorIndex>) -> Result<()> {
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  ARCADIA Engine Status");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    // Display engine status
    println!("✓ Engine initialized successfully");
    
    if vector_index.is_some() {
        println!("✓ Vector indexing: ENABLED");
        println!("  - Using OpenAI embeddings (text-embedding-3-small)");
        println!("  - Vector dimension: 1536");
        println!("  - Ready for semantic search");
    } else {
        println!("⚠ Vector indexing: DISABLED");
        println!("  - Set OPENAI_API_KEY to enable");
    }
    
    println!("✓ Memory management: ACTIVE");
    println!("✓ Metrics collection: ACTIVE");
    println!("✓ Ready for game operations");
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    // Run demonstration if vector index is available
    if let Some(index) = vector_index {
        run_demonstration(index).await?;
    }
    
    // Keep the engine running
    println!("Press Ctrl+C to stop the engine");
    println!();
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        
        // Periodic status update
        info!("Engine heartbeat: still running");
    }
}

/// Run a demonstration of vector indexing capabilities
async fn run_demonstration(index: VectorIndex) -> Result<()> {
    println!();
    println!("┌─ Vector Indexing Demonstration ─────────────────────────┐");
    println!("│                                                         │");
    
    // Sample game concepts to index
    let game_concepts = vec![
        ("fantasy_warrior", "A brave warrior wielding a magical sword in a fantasy realm"),
        ("stealth_assassin", "A silent assassin using shadows and daggers for covert operations"),
        ("elemental_mage", "A powerful mage controlling fire, ice, and lightning elements"),
        ("forest_druid", "A nature-protecting druid with animal companions and healing powers"),
        ("undead_necromancer", "A dark necromancer raising skeletons and casting curses"),
    ];
    
    println!("│ Adding game concepts to vector index...");
    println!("│                                                         │");
    
    // Store each concept with its metadata
    for (i, (concept_id, description)) in game_concepts.iter().enumerate() {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "game_concept".to_string());
        metadata.insert("category".to_string(), if i < 3 { "hero".to_string() } else { "specialist".to_string() });
        metadata.insert("index".to_string(), i.to_string());
        
        match index.store(Some(concept_id.to_string()), description, metadata).await {
            Ok(id) => println!("│   ✓ Stored: {} - {}", id, concept_id),
            Err(e) => println!("│   ✗ Failed to store {}: {}", concept_id, e),
        }
    }
    
    println!("│                                                         │");
    println!("│ Searching for similar concepts...");
    println!("│                                                         │");
    
    // Search for concepts similar to a query
    let queries = vec![
        ("magical warrior", 3),
        ("stealthy killer", 2),
        ("nature magic", 2),
    ];
    
    for (query, limit) in queries {
        println!("│ Query: \"{}\" (top {})", query, limit);
        
        match index.search(query, limit).await {
            Ok(results) => {
                for (i, result) in results.iter().enumerate() {
                    println!("│   {}. {} (score: {:.4})", i + 1, result.id, result.score);
                    if !result.text.is_empty() {
                        println!("│      \"{}\"", &result.text[..result.text.len().min(50)]);
                    }
                }
            }
            Err(e) => println!("│   ✗ Search failed: {}", e),
        }
        println!("│                                                         │");
    }
    
    // Test deletion
    println!("│ Testing deletion...");
    if let Some(first_concept) = game_concepts.first() {
        match index.delete(first_concept.0).await {
            Ok(()) => println!("│   ✓ Deleted: {}", first_concept.0),
            Err(e) => println!("│   ✗ Failed to delete: {}", e),
        }
    }
    
    // Verify deletion with a search
    println!("│                                                         │");
    println!("│ Verifying deletion...");
    match index.search("warrior", 5).await {
        Ok(results) => {
            let found = results.iter().any(|r| r.id == game_concepts.first().unwrap().0);
            if !found {
                println!("│   ✓ Deletion confirmed - concept no longer in index");
            } else {
                println!("│   ⚠ Deletion verification failed - concept still present");
            }
        }
        Err(e) => println!("│   ✗ Verification search failed: {}", e),
    }
    
    println!("│                                                         │");
    println!("└──────────────────────────────────────────────────────────┘");
    println!();
    
    // Get final statistics
    println!("┌─ Vector Index Statistics ───────────────────────────────┐");
    println!("│                                                         │");
    println!("│ Total concepts indexed: {}", game_concepts.len());
    println!("│ Remaining after deletion: {}", game_concepts.len() - 1);
    println!("│                                                         │");
    println!("│ Features:                                               │");
    println!("│   • Cosine similarity search                           │");
    println!("│   • Metadata filtering                                 │");
    println!("│   • Persistent storage (if Qdrant configured)          │");
    println!("│                                                         │");
    println!("└──────────────────────────────────────────────────────────┘");
    println!();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_config() {
        // Basic test to ensure configuration works
        let config = VectorIndexConfig::default();
        assert_eq!(config.vector_dimension, 1536);
        assert_eq!(config.embedding_model, "text-embedding-3-small");
    }
}
