//! ARCADIA Game Engine - Main Entry Point

use arcadia::{
    vector_index::{VectorIndex, VectorIndexConfig},
    cache::{CacheManager, CacheConfig},
    memory::MemoryManager,
    metrics::init_metrics,
    agentdb::{AgentDbConfig, AgentDbManager, AgentDbStats},
};
use anyhow::Result;
use std::collections::HashMap;
use tracing::{info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string())
        )
        .with_target(false)
        .init();

    print_banner();

    info!("Initializing ARCADIA Game Engine v0.1.0");

    // Initialize metrics
    if let Err(e) = init_metrics() {
        warn!("Failed to initialize metrics: {}", e);
    } else {
        info!("✓ Metrics system initialized");
    }

    // Initialize memory manager
    let memory_manager = MemoryManager::new();
    info!("✓ Memory manager initialized");

    // Initialize cache
    let cache = CacheManager::new(CacheConfig::default());
    info!("✓ Cache system initialized");

    // Initialize vector index
    let vector_index = initialize_vector_index().await;
    
    // Initialize AgentDB
    let agent_db = initialize_agentdb().await;
    
    // Run demonstrations (pass by reference to avoid moving)
    run_demonstrations(&vector_index, &cache, &memory_manager, &agent_db).await?;
    
    // Print final status
    print_status(vector_index.is_some(), agent_db.is_some());
    
    // Run engine loop
    run_engine_loop().await;

    Ok(())
}

fn print_banner() {
    println!(r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║   █████  ██████   ██████  █████   ██████  ██    ██  █████                  ║
║   ██   ██ ██   ██ ██      ██   ██ ██       ██   ██  ██   ██                 ║
║   ███████ ██████  ██      ███████ ██   ███ ██████   ███████                 ║
║   ██   ██ ██   ██ ██      ██   ██ ██    ██ ██   ██  ██   ██                 ║
║   ██   ██ ██   ██  ██████ ██   ██  ██████  ██   ██  ██   ██                 ║
║                                                                              ║
║   ARCADIA: AI-Driven Game Engine Architecture                               ║
║   Version: 0.1.0                                    by @rUv                 ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
    "#);
}

async fn initialize_vector_index() -> Option<VectorIndex> {
    match std::env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            info!("OpenAI API key found, initializing vector index...");
            
            let config = VectorIndexConfig {
                url: "https://api.openai.com".to_string(),
                api_key,
                qdrant_url: std::env::var("QDRANT_URL").ok(),
                collection_name: "arcadia_demo".to_string(),
                embedding_model: "text-embedding-3-small".to_string(),
                vector_dimension: 1536,
            };
            
            match VectorIndex::new(config).await {
                Ok(index) => {
                    info!("✓ Vector index initialized");
                    Some(index)
                }
                Err(e) => {
                    warn!("⚠ Vector index initialization failed: {}", e);
                    None
                }
            }
        }
        Err(_) => {
            info!("ℹ OpenAI API key not set, vector index disabled");
            None
        }
    }
}

async fn initialize_agentdb() -> Option<AgentDbManager> {
    let config = AgentDbConfig {
        db_name: "arcadia_agents".to_string(),
        vector_dim: 1536,
        max_memory_mb: 512,
        replay_buffer_size: 10000,
        wasm_enabled: false,
        enable_compression: true,
        auto_save_interval: 300, // Add this missing field
    };
    
    match AgentDbManager::new(config).await {
        Ok(db) => {
            info!("✓ AgentDB initialized");
            Some(db)
        }
        Err(e) => {
            warn!("⚠ AgentDB initialization failed: {}", e);
            None
        }
    }
}

async fn run_demonstrations(
    vector_index: &Option<VectorIndex>,
    cache: &CacheManager<String, String>,
    memory_manager: &MemoryManager,
    agent_db: &Option<AgentDbManager>,
) -> Result<()> {
    println!("\n{}", "═".repeat(70));
    println!("  ARCADIA Engine Demonstration");
    println!("{}\n", "═".repeat(70));
    
    // Demonstrate caching
    demonstrate_caching(cache).await?;
    
    // Demonstrate memory management
    demonstrate_memory_management(memory_manager).await?;
    
    // Demonstrate vector indexing if available
    if let Some(index) = vector_index {
        demonstrate_vector_indexing(index).await?;
    }
    
    // Demonstrate AgentDB if available
    if let Some(db) = agent_db {
        demonstrate_agentdb(db).await?;
    }
    
    Ok(())
}

async fn demonstrate_caching(cache: &CacheManager<String, String>) -> Result<()> {
    println!("\n┌─ Cache System Demonstration ─────────────────────────────────┐");
    println!("│                                                             │");
    
    // Store items in cache
    let items = vec![
        ("player_health", "100"),
        ("player_mana", "50"),
        ("player_stamina", "75"),
    ];
    
    for (key, value) in &items {
        cache.insert(key.to_string(), value.to_string()).await;
    }
    
    println!("│ Stored {} items in cache", items.len());
    println!("│                                                             │");
    
    // Retrieve items
    println!("│ Retrieving items from cache:");
    for (key, _) in &items {
        if let Some(value) = cache.get(&key.to_string()).await {
            println!("│   ✓ {} = {}", key, value);
        }
    }
    
    let stats = cache.stats();
    println!("│                                                             │");
    println!("│ Cache Statistics:                                          │");
    println!("│   Entry Count: {}", stats.entry_count);
    println!("│   Max Capacity: {}", stats.max_capacity);
    println!("│   Utilization: {:.1}%", stats.utilization());
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_memory_management(memory_manager: &MemoryManager) -> Result<()> {
    println!("\n┌─ Memory Management Demonstration ───────────────────────────┐");
    println!("│                                                             │");
    
    // Simulate memory allocations
    memory_manager.record_allocation(1024 * 1024); // 1 MB
    memory_manager.record_allocation(512 * 1024);  // 512 KB
    memory_manager.record_allocation(2 * 1024 * 1024); // 2 MB
    
    let stats = memory_manager.get_stats();
    println!("│ Memory Statistics:                                         │");
    println!("│   Total Allocated: {:.2} MB", stats.allocated_bytes as f64 / (1024.0 * 1024.0));
    println!("│   Current Usage: {:.2} MB", stats.current_bytes as f64 / (1024.0 * 1024.0));
    println!("│   Peak Usage: {:.2} MB", stats.peak_bytes as f64 / (1024.0 * 1024.0));
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_vector_indexing(index: &VectorIndex) -> Result<()> {
    println!("\n┌─ Vector Indexing Demonstration ─────────────────────────────┐");
    println!("│                                                             │");
    
    // Game concepts to index
    let concepts = vec![
        ("fantasy_warrior", "A brave warrior wielding a magical sword"),
        ("elemental_mage", "A powerful mage controlling fire and ice"),
        ("stealth_assassin", "A silent assassin using shadows"),
    ];
    
    println!("│ Indexing game concepts...");
    println!("│                                                             │");
    
    for (id, description) in &concepts {
        let metadata = HashMap::new();
        match index.store(Some(id.to_string()), description, metadata).await {
            Ok(stored_id) => println!("│   ✓ Stored: {}", stored_id),
            Err(e) => println!("│   ✗ Failed to store {}: {}", id, e),
        }
    }
    
    println!("│                                                             │");
    println!("│ Semantic Search:                                           │");
    println!("│                                                             │");
    
    // Search for concepts
    match index.search("magical warrior", 2).await {
        Ok(results) => {
            for (i, result) in results.iter().enumerate() {
                println!("│   {}. {} (score: {:.4})", i + 1, result.id, result.score);
            }
        }
        Err(e) => println!("│   ✗ Search failed: {}", e),
    }
    
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_agentdb(db: &AgentDbManager) -> Result<()> {
    println!("\n┌─ AgentDB Demonstration ─────────────────────────────────────┐");
    println!("│                                                             │");
    println!("│ AgentDB provides persistent learning for AI agents         │");
    println!("│                                                             │");
    
    // Get database stats (no .await needed if get_stats doesn't return a future)
    let stats = db.get_stats();
    println!("│ Database Statistics:                                       │");
    println!("│   Initialized: {}", stats.initialized);
    println!("│   Total Experiences: {}", stats.total_experiences);
    println!("│   Memory Usage: {:.2} MB", stats.memory_usage_mb);
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

fn print_status(vector_index_enabled: bool, agentdb_enabled: bool) {
    println!("\n{}", "═".repeat(70));
    println!("  Engine Status");
    println!("{}\n", "═".repeat(70));
    
    println!("✓ ARCADIA Engine is operational");
    println!();
    println!("Active Components:");
    println!("  • Cache System (LRU with TTL)");
    println!("  • Memory Manager (allocation tracking)");
    
    if vector_index_enabled {
        println!("  • Vector Index (OpenAI embeddings)");
    }
    
    if agentdb_enabled {
        println!("  • AgentDB (persistent learning)");
    }
    
    if !vector_index_enabled {
        println!();
        println!("ℹ To enable vector search, set OPENAI_API_KEY");
    }
    
    println!();
}

async fn run_engine_loop() {
    println!("{}", "═".repeat(70));
    println!("  ARCADIA Engine is Running");
    println!("{}\n", "═".repeat(70));
    println!("Press Ctrl+C to stop the engine\n");
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
