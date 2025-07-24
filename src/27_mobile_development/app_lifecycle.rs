//! 🔄 App Lifecycle Management
//! 
//! สาธิตการจัดการ Lifecycle ของแอปพลิเคชันมือถือ
//! รวมถึง State Management, Background Tasks, และ Memory Management

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

/// 📱 App State
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    NotRunning,     // แอปยังไม่ได้เริ่มต้น
    Inactive,       // แอปกำลังเปลี่ยนสถานะ
    Active,         // แอปทำงานอยู่และผู้ใช้กำลังใช้งาน
    Background,     // แอปทำงานในพื้นหลัง
    Suspended,      // แอปถูกระงับการทำงาน
    Terminated,     // แอปถูกปิด
}

/// 🔄 Lifecycle Event
#[derive(Debug, Clone)]
pub enum LifecycleEvent {
    AppLaunched,
    AppBecameActive,
    AppWillResignActive,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppWillTerminate,
    MemoryWarning,
    OrientationChanged,
    NetworkStatusChanged(bool), // connected/disconnected
}

/// 📊 App Metrics
#[derive(Debug, Clone)]
pub struct AppMetrics {
    pub launch_time: u64,
    pub active_time: Duration,
    pub background_time: Duration,
    pub memory_usage: u64, // bytes
    pub cpu_usage: f32,    // percentage
    pub battery_usage: f32, // percentage
    pub network_usage: u64, // bytes
    pub crash_count: u32,
    pub session_count: u32,
}

impl Default for AppMetrics {
    fn default() -> Self {
        Self {
            launch_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            active_time: Duration::new(0, 0),
            background_time: Duration::new(0, 0),
            memory_usage: 0,
            cpu_usage: 0.0,
            battery_usage: 0.0,
            network_usage: 0,
            crash_count: 0,
            session_count: 1,
        }
    }
}

/// 🎯 Lifecycle Observer Trait
pub trait LifecycleObserver {
    fn on_app_launched(&mut self);
    fn on_app_became_active(&mut self);
    fn on_app_will_resign_active(&mut self);
    fn on_app_did_enter_background(&mut self);
    fn on_app_will_enter_foreground(&mut self);
    fn on_app_will_terminate(&mut self);
    fn on_memory_warning(&mut self);
    fn on_orientation_changed(&mut self);
    fn on_network_status_changed(&mut self, connected: bool);
}

/// 📱 App Lifecycle Manager
pub struct AppLifecycleManager {
    current_state: AppState,
    previous_state: AppState,
    state_history: Vec<(AppState, u64)>,
    observers: Vec<Box<dyn LifecycleObserver + Send>>,
    metrics: AppMetrics,
    background_tasks: HashMap<String, BackgroundTask>,
    saved_state: HashMap<String, String>,
    is_low_memory: bool,
    orientation: DeviceOrientation,
    network_connected: bool,
}

impl AppLifecycleManager {
    pub fn new() -> Self {
        Self {
            current_state: AppState::NotRunning,
            previous_state: AppState::NotRunning,
            state_history: Vec::new(),
            observers: Vec::new(),
            metrics: AppMetrics::default(),
            background_tasks: HashMap::new(),
            saved_state: HashMap::new(),
            is_low_memory: false,
            orientation: DeviceOrientation::Portrait,
            network_connected: true,
        }
    }
    
    pub fn add_observer(&mut self, observer: Box<dyn LifecycleObserver + Send>) {
        self.observers.push(observer);
    }
    
    pub fn transition_to_state(&mut self, new_state: AppState) {
        if self.current_state == new_state {
            return;
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        println!("🔄 State transition: {:?} -> {:?}", self.current_state, new_state);
        
        self.previous_state = self.current_state.clone();
        self.current_state = new_state.clone();
        self.state_history.push((new_state.clone(), timestamp));
        
        // Keep only last 50 state changes
        if self.state_history.len() > 50 {
            self.state_history.remove(0);
        }
        
        // Handle state-specific logic
        match new_state {
            AppState::Active => {
                self.handle_app_became_active();
            }
            AppState::Background => {
                self.handle_app_entered_background();
            }
            AppState::Suspended => {
                self.handle_app_suspended();
            }
            AppState::Terminated => {
                self.handle_app_terminated();
            }
            _ => {}
        }
    }
    
    fn handle_app_became_active(&mut self) {
        println!("✅ App became active");
        
        // Resume background tasks if needed
        for (name, task) in &mut self.background_tasks {
            if task.should_resume_on_foreground {
                task.resume();
                println!("   ▶️ Resumed background task: {}", name);
            }
        }
        
        // Restore saved state
        self.restore_app_state();
        
        // Notify observers
        for observer in &mut self.observers {
            observer.on_app_became_active();
        }
    }
    
    fn handle_app_entered_background(&mut self) {
        println!("🔙 App entered background");
        
        // Save current state
        self.save_app_state();
        
        // Pause non-essential background tasks
        for (name, task) in &mut self.background_tasks {
            if !task.continue_in_background {
                task.pause();
                println!("   ⏸️ Paused background task: {}", name);
            }
        }
        
        // Clean up resources
        self.cleanup_resources();
        
        // Notify observers
        for observer in &mut self.observers {
            observer.on_app_did_enter_background();
        }
    }
    
    fn handle_app_suspended(&mut self) {
        println!("⏸️ App suspended");
        
        // Stop all background tasks
        for (name, task) in &mut self.background_tasks {
            task.pause();
            println!("   ⏹️ Stopped background task: {}", name);
        }
        
        // Save critical data
        self.save_critical_data();
    }
    
    fn handle_app_terminated(&mut self) {
        println!("🛑 App terminated");
        
        // Final cleanup
        self.final_cleanup();
        
        // Notify observers
        for observer in &mut self.observers {
            observer.on_app_will_terminate();
        }
    }
    
    pub fn handle_memory_warning(&mut self) {
        println!("⚠️ Memory warning received");
        self.is_low_memory = true;
        
        // Free up memory
        self.free_memory();
        
        // Notify observers
        for observer in &mut self.observers {
            observer.on_memory_warning();
        }
    }
    
    pub fn handle_orientation_change(&mut self, new_orientation: DeviceOrientation) {
        if self.orientation != new_orientation {
            println!("🔄 Orientation changed: {:?} -> {:?}", self.orientation, new_orientation);
            self.orientation = new_orientation;
            
            // Notify observers
            for observer in &mut self.observers {
                observer.on_orientation_changed();
            }
        }
    }
    
    pub fn handle_network_status_change(&mut self, connected: bool) {
        if self.network_connected != connected {
            println!("🌐 Network status changed: {}", if connected { "Connected" } else { "Disconnected" });
            self.network_connected = connected;
            
            // Handle network-dependent tasks
            for (name, task) in &mut self.background_tasks {
                if task.requires_network {
                    if connected {
                        task.resume();
                        println!("   ▶️ Resumed network task: {}", name);
                    } else {
                        task.pause();
                        println!("   ⏸️ Paused network task: {}", name);
                    }
                }
            }
            
            // Notify observers
            for observer in &mut self.observers {
                observer.on_network_status_changed(connected);
            }
        }
    }
    
    pub fn add_background_task(&mut self, name: String, task: BackgroundTask) {
        println!("➕ Added background task: {}", name);
        self.background_tasks.insert(name, task);
    }
    
    pub fn remove_background_task(&mut self, name: &str) {
        if let Some(mut task) = self.background_tasks.remove(name) {
            task.stop();
            println!("➖ Removed background task: {}", name);
        }
    }
    
    fn save_app_state(&mut self) {
        println!("💾 Saving app state...");
        
        // Save current state data
        self.saved_state.insert("last_screen".to_string(), "main_screen".to_string());
        self.saved_state.insert("user_input".to_string(), "some_user_data".to_string());
        self.saved_state.insert("scroll_position".to_string(), "150".to_string());
        
        println!("   ✅ Saved {} state items", self.saved_state.len());
    }
    
    fn restore_app_state(&mut self) {
        if !self.saved_state.is_empty() {
            println!("📂 Restoring app state...");
            
            for (key, value) in &self.saved_state {
                println!("   🔄 Restored {}: {}", key, value);
            }
            
            println!("   ✅ Restored {} state items", self.saved_state.len());
        }
    }
    
    fn save_critical_data(&self) {
        println!("🔒 Saving critical data...");
        println!("   💾 User preferences saved");
        println!("   💾 Unsaved changes saved");
        println!("   💾 Session data saved");
    }
    
    fn cleanup_resources(&mut self) {
        println!("🧹 Cleaning up resources...");
        println!("   🗑️ Cleared image cache");
        println!("   🗑️ Released unused memory");
        println!("   🗑️ Closed unnecessary connections");
    }
    
    fn free_memory(&mut self) {
        println!("🧠 Freeing memory...");
        
        // Clear caches
        println!("   🗑️ Cleared image cache");
        println!("   🗑️ Cleared data cache");
        
        // Stop non-essential tasks
        let mut tasks_to_pause = Vec::new();
        for (name, task) in &self.background_tasks {
            if !task.is_essential {
                tasks_to_pause.push(name.clone());
            }
        }
        
        for name in tasks_to_pause {
            if let Some(task) = self.background_tasks.get_mut(&name) {
                task.pause();
                println!("   ⏸️ Paused non-essential task: {}", name);
            }
        }
    }
    
    fn final_cleanup(&mut self) {
        println!("🏁 Final cleanup...");
        
        // Stop all tasks
        for (name, task) in &mut self.background_tasks {
            task.stop();
            println!("   🛑 Stopped task: {}", name);
        }
        self.background_tasks.clear();
        
        // Clear all state
        self.saved_state.clear();
        
        // Update metrics
        self.update_metrics();
        
        println!("   ✅ Final cleanup completed");
    }
    
    fn update_metrics(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Simulate metrics update
        self.metrics.memory_usage = 150 * 1024 * 1024; // 150 MB
        self.metrics.cpu_usage = 15.5; // 15.5%
        self.metrics.battery_usage = 2.3; // 2.3%
        self.metrics.network_usage = 50 * 1024 * 1024; // 50 MB
        
        if self.current_state == AppState::Active {
            self.metrics.active_time += Duration::from_secs(current_time - self.metrics.launch_time);
        } else if self.current_state == AppState::Background {
            self.metrics.background_time += Duration::from_secs(current_time - self.metrics.launch_time);
        }
    }
    
    pub fn get_current_state(&self) -> &AppState {
        &self.current_state
    }
    
    pub fn get_metrics(&self) -> &AppMetrics {
        &self.metrics
    }
    
    pub fn get_state_history(&self) -> &Vec<(AppState, u64)> {
        &self.state_history
    }
    
    pub fn is_in_background(&self) -> bool {
        matches!(self.current_state, AppState::Background | AppState::Suspended)
    }
    
    pub fn is_low_memory(&self) -> bool {
        self.is_low_memory
    }
    
    pub fn get_orientation(&self) -> &DeviceOrientation {
        &self.orientation
    }
    
    pub fn is_network_connected(&self) -> bool {
        self.network_connected
    }
}

/// 📱 Device Orientation
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceOrientation {
    Portrait,
    PortraitUpsideDown,
    LandscapeLeft,
    LandscapeRight,
    FaceUp,
    FaceDown,
    Unknown,
}

/// ⚙️ Background Task
#[derive(Debug, Clone)]
pub struct BackgroundTask {
    pub name: String,
    pub is_running: bool,
    pub is_essential: bool,
    pub continue_in_background: bool,
    pub should_resume_on_foreground: bool,
    pub requires_network: bool,
    pub max_execution_time: Duration,
    pub created_at: u64,
    pub last_executed: Option<u64>,
}

impl BackgroundTask {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_running: false,
            is_essential: false,
            continue_in_background: false,
            should_resume_on_foreground: true,
            requires_network: false,
            max_execution_time: Duration::from_secs(30),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            last_executed: None,
        }
    }
    
    pub fn essential(mut self) -> Self {
        self.is_essential = true;
        self
    }
    
    pub fn continue_in_background(mut self) -> Self {
        self.continue_in_background = true;
        self
    }
    
    pub fn requires_network(mut self) -> Self {
        self.requires_network = true;
        self
    }
    
    pub fn max_execution_time(mut self, duration: Duration) -> Self {
        self.max_execution_time = duration;
        self
    }
    
    pub fn start(&mut self) {
        if !self.is_running {
            self.is_running = true;
            self.last_executed = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            );
            println!("▶️ Started background task: {}", self.name);
        }
    }
    
    pub fn pause(&mut self) {
        if self.is_running {
            self.is_running = false;
            println!("⏸️ Paused background task: {}", self.name);
        }
    }
    
    pub fn resume(&mut self) {
        if !self.is_running {
            self.is_running = true;
            self.last_executed = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            );
            println!("▶️ Resumed background task: {}", self.name);
        }
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        println!("🛑 Stopped background task: {}", self.name);
    }
}

/// 📊 Example Lifecycle Observer
#[derive(Debug)]
pub struct ExampleLifecycleObserver {
    name: String,
    event_count: u32,
}

impl ExampleLifecycleObserver {
    pub fn new(name: String) -> Self {
        Self {
            name,
            event_count: 0,
        }
    }
}

impl LifecycleObserver for ExampleLifecycleObserver {
    fn on_app_launched(&mut self) {
        self.event_count += 1;
        println!("🚀 [{}] App launched (event #{})", self.name, self.event_count);
    }
    
    fn on_app_became_active(&mut self) {
        self.event_count += 1;
        println!("✅ [{}] App became active (event #{})", self.name, self.event_count);
    }
    
    fn on_app_will_resign_active(&mut self) {
        self.event_count += 1;
        println!("⚠️ [{}] App will resign active (event #{})", self.name, self.event_count);
    }
    
    fn on_app_did_enter_background(&mut self) {
        self.event_count += 1;
        println!("🔙 [{}] App entered background (event #{})", self.name, self.event_count);
    }
    
    fn on_app_will_enter_foreground(&mut self) {
        self.event_count += 1;
        println!("🔜 [{}] App will enter foreground (event #{})", self.name, self.event_count);
    }
    
    fn on_app_will_terminate(&mut self) {
        self.event_count += 1;
        println!("🛑 [{}] App will terminate (event #{})", self.name, self.event_count);
    }
    
    fn on_memory_warning(&mut self) {
        self.event_count += 1;
        println!("⚠️ [{}] Memory warning (event #{})", self.name, self.event_count);
    }
    
    fn on_orientation_changed(&mut self) {
        self.event_count += 1;
        println!("🔄 [{}] Orientation changed (event #{})", self.name, self.event_count);
    }
    
    fn on_network_status_changed(&mut self, connected: bool) {
        self.event_count += 1;
        println!("🌐 [{}] Network {} (event #{})", 
                self.name, 
                if connected { "connected" } else { "disconnected" }, 
                self.event_count);
    }
}

/// 💾 State Persistence Manager
#[derive(Debug)]
pub struct StatePersistenceManager {
    storage_path: String,
    auto_save_interval: Duration,
    last_save_time: u64,
    pending_changes: HashMap<String, String>,
}

impl StatePersistenceManager {
    pub fn new(storage_path: String) -> Self {
        Self {
            storage_path,
            auto_save_interval: Duration::from_secs(30),
            last_save_time: 0,
            pending_changes: HashMap::new(),
        }
    }
    
    pub fn save_state(&mut self, key: String, value: String) {
        self.pending_changes.insert(key.clone(), value);
        println!("📝 Queued state save: {}", key);
        
        // Auto-save if interval has passed
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        if current_time - self.last_save_time > self.auto_save_interval.as_secs() {
            self.flush_to_disk();
        }
    }
    
    pub fn load_state(&self, key: &str) -> Option<String> {
        // In a real implementation, this would read from persistent storage
        println!("📂 Loading state: {}", key);
        Some("loaded_value".to_string())
    }
    
    pub fn flush_to_disk(&mut self) {
        if !self.pending_changes.is_empty() {
            println!("💾 Flushing {} changes to disk: {}", self.pending_changes.len(), self.storage_path);
            
            for (key, value) in &self.pending_changes {
                println!("   💾 Saved {}: {} chars", key, value.len());
            }
            
            self.pending_changes.clear();
            self.last_save_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
    }
    
    pub fn clear_all_state(&mut self) {
        println!("🗑️ Clearing all saved state");
        self.pending_changes.clear();
    }
}

/// 🔄 สาธิตการใช้งาน App Lifecycle Management
pub fn demonstrate_app_lifecycle() {
    println!("🔄 === App Lifecycle Management Demo ===");
    
    // สร้าง Lifecycle Manager
    let mut lifecycle_manager = AppLifecycleManager::new();
    
    // เพิ่ม Observer
    let observer1 = ExampleLifecycleObserver::new("UI Manager".to_string());
    let observer2 = ExampleLifecycleObserver::new("Data Manager".to_string());
    
    lifecycle_manager.add_observer(Box::new(observer1));
    lifecycle_manager.add_observer(Box::new(observer2));
    
    // สร้าง Background Tasks
    println!("\n⚙️ Setting up background tasks:");
    
    let sync_task = BackgroundTask::new("Data Sync".to_string())
        .continue_in_background()
        .requires_network()
        .max_execution_time(Duration::from_secs(60));
    
    let analytics_task = BackgroundTask::new("Analytics".to_string())
        .essential()
        .continue_in_background();
    
    let image_processing_task = BackgroundTask::new("Image Processing".to_string())
        .max_execution_time(Duration::from_secs(30));
    
    lifecycle_manager.add_background_task("sync".to_string(), sync_task);
    lifecycle_manager.add_background_task("analytics".to_string(), analytics_task);
    lifecycle_manager.add_background_task("image_processing".to_string(), image_processing_task);
    
    // จำลอง App Lifecycle Events
    println!("\n🔄 Simulating app lifecycle events:");
    
    // App Launch
    lifecycle_manager.transition_to_state(AppState::Active);
    std::thread::sleep(Duration::from_millis(100));
    
    // App goes to background
    lifecycle_manager.transition_to_state(AppState::Background);
    std::thread::sleep(Duration::from_millis(100));
    
    // Memory warning
    lifecycle_manager.handle_memory_warning();
    std::thread::sleep(Duration::from_millis(100));
    
    // Network disconnection
    lifecycle_manager.handle_network_status_change(false);
    std::thread::sleep(Duration::from_millis(100));
    
    // Network reconnection
    lifecycle_manager.handle_network_status_change(true);
    std::thread::sleep(Duration::from_millis(100));
    
    // Orientation change
    lifecycle_manager.handle_orientation_change(DeviceOrientation::LandscapeLeft);
    std::thread::sleep(Duration::from_millis(100));
    
    // App returns to foreground
    lifecycle_manager.transition_to_state(AppState::Active);
    std::thread::sleep(Duration::from_millis(100));
    
    // App suspended
    lifecycle_manager.transition_to_state(AppState::Suspended);
    std::thread::sleep(Duration::from_millis(100));
    
    // App terminated
    lifecycle_manager.transition_to_state(AppState::Terminated);
    
    // แสดงผลสถิติ
    println!("\n📊 App Lifecycle Statistics:");
    let metrics = lifecycle_manager.get_metrics();
    println!("   🚀 Launch time: {}", metrics.launch_time);
    println!("   ⏱️ Active time: {:?}", metrics.active_time);
    println!("   🔙 Background time: {:?}", metrics.background_time);
    println!("   🧠 Memory usage: {:.1} MB", metrics.memory_usage as f64 / 1024.0 / 1024.0);
    println!("   ⚡ CPU usage: {:.1}%", metrics.cpu_usage);
    println!("   🔋 Battery usage: {:.1}%", metrics.battery_usage);
    println!("   🌐 Network usage: {:.1} MB", metrics.network_usage as f64 / 1024.0 / 1024.0);
    println!("   💥 Crash count: {}", metrics.crash_count);
    println!("   📱 Session count: {}", metrics.session_count);
    
    // แสดงประวัติการเปลี่ยนสถานะ
    println!("\n📜 State History:");
    for (state, timestamp) in lifecycle_manager.get_state_history() {
        println!("   🔄 {:?} at {}", state, timestamp);
    }
    
    // State Persistence
    println!("\n💾 State Persistence Demo:");
    demonstrate_state_persistence();
    
    // Best Practices
    println!("\n💡 App Lifecycle Best Practices:");
    show_app_lifecycle_best_practices();
}

/// 💾 สาธิต State Persistence
fn demonstrate_state_persistence() {
    let mut persistence_manager = StatePersistenceManager::new("/app/state".to_string());
    
    // บันทึกข้อมูลสถานะ
    persistence_manager.save_state("user_preferences".to_string(), "{\"theme\": \"dark\", \"language\": \"th\"}".to_string());
    persistence_manager.save_state("current_screen".to_string(), "profile_screen".to_string());
    persistence_manager.save_state("form_data".to_string(), "{\"name\": \"John\", \"email\": \"john@example.com\"}".to_string());
    
    // บังคับบันทึกลงดิสก์
    persistence_manager.flush_to_disk();
    
    // โหลดข้อมูลสถานะ
    if let Some(preferences) = persistence_manager.load_state("user_preferences") {
        println!("   📂 Loaded preferences: {}", preferences);
    }
    
    if let Some(screen) = persistence_manager.load_state("current_screen") {
        println!("   📂 Loaded current screen: {}", screen);
    }
}

/// 💡 App Lifecycle Best Practices
fn show_app_lifecycle_best_practices() {
    let practices = vec![
        "🔄 Handle all lifecycle states properly",
        "💾 Save critical data when app goes to background",
        "🧹 Clean up resources to free memory",
        "⏸️ Pause non-essential tasks in background",
        "🔋 Minimize battery usage in background",
        "🌐 Handle network connectivity changes",
        "📱 Support orientation changes gracefully",
        "⚠️ Respond to memory warnings immediately",
        "🔒 Secure sensitive data when app is backgrounded",
        "📊 Track app usage metrics for optimization",
        "🚀 Optimize app launch time",
        "💤 Use background app refresh wisely",
        "🔄 Implement proper state restoration",
        "🧪 Test lifecycle scenarios thoroughly",
        "📋 Follow platform-specific guidelines",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   🛠️ Platform-specific Considerations:");
    println!("      🍎 iOS:");
    println!("         • Use UIApplicationDelegate methods");
    println!("         • Implement State Restoration");
    println!("         • Handle background app refresh");
    println!("         • Use NSUserDefaults for preferences");
    println!("         • Implement proper memory management");
    
    println!("\n      🤖 Android:");
    println!("         • Override Activity lifecycle methods");
    println!("         • Use onSaveInstanceState/onRestoreInstanceState");
    println!("         • Handle configuration changes");
    println!("         • Use SharedPreferences for settings");
    println!("         • Implement proper memory cleanup");
    
    println!("\n   📚 Recommended Patterns:");
    println!("      • Observer Pattern for lifecycle events");
    println!("      • State Machine for app states");
    println!("      • Repository Pattern for data persistence");
    println!("      • Dependency Injection for lifecycle-aware components");
    println!("      • Command Pattern for background tasks");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_lifecycle_manager() {
        let mut manager = AppLifecycleManager::new();
        assert_eq!(manager.get_current_state(), &AppState::NotRunning);
        
        manager.transition_to_state(AppState::Active);
        assert_eq!(manager.get_current_state(), &AppState::Active);
        assert!(!manager.is_in_background());
        
        manager.transition_to_state(AppState::Background);
        assert_eq!(manager.get_current_state(), &AppState::Background);
        assert!(manager.is_in_background());
    }
    
    #[test]
    fn test_background_task() {
        let mut task = BackgroundTask::new("test_task".to_string())
            .essential()
            .continue_in_background()
            .requires_network();
        
        assert!(!task.is_running);
        assert!(task.is_essential);
        assert!(task.continue_in_background);
        assert!(task.requires_network);
        
        task.start();
        assert!(task.is_running);
        
        task.pause();
        assert!(!task.is_running);
        
        task.resume();
        assert!(task.is_running);
        
        task.stop();
        assert!(!task.is_running);
    }
    
    #[test]
    fn test_state_persistence_manager() {
        let mut manager = StatePersistenceManager::new("/test/path".to_string());
        
        // Set last_save_time to current time to prevent auto-flush
        manager.last_save_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        manager.save_state("key1".to_string(), "value1".to_string());
        manager.save_state("key2".to_string(), "value2".to_string());
        
        assert_eq!(manager.pending_changes.len(), 2);
        
        manager.flush_to_disk();
        assert_eq!(manager.pending_changes.len(), 0);
    }
    
    #[test]
    fn test_app_metrics() {
        let metrics = AppMetrics::default();
        assert_eq!(metrics.crash_count, 0);
        assert_eq!(metrics.session_count, 1);
        assert!(metrics.launch_time > 0);
    }
    
    #[test]
    fn test_device_orientation() {
        let mut manager = AppLifecycleManager::new();
        assert_eq!(manager.get_orientation(), &DeviceOrientation::Portrait);
        
        manager.handle_orientation_change(DeviceOrientation::LandscapeLeft);
        assert_eq!(manager.get_orientation(), &DeviceOrientation::LandscapeLeft);
    }
    
    #[test]
    fn test_memory_warning() {
        let mut manager = AppLifecycleManager::new();
        assert!(!manager.is_low_memory());
        
        manager.handle_memory_warning();
        assert!(manager.is_low_memory());
    }
    
    #[test]
    fn test_network_status() {
        let mut manager = AppLifecycleManager::new();
        assert!(manager.is_network_connected());
        
        manager.handle_network_status_change(false);
        assert!(!manager.is_network_connected());
        
        manager.handle_network_status_change(true);
        assert!(manager.is_network_connected());
    }
}