//! ARCADIA Demo - Quick start example

use arcadia::{
    cache::CacheManager,
    memory::MemoryManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ARCADIA Demo - Quick Start");
    println!("==========================\n");
    
    // Show cache example - use new() instead of default()
    let cache_config = arcadia::cache::CacheConfig::default();
    let cache = CacheManager::<String, String>::new(cache_config);
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
