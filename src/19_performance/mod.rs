//! 🚀 Performance Optimization in Rust - Web Development Workshop
//!
//! 🎯 บทเรียนเกี่ยวกับการเพิ่มประสิทธิภาพใน Rust สำหรับเวิร์กช็อป
//! 📚 ครอบคลุม memory management, CPU optimization, และ profiling techniques
//! 🌟 เหมาะสำหรับการเรียนรู้ performance tuning ในเวิร์กช็อป

pub mod memory_optimization;
pub mod cpu_optimization;
pub mod profiling_benchmarking;
pub mod zero_copy;
pub mod simd_vectorization;
// TODO: Add these modules when implemented
// pub mod parallel_processing;
// pub mod cache_optimization;
// pub mod allocation_strategies;
// pub mod compile_time_optimization;
// pub mod runtime_optimization;

/// 🎯 รันตัวอย่าง Performance Optimization ทั้งหมด - Web Development Workshop
/// 🌟 สำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn run_performance_examples() {
    println!("🚀 === Performance Optimization Workshop === 🚀");
    println!("🎓 Web Development Workshop - Performance Edition");
    println!();

    // Memory Optimization
    println!("🧠 Memory Optimization Workshop:");
    memory_optimization::demonstrate_memory_optimization();
    
    println!();
    
    // CPU Optimization
    println!("⚡ CPU Optimization Workshop:");
    cpu_optimization::demonstrate_cpu_optimization();
    
    println!();
    
    // Zero-Copy Techniques
    println!("📋 Zero-Copy Techniques Workshop:");
    zero_copy::demonstrate_zero_copy();
    
    println!();
    
    // SIMD Vectorization
    println!("🔢 SIMD Vectorization Workshop:");
    simd_vectorization::demonstrate_simd_vectorization();
    
    println!();
    
    // Profiling and Benchmarking
    println!("📊 Profiling and Benchmarking Workshop:");
    profiling_benchmarking::demonstrate_profiling_benchmarking();
    
    println!();
    
    // TODO: Add these demonstrations when modules are implemented
    // println!("🔄 Parallel Processing:");
    // parallel_processing::demonstrate_parallel();
    // 
    // println!("💾 Cache Optimization:");
    // cache_optimization::demonstrate_cache_optimization();
    // 
    // println!("📦 Allocation Strategies:");
    // allocation_strategies::demonstrate_allocation();
    // 
    // println!("🔧 Compile-time Optimization:");
    // compile_time_optimization::demonstrate_compile_time();
    // 
    // println!("⏱️ Runtime Optimization:");
    // runtime_optimization::demonstrate_runtime();
    
    println!("\n✅ Performance Optimization Workshop สำเร็จแล้ว!");
}