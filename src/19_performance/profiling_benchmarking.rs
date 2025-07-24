//! 📊 Profiling and Benchmarking Implementation - Web Development Workshop
//!
//! 🎯 การใช้งาน Profiling และ Benchmarking ใน Rust สำหรับเวิร์กช็อป
//! 🌟 ครอบคลุม performance measurement, memory profiling, และ benchmark testing
//! 📚 เหมาะสำหรับนักพัฒนาเว็บที่ต้องการวัดประสิทธิภาพ

use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// 📊 Performance profiler for measuring execution time - Workshop Tool
#[derive(Debug, Clone)]
pub struct PerformanceProfiler {
    measurements: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceProfiler {
    #[must_use] pub fn new() -> Self {
        Self {
            measurements: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn measure<F, R>(&self, name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        let mut measurements = self.measurements.lock().unwrap();
        measurements.entry(name.to_string())
            .or_default()
            .push(duration);
        
        result
    }
    
    #[must_use] pub fn get_stats(&self, name: &str) -> Option<ProfileStats> {
        let measurements = self.measurements.lock().unwrap();
        if let Some(durations) = measurements.get(name) {
            if durations.is_empty() {
                return None;
            }
            
            let total: Duration = durations.iter().sum();
            let count = durations.len();
            let average = total / count as u32;
            
            let mut sorted_durations = durations.clone();
            sorted_durations.sort();
            
            let min = sorted_durations[0];
            let max = sorted_durations[count - 1];
            let median = if count % 2 == 0 {
                (sorted_durations[count / 2 - 1] + sorted_durations[count / 2]) / 2
            } else {
                sorted_durations[count / 2]
            };
            
            Some(ProfileStats {
                name: name.to_string(),
                count,
                total,
                average,
                min,
                max,
                median,
            })
        } else {
            None
        }
    }
    
    pub fn print_report(&self) {
        let measurements = self.measurements.lock().unwrap();
        
        println!("\n📊 Performance Report:");
        println!("{:-<80}", "");
        println!("{:<20} {:<8} {:<12} {:<12} {:<12} {:<12} {:<12}", 
                 "Function", "Count", "Total (ms)", "Avg (μs)", "Min (μs)", "Max (μs)", "Median (μs)");
        println!("{:-<80}", "");
        
        for name in measurements.keys() {
            if let Some(stats) = self.get_stats(name) {
                println!("{:<20} {:<8} {:<12.3} {:<12.3} {:<12.3} {:<12.3} {:<12.3}",
                         stats.name,
                         stats.count,
                         stats.total.as_secs_f64() * 1000.0,
                         stats.average.as_secs_f64() * 1_000_000.0,
                         stats.min.as_secs_f64() * 1_000_000.0,
                         stats.max.as_secs_f64() * 1_000_000.0,
                         stats.median.as_secs_f64() * 1_000_000.0);
            }
        }
        
        println!("{:-<80}", "");
    }
    
    pub fn clear(&self) {
        let mut measurements = self.measurements.lock().unwrap();
        measurements.clear();
    }
}

#[derive(Debug, Clone)]
pub struct ProfileStats {
    pub name: String,
    pub count: usize,
    pub total: Duration,
    pub average: Duration,
    pub min: Duration,
    pub max: Duration,
    pub median: Duration,
}

/// 💾 Memory usage tracker - Workshop Memory Profiler
#[derive(Debug, Clone)]
pub struct MemoryProfiler {
    allocations: Arc<Mutex<HashMap<String, MemoryStats>>>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub allocations: usize,
    pub deallocations: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub total_allocated: usize,
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProfiler {
    #[must_use] pub fn new() -> Self {
        Self {
            allocations: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn track_allocation(&self, category: &str, size: usize) {
        let mut allocations = self.allocations.lock().unwrap();
        let stats = allocations.entry(category.to_string()).or_default();
        
        stats.allocations += 1;
        stats.current_usage += size;
        stats.total_allocated += size;
        
        if stats.current_usage > stats.peak_usage {
            stats.peak_usage = stats.current_usage;
        }
    }
    
    pub fn track_deallocation(&self, category: &str, size: usize) {
        let mut allocations = self.allocations.lock().unwrap();
        if let Some(stats) = allocations.get_mut(category) {
            stats.deallocations += 1;
            stats.current_usage = stats.current_usage.saturating_sub(size);
        }
    }
    
    #[must_use] pub fn get_stats(&self, category: &str) -> Option<MemoryStats> {
        let allocations = self.allocations.lock().unwrap();
        allocations.get(category).cloned()
    }
    
    pub fn print_report(&self) {
        let allocations = self.allocations.lock().unwrap();
        
        println!("\n💾 Memory Usage Report:");
        println!("{:-<80}", "");
        println!("{:<15} {:<10} {:<10} {:<12} {:<12} {:<15}", 
                 "Category", "Allocs", "Deallocs", "Current (B)", "Peak (B)", "Total Alloc (B)");
        println!("{:-<80}", "");
        
        for (category, stats) in allocations.iter() {
            println!("{:<15} {:<10} {:<10} {:<12} {:<12} {:<15}",
                     category,
                     stats.allocations,
                     stats.deallocations,
                     stats.current_usage,
                     stats.peak_usage,
                     stats.total_allocated);
        }
        
        println!("{:-<80}", "");
    }
    
    pub fn clear(&self) {
        let mut allocations = self.allocations.lock().unwrap();
        allocations.clear();
    }
}

/// 🏁 Benchmark runner for performance testing - Workshop Benchmark Tool
#[derive(Debug)]
pub struct BenchmarkRunner {
    profiler: PerformanceProfiler,
    memory_profiler: MemoryProfiler,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchmarkRunner {
    #[must_use] pub fn new() -> Self {
        Self {
            profiler: PerformanceProfiler::new(),
            memory_profiler: MemoryProfiler::new(),
        }
    }
    
    pub fn benchmark<F>(&self, name: &str, iterations: usize, f: F) -> BenchmarkResult
    where
        F: Fn() + Send + Sync,
    {
        println!("Running benchmark: {name} ({iterations} iterations)");
        
        // Warm up
        for _ in 0..std::cmp::min(iterations / 10, 100) {
            f();
        }
        
        let start_memory = self.get_memory_usage();
        let start_time = Instant::now();
        
        for i in 0..iterations {
            self.profiler.measure(&format!("{name}_iter_{i}"), || {
                f();
            });
        }
        
        let total_time = start_time.elapsed();
        let end_memory = self.get_memory_usage();
        
        let stats = self.profiler.get_stats(&format!("{name}_iter_0"))
            .unwrap_or_else(|| ProfileStats {
                name: name.to_string(),
                count: 0,
                total: Duration::ZERO,
                average: Duration::ZERO,
                min: Duration::ZERO,
                max: Duration::ZERO,
                median: Duration::ZERO,
            });
        
        BenchmarkResult {
            name: name.to_string(),
            iterations,
            total_time,
            average_time: total_time / iterations as u32,
            throughput: iterations as f64 / total_time.as_secs_f64(),
            memory_delta: end_memory.saturating_sub(start_memory),
            stats,
        }
    }
    
    pub fn compare_benchmarks<F1, F2>(&self, name1: &str, f1: F1, name2: &str, f2: F2, iterations: usize) -> ComparisonResult
    where
        F1: Fn() + Send + Sync,
        F2: Fn() + Send + Sync,
    {
        let result1 = self.benchmark(name1, iterations, f1);
        let result2 = self.benchmark(name2, iterations, f2);
        
        let speedup = result2.average_time.as_secs_f64() / result1.average_time.as_secs_f64();
        let throughput_ratio = result1.throughput / result2.throughput;
        
        ComparisonResult {
            benchmark1: result1,
            benchmark2: result2,
            speedup,
            throughput_ratio,
        }
    }
    
    const fn get_memory_usage(&self) -> usize {
        // Simplified memory usage estimation
        // In a real implementation, you might use system calls or memory allocator hooks
        std::mem::size_of::<Self>()
    }
    
    pub fn profile_concurrent<F>(&self, name: &str, num_threads: usize, work_per_thread: usize, f: F) -> ConcurrentBenchmarkResult
    where
        F: Fn() + Send + Sync + 'static,
    {
        let f = Arc::new(f);
        let profiler = self.profiler.clone();
        
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let f_clone = Arc::clone(&f);
            let profiler_clone = profiler.clone();
            let thread_name = format!("{name}_thread_{thread_id}");
            
            let handle = thread::spawn(move || {
                for i in 0..work_per_thread {
                    profiler_clone.measure(&format!("{thread_name}_work_{i}"), || {
                        f_clone();
                    });
                }
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_time = start_time.elapsed();
        let total_work = num_threads * work_per_thread;
        
        ConcurrentBenchmarkResult {
            name: name.to_string(),
            num_threads,
            work_per_thread,
            total_work,
            total_time,
            throughput: total_work as f64 / total_time.as_secs_f64(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub throughput: f64,
    pub memory_delta: usize,
    pub stats: ProfileStats,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("\n🏁 Benchmark Result: {}", self.name);
        println!("{:-<50}", "");
        println!("Iterations: {}", self.iterations);
        println!("Total time: {:.3} ms", self.total_time.as_secs_f64() * 1000.0);
        println!("Average time: {:.3} μs", self.average_time.as_secs_f64() * 1_000_000.0);
        println!("Throughput: {:.0} ops/sec", self.throughput);
        println!("Memory delta: {} bytes", self.memory_delta);
        println!("{:-<50}", "");
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub benchmark1: BenchmarkResult,
    pub benchmark2: BenchmarkResult,
    pub speedup: f64,
    pub throughput_ratio: f64,
}

impl ComparisonResult {
    pub fn print_comparison(&self) {
        println!("\n⚖️ Benchmark Comparison");
        println!("{:-<60}", "");
        println!("{:<20} vs {:<20}", self.benchmark1.name, self.benchmark2.name);
        println!("{:-<60}", "");
        
        println!("Average time:");
        println!("  {}: {:.3} μs", self.benchmark1.name, self.benchmark1.average_time.as_secs_f64() * 1_000_000.0);
        println!("  {}: {:.3} μs", self.benchmark2.name, self.benchmark2.average_time.as_secs_f64() * 1_000_000.0);
        
        println!("\nThroughput:");
        println!("  {}: {:.0} ops/sec", self.benchmark1.name, self.benchmark1.throughput);
        println!("  {}: {:.0} ops/sec", self.benchmark2.name, self.benchmark2.throughput);
        
        println!("\nComparison:");
        if self.speedup > 1.0 {
            println!("  {} is {:.2}x faster than {}", self.benchmark1.name, self.speedup, self.benchmark2.name);
        } else {
            println!("  {} is {:.2}x faster than {}", self.benchmark2.name, 1.0 / self.speedup, self.benchmark1.name);
        }
        
        println!("{:-<60}", "");
    }
}

#[derive(Debug, Clone)]
pub struct ConcurrentBenchmarkResult {
    pub name: String,
    pub num_threads: usize,
    pub work_per_thread: usize,
    pub total_work: usize,
    pub total_time: Duration,
    pub throughput: f64,
}

impl ConcurrentBenchmarkResult {
    pub fn print_summary(&self) {
        println!("\n🔄 Concurrent Benchmark Result: {}", self.name);
        println!("{:-<50}", "");
        println!("Threads: {}", self.num_threads);
        println!("Work per thread: {}", self.work_per_thread);
        println!("Total work: {}", self.total_work);
        println!("Total time: {:.3} ms", self.total_time.as_secs_f64() * 1000.0);
        println!("Throughput: {:.0} ops/sec", self.throughput);
        println!("{:-<50}", "");
    }
}

/// 🔥 CPU-intensive benchmark functions - Workshop Performance Tests
pub mod cpu_benchmarks {
    // use super::*;
    
    #[must_use] pub fn fibonacci_recursive(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
        }
    }
    
    #[must_use] pub fn fibonacci_iterative(n: u32) -> u64 {
        if n <= 1 {
            return u64::from(n);
        }
        
        let mut a = 0;
        let mut b = 1;
        
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        b
    }
    
    #[must_use] pub fn prime_sieve(limit: usize) -> Vec<usize> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        if limit > 0 {
            is_prime[1] = false;
        }
        
        for i in 2..=((limit as f64).sqrt() as usize) {
            if is_prime[i] {
                for j in ((i * i)..=limit).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        
        (2..=limit).filter(|&i| is_prime[i]).collect()
    }
    
    #[must_use] pub fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows_a = a.len();
        let cols_a = a[0].len();
        let cols_b = b[0].len();
        
        let mut result = vec![vec![0.0; cols_b]; rows_a];
        
        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        
        result
    }
    
    #[must_use] pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        
        for i in 0..n {
            for j in 0..(n - i - 1) {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
        
        arr
    }
    
    #[must_use] pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
        if arr.len() <= 1 {
            return arr;
        }
        
        let pivot_index = partition(&mut arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        
        let mut left_sorted = quick_sort(left.to_vec());
        let mut right_sorted = quick_sort(right[1..].to_vec());
        
        left_sorted.push(right[0]);
        left_sorted.append(&mut right_sorted);
        left_sorted
    }
    
    fn partition(arr: &mut [i32]) -> usize {
        let len = arr.len();
        let pivot_index = len - 1;
        let mut i = 0;
        
        for j in 0..pivot_index {
            if arr[j] <= arr[pivot_index] {
                arr.swap(i, j);
                i += 1;
            }
        }
        
        arr.swap(i, pivot_index);
        i
    }
}

/// 💾 Memory-intensive benchmark functions - Workshop Memory Tests
pub mod memory_benchmarks {
    use super::HashMap;
    
    #[must_use] pub fn allocate_vectors(count: usize, size: usize) -> Vec<Vec<u8>> {
        (0..count).map(|_| vec![0u8; size]).collect()
    }
    
    #[must_use] pub fn string_concatenation(count: usize) -> String {
        let mut result = String::new();
        for i in 0..count {
            result.push_str(&format!("Item {i}"));
        }
        result
    }
    
    #[must_use] pub fn string_builder(count: usize) -> String {
        let mut result = String::with_capacity(count * 10); // Pre-allocate
        for i in 0..count {
            result.push_str(&format!("Item {i}"));
        }
        result
    }
    
    #[must_use] pub fn hashmap_operations(count: usize) -> HashMap<String, i32> {
        let mut map = HashMap::new();
        
        // Insert
        for i in 0..count {
            map.insert(format!("key_{i}"), i as i32);
        }
        
        // Lookup
        for i in 0..count {
            let _ = map.get(&format!("key_{i}"));
        }
        
        // Update
        for i in 0..count {
            if let Some(value) = map.get_mut(&format!("key_{i}")) {
                *value *= 2;
            }
        }
        
        map
    }
    
    #[must_use] pub fn vec_operations(count: usize) -> Vec<i32> {
        let mut vec = Vec::new();
        
        // Push
        for i in 0..count {
            vec.push(i as i32);
        }
        
        // Random access
        for i in 0..count {
            let _ = vec[i % vec.len()];
        }
        
        // Sort
        vec.sort_unstable();
        
        vec
    }
}

/// 🎯 สาธิตการใช้งาน Profiling และ Benchmarking - Web Development Workshop
/// 🌟 สำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn demonstrate_profiling_benchmarking() {
    println!("⚡ Profiling and Benchmarking Workshop Examples:");
    
    let runner = BenchmarkRunner::new();
    
    // CPU-intensive benchmarks
    println!("\n🔥 CPU-Intensive Benchmarks Workshop:");
    println!("{:-<60}", "");
    
    // Fibonacci comparison
    let fib_comparison = runner.compare_benchmarks(
        "fibonacci_iterative",
        || { let _ = cpu_benchmarks::fibonacci_iterative(25); },
        "fibonacci_recursive",
        || { let _ = cpu_benchmarks::fibonacci_recursive(20); },
        10
    );
    
    fib_comparison.print_comparison();
    
    // Sorting comparison
    let test_data: Vec<i32> = (0..500).rev().collect();
    let sorting_comparison = runner.compare_benchmarks(
        "quick_sort",
        || { let _ = cpu_benchmarks::quick_sort(test_data.clone()); },
        "bubble_sort",
        || { let _ = cpu_benchmarks::bubble_sort(test_data.clone()); },
        5
    );
    
    sorting_comparison.print_comparison();
    
    // Prime sieve benchmark
    let prime_result = runner.benchmark(
        "prime_sieve",
        10,
        || { let _ = cpu_benchmarks::prime_sieve(5000); }
    );
    
    prime_result.print_summary();
    
    // Memory-intensive benchmarks
    println!("\n💾 Memory-Intensive Benchmarks Workshop:");
    println!("{:-<60}", "");
    
    // String operations comparison
    let string_comparison = runner.compare_benchmarks(
        "string_builder",
        || { let _ = memory_benchmarks::string_builder(500); },
        "string_concatenation",
        || { let _ = memory_benchmarks::string_concatenation(500); },
        20
    );
    
    string_comparison.print_comparison();
    
    // Collection operations
    let hashmap_result = runner.benchmark(
        "hashmap_operations",
        10,
        || { let _ = memory_benchmarks::hashmap_operations(500); }
    );
    
    hashmap_result.print_summary();
    
    let vec_result = runner.benchmark(
        "vec_operations",
        20,
        || { let _ = memory_benchmarks::vec_operations(500); }
    );
    
    vec_result.print_summary();
    
    // Concurrent benchmarks
    println!("\n🔄 Concurrent Benchmarks Workshop:");
    println!("{:-<60}", "");
    
    let concurrent_result = runner.profile_concurrent(
        "concurrent_fibonacci",
        2,
        10,
        || { let _ = cpu_benchmarks::fibonacci_iterative(20); }
    );
    
    concurrent_result.print_summary();
    
    // Matrix multiplication benchmark
    let matrix_a: Vec<Vec<f64>> = (0..50).map(|i| {
        (0..50).map(|j| f64::from(i * j)).collect()
    }).collect();
    
    let matrix_b: Vec<Vec<f64>> = (0..50).map(|i| {
        (0..50).map(|j| f64::from(i + j)).collect()
    }).collect();
    
    let matrix_result = runner.benchmark(
        "matrix_multiply",
        5,
        || { let _ = cpu_benchmarks::matrix_multiply(&matrix_a, &matrix_b); }
    );
    
    matrix_result.print_summary();
    
    // Performance profiler demonstration
    println!("\n📊 Performance Profiler Workshop Demonstration:");
    println!("{:-<60}", "");
    
    let profiler = PerformanceProfiler::new();
    
    // Profile different operations
    for i in 0..10 {
        profiler.measure("small_fibonacci", || {
            let _ = cpu_benchmarks::fibonacci_iterative(20);
        });
        
        profiler.measure("vector_allocation", || {
            let _vec: Vec<i32> = (0..1000).collect();
        });
        
        profiler.measure("string_operation", || {
            let _s = format!("Iteration {i}");
        });
    }
    
    profiler.print_report();
    
    // Memory profiler demonstration
    println!("\n💾 Memory Profiler Workshop Demonstration:");
    println!("{:-<60}", "");
    
    let memory_profiler = MemoryProfiler::new();
    
    // Simulate memory allocations
    for i in 0..100 {
        memory_profiler.track_allocation("vectors", 1000 * std::mem::size_of::<i32>());
        memory_profiler.track_allocation("strings", 50);
        
        if i % 10 == 0 {
            memory_profiler.track_deallocation("vectors", 1000 * std::mem::size_of::<i32>());
        }
    }
    
    memory_profiler.print_report();
    
    println!("\n✅ Profiling and Benchmarking Workshop สำเร็จแล้ว! 🎉");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::cpu_benchmarks::*;
    use super::memory_benchmarks::*;

    #[test]
    fn test_performance_profiler() {
        let profiler = PerformanceProfiler::new();
        
        // Measure some operations
        profiler.measure("test_operation", || {
            thread::sleep(Duration::from_millis(1));
        });
        
        profiler.measure("test_operation", || {
            thread::sleep(Duration::from_millis(2));
        });
        
        let stats = profiler.get_stats("test_operation").unwrap();
        assert_eq!(stats.count, 2);
        assert!(stats.total >= Duration::from_millis(3));
        assert!(stats.average >= Duration::from_millis(1));
    }

    #[test]
    fn test_memory_profiler() {
        let profiler = MemoryProfiler::new();
        
        profiler.track_allocation("test", 1000);
        profiler.track_allocation("test", 500);
        profiler.track_deallocation("test", 300);
        
        let stats = profiler.get_stats("test").unwrap();
        assert_eq!(stats.allocations, 2);
        assert_eq!(stats.deallocations, 1);
        assert_eq!(stats.current_usage, 1200);
        assert_eq!(stats.peak_usage, 1500);
        assert_eq!(stats.total_allocated, 1500);
    }

    #[test]
    fn test_benchmark_runner() {
        let runner = BenchmarkRunner::new();
        
        let result = runner.benchmark(
            "test_benchmark",
            10,
            || {
                // Simple operation
                let _sum: i32 = (0..100).sum();
            }
        );
        
        assert_eq!(result.name, "test_benchmark");
        assert_eq!(result.iterations, 10);
        assert!(result.total_time > Duration::ZERO);
        assert!(result.throughput > 0.0);
    }

    #[test]
    fn test_fibonacci_algorithms() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(10), 55);
        
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(10), 55);
        
        // Both should produce same results
        for i in 0..=15 {
            assert_eq!(fibonacci_iterative(i), fibonacci_recursive(i));
        }
    }

    #[test]
    fn test_prime_sieve() {
        let primes = prime_sieve(30);
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(primes, expected);
    }

    #[test]
    fn test_matrix_multiply() {
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0]
        ];
        
        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0]
        ];
        
        let result = matrix_multiply(&a, &b);
        let expected = vec![
            vec![19.0, 22.0],
            vec![43.0, 50.0]
        ];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sorting_algorithms() {
        let test_data = vec![64, 34, 25, 12, 22, 11, 90];
        let expected = vec![11, 12, 22, 25, 34, 64, 90];
        
        let bubble_result = bubble_sort(test_data.clone());
        assert_eq!(bubble_result, expected);
        
        let quick_result = quick_sort(test_data);
        assert_eq!(quick_result, expected);
    }

    #[test]
    fn test_memory_benchmarks() {
        let vectors = allocate_vectors(10, 100);
        assert_eq!(vectors.len(), 10);
        assert_eq!(vectors[0].len(), 100);
        
        let concatenated = string_concatenation(5);
        assert!(concatenated.contains("Item 0"));
        assert!(concatenated.contains("Item 4"));
        
        let built = string_builder(5);
        assert_eq!(concatenated, built);
        
        let map = hashmap_operations(10);
        assert_eq!(map.len(), 10);
        assert_eq!(map.get("key_0"), Some(&0)); // Should be doubled from 0 to 0
        assert_eq!(map.get("key_5"), Some(&10)); // Should be doubled from 5 to 10
        
        let vec = vec_operations(100);
        assert_eq!(vec.len(), 100);
        assert!(vec.windows(2).all(|w| w[0] <= w[1])); // Should be sorted
    }

    #[test]
    fn test_concurrent_benchmark() {
        let runner = BenchmarkRunner::new();
        
        let result = runner.profile_concurrent(
            "test_concurrent",
            2,
            5,
            || {
                // Simple work
                let _sum: i32 = (0..100).sum();
            }
        );
        
        assert_eq!(result.name, "test_concurrent");
        assert_eq!(result.num_threads, 2);
        assert_eq!(result.work_per_thread, 5);
        assert_eq!(result.total_work, 10);
        assert!(result.throughput > 0.0);
    }

    #[test]
    fn test_benchmark_comparison() {
        let runner = BenchmarkRunner::new();
        
        let comparison = runner.compare_benchmarks(
            "fast_operation",
            || { let _x = 1 + 1; },
            "slow_operation",
            || { thread::sleep(Duration::from_nanos(100)); },
            10
        );
        
        assert_eq!(comparison.benchmark1.name, "fast_operation");
        assert_eq!(comparison.benchmark2.name, "slow_operation");
        assert!(comparison.speedup > 0.0);
        assert!(comparison.throughput_ratio > 0.0);
    }
}