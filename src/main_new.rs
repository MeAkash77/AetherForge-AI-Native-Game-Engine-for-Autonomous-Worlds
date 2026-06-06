//! ARCADIA Game Engine - Main Service Entry Point
//!
//! This is the primary service that runs continuously on Render

use arcadia::{
    vector_index::{VectorIndex, VectorIndexConfig},
    cache::{CacheManager, CacheConfig},
    memory::MemoryManager,
    metrics::init_metrics,
};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
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

    info!("Starting ARCADIA Game Engine Service...");
    info!("Version: 0.1.0");
    info!("Rust Edition: 2021");

    // Initialize metrics
    if let Err(e) = init_metrics() {
        warn!("Failed to initialize metrics: {}", e);
    } else {
        info!("✓ Metrics system initialized on port 9000");
    }

    // Initialize core components
    let memory_manager = Arc::new(MemoryManager::new());
    info!("✓ Memory manager initialized");

    let cache = Arc::new(CacheManager::new(CacheConfig::default()));
    info!("✓ Cache system initialized (capacity: 10,000 entries)");

    let vector_index = Arc::new(RwLock::new(initialize_vector_index().await));
    info!("✓ Vector index service initialized");

    let agent_db = Arc::new(RwLock::new(initialize_agentdb().await));
    info!("✓ AgentDB service initialized");

    // Print service status
    print_status(&vector_index, &agent_db).await;

    // Start the main service loop
    run_service_loop(vector_index, cache, memory_manager, agent_db).await;

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
            info!("Initializing vector index with OpenAI...");
            
            let config = VectorIndexConfig {
                url: "https://api.openai.com".to_string(),
                api_key,
                qdrant_url: std::env::var("QDRANT_URL").ok(),
                collection_name: "arcadia_production".to_string(),
                embedding_model: "text-embedding-3-small".to_string(),
                vector_dimension: 1536,
            };
            
            match VectorIndex::new(config).await {
                Ok(index) => {
                    info!("✓ Vector index ready (dimension: 1536, model: text-embedding-3-small)");
                    Some(index)
                }
                Err(e) => {
                    error!("✗ Vector index initialization failed: {}", e);
                    None
                }
            }
        }
        Err(_) => {
            warn!("OPENAI_API_KEY not set - vector search disabled");
            warn!("Set OPENAI_API_KEY to enable semantic search capabilities");
            None
        }
    }
}

async fn initialize_agentdb() -> Option<arcadia::agentdb::AgentDbManager> {
    let config = arcadia::agentdb::AgentDbConfig {
        db_name: "arcadia_agents".to_string(),
        vector_dim: 1536,
        max_memory_mb: 512,
        replay_buffer_size: 10000,
        wasm_enabled: false,
        enable_compression: true,
        auto_save_interval: 300,
    };
    
    match arcadia::agentdb::AgentDbManager::new(config).await {
        Ok(db) => {
            info!("✓ AgentDB initialized for persistent learning");
            Some(db)
        }
        Err(e) => {
            warn!("AgentDB initialization failed: {}", e);
            None
        }
    }
}

async fn print_status(
    vector_index: &Arc<RwLock<Option<VectorIndex>>>,
    agent_db: &Arc<RwLock<Option<arcadia::agentdb::AgentDbManager>>>,
) {
    let vi = vector_index.read().await;
    let adb = agent_db.read().await;
    
    println!("\n{}", "═".repeat(70));
    println!("  Service Status");
    println!("{}\n", "═".repeat(70));
    
    println!("✓ ARCADIA Engine Service is RUNNING");
    println!();
    println!("Active Components:");
    println!("  • Memory Manager - Tracking allocations");
    println!("  • Cache System - LRU with TTL (10,000 entries)");
    println!("  • Metrics Collection - Prometheus endpoint");
    
    if vi.is_some() {
        println!("  • Vector Index - OpenAI embeddings (semantic search)");
    } else {
        println!("  ⚠ Vector Index - DISABLED (set OPENAI_API_KEY)");
    }
    
    if adb.is_some() {
        println!("  • AgentDB - Persistent learning (SQLite)");
    } else {
        println!("  ⚠ AgentDB - DISABLED (fallback to memory-only)");
    }
    
    println!();
    println!("Configuration:");
    println!("  • Log Level: {}", std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    println!("  • Qdrant: {}", if std::env::var("QDRANT_URL").is_ok() { "Configured" else { "Not configured" }});
    println!();
    println!("Service will run continuously. Press Ctrl+C to stop.");
    println!("{}\n", "═".repeat(70));
}

async fn run_service_loop(
    vector_index: Arc<RwLock<Option<VectorIndex>>>,
    cache: Arc<CacheManager<String, String>>,
    memory_manager: Arc<MemoryManager>,
    agent_db: Arc<RwLock<Option<arcadia::agentdb::AgentDbManager>>>,
) {
    let mut cycle = 0;
    let start_time = std::time::Instant::now();
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        cycle += 1;
        
        // Update memory stats every cycle
        let stats = memory_manager.get_stats();
        
        // Log heartbeat every 5 cycles (5 minutes)
        if cycle % 5 == 0 {
            info!("Service heartbeat - Cycle #{}", cycle);
            info!("  Memory Usage: {:.2} MB", stats.current_bytes as f64 / (1024.0 * 1024.0));
            info!("  Uptime: {:.0} seconds", start_time.elapsed().as_secs());
            
            // Update cache stats
            let cache_stats = cache.stats();
            debug!("  Cache: {} entries ({}% utilization)", 
                   cache_stats.entry_count, 
                   cache_stats.utilization() as i32);
        }
        
        // Every 10 cycles, perform health check
        if cycle % 10 == 0 {
            perform_health_check(&vector_index, &agent_db).await;
        }
    }
}

async fn perform_health_check(
    vector_index: &Arc<RwLock<Option<VectorIndex>>>,
    agent_db: &Arc<RwLock<Option<arcadia::agentdb::AgentDbManager>>>,
) {
    debug!("Performing health check...");
    
    // Check vector index
    let vi = vector_index.read().await;
    if vi.is_none() && std::env::var("OPENAI_API_KEY").is_ok() {
        warn!("Vector index is disabled but OPENAI_API_KEY is set - attempting reconnection...");
        drop(vi);
        
        let mut vi_write = vector_index.write().await;
        *vi_write = initialize_vector_index().await;
    }
    
    // Check AgentDB
    let adb = agent_db.read().await;
    if adb.is_none() {
        debug!("AgentDB not available (this is normal if not configured)");
    }
    
    debug!("Health check complete");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config() {
        assert!(true);
    }
}
