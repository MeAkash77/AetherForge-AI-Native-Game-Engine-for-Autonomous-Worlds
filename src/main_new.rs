//! ARCADIA Demo - Quick start example

use arcadia::{
    vector_index::{VectorIndex, VectorIndexConfig},
    cache::{CacheManager, CacheConfig},  // Added CacheConfig
    memory::MemoryManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ARCADIA Demo - Quick Start");
    println!("==========================\n");
    
    // Show cache example - Fixed: use new() instead of default()
    let cache = CacheManager::<String, String>::new(CacheConfig::default());
    cache.insert("demo_key".to_string(), "demo_value".to_string()).await;
    
    if let Some(value) = cache.get(&"demo_key".to_string()).await {
        println!("✓ Cache working: {}", value);
    }
    
    // Show memory management
    let memory = MemoryManager::new();
    memory.record_allocation(1024);
    let stats = memory.get_stats();
    println!("✓ Memory tracked: {} bytes allocated", stats.allocated_bytes);
    
    println!("\nARCADIA is ready for game development!");
    println!("Check the documentation at https://docs.rs/arcadia");
    
    Ok(())
}
