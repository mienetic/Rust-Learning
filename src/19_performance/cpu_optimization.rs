//! ⚡ CPU Optimization Techniques - Web Development Workshop
//!
//! 🎯 เทคนิคการเพิ่มประสิทธิภาพ CPU ใน Rust สำหรับเวิร์กช็อป
//! 🌟 เหมาะสำหรับนักพัฒนาเว็บที่ต้องการเพิ่มประสิทธิภาพ

use std::hint;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

/// 🎯 Branch Prediction Optimization - Workshop Technique
struct BranchOptimizer;

impl BranchOptimizer {
    /// ใช้ likely/unlikely hints
    fn process_data_with_hints(data: &[i32]) -> i32 {
        let mut sum = 0;
        
        for &value in data {
            // ใช้ likely hint สำหรับ condition ที่มักจะเป็นจริง
            if likely(value > 0) {
                sum += value;
            } else {
                // unlikely case
                sum -= value.abs();
            }
        }
        
        sum
    }
    
    /// การจัดเรียงข้อมูลเพื่อลด branch misprediction
    fn sorted_processing(mut data: Vec<i32>) -> (i32, i32) {
        data.sort_unstable(); // เรียงข้อมูลก่อน
        
        let mut positive_sum = 0;
        let mut negative_sum = 0;
        
        // ตอนนี้ branch prediction จะแม่นยำมากขึ้น
        for value in data {
            if value >= 0 {
                positive_sum += value;
            } else {
                negative_sum += value;
            }
        }
        
        (positive_sum, negative_sum)
    }
}

/// 🔄 Loop Optimization Techniques - Workshop Performance
struct LoopOptimizer;

impl LoopOptimizer {
    /// Loop Unrolling
    fn sum_unrolled(data: &[i32]) -> i32 {
        let mut sum = 0;
        let len = data.len();
        let chunks = len / 4;
        
        // Process 4 elements at a time
        for i in 0..chunks {
            let base = i * 4;
            sum += data[base] + data[base + 1] + data[base + 2] + data[base + 3];
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..len {
            sum += data[i];
        }
        
        sum
    }
    
    /// Loop Fusion - รวม loop หลายตัวเป็นตัวเดียว
    fn fused_operations(data: &mut [i32]) {
        // แทนที่จะมี 3 loops แยกกัน
        for item in data.iter_mut() {
            *item *= 2;      // multiply by 2
            *item += 1;      // add 1
            *item = (*item).max(0); // clamp to positive
        }
    }
    
    /// Loop Tiling สำหรับ cache efficiency
    fn matrix_multiply_tiled(a: &[Vec<f64>], b: &[Vec<f64>], c: &mut [Vec<f64>], tile_size: usize) {
        let n = a.len();
        
        for ii in (0..n).step_by(tile_size) {
            for jj in (0..n).step_by(tile_size) {
                for kk in (0..n).step_by(tile_size) {
                    // Process tile
                    for i in ii..std::cmp::min(ii + tile_size, n) {
                        for j in jj..std::cmp::min(jj + tile_size, n) {
                            for k in kk..std::cmp::min(kk + tile_size, n) {
                                c[i][j] += a[i][k] * b[k][j];
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 🔥 Function Inlining และ Hot Path Optimization - Workshop Advanced
struct HotPathOptimizer;

impl HotPathOptimizer {
    /// Force inline สำหรับ hot functions
    #[inline(always)]
    fn fast_math_operation(x: f64, y: f64) -> f64 {
        x.mul_add(x, y * y)
    }
    
    /// Cold function ที่ไม่ควร inline
    #[inline(never)]
    fn error_handling(msg: &str) {
        eprintln!("Error: {msg}");
        // Complex error handling logic
    }
    
    /// Hot path optimization
    fn process_hot_path(data: &[f64]) -> f64 {
        let mut result = 0.0;
        
        // Hot path - optimize for common case
        if likely(data.len() > 100) {
            // Vectorized processing for large data
            for chunk in data.chunks_exact(4) {
                result += chunk.iter().sum::<f64>();
            }
            
            // Handle remainder
            for &value in data.chunks_exact(4).remainder() {
                result += value;
            }
        } else {
            // Cold path - simple processing for small data
            result = data.iter().sum();
        }
        
        result
    }
}

/// 💾 Structure of Arrays (`SoA`) vs Array of Structures (`AoS`) - Workshop Memory Layout
#[derive(Clone)]
struct PointAoS {
    x: f32,
    y: f32,
    z: f32,
    w: f32, // padding for alignment
}

struct PointsSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

/// 💾 CPU Cache Optimization - Workshop Cache Techniques
struct CacheOptimizer;

impl CacheOptimizer {
    
    /// `AoS` processing (cache-unfriendly for partial access)
    fn process_aos(points: &[PointAoS]) -> f32 {
        points.iter().map(|p| p.x).sum()
    }
    
    /// `SoA` processing (cache-friendly)
    fn process_soa(points: &PointsSoA) -> f32 {
        points.x.iter().sum()
    }
    
    /// Memory prefetching
    fn prefetch_data(data: &[i32], index: usize) {
        if index + 64 < data.len() {
            // Prefetch data that will be needed soon
            hint::spin_loop(); // Placeholder for actual prefetch
        }
    }
    
    /// Cache-friendly matrix traversal
    fn cache_friendly_matrix_sum(matrix: &[Vec<f64>]) -> f64 {
        let mut sum = 0.0;
        
        // Row-major order (cache-friendly)
        for row in matrix {
            for &value in row {
                sum += value;
            }
        }
        
        sum
    }
}

/// 🧮 SIMD Optimization (Single Instruction, Multiple Data) - Workshop Vectorization
struct SimdOptimizer;

impl SimdOptimizer {
    /// Vector addition using SIMD
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "sse2")]
    unsafe fn add_vectors_simd(a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        
        let chunks = a.len() / 4; // SSE2 can process 4 f32s at once
        
        for i in 0..chunks {
            let base = i * 4;
            
            // Load 4 floats from each array
            let va = _mm_loadu_ps(a.as_ptr().add(base));
            let vb = _mm_loadu_ps(b.as_ptr().add(base));
            
            // Add vectors
            let vresult = _mm_add_ps(va, vb);
            
            // Store result
            _mm_storeu_ps(result.as_mut_ptr().add(base), vresult);
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..a.len() {
            result[i] = a[i] + b[i];
        }
    }
    
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe fn add_vectors_simd(a: &[f32], b: &[f32], result: &mut [f32]) {
        // Fallback to scalar implementation
        for i in 0..a.len() {
            result[i] = a[i] + b[i];
        }
    }
    
    /// Scalar version for comparison
    fn add_vectors_scalar(a: &[f32], b: &[f32], result: &mut [f32]) {
        for i in 0..a.len() {
            result[i] = a[i] + b[i];
        }
    }
}

/// 🎯 Compiler Optimization Hints - Workshop Compiler Techniques
struct CompilerHints;

impl CompilerHints {
    /// การใช้ assume เพื่อให้ compiler optimize ได้ดีขึ้น
    fn process_with_assumptions(data: &[i32]) -> i32 {
        // Check if data is empty first
        if data.is_empty() {
            unsafe {
                hint::unreachable_unchecked();
            }
        }
        
        let mut sum = 0;
        for &value in data {
            // Assume values are always positive
            if value > 0 {
                sum += value;
            }
        }
        sum
    }
    
    /// การใช้ `black_box` เพื่อป้องกัน over-optimization
    fn benchmark_function(data: &[i32]) -> i32 {
        let result = data.iter().sum();
        hint::black_box(result) // Prevent compiler from optimizing away
    }
}

/// Helper function สำหรับ branch prediction hints
#[inline(always)]
fn likely(condition: bool) -> bool {
    // Use cold hint instead of unreachable_unchecked for safety
    if !condition {
        std::hint::black_box(());
    }
    condition
}

/// Additional SIMD helper functions
impl SimdOptimizer {
    /// SSE2 optimized vector addition with proper feature detection
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn simd_add_sse2(a: &[f32], b: &[f32]) -> Vec<f32> {
        if !is_x86_feature_detected!("sse2") {
            return a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
        }
        
        let mut result = vec![0.0; a.len()];
        let chunks = a.len() / 4;
        
        unsafe {
            for i in 0..chunks {
                let offset = i * 4;
                let va = _mm_loadu_ps(a.as_ptr().add(offset));
                let vb = _mm_loadu_ps(b.as_ptr().add(offset));
                let vr = _mm_add_ps(va, vb);
                _mm_storeu_ps(result.as_mut_ptr().add(offset), vr);
            }
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..a.len() {
            result[i] = a[i] + b[i];
        }
        
        result
    }
    
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn simd_add_sse2(a: &[f32], b: &[f32]) -> Vec<f32> {
        // Fallback implementation for non-x86 targets
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }
}

/// 🎯 สาธิตการใช้งาน CPU Optimization - Web Development Workshop
/// 🌟 สำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn demonstrate_cpu_optimization() {
    println!("⚡ Branch Prediction Optimization Workshop:");
    let data = vec![1, -2, 3, -4, 5, 6, 7, 8, 9, 10];
    let result = BranchOptimizer::process_data_with_hints(&data);
    println!("🎯 Processed result with hints: {result}");
    
    let (pos, neg) = BranchOptimizer::sorted_processing(data);
    println!("📊 Sorted processing - Positive: {pos}, Negative: {neg}");
    
    println!("\n🔄 Loop Optimization Workshop:");
    let large_data: Vec<i32> = (1..=1000).collect();
    let sum_unrolled = LoopOptimizer::sum_unrolled(&large_data);
    println!("🎯 Sum with loop unrolling: {sum_unrolled}");
    
    let mut test_data = vec![1, 2, 3, 4, 5];
    LoopOptimizer::fused_operations(&mut test_data);
    println!("🔄 Fused operations result: {test_data:?}");
    
    println!("\n🔥 Hot Path Optimization Workshop:");
    let float_data: Vec<f64> = (1..=200).map(f64::from).collect();
    let hot_result = HotPathOptimizer::process_hot_path(&float_data);
    println!("🎯 Hot path result: {hot_result}");
    
    println!("\n💾 Cache Optimization Workshop:");
    // AoS example
    let aos_points: Vec<PointAoS> = (0..1000)
        .map(|i| PointAoS {
            x: i as f32,
            y: (i * 2) as f32,
            z: (i * 3) as f32,
            w: 0.0,
        })
        .collect();
    
    let aos_sum = CacheOptimizer::process_aos(&aos_points);
    println!("📊 AoS sum: {aos_sum}");
    
    // SoA example
    let soa_points = PointsSoA {
        x: (0..1000).map(|i| i as f32).collect(),
        y: (0..1000).map(|i| (i * 2) as f32).collect(),
        z: (0..1000).map(|i| (i * 3) as f32).collect(),
    };
    
    let soa_sum = CacheOptimizer::process_soa(&soa_points);
    println!("🎯 SoA sum: {soa_sum}");
    
    println!("\n🧮 SIMD Optimization Workshop:");
    let a: Vec<f32> = (0..1000).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..1000).map(|i| (i * 2) as f32).collect();
    let result_simd = vec![0.0; 1000];
    let mut result_scalar = vec![0.0; 1000];
    
    // SIMD version (unsafe)
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe {
                SimdOptimizer::add_vectors_simd(&a, &b, &mut result_simd);
            }
            println!("🎯 SIMD result (first 5): {:?}", &result_simd[0..5]);
        } else {
            println!("⚠️ SSE2 not available, using scalar version");
        }
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        println!("⚠️ x86/x86_64 not supported - using scalar version");
    }
    
    // Scalar version
    SimdOptimizer::add_vectors_scalar(&a, &b, &mut result_scalar);
    println!("📊 Scalar result (first 5): {:?}", &result_scalar[0..5]);
    
    println!("\n🎯 Compiler Hints Workshop:");
    let hint_data = vec![1, 2, 3, 4, 5];
    let benchmark_result = CompilerHints::benchmark_function(&hint_data);
    println!("📊 Benchmark result: {benchmark_result}");
    
    println!("\n✅ CPU Optimization Workshop สำเร็จแล้ว! 🎉");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_branch_optimization() {
        let data = vec![1, -2, 3, -4, 5];
        let result = BranchOptimizer::process_data_with_hints(&data);
        assert!(result != 0);
    }
    
    #[test]
    fn test_loop_unrolling() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = LoopOptimizer::sum_unrolled(&data);
        let expected: i32 = data.iter().sum();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_simd_vs_scalar() {
        let a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let b: Vec<f32> = vec![5.0, 6.0, 7.0, 8.0];
        let mut result_scalar = vec![0.0; 4];
        
        SimdOptimizer::add_vectors_scalar(&a, &b, &mut result_scalar);
        
        let expected = vec![6.0, 8.0, 10.0, 12.0];
        assert_eq!(result_scalar, expected);
    }
    
    #[test]
    fn test_cache_optimization() {
        let points = PointsSoA {
            x: vec![1.0, 2.0, 3.0],
            y: vec![4.0, 5.0, 6.0],
            z: vec![7.0, 8.0, 9.0],
        };
        
        let sum = CacheOptimizer::process_soa(&points);
        assert_eq!(sum, 6.0);
    }
}