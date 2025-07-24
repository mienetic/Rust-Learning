//! 🎯 Game Engine Basics - พื้นฐานของ Game Engine
//! 
//! โมดูลนี้สาธิตการสร้าง Game Engine พื้นฐานด้วย Rust
//! รวมถึง Game Loop, Time Management, และ Resource Management

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;

/// 🎮 Game Engine Core
pub struct GameEngine {
    pub name: String,
    pub is_running: bool,
    pub target_fps: u32,
    pub frame_time: Duration,
    pub current_time: Instant,
    pub last_frame_time: Instant,
    pub delta_time: Duration,
    pub frame_count: u64,
    pub systems: Vec<Box<dyn GameSystem>>,
    pub resources: ResourceManager,
}

impl GameEngine {
    /// สร้าง GameEngine ใหม่
    pub fn new(name: &str, target_fps: u32) -> Self {
        let now = Instant::now();
        
        Self {
            name: name.to_string(),
            is_running: false,
            target_fps,
            frame_time: Duration::from_nanos(1_000_000_000 / target_fps as u64),
            current_time: now,
            last_frame_time: now,
            delta_time: Duration::from_millis(0),
            frame_count: 0,
            systems: Vec::new(),
            resources: ResourceManager::new(),
        }
    }
    
    /// เพิ่ม system เข้าไปใน engine
    pub fn add_system(&mut self, system: Box<dyn GameSystem>) {
        self.systems.push(system);
    }
    
    /// เริ่มต้น engine
    pub fn start(&mut self) {
        println!("🚀 Starting game engine: {}", self.name);
        println!("🎯 Target FPS: {}", self.target_fps);
        
        self.is_running = true;
        self.current_time = Instant::now();
        self.last_frame_time = self.current_time;
        
        // Initialize systems
        for system in &mut self.systems {
            system.initialize();
        }
        
        self.run_game_loop();
    }
    
    /// หยุด engine
    pub fn stop(&mut self) {
        println!("🛑 Stopping game engine: {}", self.name);
        self.is_running = false;
        
        // Cleanup systems
        for system in &mut self.systems {
            system.cleanup();
        }
    }
    
    /// Game Loop หลัก
    fn run_game_loop(&mut self) {
        println!("🔄 Starting game loop...");
        
        while self.is_running {
            let frame_start = Instant::now();
            
            // คำนวณ delta time
            self.current_time = frame_start;
            self.delta_time = self.current_time.duration_since(self.last_frame_time);
            self.last_frame_time = self.current_time;
            
            // Update systems
            self.update();
            
            // Render
            self.render();
            
            // Frame timing
            let frame_duration = frame_start.elapsed();
            if frame_duration < self.frame_time {
                let sleep_time = self.frame_time - frame_duration;
                thread::sleep(sleep_time);
            }
            
            self.frame_count += 1;
            
            // จำลองการหยุดหลังจาก 100 frames
            if self.frame_count >= 100 {
                self.is_running = false;
            }
        }
        
        println!("✅ Game loop finished. Total frames: {}", self.frame_count);
    }
    
    /// อัปเดต game logic
    fn update(&mut self) {
        for system in &mut self.systems {
            system.update(self.delta_time);
        }
    }
    
    /// Render graphics
    fn render(&mut self) {
        for system in &mut self.systems {
            system.render();
        }
    }
    
    /// ดูสถิติ engine
    pub fn get_stats(&self) -> EngineStats {
        let avg_frame_time = if self.frame_count > 0 {
            self.current_time.duration_since(Instant::now() - Duration::from_secs(1)) / self.frame_count as u32
        } else {
            Duration::from_millis(0)
        };
        
        EngineStats {
            frame_count: self.frame_count,
            target_fps: self.target_fps,
            actual_fps: if avg_frame_time.as_millis() > 0 {
                1000 / avg_frame_time.as_millis() as u32
            } else {
                0
            },
            delta_time: self.delta_time,
            systems_count: self.systems.len(),
        }
    }
}

/// 📊 Engine Statistics
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub frame_count: u64,
    pub target_fps: u32,
    pub actual_fps: u32,
    pub delta_time: Duration,
    pub systems_count: usize,
}

/// 🔧 Game System Trait
pub trait GameSystem {
    fn initialize(&mut self);
    fn update(&mut self, delta_time: Duration);
    fn render(&mut self);
    fn cleanup(&mut self);
    fn name(&self) -> &str;
}

/// 🎮 Input System
#[derive(Debug)]
pub struct InputSystem {
    pub name: String,
    pub keys_pressed: HashMap<String, bool>,
    pub mouse_position: (f32, f32),
    pub mouse_buttons: HashMap<String, bool>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            name: "Input System".to_string(),
            keys_pressed: HashMap::new(),
            mouse_position: (0.0, 0.0),
            mouse_buttons: HashMap::new(),
        }
    }
    
    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.keys_pressed.get(key).copied().unwrap_or(false)
    }
    
    pub fn set_key_state(&mut self, key: &str, pressed: bool) {
        self.keys_pressed.insert(key.to_string(), pressed);
    }
    
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSystem for InputSystem {
    fn initialize(&mut self) {
        println!("🎹 Initializing Input System");
    }
    
    fn update(&mut self, _delta_time: Duration) {
        // จำลองการรับ input
        if self.frame_count() % 30 == 0 {
            self.set_key_state("SPACE", !self.is_key_pressed("SPACE"));
        }
    }
    
    fn render(&mut self) {
        // Input system ไม่ต้อง render
    }
    
    fn cleanup(&mut self) {
        println!("🧹 Cleaning up Input System");
        self.keys_pressed.clear();
        self.mouse_buttons.clear();
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl InputSystem {
    fn frame_count(&self) -> u64 {
        // จำลอง frame count
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 / 16 // สมมติ 60 FPS
    }
}

/// 🎨 Render System
#[derive(Debug)]
pub struct RenderSystem {
    pub name: String,
    pub render_calls: u32,
    pub vertices_rendered: u32,
    pub textures_loaded: u32,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            name: "Render System".to_string(),
            render_calls: 0,
            vertices_rendered: 0,
            textures_loaded: 0,
        }
    }
    
    pub fn draw_sprite(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.render_calls += 1;
        self.vertices_rendered += 6; // 2 triangles = 6 vertices
        
        // จำลองการ render sprite
        if self.render_calls % 20 == 0 {
            println!("🎨 Rendering sprite at ({:.1}, {:.1}) size {:.1}x{:.1}", x, y, width, height);
        }
    }
    
    pub fn clear_screen(&mut self) {
        // จำลองการล้างหน้าจอ
    }
    
    pub fn present(&mut self) {
        // จำลองการแสดงผล
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSystem for RenderSystem {
    fn initialize(&mut self) {
        println!("🎨 Initializing Render System");
        self.textures_loaded = 5; // จำลองการโหลด textures
    }
    
    fn update(&mut self, _delta_time: Duration) {
        // Render system อัปเดตใน render phase
    }
    
    fn render(&mut self) {
        self.clear_screen();
        
        // จำลองการ render objects
        self.draw_sprite(100.0, 100.0, 64.0, 64.0);
        self.draw_sprite(200.0, 150.0, 32.0, 32.0);
        
        self.present();
    }
    
    fn cleanup(&mut self) {
        println!("🧹 Cleaning up Render System");
        println!("   📊 Total render calls: {}", self.render_calls);
        println!("   📊 Total vertices rendered: {}", self.vertices_rendered);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// 🔊 Audio System
#[derive(Debug)]
pub struct AudioSystem {
    pub name: String,
    pub sounds_playing: u32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            name: "Audio System".to_string(),
            sounds_playing: 0,
            music_volume: 0.8,
            sfx_volume: 1.0,
        }
    }
    
    pub fn play_sound(&mut self, sound_name: &str) {
        self.sounds_playing += 1;
        println!("🔊 Playing sound: {}", sound_name);
    }
    
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSystem for AudioSystem {
    fn initialize(&mut self) {
        println!("🔊 Initializing Audio System");
        println!("   🎵 Music volume: {:.1}", self.music_volume);
        println!("   🔊 SFX volume: {:.1}", self.sfx_volume);
    }
    
    fn update(&mut self, _delta_time: Duration) {
        // จำลองการเล่นเสียง
        if self.sounds_playing > 0 && self.sounds_playing % 50 == 0 {
            self.play_sound("background_music");
        }
    }
    
    fn render(&mut self) {
        // Audio system ไม่ต้อง render
    }
    
    fn cleanup(&mut self) {
        println!("🧹 Cleaning up Audio System");
        println!("   📊 Total sounds played: {}", self.sounds_playing);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// 📦 Resource Manager
#[derive(Debug)]
pub struct ResourceManager {
    pub textures: HashMap<String, TextureResource>,
    pub sounds: HashMap<String, SoundResource>,
    pub fonts: HashMap<String, FontResource>,
    pub total_memory_used: u64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
            total_memory_used: 0,
        }
    }
    
    pub fn load_texture(&mut self, name: &str, path: &str) -> Result<(), String> {
        let texture = TextureResource {
            name: name.to_string(),
            path: path.to_string(),
            width: 256,
            height: 256,
            memory_size: 256 * 256 * 4, // RGBA
        };
        
        self.total_memory_used += texture.memory_size;
        self.textures.insert(name.to_string(), texture);
        
        println!("🖼️ Loaded texture: {} from {}", name, path);
        Ok(())
    }
    
    pub fn load_sound(&mut self, name: &str, path: &str) -> Result<(), String> {
        let sound = SoundResource {
            name: name.to_string(),
            path: path.to_string(),
            duration: Duration::from_secs(3),
            memory_size: 1024 * 1024, // 1MB
        };
        
        self.total_memory_used += sound.memory_size;
        self.sounds.insert(name.to_string(), sound);
        
        println!("🔊 Loaded sound: {} from {}", name, path);
        Ok(())
    }
    
    pub fn load_font(&mut self, name: &str, path: &str) -> Result<(), String> {
        let font = FontResource {
            name: name.to_string(),
            path: path.to_string(),
            size: 16,
            memory_size: 512 * 1024, // 512KB
        };
        
        self.total_memory_used += font.memory_size;
        self.fonts.insert(name.to_string(), font);
        
        println!("🔤 Loaded font: {} from {}", name, path);
        Ok(())
    }
    
    pub fn get_texture(&self, name: &str) -> Option<&TextureResource> {
        self.textures.get(name)
    }
    
    pub fn get_sound(&self, name: &str) -> Option<&SoundResource> {
        self.sounds.get(name)
    }
    
    pub fn get_font(&self, name: &str) -> Option<&FontResource> {
        self.fonts.get(name)
    }
    
    pub fn unload_all(&mut self) {
        println!("🧹 Unloading all resources...");
        println!("   📊 Total memory freed: {} bytes", self.total_memory_used);
        
        self.textures.clear();
        self.sounds.clear();
        self.fonts.clear();
        self.total_memory_used = 0;
    }
    
    pub fn get_memory_usage(&self) -> u64 {
        self.total_memory_used
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 🖼️ Texture Resource
#[derive(Debug, Clone)]
pub struct TextureResource {
    pub name: String,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub memory_size: u64,
}

/// 🔊 Sound Resource
#[derive(Debug, Clone)]
pub struct SoundResource {
    pub name: String,
    pub path: String,
    pub duration: Duration,
    pub memory_size: u64,
}

/// 🔤 Font Resource
#[derive(Debug, Clone)]
pub struct FontResource {
    pub name: String,
    pub path: String,
    pub size: u32,
    pub memory_size: u64,
}

/// ⏱️ Time Manager
#[derive(Debug)]
pub struct TimeManager {
    pub start_time: Instant,
    pub current_time: Instant,
    pub delta_time: Duration,
    pub time_scale: f32,
    pub total_time: Duration,
    pub frame_count: u64,
}

impl TimeManager {
    pub fn new() -> Self {
        let now = Instant::now();
        
        Self {
            start_time: now,
            current_time: now,
            delta_time: Duration::from_millis(0),
            time_scale: 1.0,
            total_time: Duration::from_millis(0),
            frame_count: 0,
        }
    }
    
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.current_time);
        self.current_time = now;
        self.total_time = now.duration_since(self.start_time);
        self.frame_count += 1;
    }
    
    pub fn get_delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32() * self.time_scale
    }
    
    pub fn get_total_seconds(&self) -> f32 {
        self.total_time.as_secs_f32()
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }
    
    pub fn pause(&mut self) {
        self.time_scale = 0.0;
    }
    
    pub fn resume(&mut self) {
        self.time_scale = 1.0;
    }
    
    pub fn get_fps(&self) -> f32 {
        if self.delta_time.as_millis() > 0 {
            1000.0 / self.delta_time.as_millis() as f32
        } else {
            0.0
        }
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 🎯 สาธิตการทำงานของ Game Engine
pub fn demonstrate_game_engine_basics() {
    println!("\n🎯 === Game Engine Basics Demo ===");
    
    // 1. สร้าง Game Engine
    println!("\n1️⃣ Creating Game Engine:");
    demonstrate_engine_creation();
    
    // 2. Resource Management
    println!("\n2️⃣ Resource Management:");
    demonstrate_resource_management();
    
    // 3. Time Management
    println!("\n3️⃣ Time Management:");
    demonstrate_time_management();
    
    // 4. Game Systems
    println!("\n4️⃣ Game Systems:");
    demonstrate_game_systems();
    
    // 5. Engine Best Practices
    println!("\n5️⃣ Engine Best Practices:");
    show_engine_best_practices();
    
    println!("\n✅ จบการสาธิต Game Engine Basics!");
}

/// 🎮 สาธิตการสร้าง Engine
fn demonstrate_engine_creation() {
    println!("🎮 การสร้าง Game Engine:");
    
    let mut engine = GameEngine::new("Demo Engine", 60);
    
    // เพิ่ม systems
    engine.add_system(Box::new(InputSystem::new()));
    engine.add_system(Box::new(RenderSystem::new()));
    engine.add_system(Box::new(AudioSystem::new()));
    
    println!("✅ Engine created with {} systems", engine.systems.len());
    
    // แสดงข้อมูล engine
    let stats = engine.get_stats();
    println!("📊 Engine Stats:");
    println!("   • Target FPS: {}", stats.target_fps);
    println!("   • Systems: {}", stats.systems_count);
    println!("   • Frame Time: {:?}", engine.frame_time);
    
    // เริ่มต้น engine (จำลอง)
    println!("\n🚀 Starting engine (simulated)...");
    engine.start();
}

/// 📦 สาธิต Resource Management
fn demonstrate_resource_management() {
    println!("📦 การจัดการ Resources:");
    
    let mut resource_manager = ResourceManager::new();
    
    // โหลด resources
    resource_manager.load_texture("player", "assets/player.png").unwrap();
    resource_manager.load_texture("enemy", "assets/enemy.png").unwrap();
    resource_manager.load_sound("jump", "assets/jump.wav").unwrap();
    resource_manager.load_sound("shoot", "assets/shoot.wav").unwrap();
    resource_manager.load_font("ui_font", "assets/font.ttf").unwrap();
    
    println!("\n📊 Resource Stats:");
    println!("   • Textures: {}", resource_manager.textures.len());
    println!("   • Sounds: {}", resource_manager.sounds.len());
    println!("   • Fonts: {}", resource_manager.fonts.len());
    println!("   • Memory Used: {} bytes ({:.2} MB)", 
        resource_manager.get_memory_usage(),
        resource_manager.get_memory_usage() as f64 / 1_000_000.0
    );
    
    // ใช้งาน resources
    if let Some(texture) = resource_manager.get_texture("player") {
        println!("\n🖼️ Player texture: {}x{} pixels", texture.width, texture.height);
    }
    
    if let Some(sound) = resource_manager.get_sound("jump") {
        println!("🔊 Jump sound duration: {:?}", sound.duration);
    }
    
    // ล้าง resources
    resource_manager.unload_all();
}

/// ⏱️ สาธิต Time Management
fn demonstrate_time_management() {
    println!("⏱️ การจัดการเวลา:");
    
    let mut time_manager = TimeManager::new();
    
    println!("🕐 Starting time manager...");
    
    // จำลองการทำงานหลาย frames
    for frame in 1..=5 {
        thread::sleep(Duration::from_millis(16)); // จำลอง 60 FPS
        time_manager.update();
        
        println!("   Frame {}: Delta: {:.3}s, Total: {:.3}s, FPS: {:.1}",
            frame,
            time_manager.get_delta_seconds(),
            time_manager.get_total_seconds(),
            time_manager.get_fps()
        );
    }
    
    // ทดสอบ time scale
    println!("\n⏸️ Testing time scale:");
    time_manager.set_time_scale(0.5); // ช้าลง 50%
    println!("   Time scale set to 0.5 (slow motion)");
    
    time_manager.pause();
    println!("   Game paused (time scale = 0.0)");
    
    time_manager.resume();
    println!("   Game resumed (time scale = 1.0)");
}

/// 🔧 สาธิต Game Systems
fn demonstrate_game_systems() {
    println!("🔧 การทำงานของ Game Systems:");
    
    // Input System
    let mut input_system = InputSystem::new();
    input_system.initialize();
    
    input_system.set_key_state("W", true);
    input_system.set_key_state("A", false);
    input_system.set_mouse_position(100.0, 200.0);
    
    println!("🎹 Input System:");
    println!("   • W key pressed: {}", input_system.is_key_pressed("W"));
    println!("   • A key pressed: {}", input_system.is_key_pressed("A"));
    println!("   • Mouse position: {:?}", input_system.mouse_position);
    
    // Render System
    let mut render_system = RenderSystem::new();
    render_system.initialize();
    
    println!("\n🎨 Render System:");
    render_system.draw_sprite(50.0, 50.0, 32.0, 32.0);
    render_system.draw_sprite(100.0, 100.0, 64.0, 64.0);
    
    println!("   • Render calls: {}", render_system.render_calls);
    println!("   • Vertices rendered: {}", render_system.vertices_rendered);
    println!("   • Textures loaded: {}", render_system.textures_loaded);
    
    // Audio System
    let mut audio_system = AudioSystem::new();
    audio_system.initialize();
    
    audio_system.play_sound("jump");
    audio_system.play_sound("coin");
    audio_system.set_music_volume(0.6);
    
    println!("\n🔊 Audio System:");
    println!("   • Sounds playing: {}", audio_system.sounds_playing);
    println!("   • Music volume: {:.1}", audio_system.music_volume);
    println!("   • SFX volume: {:.1}", audio_system.sfx_volume);
    
    // Cleanup
    input_system.cleanup();
    render_system.cleanup();
    audio_system.cleanup();
}

/// 📋 แสดง Engine Best Practices
fn show_engine_best_practices() {
    println!("📋 Game Engine Best Practices:");
    
    let practices = vec![
        ("🔄", "Fixed Timestep", "ใช้ fixed timestep สำหรับ physics"),
        ("🎯", "System Separation", "แยก systems ให้ชัดเจน"),
        ("📦", "Resource Pooling", "ใช้ object pooling สำหรับ temporary objects"),
        ("⚡", "Performance", "วัดและปรับปรุงประสิทธิภาพอย่างสม่ำเสมอ"),
        ("🧪", "Testing", "ทดสอบ game logic แยกจาก rendering"),
        ("📊", "Profiling", "ใช้ profiler หา bottlenecks"),
        ("🔧", "Modularity", "ออกแบบ systems ให้เป็น modular"),
        ("💾", "Memory Management", "จัดการ memory อย่างมีประสิทธิภาพ"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎮 Game Loop Best Practices:");
    println!("   • แยก Update และ Render phases");
    println!("   • ใช้ delta time สำหรับ frame-independent movement");
    println!("   • จำกัด frame rate เพื่อประหยัดพลังงาน");
    println!("   • Handle input ทุก frame");
    println!("   • Batch rendering calls");
    
    println!("\n🔧 System Design Patterns:");
    println!("   • Component Pattern - แยก data และ behavior");
    println!("   • Observer Pattern - สำหรับ events");
    println!("   • Command Pattern - สำหรับ input handling");
    println!("   • State Pattern - จัดการ game states");
    println!("   • Factory Pattern - สร้าง game objects");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_engine_creation() {
        let engine = GameEngine::new("Test Engine", 60);
        assert_eq!(engine.name, "Test Engine");
        assert_eq!(engine.target_fps, 60);
        assert!(!engine.is_running);
        assert_eq!(engine.frame_count, 0);
    }
    
    #[test]
    fn test_input_system() {
        let mut input_system = InputSystem::new();
        
        assert!(!input_system.is_key_pressed("SPACE"));
        
        input_system.set_key_state("SPACE", true);
        assert!(input_system.is_key_pressed("SPACE"));
        
        input_system.set_mouse_position(100.0, 200.0);
        assert_eq!(input_system.mouse_position, (100.0, 200.0));
    }
    
    #[test]
    fn test_render_system() {
        let mut render_system = RenderSystem::new();
        
        assert_eq!(render_system.render_calls, 0);
        assert_eq!(render_system.vertices_rendered, 0);
        
        render_system.draw_sprite(0.0, 0.0, 32.0, 32.0);
        
        assert_eq!(render_system.render_calls, 1);
        assert_eq!(render_system.vertices_rendered, 6);
    }
    
    #[test]
    fn test_resource_manager() {
        let mut resource_manager = ResourceManager::new();
        
        assert_eq!(resource_manager.get_memory_usage(), 0);
        
        resource_manager.load_texture("test", "test.png").unwrap();
        assert!(resource_manager.get_texture("test").is_some());
        assert!(resource_manager.get_memory_usage() > 0);
        
        resource_manager.unload_all();
        assert_eq!(resource_manager.get_memory_usage(), 0);
    }
    
    #[test]
    fn test_time_manager() {
        let mut time_manager = TimeManager::new();
        
        assert_eq!(time_manager.frame_count, 0);
        assert_eq!(time_manager.time_scale, 1.0);
        
        time_manager.update();
        assert_eq!(time_manager.frame_count, 1);
        
        time_manager.set_time_scale(0.5);
        assert_eq!(time_manager.time_scale, 0.5);
        
        time_manager.pause();
        assert_eq!(time_manager.time_scale, 0.0);
        
        time_manager.resume();
        assert_eq!(time_manager.time_scale, 1.0);
    }
}