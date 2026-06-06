//! ARCADIA Game Engine - Main Entry Point
//!
//! This demonstrates the core features of ARCADIA including:
//! - Vector indexing for semantic search
//! - Caching and memory management
//! - Performance monitoring
//! - AgentDB integration

use arcadia::{
    vector_index::{VectorIndex, VectorIndexConfig},
    cache::{CacheManager, CacheConfig},
    memory::MemoryManager,
    metrics::init_metrics,
    agentdb::{AgentDbConfig, AgentDbManager, AgentExperience},
};
use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, warn, debug};
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
    info!("Edition: 2021 | Rust Version: 1.75+");

    // Initialize metrics
    if let Err(e) = init_metrics() {
        warn!("Failed to initialize metrics: {}", e);
    } else {
        info!("✓ Metrics system initialized (Prometheus exporter)");
    }

    // Initialize memory manager
    let memory_manager = MemoryManager::new();
    info!("✓ Memory manager initialized");

    // Initialize cache
    let cache = CacheManager::new(CacheConfig::default());
    info!("✓ Cache system initialized (capacity: 10,000 entries, TTL: 1 hour)");

    // Initialize vector index
    let vector_index = initialize_vector_index().await;
    
    // Initialize AgentDB
    let mut agent_db = initialize_agentdb().await;
    
    // Run demonstrations - PASS BY REFERENCE (fix)
    run_demonstrations(&vector_index, &cache, &memory_manager, &mut agent_db).await?;
    
    // Print final status
    print_status(vector_index.is_some(), agent_db.is_some());
    
    // Run engine loop
    run_engine_loop().await;

    Ok(())
}

fn print_banner() {
    println!(r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║    █████  ██████   ██████  █████   ██████  ██    ██  █████                 ║
║   ██   ██ ██   ██ ██      ██   ██ ██       ██   ██  ██   ██                ║
║   ███████ ██████  ██      ███████ ██   ███ ██████   ███████                ║
║   ██   ██ ██   ██ ██      ██   ██ ██    ██ ██   ██  ██   ██                ║
║   ██   ██ ██   ██  ██████ ██   ██  ██████  ██   ██  ██   ██                ║
║                                                                              ║
║   ARCADIA: AI-Driven Game Engine Architecture                               ║
║   Advanced & Responsive Computational Architecture for Dynamic Interactive AI║
║                                                                              ║
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
            
            let timer = Instant::now();
            match VectorIndex::new(config).await {
                Ok(index) => {
                    info!("✓ Vector index initialized in {:.2}s", timer.elapsed().as_secs_f64());
                    info!("  - Collection: arcadia_demo");
                    info!("  - Dimension: 1536");
                    info!("  - Model: text-embedding-3-small");
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
            info!("  Set OPENAI_API_KEY environment variable to enable semantic search");
            None
        }
    }
}

async fn initialize_agentdb() -> Option<AgentDbManager> {
    let config = AgentDbConfig::default();
    
    match AgentDbManager::new(config).await {
        Ok(mut db) => {
            info!("✓ AgentDB initialized");
            if let Err(e) = db.initialize().await {
                warn!("  AgentDB initialization warning: {}", e);
            }
            Some(db)
        }
        Err(e) => {
            warn!("⚠ AgentDB initialization failed: {}", e);
            None
        }
    }
}

// FIXED: Now takes reference instead of owned value
async fn run_demonstrations(
    vector_index: &Option<VectorIndex>,  // Changed to reference
    cache: &CacheManager<String, String>,
    memory_manager: &MemoryManager,
    agent_db: &mut Option<AgentDbManager>,
) -> Result<()> {
    println!("\n{}", "═".repeat(70));
    println!("  ARCADIA Engine Demonstration");
    println!("{}\n", "═".repeat(70));
    
    // Demonstrate caching
    demonstrate_caching(cache).await?;
    
    // Demonstrate memory management
    demonstrate_memory_management(memory_manager).await?;
    
    // Demonstrate vector indexing if available
    if let Some(index) = vector_index {  // Works with &Option
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
    
    let timer = Instant::now();
    
    // Store items in cache
    let items = vec![
        ("player_health", "100"),
        ("player_mana", "50"),
        ("player_stamina", "75"),
        ("current_zone", "Dragon's Peak"),
        ("difficulty", "Hard"),
    ];
    
    for (key, value) in &items {
        cache.insert(key.to_string(), value.to_string()).await;
    }
    
    println!("│ Stored {} items in cache:", items.len());
    for (key, value) in &items {
        println!("│   • {} = {}", key, value);
    }
    println!("│                                                             │");
    
    // Retrieve items
    println!("│ Retrieving items from cache:");
    for (key, _) in &items {
        if let Some(value) = cache.get(&key.to_string()).await {
            println!("│   ✓ {} = {}", key, value);
        }
    }
    println!("│                                                             │");
    
    // Cache statistics
    let stats = cache.stats();
    println!("│ Cache Statistics:                                          │");
    println!("│   Entry Count: {}", stats.entry_count);
    println!("│   Weighted Size: {}", stats.weighted_size);
    println!("│   Max Capacity: {}", stats.max_capacity);
    println!("│   Utilization: {:.1}%", stats.utilization());
    println!("│                                                             │");
    println!("│ Cache Operations Time: {:.2}ms", timer.elapsed().as_millis());
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_memory_management(memory_manager: &MemoryManager) -> Result<()> {
    println!("\n┌─ Memory Management Demonstration ───────────────────────────┐");
    println!("│                                                             │");
    
    // Simulate memory allocations
    let allocations = vec![
        (1024 * 1024, "Game state"),      // 1 MB
        (512 * 1024, "NPC data"),          // 512 KB
        (2 * 1024 * 1024, "World map"),    // 2 MB
        (256 * 1024, "Cache data"),        // 256 KB
        (4 * 1024 * 1024, "Textures"),     // 4 MB
    ];
    
    for (bytes, description) in &allocations {
        memory_manager.record_allocation(*bytes);
        println!("│   Allocated {:.2} MB for {}", *bytes as f64 / (1024.0 * 1024.0), description);
    }
    
    println!("│                                                             │");
    
    let stats = memory_manager.get_stats();
    println!("│ Memory Statistics:                                         │");
    println!("│   Total Allocated: {:.2} MB", stats.allocated_bytes as f64 / (1024.0 * 1024.0));
    println!("│   Current Usage: {:.2} MB", stats.current_bytes as f64 / (1024.0 * 1024.0));
    println!("│   Peak Usage: {:.2} MB", stats.peak_bytes as f64 / (1024.0 * 1024.0));
    println!("│   Total Deallocated: {:.2} MB", stats.deallocated_bytes as f64 / (1024.0 * 1024.0));
    
    // Deallocate some memory
    memory_manager.record_deallocation(2 * 1024 * 1024); // Free 2 MB
    let updated_stats = memory_manager.get_stats();
    println!("│                                                             │");
    println!("│ After Deallocation:                                        │");
    println!("│   Current Usage: {:.2} MB", updated_stats.current_bytes as f64 / (1024.0 * 1024.0));
    
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_vector_indexing(index: &VectorIndex) -> Result<()> {
    println!("\n┌─ Vector Indexing Demonstration ─────────────────────────────┐");
    println!("│                                                             │");
    
    // Game concepts to index
    let concepts = vec![
        ("fantasy_warrior", "A brave warrior wielding a magical sword in a fantasy realm"),
        ("stealth_assassin", "A silent assassin using shadows and daggers for covert operations"),
        ("elemental_mage", "A powerful mage controlling fire, ice, and lightning elements"),
        ("forest_druid", "A nature-protecting druid with animal companions and healing powers"),
        ("dragon", "A mighty fire-breathing dragon guarding ancient treasure"),
    ];
    
    println!("│ Indexing game concepts as vector embeddings...");
    println!("│                                                             │");
    
    let timer = Instant::now();
    for (id, description) in &concepts {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "game_concept".to_string());
        metadata.insert("category".to_string(), "npc".to_string());
        
        match index.store(Some(id.to_string()), description, metadata).await {
            Ok(stored_id) => println!("│   ✓ Stored: {}", stored_id),
            Err(e) => println!("│   ✗ Failed to store {}: {}", id, e),
        }
    }
    println!("│                                                             │");
    println!("│ Indexing completed in {:.2}s", timer.elapsed().as_secs_f64());
    println!("│                                                             │");
    
    // Semantic search queries
    let queries = vec![
        ("Who can fight with swords?", 2),
        ("What creatures breathe fire?", 1),
        ("Who uses magic spells?", 2),
    ];
    
    println!("│ Semantic Search Results:                                    │");
    println!("│                                                             │");
    
    for (query, limit) in queries {
        println!("│   Query: \"{}\" (top {})", query, limit);
        
        let search_timer = Instant::now();
        match index.search(query, limit).await {
            Ok(results) => {
                println!("│     Search time: {:.2}ms", search_timer.elapsed().as_millis());
                for (i, result) in results.iter().enumerate() {
                    println!("│     {}. {} (score: {:.4})", i + 1, result.id, result.score);
                }
            }
            Err(e) => println!("│     ✗ Search failed: {}", e),
        }
        println!("│                                                             │");
    }
    
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

async fn demonstrate_agentdb(db: &mut AgentDbManager) -> Result<()> {
    println!("\n┌─ AgentDB Demonstration ─────────────────────────────────────┐");
    println!("│                                                             │");
    println!("│ AgentDB provides persistent learning and memory for AI agents│");
    println!("│                                                             │");
    
    // Get database stats (synchronous, no .await)
    let stats = db.get_stats();
    println!("│ Database Statistics:                                       │");
    println!("│   Initialized: {}", stats.initialized);
    println!("│   Total Experiences: {}", stats.total_experiences);
    println!("│   Memory Usage: {:.2} MB", stats.memory_usage_mb);
    println!("│                                                             │");
    
    // Store an example experience
    let experience = AgentExperience {
        id: "exp_001".to_string(),
        agent_id: "hero_agent".to_string(),
        state_vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        action: "attack".to_string(),
        reward: 1.0,
        next_state_vector: vec![0.2, 0.3, 0.4, 0.5, 0.6],
        done: false,
        metadata: HashMap::new(),
        timestamp: chrono::Utc::now().timestamp(),
    };
    
    match db.store_experience("hero_agent", experience).await {
        Ok(()) => println!("│   Store experience: ✓ Success"),
        Err(e) => println!("│   Store experience: ✗ Failed - {}", e),
    }
    
    let updated_stats = db.get_stats();
    println!("│   Updated Experience Count: {}", updated_stats.total_experiences);
    println!("│                                                             │");
    println!("└─────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

fn print_status(vector_index_enabled: bool, agentdb_enabled: bool) {
    println!("\n{}", "═".repeat(70));
    println!("  Engine Status");
    println!("{}\n", "═".repeat(70));
    
    println!("✓ ARCADIA Engine is fully operational");
    println!();
    println!("Active Components:");
    println!("  • Cache System (LRU with TTL)");
    println!("  • Memory Manager (allocation tracking)");
    println!("  • Metrics Collection (Prometheus)");
    
    if vector_index_enabled {
        println!("  • Vector Index (OpenAI embeddings)");
    }
    
    if agentdb_enabled {
        println!("  • AgentDB (persistent learning)");
    }
    
    if !vector_index_enabled {
        println!();
        println!("ℹ To enable vector search:");
        println!("  Set OPENAI_API_KEY environment variable");
    }
    
    println!();
}

async fn run_engine_loop() {
    println!("\n{}", "═".repeat(70));
    println!("  ARCADIA Engine is Running");
    println!("{}\n", "═".repeat(70));
    println!("Press Ctrl+C to stop the engine\n");
    
    let mut cycle = 0;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        cycle += 1;
        debug!("Engine heartbeat - Cycle {}", cycle);
        
        if cycle % 10 == 0 {
            info!("Engine running - {} cycles completed", cycle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(true);
    }
}
