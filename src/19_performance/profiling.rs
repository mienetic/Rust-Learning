//! 🚀 Performance Profiling Workshop - เครื่องมือวัดประสิทธิภาพ! 📊
//!
//! ยินดีต้อนรับสู่ Workshop การเรียนรู้ Profiling และ Benchmarking! 🎉
//! เหมือนการเป็นนักสืบสวนประสิทธิภาพ - ตามหาจุดที่ช้าและปรับปรุง! 🔍
//!
//! 🎯 สิ่งที่จะได้เรียนรู้:
//! - 📊 Performance Profiling
//! - 💾 Memory Tracking
//! - ⏱️ Benchmarking Techniques
//! - 📈 Performance Analysis

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// 📊 Simple Profiler - เครื่องมือวัดเวลาการทำงาน!
/// เหมือนนาฬิกาจับเวลาสำหรับโค้ด! ⏱️
struct SimpleProfiler {
    measurements: HashMap<String, Vec<Duration>>,
    start_times: HashMap<String, Instant>,
}

impl SimpleProfiler {
    fn new() -> Self {
        Self {
            measurements: HashMap::new(),
            start_times: HashMap::new(),
        }
    }
    
    fn start(&mut self, name: &str) {
        self.start_times.insert(name.to_string(), Instant::now());
    }
    
    fn end(&mut self, name: &str) -> Option<Duration> {
        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            self.measurements.entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
            Some(duration)
        } else {
            None
        }
    }
    
    fn get_stats(&self, name: &str) -> Option<ProfileStats> {
        self.measurements.get(name).map(|durations| {
            let total: Duration = durations.iter().sum();
            let count = durations.len();
            let avg = total / count as u32;
            
            let mut sorted = durations.clone();
            sorted.sort();
            
            let min = *sorted.first().unwrap();
            let max = *sorted.last().unwrap();
            let median = if count % 2 == 0 {
                (sorted[count / 2 - 1] + sorted[count / 2]) / 2
            } else {
                sorted[count / 2]
            };
            
            ProfileStats {
                name: name.to_string(),
                count,
                total,
                avg,
                min,
                max,
                median,
            }
        })
    }
    
    fn print_report(&self) {
        println!("\n📊 Performance Report:");
        println!("{:-<80}", "");
        println!("{:<20} {:<8} {:<12} {:<12} {:<12} {:<12}", 
                "Function", "Count", "Total(ms)", "Avg(μs)", "Min(μs)", "Max(μs)");
        println!("{:-<80}", "");
        
        for name in self.measurements.keys() {
            if let Some(stats) = self.get_stats(name) {
                println!("{:<20} {:<8} {:<12.2} {:<12.2} {:<12.2} {:<12.2}",
                    stats.name,
                    stats.count,
                    stats.total.as_secs_f64() * 1000.0,
                    stats.avg.as_secs_f64() * 1_000_000.0,
                    stats.min.as_secs_f64() * 1_000_000.0,
                    stats.max.as_secs_f64() * 1_000_000.0
                );
            }
        }
        println!("{:-<80}", "");
    }
}

/// 📈 Profile Statistics - สถิติการทำงาน!
/// เหมือนรายงานผลการแข่งขัน! 🏆
#[derive(Debug, Clone)]
struct ProfileStats {
    name: String,
    count: usize,
    total: Duration,
    avg: Duration,
    min: Duration,
    max: Duration,
    median: Duration,
}

/// 🎯 Macro สำหรับ Profiling ง่ายๆ!
/// เหมือนการใส่เซ็นเซอร์วัดความเร็วในโค้ด! 📡
macro_rules! profile {
    ($profiler:expr, $name:expr, $code:block) => {
        $profiler.start($name);
        let result = $code;
        $profiler.end($name);
        result
    };
}

/// 💾 Memory Tracker - ตัวติดตามการใช้หน่วยความจำ!
/// เหมือนการดูว่าใครใช้พื้นที่ในบ้านมากที่สุด! 🏠
struct MemoryTracker {
    allocations: HashMap<String, usize>,
    peak_usage: usize,
    current_usage: usize,
}

impl MemoryTracker {
    fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            peak_usage: 0,
            current_usage: 0,
        }
    }
    
    fn allocate(&mut self, name: &str, size: usize) {
        self.allocations.insert(name.to_string(), size);
        self.current_usage += size;
        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
    }
    
    fn deallocate(&mut self, name: &str) {
        if let Some(size) = self.allocations.remove(name) {
            self.current_usage = self.current_usage.saturating_sub(size);
        }
    }
    
    fn get_current_usage(&self) -> usize {
        self.current_usage
    }
    
    fn get_peak_usage(&self) -> usize {
        self.peak_usage
    }
    
    fn print_report(&self) {
        println!("\n💾 Memory Usage Report:");
        println!("Current Usage: {} bytes", self.current_usage);
        println!("Peak Usage: {} bytes", self.peak_usage);
        println!("Active Allocations: {}", self.allocations.len());
        
        if !self.allocations.is_empty() {
            println!("\nActive allocations:");
            for (name, size) in &self.allocations {
                println!("  {}: {} bytes", name, size);
            }
        }
    }
}

/// 🏃‍♂️ Benchmark Runner - ตัวจัดการการทดสอบประสิทธิภาพ!
/// เหมือนโค้ชที่จัดการการแข่งขันความเร็ว! 🏁
struct BenchmarkRunner {
    profiler: SimpleProfiler,
    memory_tracker: MemoryTracker,
}

impl BenchmarkRunner {
    fn new() -> Self {
        Self {
            profiler: SimpleProfiler::new(),
            memory_tracker: MemoryTracker::new(),
        }
    }
    
    fn benchmark<F, R>(&mut self, name: &str, iterations: usize, mut func: F) -> Vec<R>
    where
        F: FnMut() -> R,
    {
        let mut results = Vec::with_capacity(iterations);
        
        for i in 0..iterations {
            let iteration_name = format!("{}_iter_{}", name, i);
            self.profiler.start(&iteration_name);
            
            let result = func();
            
            self.profiler.end(&iteration_name);
            results.push(result);
        }
        
        // Aggregate all iterations under the main name
        if let Some(stats) = self.get_aggregated_stats(name, iterations) {
            self.profiler.measurements.insert(name.to_string(), vec![stats.avg]);
        }
        
        results
    }
    
    fn get_aggregated_stats(&self, name: &str, iterations: usize) -> Option<ProfileStats> {
        let mut all_durations = Vec::new();
        
        for i in 0..iterations {
            let iteration_name = format!("{}_iter_{}", name, i);
            if let Some(durations) = self.profiler.measurements.get(&iteration_name) {
                all_durations.extend(durations.iter().cloned());
            }
        }
        
        if all_durations.is_empty() {
            return None;
        }
        
        let total: Duration = all_durations.iter().sum();
        let count = all_durations.len();
        let avg = total / count as u32;
        
        let mut sorted = all_durations.clone();
        sorted.sort();
        
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let median = if count % 2 == 0 {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2
        } else {
            sorted[count / 2]
        };
        
        Some(ProfileStats {
            name: name.to_string(),
            count,
            total,
            avg,
            min,
            max,
            median,
        })
    }
    
    fn compare_algorithms<F1, F2, R>(&mut self, name1: &str, func1: F1, name2: &str, func2: F2, iterations: usize)
    where
        F1: Fn() -> R,
        F2: Fn() -> R,
    {
        println!("\n🏁 Comparing algorithms: {} vs {}", name1, name2);
        
        // Benchmark first algorithm
        self.benchmark(name1, iterations, || func1());
        
        // Benchmark second algorithm
        self.benchmark(name2, iterations, || func2());
        
        // Compare results
        if let (Some(stats1), Some(stats2)) = (self.profiler.get_stats(name1), self.profiler.get_stats(name2)) {
            println!("\n📈 Comparison Results:");
            println!("{}: {:.2}μs average", name1, stats1.avg.as_secs_f64() * 1_000_000.0);
            println!("{}: {:.2}μs average", name2, stats2.avg.as_secs_f64() * 1_000_000.0);
            
            let ratio = stats2.avg.as_secs_f64() / stats1.avg.as_secs_f64();
            if ratio > 1.0 {
                println!("{} is {:.2}x faster than {}", name1, ratio, name2);
            } else {
                println!("{} is {:.2}x faster than {}", name2, 1.0 / ratio, name1);
            }
        }
    }
    
    fn print_full_report(&self) {
        self.profiler.print_report();
        self.memory_tracker.print_report();
    }
}

/// Example algorithms for benchmarking
fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    
    let pivot = arr.len() / 2;
    let pivot_value = arr[pivot];
    arr.remove(pivot);
    
    let mut less = Vec::new();
    let mut greater = Vec::new();
    
    for item in arr {
        if item <= pivot_value {
            less.push(item);
        } else {
            greater.push(item);
        }
    }
    
    let mut result = quick_sort(less);
    result.push(pivot_value);
    result.extend(quick_sort(greater));
    result
}

fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
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

/// สาธิตการใช้งาน Profiling
pub fn demonstrate_profiling() {
    println!("📊 Profiling and Benchmarking Examples:");
    
    let mut runner = BenchmarkRunner::new();
    
    // Simple profiling example
    println!("\n⏱️ Simple Profiling:");
    
    profile!(runner.profiler, "sleep_100ms", {
        std::thread::sleep(Duration::from_millis(10)); // Reduced for demo
        "Sleep completed"
    });
    
    profile!(runner.profiler, "computation", {
        let mut sum = 0;
        for i in 0..1000000 {
            sum += i;
        }
        sum
    });
    
    // Memory tracking example
    println!("\n💾 Memory Tracking:");
    runner.memory_tracker.allocate("large_vector", 1024 * 1024); // 1MB
    runner.memory_tracker.allocate("small_buffer", 1024); // 1KB
    runner.memory_tracker.print_report();
    
    runner.memory_tracker.deallocate("small_buffer");
    println!("\nAfter deallocating small_buffer:");
    runner.memory_tracker.print_report();
    
    // Algorithm comparison
    println!("\n🏁 Algorithm Comparison:");
    
    let test_data = vec![64, 34, 25, 12, 22, 11, 90, 5, 77, 30];
    
    runner.compare_algorithms(
        "bubble_sort",
        || bubble_sort(test_data.clone()),
        "quick_sort",
        || quick_sort(test_data.clone()),
        100
    );
    
    // Fibonacci comparison
    println!("\n🔢 Fibonacci Comparison:");
    
    runner.compare_algorithms(
        "fibonacci_recursive",
        || fibonacci_recursive(20),
        "fibonacci_iterative",
        || fibonacci_iterative(20),
        1000
    );
    
    // Benchmark different data structures
    println!("\n📚 Data Structure Benchmarks:");
    
    let mut vec_data = Vec::new();
    let mut hash_data = HashMap::new();
    
    // Vector insertion benchmark
    runner.benchmark("vector_insert", 1000, || {
        vec_data.push(42);
    });
    
    // HashMap insertion benchmark
    let mut counter = 0;
    runner.benchmark("hashmap_insert", 1000, || {
        hash_data.insert(counter, 42);
        counter += 1;
    });
    
    // Print comprehensive report
    runner.print_full_report();
    
    println!("\n✅ Profiling demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_profiler() {
        let mut profiler = SimpleProfiler::new();
        
        profiler.start("test");
        std::thread::sleep(Duration::from_millis(1));
        let duration = profiler.end("test").unwrap();
        
        assert!(duration >= Duration::from_millis(1));
        
        let stats = profiler.get_stats("test").unwrap();
        assert_eq!(stats.count, 1);
        assert_eq!(stats.name, "test");
    }
    
    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        
        tracker.allocate("test", 1024);
        assert_eq!(tracker.get_current_usage(), 1024);
        assert_eq!(tracker.get_peak_usage(), 1024);
        
        tracker.allocate("test2", 2048);
        assert_eq!(tracker.get_current_usage(), 3072);
        assert_eq!(tracker.get_peak_usage(), 3072);
        
        tracker.deallocate("test");
        assert_eq!(tracker.get_current_usage(), 2048);
        assert_eq!(tracker.get_peak_usage(), 3072); // Peak should remain
    }
    
    #[test]
    fn test_sorting_algorithms() {
        let data = vec![64, 34, 25, 12, 22, 11, 90, 5];
        let expected = vec![5, 11, 12, 22, 25, 34, 64, 90];
        
        assert_eq!(bubble_sort(data.clone()), expected);
        assert_eq!(quick_sort(data.clone()), expected);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_recursive(10), 55);
        assert_eq!(fibonacci_iterative(10), 55);
        
        // Test that both implementations give same results
        for i in 0..20 {
            assert_eq!(fibonacci_recursive(i), fibonacci_iterative(i));
        }
    }
}