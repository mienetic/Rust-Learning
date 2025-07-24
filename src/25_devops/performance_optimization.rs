//! ⚡ Performance Optimization - การปรับปรุงประสิทธิภาพ
//! 
//! โมดูลนี้สาธิตเทคนิคการปรับปรุงประสิทธิภาพสำหรับ Rust applications
//! รวมถึง Profiling, Benchmarking, Memory Optimization, และ Caching

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

/// ⚡ Performance Metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub response_time: Duration,
    pub throughput: f64,
    pub error_rate: f64,
    pub timestamp: Instant,
}

impl PerformanceMetrics {
    /// สร้าง PerformanceMetrics ใหม่
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            response_time: Duration::from_millis(0),
            throughput: 0.0,
            error_rate: 0.0,
            timestamp: Instant::now(),
        }
    }
    
    /// อัปเดต CPU usage
    pub fn with_cpu_usage(mut self, usage: f64) -> Self {
        self.cpu_usage = usage;
        self
    }
    
    /// อัปเดต Memory usage
    pub fn with_memory_usage(mut self, usage: u64) -> Self {
        self.memory_usage = usage;
        self
    }
    
    /// อัปเดต Response time
    pub fn with_response_time(mut self, time: Duration) -> Self {
        self.response_time = time;
        self
    }
    
    /// อัปเดต Throughput
    pub fn with_throughput(mut self, throughput: f64) -> Self {
        self.throughput = throughput;
        self
    }
    
    /// อัปเดต Error rate
    pub fn with_error_rate(mut self, rate: f64) -> Self {
        self.error_rate = rate;
        self
    }
    
    /// คำนวณ performance score
    pub fn calculate_score(&self) -> f64 {
        let cpu_score = (100.0 - self.cpu_usage) / 100.0;
        let memory_score = if self.memory_usage < 1_000_000_000 { 1.0 } else { 0.5 };
        let response_score = if self.response_time.as_millis() < 100 { 1.0 } else { 0.5 };
        let throughput_score = (self.throughput / 1000.0).min(1.0);
        let error_score = (100.0 - self.error_rate) / 100.0;
        
        (cpu_score + memory_score + response_score + throughput_score + error_score) / 5.0
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// 📊 Performance Profiler
#[derive(Debug)]
pub struct PerformanceProfiler {
    pub name: String,
    pub start_time: Option<Instant>,
    pub measurements: Vec<(String, Duration)>,
    pub memory_snapshots: Vec<(String, u64)>,
}

impl PerformanceProfiler {
    /// สร้าง PerformanceProfiler ใหม่
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: None,
            measurements: Vec::new(),
            memory_snapshots: Vec::new(),
        }
    }
    
    /// เริ่มต้น profiling
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        println!("🚀 เริ่มต้น profiling: {}", self.name);
    }
    
    /// วัดเวลาของ operation
    pub fn measure<F, R>(&mut self, operation_name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        
        self.measurements.push((operation_name.to_string(), duration));
        println!("⏱️  {}: {:?}", operation_name, duration);
        
        result
    }
    
    /// บันทึก memory snapshot
    pub fn snapshot_memory(&mut self, label: &str) {
        // จำลอง memory usage (ในการใช้งานจริงจะใช้ system calls)
        let memory_usage = self.estimate_memory_usage();
        self.memory_snapshots.push((label.to_string(), memory_usage));
        println!("💾 Memory snapshot {}: {} bytes", label, memory_usage);
    }
    
    /// ประมาณการ memory usage
    fn estimate_memory_usage(&self) -> u64 {
        // จำลองการวัด memory usage
        let base_usage = 1_000_000; // 1MB base
        let measurements_size = self.measurements.len() as u64 * 100;
        let snapshots_size = self.memory_snapshots.len() as u64 * 50;
        
        base_usage + measurements_size + snapshots_size
    }
    
    /// สิ้นสุด profiling และแสดงผลสรุป
    pub fn finish(&self) {
        if let Some(start_time) = self.start_time {
            let total_duration = start_time.elapsed();
            
            println!("\n📊 === Profiling Report: {} ===", self.name);
            println!("⏱️  Total Duration: {:?}", total_duration);
            
            println!("\n🔍 Operation Measurements:");
            for (operation, duration) in &self.measurements {
                let percentage = (duration.as_nanos() as f64 / total_duration.as_nanos() as f64) * 100.0;
                println!("   • {}: {:?} ({:.2}%)", operation, duration, percentage);
            }
            
            println!("\n💾 Memory Snapshots:");
            for (label, memory) in &self.memory_snapshots {
                println!("   • {}: {} bytes ({:.2} MB)", label, memory, *memory as f64 / 1_000_000.0);
            }
            
            // หา bottlenecks
            if let Some((slowest_op, slowest_time)) = self.measurements.iter().max_by_key(|(_, duration)| duration) {
                println!("\n🐌 Slowest Operation: {} ({:?})", slowest_op, slowest_time);
            }
            
            println!("\n✅ Profiling completed!");
        }
    }
}

/// 🏃 Benchmark Runner
#[derive(Debug)]
pub struct BenchmarkRunner {
    pub name: String,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub results: Vec<Duration>,
}

impl BenchmarkRunner {
    /// สร้าง BenchmarkRunner ใหม่
    pub fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            iterations,
            warmup_iterations: iterations / 10,
            results: Vec::new(),
        }
    }
    
    /// รัน benchmark
    pub fn run<F>(&mut self, benchmark_fn: F)
    where
        F: Fn() + Clone,
    {
        println!("🏃 เริ่มต้น benchmark: {}", self.name);
        
        // Warmup
        println!("🔥 Warmup ({} iterations)...", self.warmup_iterations);
        for _ in 0..self.warmup_iterations {
            benchmark_fn();
        }
        
        // Actual benchmark
        println!("⚡ Running benchmark ({} iterations)...", self.iterations);
        self.results.clear();
        
        for i in 0..self.iterations {
            let start = Instant::now();
            benchmark_fn();
            let duration = start.elapsed();
            self.results.push(duration);
            
            if (i + 1) % (self.iterations / 10) == 0 {
                println!("   Progress: {}/{}", i + 1, self.iterations);
            }
        }
        
        self.analyze_results();
    }
    
    /// วิเคราะห์ผลลัพธ์
    fn analyze_results(&self) {
        if self.results.is_empty() {
            return;
        }
        
        let total_nanos: u128 = self.results.iter().map(|d| d.as_nanos()).sum();
        let avg_nanos = total_nanos / self.results.len() as u128;
        let avg_duration = Duration::from_nanos(avg_nanos as u64);
        
        let mut sorted_results = self.results.clone();
        sorted_results.sort();
        
        let min_duration = sorted_results.first().unwrap();
        let max_duration = sorted_results.last().unwrap();
        let median_duration = &sorted_results[sorted_results.len() / 2];
        let p95_index = (sorted_results.len() as f64 * 0.95) as usize;
        let p95_duration = &sorted_results[p95_index.min(sorted_results.len() - 1)];
        
        println!("\n📊 === Benchmark Results: {} ===", self.name);
        println!("🔢 Iterations: {}", self.iterations);
        println!("⏱️  Average: {:?}", avg_duration);
        println!("⚡ Min: {:?}", min_duration);
        println!("🐌 Max: {:?}", max_duration);
        println!("📊 Median: {:?}", median_duration);
        println!("📈 95th Percentile: {:?}", p95_duration);
        
        // คำนวณ throughput
        let throughput = 1_000_000_000.0 / avg_nanos as f64; // operations per second
        println!("🚀 Throughput: {:.2} ops/sec", throughput);
        
        // คำนวณ standard deviation
        let variance: f64 = self.results.iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - avg_nanos as f64;
                diff * diff
            })
            .sum::<f64>() / self.results.len() as f64;
        let std_dev = variance.sqrt();
        let std_dev_duration = Duration::from_nanos(std_dev as u64);
        
        println!("📏 Standard Deviation: {:?}", std_dev_duration);
        
        // Performance rating
        let rating = if avg_duration.as_millis() < 1 {
            "🚀 Excellent"
        } else if avg_duration.as_millis() < 10 {
            "✅ Good"
        } else if avg_duration.as_millis() < 100 {
            "⚠️ Fair"
        } else {
            "🐌 Needs Optimization"
        };
        
        println!("🏆 Performance Rating: {}", rating);
    }
}

/// 💾 Memory Pool สำหรับ optimization
#[derive(Debug, Clone)]
pub struct MemoryPool<T> {
    pub pool: Arc<Mutex<Vec<T>>>,
    pub max_size: usize,
    pub created_count: Arc<Mutex<usize>>,
    pub reused_count: Arc<Mutex<usize>>,
}

impl<T: Default> MemoryPool<T> {
    /// สร้าง MemoryPool ใหม่
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            max_size,
            created_count: Arc::new(Mutex::new(0)),
            reused_count: Arc::new(Mutex::new(0)),
        }
    }
    
    /// ขอ object จาก pool
    pub fn acquire(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        
        if let Some(item) = pool.pop() {
            *self.reused_count.lock().unwrap() += 1;
            item
        } else {
            *self.created_count.lock().unwrap() += 1;
            T::default()
        }
    }
    
    /// คืน object กลับไปยัง pool
    pub fn release(&self, item: T) {
        let mut pool = self.pool.lock().unwrap();
        
        if pool.len() < self.max_size {
            pool.push(item);
        }
        // ถ้า pool เต็ม จะทิ้ง object
    }
    
    /// ดูสถิติการใช้งาน pool
    pub fn stats(&self) -> (usize, usize, usize) {
        let pool_size = self.pool.lock().unwrap().len();
        let created = *self.created_count.lock().unwrap();
        let reused = *self.reused_count.lock().unwrap();
        
        (pool_size, created, reused)
    }
    
    /// คำนวณ hit rate
    pub fn hit_rate(&self) -> f64 {
        let (_, created, reused) = self.stats();
        let total = created + reused;
        
        if total == 0 {
            0.0
        } else {
            reused as f64 / total as f64
        }
    }
}

/// 🗄️ Simple Cache Implementation
#[derive(Debug)]
pub struct SimpleCache<K, V> 
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub data: Arc<Mutex<HashMap<K, (V, Instant)>>>,
    pub max_size: usize,
    pub ttl: Duration,
    pub hits: Arc<Mutex<u64>>,
    pub misses: Arc<Mutex<u64>>,
}

impl<K, V> SimpleCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// สร้าง SimpleCache ใหม่
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            ttl,
            hits: Arc::new(Mutex::new(0)),
            misses: Arc::new(Mutex::new(0)),
        }
    }
    
    /// ดึงข้อมูลจาก cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                *self.hits.lock().unwrap() += 1;
                return Some(value.clone());
            } else {
                // ข้อมูลหมดอายุ
                data.remove(key);
            }
        }
        
        *self.misses.lock().unwrap() += 1;
        None
    }
    
    /// เก็บข้อมูลใน cache
    pub fn put(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        
        // ลบข้อมูลเก่าถ้า cache เต็ม
        if data.len() >= self.max_size {
            // Simple LRU: ลบ entry แรกที่เจอ
            if let Some(first_key) = data.keys().next().cloned() {
                data.remove(&first_key);
            }
        }
        
        data.insert(key, (value, Instant::now()));
    }
    
    /// ลบข้อมูลที่หมดอายุ
    pub fn cleanup_expired(&self) {
        let mut data = self.data.lock().unwrap();
        let now = Instant::now();
        
        data.retain(|_, (_, timestamp)| now.duration_since(*timestamp) < self.ttl);
    }
    
    /// ดูสถิติ cache
    pub fn stats(&self) -> (u64, u64, f64, usize) {
        let hits = *self.hits.lock().unwrap();
        let misses = *self.misses.lock().unwrap();
        let total = hits + misses;
        let hit_rate = if total == 0 { 0.0 } else { hits as f64 / total as f64 };
        let size = self.data.lock().unwrap().len();
        
        (hits, misses, hit_rate, size)
    }
}

/// 🔧 Performance Optimizer
#[derive(Debug)]
pub struct PerformanceOptimizer {
    pub name: String,
    pub optimizations: Vec<String>,
    pub baseline_metrics: Option<PerformanceMetrics>,
    pub current_metrics: Option<PerformanceMetrics>,
}

impl PerformanceOptimizer {
    /// สร้าง PerformanceOptimizer ใหม่
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            optimizations: Vec::new(),
            baseline_metrics: None,
            current_metrics: None,
        }
    }
    
    /// ตั้งค่า baseline metrics
    pub fn set_baseline(&mut self, metrics: PerformanceMetrics) {
        self.baseline_metrics = Some(metrics);
        println!("📊 Baseline metrics set for {}", self.name);
    }
    
    /// อัปเดต current metrics
    pub fn update_metrics(&mut self, metrics: PerformanceMetrics) {
        self.current_metrics = Some(metrics);
    }
    
    /// เพิ่ม optimization
    pub fn add_optimization(&mut self, optimization: &str) {
        self.optimizations.push(optimization.to_string());
        println!("🔧 Added optimization: {}", optimization);
    }
    
    /// วิเคราะห์การปรับปรุง
    pub fn analyze_improvement(&self) -> Option<f64> {
        if let (Some(baseline), Some(current)) = (&self.baseline_metrics, &self.current_metrics) {
            let baseline_score = baseline.calculate_score();
            let current_score = current.calculate_score();
            let improvement = ((current_score - baseline_score) / baseline_score) * 100.0;
            
            println!("\n📈 === Performance Analysis: {} ===", self.name);
            println!("📊 Baseline Score: {:.2}", baseline_score);
            println!("📊 Current Score: {:.2}", current_score);
            println!("📈 Improvement: {:.2}%", improvement);
            
            if improvement > 0.0 {
                println!("✅ Performance improved!");
            } else {
                println!("⚠️ Performance degraded!");
            }
            
            println!("\n🔧 Applied Optimizations:");
            for (i, opt) in self.optimizations.iter().enumerate() {
                println!("   {}. {}", i + 1, opt);
            }
            
            Some(improvement)
        } else {
            println!("❌ Cannot analyze: missing baseline or current metrics");
            None
        }
    }
    
    /// แนะนำการปรับปรุง
    pub fn suggest_optimizations(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if let Some(current) = &self.current_metrics {
            if current.cpu_usage > 80.0 {
                suggestions.push("🔥 Optimize CPU-intensive operations".to_string());
                suggestions.push("⚡ Consider parallel processing".to_string());
            }
            
            if current.memory_usage > 1_000_000_000 { // > 1GB
                suggestions.push("💾 Implement memory pooling".to_string());
                suggestions.push("🗑️ Review memory leaks".to_string());
            }
            
            if current.response_time.as_millis() > 100 {
                suggestions.push("🚀 Add caching layer".to_string());
                suggestions.push("📊 Optimize database queries".to_string());
            }
            
            if current.error_rate > 1.0 {
                suggestions.push("🛡️ Improve error handling".to_string());
                suggestions.push("🔍 Add better monitoring".to_string());
            }
            
            if current.throughput < 100.0 {
                suggestions.push("⚡ Optimize I/O operations".to_string());
                suggestions.push("🔄 Consider async processing".to_string());
            }
        }
        
        if suggestions.is_empty() {
            suggestions.push("✅ Performance looks good!".to_string());
        }
        
        suggestions
    }
}

/// 🎯 สาธิตการทำงานกับ Performance Optimization
pub fn demonstrate_performance_optimization() {
    println!("\n⚡ === Performance Optimization Demo ===");
    
    // 1. Performance Profiling
    println!("\n1️⃣ Performance Profiling:");
    demonstrate_profiling();
    
    // 2. Benchmarking
    println!("\n2️⃣ Benchmarking:");
    demonstrate_benchmarking();
    
    // 3. Memory Optimization
    println!("\n3️⃣ Memory Optimization:");
    demonstrate_memory_optimization();
    
    // 4. Caching
    println!("\n4️⃣ Caching:");
    demonstrate_caching();
    
    // 5. Performance Analysis
    println!("\n5️⃣ Performance Analysis:");
    demonstrate_performance_analysis();
    
    // 6. Optimization Best Practices
    println!("\n6️⃣ Optimization Best Practices:");
    show_optimization_best_practices();
    
    println!("\n✅ จบการสาธิต Performance Optimization!");
}

/// 📊 สาธิต Profiling
fn demonstrate_profiling() {
    println!("📊 การใช้งาน Performance Profiler:");
    
    let mut profiler = PerformanceProfiler::new("Sample Application");
    profiler.start();
    
    // จำลองการทำงานต่างๆ
    profiler.snapshot_memory("startup");
    
    profiler.measure("database_connection", || {
        thread::sleep(Duration::from_millis(50));
    });
    
    profiler.measure("data_processing", || {
        thread::sleep(Duration::from_millis(100));
    });
    
    profiler.snapshot_memory("after_processing");
    
    profiler.measure("response_generation", || {
        thread::sleep(Duration::from_millis(30));
    });
    
    profiler.snapshot_memory("final");
    profiler.finish();
}

/// 🏃 สาธิต Benchmarking
fn demonstrate_benchmarking() {
    println!("🏃 การใช้งาน Benchmark Runner:");
    
    // Benchmark การคำนวณ
    let mut calc_benchmark = BenchmarkRunner::new("Mathematical Calculation", 1000);
    calc_benchmark.run(|| {
        let _result: f64 = (1..100).map(|x| (x as f64).sqrt()).sum();
    });
    
    // Benchmark การจัดการ String
    let mut string_benchmark = BenchmarkRunner::new("String Operations", 500);
    string_benchmark.run(|| {
        let mut s = String::new();
        for i in 0..100 {
            s.push_str(&format!("item_{}", i));
        }
    });
}

/// 💾 สาธิต Memory Optimization
fn demonstrate_memory_optimization() {
    println!("💾 การใช้งาน Memory Pool:");
    
    let pool = MemoryPool::<Vec<u8>>::new(10);
    
    // จำลองการใช้งาน memory pool
    let mut handles = Vec::new();
    
    for i in 0..20 {
        let pool_clone = pool.clone();
        let handle = thread::spawn(move || {
            let mut buffer = pool_clone.acquire();
            buffer.clear();
            buffer.extend_from_slice(&vec![i; 1000]);
            
            // จำลองการทำงาน
            thread::sleep(Duration::from_millis(10));
            
            pool_clone.release(buffer);
        });
        handles.push(handle);
    }
    
    // รอให้ threads เสร็จ
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (pool_size, created, reused) = pool.stats();
    println!("📊 Pool Stats:");
    println!("   • Pool Size: {}", pool_size);
    println!("   • Objects Created: {}", created);
    println!("   • Objects Reused: {}", reused);
    println!("   • Hit Rate: {:.2}%", pool.hit_rate() * 100.0);
}

/// 🗄️ สาธิต Caching
fn demonstrate_caching() {
    println!("🗄️ การใช้งาน Cache:");
    
    let cache = SimpleCache::<String, String>::new(5, Duration::from_secs(2));
    
    // เพิ่มข้อมูลใน cache
    cache.put("user:1".to_string(), "John Doe".to_string());
    cache.put("user:2".to_string(), "Jane Smith".to_string());
    cache.put("user:3".to_string(), "Bob Johnson".to_string());
    
    // ทดสอบ cache hits
    println!("🔍 Testing cache hits:");
    for i in 1..=3 {
        let key = format!("user:{}", i);
        if let Some(value) = cache.get(&key) {
            println!("   ✅ Cache hit: {} = {}", key, value);
        } else {
            println!("   ❌ Cache miss: {}", key);
        }
    }
    
    // ทดสอบ cache miss
    println!("\n🔍 Testing cache miss:");
    if let Some(value) = cache.get(&"user:999".to_string()) {
        println!("   ✅ Cache hit: user:999 = {}", value);
    } else {
        println!("   ❌ Cache miss: user:999");
    }
    
    // รอให้ข้อมูลหมดอายุ
    println!("\n⏰ Waiting for TTL expiration...");
    thread::sleep(Duration::from_secs(3));
    
    // ทดสอบหลังหมดอายุ
    println!("\n🔍 Testing after TTL expiration:");
    if let Some(value) = cache.get(&"user:1".to_string()) {
        println!("   ✅ Cache hit: user:1 = {}", value);
    } else {
        println!("   ❌ Cache miss: user:1 (expired)");
    }
    
    let (hits, misses, hit_rate, size) = cache.stats();
    println!("\n📊 Cache Stats:");
    println!("   • Hits: {}", hits);
    println!("   • Misses: {}", misses);
    println!("   • Hit Rate: {:.2}%", hit_rate * 100.0);
    println!("   • Current Size: {}", size);
}

/// 📈 สาธิต Performance Analysis
fn demonstrate_performance_analysis() {
    println!("📈 การวิเคราะห์ Performance:");
    
    let mut optimizer = PerformanceOptimizer::new("Web API");
    
    // ตั้งค่า baseline
    let baseline = PerformanceMetrics::new()
        .with_cpu_usage(75.0)
        .with_memory_usage(800_000_000)
        .with_response_time(Duration::from_millis(150))
        .with_throughput(200.0)
        .with_error_rate(2.0);
    
    optimizer.set_baseline(baseline);
    
    // เพิ่ม optimizations
    optimizer.add_optimization("Added connection pooling");
    optimizer.add_optimization("Implemented caching layer");
    optimizer.add_optimization("Optimized database queries");
    
    // อัปเดต metrics หลัง optimization
    let optimized = PerformanceMetrics::new()
        .with_cpu_usage(60.0)
        .with_memory_usage(600_000_000)
        .with_response_time(Duration::from_millis(80))
        .with_throughput(350.0)
        .with_error_rate(0.5);
    
    optimizer.update_metrics(optimized);
    
    // วิเคราะห์การปรับปรุง
    optimizer.analyze_improvement();
    
    // แนะนำการปรับปรุงเพิ่มเติม
    println!("\n💡 Optimization Suggestions:");
    let suggestions = optimizer.suggest_optimizations();
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("   {}. {}", i + 1, suggestion);
    }
}

/// 📋 แสดง Optimization Best Practices
fn show_optimization_best_practices() {
    println!("📋 Performance Optimization Best Practices:");
    
    let practices = vec![
        ("📊", "Measure First", "วัดประสิทธิภาพก่อนทำการปรับปรุง"),
        ("🎯", "Identify Bottlenecks", "หา bottlenecks ที่แท้จริง"),
        ("⚡", "Optimize Hot Paths", "ปรับปรุง code ที่ใช้บ่อยที่สุด"),
        ("💾", "Memory Management", "จัดการ memory อย่างมีประสิทธิภาพ"),
        ("🗄️", "Caching Strategy", "ใช้ caching อย่างเหมาะสม"),
        ("🔄", "Async Processing", "ใช้ async สำหรับ I/O operations"),
        ("📈", "Monitor Continuously", "ติดตามประสิทธิภาพอย่างต่อเนื่อง"),
        ("🧪", "Test Performance", "ทดสอบประสิทธิภาพใน CI/CD"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🔧 Rust-Specific Optimizations:");
    println!("   • ใช้ `cargo build --release` สำหรับ production");
    println!("   • ปรับแต่ง compiler flags ใน Cargo.toml");
    println!("   • ใช้ `#[inline]` สำหรับ hot functions");
    println!("   • หลีกเลี่ยง unnecessary allocations");
    println!("   • ใช้ `Vec::with_capacity()` เมื่อทราบขนาด");
    println!("   • ใช้ `&str` แทน `String` เมื่อเป็นไปได้");
    println!("   • ใช้ `Cow<str>` สำหรับ conditional ownership");
    
    println!("\n📊 Profiling Tools:");
    println!("   • cargo flamegraph - สร้าง flame graphs");
    println!("   • perf - Linux performance profiler");
    println!("   • Instruments - macOS profiler");
    println!("   • criterion - Rust benchmarking library");
    println!("   • valgrind - Memory profiler");
    
    println!("\n⚠️ Common Performance Pitfalls:");
    println!("   • การ clone ข้อมูลโดยไม่จำเป็น");
    println!("   • การใช้ String concatenation ใน loops");
    println!("   • การไม่ใช้ iterators อย่างมีประสิทธิภาพ");
    println!("   • การ allocate memory ใน hot paths");
    println!("   • การไม่ใช้ appropriate data structures");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::new()
            .with_cpu_usage(50.0)
            .with_memory_usage(500_000_000)
            .with_response_time(Duration::from_millis(100))
            .with_throughput(500.0)
            .with_error_rate(1.0);
        
        assert_eq!(metrics.cpu_usage, 50.0);
        assert_eq!(metrics.memory_usage, 500_000_000);
        assert_eq!(metrics.response_time, Duration::from_millis(100));
        assert_eq!(metrics.throughput, 500.0);
        assert_eq!(metrics.error_rate, 1.0);
        
        let score = metrics.calculate_score();
        assert!(score > 0.0 && score <= 1.0);
    }
    
    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::<Vec<u8>>::new(2);
        
        // ขอ objects จาก pool
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();
        
        // คืน objects กลับไป
        pool.release(obj1);
        pool.release(obj2);
        
        // ขอ object ใหม่ (ควรได้จาก pool)
        let _obj3 = pool.acquire();
        
        let (pool_size, created, reused) = pool.stats();
        assert_eq!(created, 2);
        assert_eq!(reused, 1);
        assert_eq!(pool_size, 1);
    }
    
    #[test]
    fn test_simple_cache() {
        let cache = SimpleCache::new(2, Duration::from_secs(1));
        
        // เพิ่มข้อมูล
        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());
        
        // ทดสอบ cache hit
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
        
        // ทดสอบ cache miss
        assert_eq!(cache.get(&"key3".to_string()), None);
        
        let (hits, misses, hit_rate, size) = cache.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
        assert_eq!(size, 2);
        assert!((hit_rate - 0.6667).abs() < 0.001);
    }
    
    #[test]
    fn test_benchmark_runner() {
        let mut benchmark = BenchmarkRunner::new("Test Benchmark", 10);
        
        benchmark.run(|| {
            // จำลองการทำงาน
            let _sum: u32 = (1..100).sum();
        });
        
        assert_eq!(benchmark.results.len(), 10);
        assert!(benchmark.results.iter().all(|d| d.as_nanos() > 0));
    }
    
    #[test]
    fn test_performance_optimizer() {
        let mut optimizer = PerformanceOptimizer::new("Test App");
        
        let baseline = PerformanceMetrics::new()
            .with_cpu_usage(80.0)
            .with_throughput(100.0);
        
        let improved = PerformanceMetrics::new()
            .with_cpu_usage(60.0)
            .with_throughput(150.0);
        
        optimizer.set_baseline(baseline);
        optimizer.update_metrics(improved);
        optimizer.add_optimization("Test optimization");
        
        let improvement = optimizer.analyze_improvement();
        assert!(improvement.is_some());
        assert!(improvement.unwrap() > 0.0);
    }
}