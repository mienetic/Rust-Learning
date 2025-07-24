//! ⚡ Mobile Performance Optimization
//! 
//! สาธิตเทคนิคการปรับปรุงประสิทธิภาพสำหรับแอปพลิเคชันมือถือ
//! รวมถึง Memory Management, Battery Optimization, และ Rendering Performance

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

/// 📊 Performance Metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cpu_usage: f32,           // percentage
    pub memory_usage: u64,        // bytes
    pub battery_usage: f32,       // percentage per hour
    pub network_usage: u64,       // bytes
    pub frame_rate: f32,          // fps
    pub frame_drops: u32,         // count
    pub app_launch_time: Duration,
    pub screen_load_time: Duration,
    pub api_response_time: Duration,
    pub disk_io_time: Duration,
    pub timestamp: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            battery_usage: 0.0,
            network_usage: 0,
            frame_rate: 60.0,
            frame_drops: 0,
            app_launch_time: Duration::new(0, 0),
            screen_load_time: Duration::new(0, 0),
            api_response_time: Duration::new(0, 0),
            disk_io_time: Duration::new(0, 0),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// 📈 Performance Monitor
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics_history: VecDeque<PerformanceMetrics>,
    max_history_size: usize,
    monitoring_enabled: bool,
    alert_thresholds: PerformanceThresholds,
    frame_times: VecDeque<Duration>,
    memory_snapshots: Vec<MemorySnapshot>,
}

#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_cpu_usage: f32,
    pub max_memory_usage: u64,
    pub max_battery_usage: f32,
    pub min_frame_rate: f32,
    pub max_frame_drops: u32,
    pub max_launch_time: Duration,
    pub max_api_response_time: Duration,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_cpu_usage: 80.0,           // 80%
            max_memory_usage: 512 * 1024 * 1024, // 512 MB
            max_battery_usage: 10.0,       // 10% per hour
            min_frame_rate: 30.0,          // 30 fps
            max_frame_drops: 5,            // 5 drops per second
            max_launch_time: Duration::from_secs(3),
            max_api_response_time: Duration::from_secs(5),
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics_history: VecDeque::new(),
            max_history_size: 1000,
            monitoring_enabled: true,
            alert_thresholds: PerformanceThresholds::default(),
            frame_times: VecDeque::new(),
            memory_snapshots: Vec::new(),
        }
    }
    
    pub fn start_monitoring(&mut self) {
        self.monitoring_enabled = true;
        println!("📊 Performance monitoring started");
    }
    
    pub fn stop_monitoring(&mut self) {
        self.monitoring_enabled = false;
        println!("⏹️ Performance monitoring stopped");
    }
    
    pub fn record_metrics(&mut self, metrics: PerformanceMetrics) {
        if !self.monitoring_enabled {
            return;
        }
        
        // Check for performance alerts
        self.check_performance_alerts(&metrics);
        
        // Add to history
        self.metrics_history.push_back(metrics);
        
        // Maintain history size
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.pop_front();
        }
    }
    
    pub fn record_frame_time(&mut self, frame_time: Duration) {
        self.frame_times.push_back(frame_time);
        
        // Keep only last 60 frames (1 second at 60fps)
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }
    }
    
    pub fn get_average_frame_rate(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let total_time: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total_time.as_secs_f32() / self.frame_times.len() as f32;
        
        if avg_frame_time > 0.0 {
            1.0 / avg_frame_time
        } else {
            0.0
        }
    }
    
    pub fn get_frame_drops(&self) -> u32 {
        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS
        
        self.frame_times
            .iter()
            .filter(|&&time| time > target_frame_time * 2) // Consider dropped if > 2x target
            .count() as u32
    }
    
    fn check_performance_alerts(&self, metrics: &PerformanceMetrics) {
        if metrics.cpu_usage > self.alert_thresholds.max_cpu_usage {
            println!("⚠️ High CPU usage: {:.1}%", metrics.cpu_usage);
        }
        
        if metrics.memory_usage > self.alert_thresholds.max_memory_usage {
            println!("⚠️ High memory usage: {:.1} MB", 
                    metrics.memory_usage as f64 / 1024.0 / 1024.0);
        }
        
        if metrics.battery_usage > self.alert_thresholds.max_battery_usage {
            println!("⚠️ High battery usage: {:.1}% per hour", metrics.battery_usage);
        }
        
        if metrics.frame_rate < self.alert_thresholds.min_frame_rate {
            println!("⚠️ Low frame rate: {:.1} fps", metrics.frame_rate);
        }
        
        if metrics.frame_drops > self.alert_thresholds.max_frame_drops {
            println!("⚠️ Frame drops detected: {}", metrics.frame_drops);
        }
        
        if metrics.app_launch_time > self.alert_thresholds.max_launch_time {
            println!("⚠️ Slow app launch: {:?}", metrics.app_launch_time);
        }
        
        if metrics.api_response_time > self.alert_thresholds.max_api_response_time {
            println!("⚠️ Slow API response: {:?}", metrics.api_response_time);
        }
    }
    
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        if self.metrics_history.is_empty() {
            return PerformanceSummary::default();
        }
        
        let count = self.metrics_history.len() as f32;
        
        let avg_cpu = self.metrics_history.iter()
            .map(|m| m.cpu_usage)
            .sum::<f32>() / count;
        
        let avg_memory = self.metrics_history.iter()
            .map(|m| m.memory_usage)
            .sum::<u64>() / self.metrics_history.len() as u64;
        
        let avg_battery = self.metrics_history.iter()
            .map(|m| m.battery_usage)
            .sum::<f32>() / count;
        
        let avg_frame_rate = self.metrics_history.iter()
            .map(|m| m.frame_rate)
            .sum::<f32>() / count;
        
        let total_frame_drops = self.metrics_history.iter()
            .map(|m| m.frame_drops)
            .sum::<u32>();
        
        PerformanceSummary {
            avg_cpu_usage: avg_cpu,
            avg_memory_usage: avg_memory,
            avg_battery_usage: avg_battery,
            avg_frame_rate,
            total_frame_drops,
            sample_count: self.metrics_history.len(),
        }
    }
    
    pub fn take_memory_snapshot(&mut self, label: String) {
        let snapshot = MemorySnapshot {
            label,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            heap_size: self.get_simulated_heap_size(),
            stack_size: self.get_simulated_stack_size(),
            allocated_objects: self.get_simulated_object_count(),
        };
        
        self.memory_snapshots.push(snapshot);
        println!("📸 Memory snapshot taken: {}", self.memory_snapshots.last().unwrap().label);
    }
    
    fn get_simulated_heap_size(&self) -> u64 {
        // Simulate heap size calculation
        100 * 1024 * 1024 // 100 MB
    }
    
    fn get_simulated_stack_size(&self) -> u64 {
        // Simulate stack size calculation
        8 * 1024 * 1024 // 8 MB
    }
    
    fn get_simulated_object_count(&self) -> u32 {
        // Simulate object count
        50000
    }
    
    pub fn get_memory_snapshots(&self) -> &Vec<MemorySnapshot> {
        &self.memory_snapshots
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub avg_cpu_usage: f32,
    pub avg_memory_usage: u64,
    pub avg_battery_usage: f32,
    pub avg_frame_rate: f32,
    pub total_frame_drops: u32,
    pub sample_count: usize,
}

impl Default for PerformanceSummary {
    fn default() -> Self {
        Self {
            avg_cpu_usage: 0.0,
            avg_memory_usage: 0,
            avg_battery_usage: 0.0,
            avg_frame_rate: 0.0,
            total_frame_drops: 0,
            sample_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub label: String,
    pub timestamp: u64,
    pub heap_size: u64,
    pub stack_size: u64,
    pub allocated_objects: u32,
}

/// 🔋 Battery Optimizer
#[derive(Debug)]
pub struct BatteryOptimizer {
    optimization_level: OptimizationLevel,
    background_tasks: HashMap<String, BackgroundTaskInfo>,
    network_requests: VecDeque<NetworkRequest>,
    screen_brightness: f32,
    location_accuracy: LocationAccuracy,
    push_notifications_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    None,
    Low,
    Medium,
    High,
    Extreme,
}

#[derive(Debug, Clone)]
pub struct BackgroundTaskInfo {
    pub name: String,
    pub priority: TaskPriority,
    pub estimated_battery_impact: f32, // percentage per hour
    pub is_essential: bool,
    pub can_be_deferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}

#[derive(Debug, Clone)]
pub struct NetworkRequest {
    pub url: String,
    pub method: String,
    pub size: u64,
    pub timestamp: u64,
    pub is_cacheable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocationAccuracy {
    Best,
    NearestTenMeters,
    HundredMeters,
    Kilometer,
    ThreeKilometers,
}

impl BatteryOptimizer {
    pub fn new() -> Self {
        Self {
            optimization_level: OptimizationLevel::Medium,
            background_tasks: HashMap::new(),
            network_requests: VecDeque::new(),
            screen_brightness: 0.8,
            location_accuracy: LocationAccuracy::HundredMeters,
            push_notifications_enabled: true,
        }
    }
    
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        println!("🔋 Setting battery optimization level: {:?}", level);
        self.optimization_level = level;
        self.apply_optimizations();
    }
    
    fn apply_optimizations(&mut self) {
        match self.optimization_level {
            OptimizationLevel::None => {
                println!("   • No battery optimizations applied");
            }
            OptimizationLevel::Low => {
                self.optimize_background_tasks(TaskPriority::Background);
                println!("   • Reduced background task frequency");
            }
            OptimizationLevel::Medium => {
                self.optimize_background_tasks(TaskPriority::Low);
                self.optimize_network_usage();
                println!("   • Optimized background tasks and network usage");
            }
            OptimizationLevel::High => {
                self.optimize_background_tasks(TaskPriority::Medium);
                self.optimize_network_usage();
                self.reduce_location_accuracy();
                println!("   • Aggressive background task optimization");
                println!("   • Reduced location accuracy");
            }
            OptimizationLevel::Extreme => {
                self.optimize_background_tasks(TaskPriority::High);
                self.optimize_network_usage();
                self.reduce_location_accuracy();
                self.disable_non_essential_features();
                println!("   • Maximum battery saving mode");
                println!("   • Disabled non-essential features");
            }
        }
    }
    
    fn optimize_background_tasks(&mut self, min_priority: TaskPriority) {
        let mut deferred_count = 0;
        
        for (name, task) in &mut self.background_tasks {
            if task.priority as u8 >= min_priority as u8 && task.can_be_deferred && !task.is_essential {
                println!("   ⏸️ Deferred background task: {}", name);
                deferred_count += 1;
            }
        }
        
        if deferred_count > 0 {
            println!("   📊 Deferred {} background tasks", deferred_count);
        }
    }
    
    fn optimize_network_usage(&mut self) {
        // Batch network requests
        let batchable_requests: Vec<_> = self.network_requests
            .iter()
            .filter(|req| req.is_cacheable)
            .collect();
        
        if !batchable_requests.is_empty() {
            println!("   📦 Batched {} network requests", batchable_requests.len());
        }
        
        // Enable request compression
        println!("   🗜️ Enabled network compression");
        
        // Use WiFi when available
        println!("   📶 Prioritizing WiFi over cellular");
    }
    
    fn reduce_location_accuracy(&mut self) {
        match self.optimization_level {
            OptimizationLevel::High => {
                self.location_accuracy = LocationAccuracy::Kilometer;
            }
            OptimizationLevel::Extreme => {
                self.location_accuracy = LocationAccuracy::ThreeKilometers;
            }
            _ => {}
        }
        
        println!("   📍 Reduced location accuracy to: {:?}", self.location_accuracy);
    }
    
    fn disable_non_essential_features(&mut self) {
        // Reduce screen brightness
        self.screen_brightness = 0.3;
        println!("   🔅 Reduced screen brightness to 30%");
        
        // Disable push notifications for non-critical apps
        println!("   🔕 Disabled non-critical push notifications");
        
        // Reduce animation frame rate
        println!("   🎬 Reduced animation frame rate");
        
        // Disable background app refresh
        println!("   🔄 Disabled background app refresh");
    }
    
    pub fn add_background_task(&mut self, task: BackgroundTaskInfo) {
        println!("➕ Added background task: {} (Impact: {:.1}%/hour)", 
                task.name, task.estimated_battery_impact);
        self.background_tasks.insert(task.name.clone(), task);
    }
    
    pub fn add_network_request(&mut self, request: NetworkRequest) {
        self.network_requests.push_back(request);
        
        // Keep only last 100 requests
        if self.network_requests.len() > 100 {
            self.network_requests.pop_front();
        }
    }
    
    pub fn get_estimated_battery_impact(&self) -> f32 {
        let task_impact: f32 = self.background_tasks
            .values()
            .map(|task| task.estimated_battery_impact)
            .sum();
        
        let network_impact = self.network_requests.len() as f32 * 0.1; // 0.1% per request
        let location_impact = match self.location_accuracy {
            LocationAccuracy::Best => 5.0,
            LocationAccuracy::NearestTenMeters => 3.0,
            LocationAccuracy::HundredMeters => 1.5,
            LocationAccuracy::Kilometer => 0.8,
            LocationAccuracy::ThreeKilometers => 0.3,
        };
        
        task_impact + network_impact + location_impact
    }
}

/// 🖼️ Rendering Optimizer
#[derive(Debug)]
pub struct RenderingOptimizer {
    target_fps: f32,
    vsync_enabled: bool,
    texture_compression: bool,
    mipmap_enabled: bool,
    culling_enabled: bool,
    lod_enabled: bool, // Level of Detail
    shadow_quality: ShadowQuality,
    anti_aliasing: AntiAliasing,
    render_scale: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShadowQuality {
    Off,
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AntiAliasing {
    Off,
    FXAA,
    MSAA2x,
    MSAA4x,
    MSAA8x,
}

impl RenderingOptimizer {
    pub fn new() -> Self {
        Self {
            target_fps: 60.0,
            vsync_enabled: true,
            texture_compression: true,
            mipmap_enabled: true,
            culling_enabled: true,
            lod_enabled: true,
            shadow_quality: ShadowQuality::Medium,
            anti_aliasing: AntiAliasing::FXAA,
            render_scale: 1.0,
        }
    }
    
    pub fn optimize_for_device(&mut self, device_tier: DeviceTier) {
        println!("🖼️ Optimizing rendering for device tier: {:?}", device_tier);
        
        match device_tier {
            DeviceTier::Low => {
                self.target_fps = 30.0;
                self.shadow_quality = ShadowQuality::Off;
                self.anti_aliasing = AntiAliasing::Off;
                self.render_scale = 0.75;
                self.texture_compression = true;
                println!("   • Target FPS: 30");
                println!("   • Shadows: Disabled");
                println!("   • Anti-aliasing: Disabled");
                println!("   • Render scale: 75%");
            }
            DeviceTier::Medium => {
                self.target_fps = 60.0;
                self.shadow_quality = ShadowQuality::Low;
                self.anti_aliasing = AntiAliasing::FXAA;
                self.render_scale = 0.9;
                println!("   • Target FPS: 60");
                println!("   • Shadows: Low quality");
                println!("   • Anti-aliasing: FXAA");
                println!("   • Render scale: 90%");
            }
            DeviceTier::High => {
                self.target_fps = 60.0;
                self.shadow_quality = ShadowQuality::High;
                self.anti_aliasing = AntiAliasing::MSAA4x;
                self.render_scale = 1.0;
                println!("   • Target FPS: 60");
                println!("   • Shadows: High quality");
                println!("   • Anti-aliasing: MSAA 4x");
                println!("   • Render scale: 100%");
            }
            DeviceTier::Ultra => {
                self.target_fps = 120.0;
                self.shadow_quality = ShadowQuality::Ultra;
                self.anti_aliasing = AntiAliasing::MSAA8x;
                self.render_scale = 1.0;
                println!("   • Target FPS: 120");
                println!("   • Shadows: Ultra quality");
                println!("   • Anti-aliasing: MSAA 8x");
                println!("   • Render scale: 100%");
            }
        }
    }
    
    pub fn enable_dynamic_resolution(&mut self, enabled: bool) {
        if enabled {
            println!("🔄 Dynamic resolution scaling enabled");
            println!("   • Will adjust render scale based on performance");
        } else {
            println!("🔒 Dynamic resolution scaling disabled");
        }
    }
    
    pub fn optimize_for_battery(&mut self) {
        println!("🔋 Optimizing rendering for battery life");
        self.target_fps = 30.0;
        self.shadow_quality = ShadowQuality::Low;
        self.anti_aliasing = AntiAliasing::Off;
        self.render_scale = 0.8;
        println!("   • Reduced target FPS to 30");
        println!("   • Lowered shadow quality");
        println!("   • Disabled anti-aliasing");
        println!("   • Reduced render scale to 80%");
    }
    
    pub fn get_estimated_gpu_load(&self) -> f32 {
        let mut load = 0.0;
        
        // Base load from FPS
        load += self.target_fps / 120.0 * 30.0; // 30% at 120fps
        
        // Shadow quality impact
        load += match self.shadow_quality {
            ShadowQuality::Off => 0.0,
            ShadowQuality::Low => 5.0,
            ShadowQuality::Medium => 10.0,
            ShadowQuality::High => 20.0,
            ShadowQuality::Ultra => 35.0,
        };
        
        // Anti-aliasing impact
        load += match self.anti_aliasing {
            AntiAliasing::Off => 0.0,
            AntiAliasing::FXAA => 2.0,
            AntiAliasing::MSAA2x => 5.0,
            AntiAliasing::MSAA4x => 10.0,
            AntiAliasing::MSAA8x => 20.0,
        };
        
        // Render scale impact
        load *= self.render_scale * self.render_scale; // Quadratic impact
        
        load.min(100.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceTier {
    Low,    // Old/budget devices
    Medium, // Mid-range devices
    High,   // Flagship devices
    Ultra,  // Latest flagship devices
}

/// 🧠 Memory Manager
#[derive(Debug)]
pub struct MemoryManager {
    memory_pools: HashMap<String, MemoryPool>,
    object_cache: HashMap<String, CachedObject>,
    max_cache_size: usize,
    gc_threshold: f32, // Trigger GC when memory usage exceeds this percentage
    memory_warnings: Vec<MemoryWarning>,
}

#[derive(Debug, Clone)]
pub struct MemoryPool {
    pub name: String,
    pub allocated_size: usize,
    pub max_size: usize,
    pub object_count: usize,
    pub fragmentation: f32,
}

#[derive(Debug, Clone)]
pub struct CachedObject {
    pub key: String,
    pub size: usize,
    pub access_count: u32,
    pub last_accessed: u64,
    pub is_essential: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryWarning {
    pub timestamp: u64,
    pub memory_usage: usize,
    pub warning_level: MemoryWarningLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryWarningLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            memory_pools: HashMap::new(),
            object_cache: HashMap::new(),
            max_cache_size: 100 * 1024 * 1024, // 100 MB
            gc_threshold: 0.8, // 80%
            memory_warnings: Vec::new(),
        }
    }
    
    pub fn create_memory_pool(&mut self, name: String, max_size: usize) {
        let pool = MemoryPool {
            name: name.clone(),
            allocated_size: 0,
            max_size,
            object_count: 0,
            fragmentation: 0.0,
        };
        
        self.memory_pools.insert(name.clone(), pool);
        println!("🧠 Created memory pool: {} ({} MB)", name, max_size / 1024 / 1024);
    }
    
    pub fn allocate_from_pool(&mut self, pool_name: &str, size: usize) -> Result<(), String> {
        if let Some(pool) = self.memory_pools.get_mut(pool_name) {
            if pool.allocated_size + size > pool.max_size {
                return Err(format!("Pool {} is full", pool_name));
            }
            
            pool.allocated_size += size;
            pool.object_count += 1;
            
            println!("📦 Allocated {} bytes from pool: {}", size, pool_name);
            Ok(())
        } else {
            Err(format!("Pool {} not found", pool_name))
        }
    }
    
    pub fn cache_object(&mut self, key: String, size: usize, is_essential: bool) {
        // Check if cache is full
        let current_cache_size: usize = self.object_cache.values()
            .map(|obj| obj.size)
            .sum();
        
        if current_cache_size + size > self.max_cache_size {
            self.evict_cache_objects(size);
        }
        
        let cached_object = CachedObject {
            key: key.clone(),
            size,
            access_count: 1,
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_essential,
        };
        
        self.object_cache.insert(key.clone(), cached_object);
        println!("🗂️ Cached object: {} ({} bytes)", key, size);
    }
    
    fn evict_cache_objects(&mut self, needed_size: usize) {
        println!("🧹 Evicting cache objects to free {} bytes", needed_size);
        
        // Sort by access frequency and recency (LRU with frequency)
        let mut objects: Vec<_> = self.object_cache.iter().collect();
        objects.sort_by(|a, b| {
            if a.1.is_essential != b.1.is_essential {
                return a.1.is_essential.cmp(&b.1.is_essential);
            }
            
            let score_a = a.1.access_count as f64 / (a.1.last_accessed as f64 + 1.0);
            let score_b = b.1.access_count as f64 / (b.1.last_accessed as f64 + 1.0);
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let mut freed_size = 0;
        let mut keys_to_remove = Vec::new();
        
        for (key, obj) in objects {
            if !obj.is_essential && freed_size < needed_size {
                freed_size += obj.size;
                keys_to_remove.push(key.clone());
                println!("   🗑️ Evicted: {} ({} bytes)", key, obj.size);
            }
        }
        
        for key in keys_to_remove {
            self.object_cache.remove(&key);
        }
        
        println!("   ✅ Freed {} bytes from cache", freed_size);
    }
    
    pub fn trigger_garbage_collection(&mut self) {
        println!("🗑️ Triggering garbage collection");
        
        // Simulate GC process
        let mut total_freed = 0;
        
        // Clean up memory pools
        for (name, pool) in &mut self.memory_pools {
            let fragmentation_size = (pool.allocated_size as f32 * pool.fragmentation) as usize;
            pool.allocated_size -= fragmentation_size;
            pool.fragmentation = 0.0;
            total_freed += fragmentation_size;
            
            if fragmentation_size > 0 {
                println!("   🧹 Defragmented pool {}: freed {} bytes", name, fragmentation_size);
            }
        }
        
        // Clean up expired cache objects
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let expired_keys: Vec<_> = self.object_cache
            .iter()
            .filter(|(_, obj)| current_time - obj.last_accessed > 3600) // 1 hour
            .map(|(key, _)| key.clone())
            .collect();
        
        for key in expired_keys {
            if let Some(obj) = self.object_cache.remove(&key) {
                total_freed += obj.size;
                println!("   🗑️ Removed expired cache object: {} ({} bytes)", key, obj.size);
            }
        }
        
        println!("   ✅ Garbage collection completed: freed {} bytes", total_freed);
    }
    
    pub fn check_memory_pressure(&mut self) -> MemoryWarningLevel {
        let total_allocated: usize = self.memory_pools.values()
            .map(|pool| pool.allocated_size)
            .sum();
        
        let cache_size: usize = self.object_cache.values()
            .map(|obj| obj.size)
            .sum();
        
        let total_memory = total_allocated + cache_size;
        let max_memory = self.memory_pools.values()
            .map(|pool| pool.max_size)
            .sum::<usize>() + self.max_cache_size;
        
        let usage_ratio = total_memory as f32 / max_memory as f32;
        
        let warning_level = if usage_ratio > 0.95 {
            MemoryWarningLevel::Critical
        } else if usage_ratio > 0.85 {
            MemoryWarningLevel::High
        } else if usage_ratio > 0.7 {
            MemoryWarningLevel::Medium
        } else {
            MemoryWarningLevel::Low
        };
        
        if warning_level != MemoryWarningLevel::Low {
            let warning = MemoryWarning {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                memory_usage: total_memory,
                warning_level: warning_level.clone(),
                message: format!("Memory usage at {:.1}%", usage_ratio * 100.0),
            };
            
            self.memory_warnings.push(warning);
            println!("⚠️ Memory pressure: {:?} ({:.1}%)", warning_level, usage_ratio * 100.0);
            
            if usage_ratio > self.gc_threshold {
                self.trigger_garbage_collection();
            }
        }
        
        warning_level
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_allocated: usize = self.memory_pools.values()
            .map(|pool| pool.allocated_size)
            .sum();
        
        let cache_size: usize = self.object_cache.values()
            .map(|obj| obj.size)
            .sum();
        
        let total_objects: usize = self.memory_pools.values()
            .map(|pool| pool.object_count)
            .sum::<usize>() + self.object_cache.len();
        
        MemoryStats {
            total_allocated,
            cache_size,
            total_objects,
            pool_count: self.memory_pools.len(),
            cached_objects: self.object_cache.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub cache_size: usize,
    pub total_objects: usize,
    pub pool_count: usize,
    pub cached_objects: usize,
}

/// ⚡ สาธิตการใช้งาน Mobile Performance Optimization
pub fn demonstrate_mobile_performance() {
    println!("⚡ === Mobile Performance Optimization Demo ===");
    
    // Performance Monitoring
    println!("\n📊 Performance Monitoring:");
    demonstrate_performance_monitoring();
    
    // Battery Optimization
    println!("\n🔋 Battery Optimization:");
    demonstrate_battery_optimization();
    
    // Rendering Optimization
    println!("\n🖼️ Rendering Optimization:");
    demonstrate_rendering_optimization();
    
    // Memory Management
    println!("\n🧠 Memory Management:");
    demonstrate_memory_management();
    
    // Best Practices
    println!("\n💡 Mobile Performance Best Practices:");
    show_mobile_performance_best_practices();
}

/// 📊 สาธิต Performance Monitoring
fn demonstrate_performance_monitoring() {
    let mut monitor = PerformanceMonitor::new();
    monitor.start_monitoring();
    
    // Simulate performance metrics
    for i in 0..5 {
        let metrics = PerformanceMetrics {
            cpu_usage: 20.0 + (i as f32 * 10.0),
            memory_usage: (100 + i * 50) * 1024 * 1024, // MB
            battery_usage: 2.0 + (i as f32 * 0.5),
            network_usage: (i as u64 + 1) * 10 * 1024 * 1024, // MB
            frame_rate: 60.0 - (i as f32 * 2.0),
            frame_drops: i as u32,
            app_launch_time: Duration::from_millis(1000 + i as u64 * 200),
            screen_load_time: Duration::from_millis(500 + i as u64 * 100),
            api_response_time: Duration::from_millis(200 + i as u64 * 50),
            disk_io_time: Duration::from_millis(50 + i as u64 * 10),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        monitor.record_metrics(metrics);
        
        // Simulate frame times
        let frame_time = Duration::from_millis(16 + i as u64 * 2); // 16ms = 60fps
        monitor.record_frame_time(frame_time);
        
        std::thread::sleep(Duration::from_millis(100));
    }
    
    // Take memory snapshots
    monitor.take_memory_snapshot("App Launch".to_string());
    monitor.take_memory_snapshot("After Loading".to_string());
    
    // Show performance summary
    let summary = monitor.get_performance_summary();
    println!("   📈 Performance Summary:");
    println!("      • Average CPU: {:.1}%", summary.avg_cpu_usage);
    println!("      • Average Memory: {:.1} MB", summary.avg_memory_usage as f64 / 1024.0 / 1024.0);
    println!("      • Average Battery: {:.1}%/hour", summary.avg_battery_usage);
    println!("      • Average FPS: {:.1}", summary.avg_frame_rate);
    println!("      • Total Frame Drops: {}", summary.total_frame_drops);
    println!("      • Sample Count: {}", summary.sample_count);
    
    // Show memory snapshots
    println!("   📸 Memory Snapshots:");
    for snapshot in monitor.get_memory_snapshots() {
        println!("      • {}: Heap {:.1}MB, Stack {:.1}MB, Objects: {}", 
                snapshot.label,
                snapshot.heap_size as f64 / 1024.0 / 1024.0,
                snapshot.stack_size as f64 / 1024.0 / 1024.0,
                snapshot.allocated_objects);
    }
}

/// 🔋 สาธิต Battery Optimization
fn demonstrate_battery_optimization() {
    let mut optimizer = BatteryOptimizer::new();
    
    // Add background tasks
    optimizer.add_background_task(BackgroundTaskInfo {
        name: "Data Sync".to_string(),
        priority: TaskPriority::Medium,
        estimated_battery_impact: 2.5,
        is_essential: false,
        can_be_deferred: true,
    });
    
    optimizer.add_background_task(BackgroundTaskInfo {
        name: "Push Notifications".to_string(),
        priority: TaskPriority::High,
        estimated_battery_impact: 1.0,
        is_essential: true,
        can_be_deferred: false,
    });
    
    optimizer.add_background_task(BackgroundTaskInfo {
        name: "Analytics".to_string(),
        priority: TaskPriority::Low,
        estimated_battery_impact: 0.5,
        is_essential: false,
        can_be_deferred: true,
    });
    
    // Add network requests
    for i in 0..5 {
        optimizer.add_network_request(NetworkRequest {
            url: format!("https://api.example.com/data/{}", i),
            method: "GET".to_string(),
            size: 1024 * (i + 1) as u64,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_cacheable: i % 2 == 0,
        });
    }
    
    println!("   🔋 Initial battery impact: {:.1}%/hour", optimizer.get_estimated_battery_impact());
    
    // Test different optimization levels
    for level in [OptimizationLevel::Low, OptimizationLevel::Medium, OptimizationLevel::High, OptimizationLevel::Extreme] {
        optimizer.set_optimization_level(level.clone());
        println!("   📊 Battery impact with {:?}: {:.1}%/hour", level, optimizer.get_estimated_battery_impact());
    }
}

/// 🖼️ สาธิต Rendering Optimization
fn demonstrate_rendering_optimization() {
    let mut optimizer = RenderingOptimizer::new();
    
    println!("   🖼️ Initial GPU load: {:.1}%", optimizer.get_estimated_gpu_load());
    
    // Test optimization for different device tiers
    for tier in [DeviceTier::Low, DeviceTier::Medium, DeviceTier::High, DeviceTier::Ultra] {
        optimizer.optimize_for_device(tier.clone());
        println!("   📊 GPU load for {:?}: {:.1}%", tier, optimizer.get_estimated_gpu_load());
    }
    
    // Battery optimization
    optimizer.optimize_for_battery();
    println!("   🔋 GPU load with battery optimization: {:.1}%", optimizer.get_estimated_gpu_load());
    
    // Dynamic resolution
    optimizer.enable_dynamic_resolution(true);
}

/// 🧠 สาธิต Memory Management
fn demonstrate_memory_management() {
    let mut memory_manager = MemoryManager::new();
    
    // Create memory pools
    memory_manager.create_memory_pool("Textures".to_string(), 200 * 1024 * 1024); // 200 MB
    memory_manager.create_memory_pool("Audio".to_string(), 50 * 1024 * 1024);     // 50 MB
    memory_manager.create_memory_pool("General".to_string(), 100 * 1024 * 1024);  // 100 MB
    
    // Allocate memory
    let _ = memory_manager.allocate_from_pool("Textures", 50 * 1024 * 1024);
    let _ = memory_manager.allocate_from_pool("Audio", 20 * 1024 * 1024);
    let _ = memory_manager.allocate_from_pool("General", 30 * 1024 * 1024);
    
    // Cache objects
    memory_manager.cache_object("user_avatar".to_string(), 2 * 1024 * 1024, true);
    memory_manager.cache_object("background_image".to_string(), 5 * 1024 * 1024, false);
    memory_manager.cache_object("ui_icons".to_string(), 1 * 1024 * 1024, true);
    
    // Check memory pressure
    let warning_level = memory_manager.check_memory_pressure();
    println!("   ⚠️ Memory warning level: {:?}", warning_level);
    
    // Show memory stats
    let stats = memory_manager.get_memory_stats();
    println!("   📊 Memory Statistics:");
    println!("      • Total Allocated: {:.1} MB", stats.total_allocated as f64 / 1024.0 / 1024.0);
    println!("      • Cache Size: {:.1} MB", stats.cache_size as f64 / 1024.0 / 1024.0);
    println!("      • Total Objects: {}", stats.total_objects);
    println!("      • Memory Pools: {}", stats.pool_count);
    println!("      • Cached Objects: {}", stats.cached_objects);
    
    // Simulate memory pressure
    for i in 0..5 {
        memory_manager.cache_object(
            format!("temp_object_{}", i),
            20 * 1024 * 1024, // 20 MB each
            false,
        );
    }
    
    // This should trigger cache eviction
    memory_manager.cache_object("large_object".to_string(), 30 * 1024 * 1024, false);
}

/// 💡 Mobile Performance Best Practices
fn show_mobile_performance_best_practices() {
    let practices = vec![
        "⚡ Optimize app launch time (< 3 seconds)",
        "🖼️ Use appropriate image formats and compression",
        "🔋 Minimize background processing",
        "📊 Monitor and profile performance regularly",
        "🧠 Implement efficient memory management",
        "🎬 Maintain consistent frame rates (60 FPS)",
        "🌐 Optimize network requests and caching",
        "💾 Use lazy loading for non-critical resources",
        "🔄 Implement proper lifecycle management",
        "📱 Test on various device configurations",
        "🎯 Use device-specific optimizations",
        "🗜️ Enable data compression",
        "⏰ Implement request timeouts",
        "🔍 Use performance monitoring tools",
        "🧹 Regular cleanup of unused resources",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   🛠️ Performance Tools:");
    println!("      🍎 iOS:");
    println!("         • Instruments (Time Profiler, Allocations)");
    println!("         • Xcode Memory Graph Debugger");
    println!("         • MetricKit for app metrics");
    println!("         • Energy Impact in Xcode");
    
    println!("\n      🤖 Android:");
    println!("         • Android Profiler (CPU, Memory, Network)");
    println!("         • Systrace for system-level analysis");
    println!("         • GPU Inspector for graphics");
    println!("         • Battery Historian");
    
    println!("\n   📚 Optimization Strategies:");
    println!("      • Use object pooling for frequently created objects");
    println!("      • Implement texture atlasing for graphics");
    println!("      • Use level-of-detail (LOD) for 3D models");
    println!("      • Implement occlusion culling");
    println!("      • Use compressed texture formats");
    println!("      • Optimize shader complexity");
    println!("      • Implement dynamic batching");
    println!("      • Use async/await for non-blocking operations");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        
        let metrics = PerformanceMetrics::default();
        monitor.record_metrics(metrics);
        
        assert_eq!(monitor.metrics_history.len(), 1);
        
        let summary = monitor.get_performance_summary();
        assert_eq!(summary.sample_count, 1);
    }
    
    #[test]
    fn test_battery_optimizer() {
        let mut optimizer = BatteryOptimizer::new();
        
        let initial_impact = optimizer.get_estimated_battery_impact();
        
        optimizer.add_background_task(BackgroundTaskInfo {
            name: "Test Task".to_string(),
            priority: TaskPriority::Low,
            estimated_battery_impact: 1.0,
            is_essential: false,
            can_be_deferred: true,
        });
        
        let new_impact = optimizer.get_estimated_battery_impact();
        assert!(new_impact > initial_impact);
    }
    
    #[test]
    fn test_rendering_optimizer() {
        let mut optimizer = RenderingOptimizer::new();
        
        let initial_load = optimizer.get_estimated_gpu_load();
        
        optimizer.optimize_for_device(DeviceTier::Low);
        let low_tier_load = optimizer.get_estimated_gpu_load();
        
        optimizer.optimize_for_device(DeviceTier::Ultra);
        let ultra_tier_load = optimizer.get_estimated_gpu_load();
        
        assert!(low_tier_load < ultra_tier_load);
    }
    
    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        
        manager.create_memory_pool("Test Pool".to_string(), 1024 * 1024);
        assert!(manager.allocate_from_pool("Test Pool", 512 * 1024).is_ok());
        
        manager.cache_object("test_object".to_string(), 1024, false);
        
        let stats = manager.get_memory_stats();
        assert_eq!(stats.cached_objects, 1);
        assert_eq!(stats.pool_count, 1);
    }
    
    #[test]
    fn test_memory_pressure() {
        let mut manager = MemoryManager::new();
        
        // Create a larger pool to trigger memory pressure
        manager.create_memory_pool("Test Pool".to_string(), 80 * 1024 * 1024); // 80 MB
        let _ = manager.allocate_from_pool("Test Pool", 70 * 1024 * 1024); // 70 MB
        
        // Add cache objects to increase memory usage
        manager.cache_object("large_object".to_string(), 20 * 1024 * 1024, false); // 20 MB
        
        let warning_level = manager.check_memory_pressure();
        // Total: 70MB (pool) + 20MB (cache) = 90MB
        // Max: 80MB (pool) + 100MB (cache) = 180MB
        // Usage ratio: 90/180 = 0.5, should still be Low
        // Let's increase usage to trigger Medium warning (>0.7)
        manager.cache_object("another_object".to_string(), 40 * 1024 * 1024, false); // 40 MB more
        
        let warning_level = manager.check_memory_pressure();
        // Total: 70MB + 60MB = 130MB, Usage: 130/180 = 0.72 > 0.7 = Medium
        assert!(warning_level != MemoryWarningLevel::Low);
    }
}