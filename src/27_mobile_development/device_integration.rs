//! 📲 Device Integration
//! 
//! สาธิตการเชื่อมต่อกับฟีเจอร์ต่างๆ ของอุปกรณ์มือถือ
//! รวมถึง Camera, GPS, Sensors, Storage, และ Native APIs

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 📱 Device Platform
#[derive(Debug, Clone, PartialEq)]
pub enum DevicePlatform {
    iOS,
    Android,
    Unknown,
}

/// 📍 Location Data
#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: f64,
    pub timestamp: u64,
    pub speed: Option<f64>,
    pub bearing: Option<f64>,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude: None,
            accuracy: 10.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            speed: None,
            bearing: None,
        }
    }
    
    pub fn distance_to(&self, other: &Location) -> f64 {
        // Haversine formula for calculating distance between two points
        let r = 6371.0; // Earth's radius in kilometers
        
        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) +
                lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }
}

/// 🧭 Location Manager
#[derive(Debug)]
pub struct LocationManager {
    platform: DevicePlatform,
    is_enabled: bool,
    current_location: Option<Location>,
    location_history: Vec<Location>,
    accuracy_threshold: f64,
    update_interval: Duration,
}

impl LocationManager {
    pub fn new(platform: DevicePlatform) -> Self {
        Self {
            platform,
            is_enabled: false,
            current_location: None,
            location_history: Vec::new(),
            accuracy_threshold: 100.0, // meters
            update_interval: Duration::from_secs(10),
        }
    }
    
    pub fn request_permission(&mut self) -> Result<(), String> {
        match self.platform {
            DevicePlatform::iOS => {
                println!("📍 Requesting iOS location permission...");
                println!("   • NSLocationWhenInUseUsageDescription required in Info.plist");
                println!("   • CLLocationManager.requestWhenInUseAuthorization()");
            }
            DevicePlatform::Android => {
                println!("📍 Requesting Android location permission...");
                println!("   • ACCESS_FINE_LOCATION permission in AndroidManifest.xml");
                println!("   • Runtime permission request for API 23+");
            }
            DevicePlatform::Unknown => {
                return Err("Unknown platform".to_string());
            }
        }
        
        // Simulate permission granted
        self.is_enabled = true;
        Ok(())
    }
    
    pub fn start_location_updates(&mut self) -> Result<(), String> {
        if !self.is_enabled {
            return Err("Location permission not granted".to_string());
        }
        
        println!("🎯 Starting location updates...");
        
        // Simulate getting current location
        let location = Location {
            latitude: 13.7563, // Bangkok coordinates
            longitude: 100.5018,
            altitude: Some(2.0),
            accuracy: 5.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            speed: Some(0.0),
            bearing: None,
        };
        
        self.update_location(location);
        Ok(())
    }
    
    pub fn stop_location_updates(&mut self) {
        println!("⏹️ Stopping location updates...");
    }
    
    pub fn update_location(&mut self, location: Location) {
        if location.accuracy <= self.accuracy_threshold {
            self.location_history.push(location.clone());
            self.current_location = Some(location);
            
            // Keep only last 100 locations
            if self.location_history.len() > 100 {
                self.location_history.remove(0);
            }
        }
    }
    
    pub fn get_current_location(&self) -> Option<&Location> {
        self.current_location.as_ref()
    }
    
    pub fn get_location_history(&self) -> &Vec<Location> {
        &self.location_history
    }
}

/// 📷 Camera Configuration
#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub resolution: CameraResolution,
    pub quality: f32, // 0.0 - 1.0
    pub flash_mode: FlashMode,
    pub focus_mode: FocusMode,
    pub camera_facing: CameraFacing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CameraResolution {
    Low,     // 640x480
    Medium,  // 1280x720
    High,    // 1920x1080
    Ultra,   // 3840x2160
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlashMode {
    Off,
    On,
    Auto,
    Torch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusMode {
    Auto,
    Manual,
    Continuous,
    Infinity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CameraFacing {
    Front,
    Back,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            resolution: CameraResolution::High,
            quality: 0.8,
            flash_mode: FlashMode::Auto,
            focus_mode: FocusMode::Auto,
            camera_facing: CameraFacing::Back,
        }
    }
}

/// 📸 Camera Manager
#[derive(Debug)]
pub struct CameraManager {
    platform: DevicePlatform,
    is_available: bool,
    config: CameraConfig,
    is_recording: bool,
    captured_photos: Vec<String>, // File paths
}

impl CameraManager {
    pub fn new(platform: DevicePlatform) -> Self {
        Self {
            platform,
            is_available: false,
            config: CameraConfig::default(),
            is_recording: false,
            captured_photos: Vec::new(),
        }
    }
    
    pub fn request_permission(&mut self) -> Result<(), String> {
        match self.platform {
            DevicePlatform::iOS => {
                println!("📷 Requesting iOS camera permission...");
                println!("   • NSCameraUsageDescription required in Info.plist");
                println!("   • AVCaptureDevice.requestAccess(for: .video)");
            }
            DevicePlatform::Android => {
                println!("📷 Requesting Android camera permission...");
                println!("   • CAMERA permission in AndroidManifest.xml");
                println!("   • Runtime permission request for API 23+");
            }
            DevicePlatform::Unknown => {
                return Err("Unknown platform".to_string());
            }
        }
        
        self.is_available = true;
        Ok(())
    }
    
    pub fn configure_camera(&mut self, config: CameraConfig) {
        self.config = config;
        println!("⚙️ Camera configured: {:?}", self.config);
    }
    
    pub fn capture_photo(&mut self) -> Result<String, String> {
        if !self.is_available {
            return Err("Camera permission not granted".to_string());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let filename = format!("photo_{}_{:?}.jpg", timestamp, self.config.resolution);
        
        println!("📸 Capturing photo: {}", filename);
        println!("   • Resolution: {:?}", self.config.resolution);
        println!("   • Quality: {:.1}%", self.config.quality * 100.0);
        println!("   • Flash: {:?}", self.config.flash_mode);
        
        self.captured_photos.push(filename.clone());
        Ok(filename)
    }
    
    pub fn start_video_recording(&mut self) -> Result<(), String> {
        if !self.is_available {
            return Err("Camera permission not granted".to_string());
        }
        
        if self.is_recording {
            return Err("Already recording".to_string());
        }
        
        self.is_recording = true;
        println!("🎥 Started video recording");
        Ok(())
    }
    
    pub fn stop_video_recording(&mut self) -> Result<String, String> {
        if !self.is_recording {
            return Err("Not currently recording".to_string());
        }
        
        self.is_recording = false;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let filename = format!("video_{}_{:?}.mp4", timestamp, self.config.resolution);
        println!("⏹️ Stopped video recording: {}", filename);
        
        Ok(filename)
    }
    
    pub fn get_captured_photos(&self) -> &Vec<String> {
        &self.captured_photos
    }
}

/// 📊 Sensor Data
#[derive(Debug, Clone)]
pub struct SensorData {
    pub accelerometer: Option<AccelerometerData>,
    pub gyroscope: Option<GyroscopeData>,
    pub magnetometer: Option<MagnetometerData>,
    pub ambient_light: Option<f32>,
    pub proximity: Option<f32>,
    pub battery_level: Option<f32>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct AccelerometerData {
    pub x: f32, // m/s²
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct GyroscopeData {
    pub x: f32, // rad/s
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct MagnetometerData {
    pub x: f32, // μT (microtesla)
    pub y: f32,
    pub z: f32,
}

/// 🔬 Sensor Manager
#[derive(Debug)]
pub struct SensorManager {
    platform: DevicePlatform,
    available_sensors: Vec<SensorType>,
    active_sensors: Vec<SensorType>,
    sensor_data: SensorData,
    update_frequency: f32, // Hz
}

#[derive(Debug, Clone, PartialEq)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    AmbientLight,
    Proximity,
    Battery,
    Barometer,
    Temperature,
    Humidity,
}

impl SensorManager {
    pub fn new(platform: DevicePlatform) -> Self {
        let available_sensors = match platform {
            DevicePlatform::iOS => vec![
                SensorType::Accelerometer,
                SensorType::Gyroscope,
                SensorType::Magnetometer,
                SensorType::AmbientLight,
                SensorType::Proximity,
                SensorType::Battery,
                SensorType::Barometer,
            ],
            DevicePlatform::Android => vec![
                SensorType::Accelerometer,
                SensorType::Gyroscope,
                SensorType::Magnetometer,
                SensorType::AmbientLight,
                SensorType::Proximity,
                SensorType::Battery,
                SensorType::Barometer,
                SensorType::Temperature,
                SensorType::Humidity,
            ],
            DevicePlatform::Unknown => vec![],
        };
        
        Self {
            platform,
            available_sensors,
            active_sensors: Vec::new(),
            sensor_data: SensorData {
                accelerometer: None,
                gyroscope: None,
                magnetometer: None,
                ambient_light: None,
                proximity: None,
                battery_level: None,
                timestamp: 0,
            },
            update_frequency: 60.0, // 60 Hz
        }
    }
    
    pub fn start_sensor(&mut self, sensor_type: SensorType) -> Result<(), String> {
        if !self.available_sensors.contains(&sensor_type) {
            return Err(format!("Sensor {:?} not available on this device", sensor_type));
        }
        
        if !self.active_sensors.contains(&sensor_type) {
            self.active_sensors.push(sensor_type.clone());
            println!("🔬 Started sensor: {:?}", sensor_type);
        }
        
        Ok(())
    }
    
    pub fn stop_sensor(&mut self, sensor_type: &SensorType) {
        if let Some(pos) = self.active_sensors.iter().position(|s| s == sensor_type) {
            self.active_sensors.remove(pos);
            println!("⏹️ Stopped sensor: {:?}", sensor_type);
        }
    }
    
    pub fn update_sensor_data(&mut self) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Simulate sensor data
        if self.active_sensors.contains(&SensorType::Accelerometer) {
            self.sensor_data.accelerometer = Some(AccelerometerData {
                x: 0.1,
                y: 0.2,
                z: 9.8, // Gravity
            });
        }
        
        if self.active_sensors.contains(&SensorType::Gyroscope) {
            self.sensor_data.gyroscope = Some(GyroscopeData {
                x: 0.01,
                y: 0.02,
                z: 0.03,
            });
        }
        
        if self.active_sensors.contains(&SensorType::Magnetometer) {
            self.sensor_data.magnetometer = Some(MagnetometerData {
                x: 25.0,
                y: -15.0,
                z: 45.0,
            });
        }
        
        if self.active_sensors.contains(&SensorType::AmbientLight) {
            self.sensor_data.ambient_light = Some(300.0); // lux
        }
        
        if self.active_sensors.contains(&SensorType::Proximity) {
            self.sensor_data.proximity = Some(5.0); // cm
        }
        
        if self.active_sensors.contains(&SensorType::Battery) {
            self.sensor_data.battery_level = Some(0.75); // 75%
        }
        
        self.sensor_data.timestamp = timestamp;
    }
    
    pub fn get_sensor_data(&self) -> &SensorData {
        &self.sensor_data
    }
    
    pub fn get_available_sensors(&self) -> &Vec<SensorType> {
        &self.available_sensors
    }
    
    pub fn get_active_sensors(&self) -> &Vec<SensorType> {
        &self.active_sensors
    }
}

/// 💾 Storage Manager
#[derive(Debug)]
pub struct StorageManager {
    platform: DevicePlatform,
    app_documents_path: String,
    cache_path: String,
    external_storage_available: bool,
}

impl StorageManager {
    pub fn new(platform: DevicePlatform) -> Self {
        let (documents_path, cache_path, external_available) = match platform {
            DevicePlatform::iOS => (
                "/var/mobile/Containers/Data/Application/[UUID]/Documents".to_string(),
                "/var/mobile/Containers/Data/Application/[UUID]/Library/Caches".to_string(),
                false,
            ),
            DevicePlatform::Android => (
                "/data/data/com.example.app/files".to_string(),
                "/data/data/com.example.app/cache".to_string(),
                true,
            ),
            DevicePlatform::Unknown => (
                "./documents".to_string(),
                "./cache".to_string(),
                false,
            ),
        };
        
        Self {
            platform,
            app_documents_path: documents_path,
            cache_path,
            external_storage_available: external_available,
        }
    }
    
    pub fn save_file(&self, filename: &str, data: &[u8]) -> Result<String, String> {
        let file_path = format!("{}/{}", self.app_documents_path, filename);
        
        println!("💾 Saving file: {} ({} bytes)", file_path, data.len());
        
        // In a real implementation, this would write to the file system
        match self.platform {
            DevicePlatform::iOS => {
                println!("   • Using NSFileManager for iOS file operations");
            }
            DevicePlatform::Android => {
                println!("   • Using Java File API for Android file operations");
            }
            DevicePlatform::Unknown => {
                println!("   • Using standard file operations");
            }
        }
        
        Ok(file_path)
    }
    
    pub fn load_file(&self, filename: &str) -> Result<Vec<u8>, String> {
        let file_path = format!("{}/{}", self.app_documents_path, filename);
        
        println!("📂 Loading file: {}", file_path);
        
        // Simulate file loading
        Ok(b"simulated file content".to_vec())
    }
    
    pub fn delete_file(&self, filename: &str) -> Result<(), String> {
        let file_path = format!("{}/{}", self.app_documents_path, filename);
        
        println!("🗑️ Deleting file: {}", file_path);
        Ok(())
    }
    
    pub fn get_storage_info(&self) -> StorageInfo {
        // Simulate storage information
        StorageInfo {
            total_space: 64 * 1024 * 1024 * 1024, // 64 GB
            available_space: 32 * 1024 * 1024 * 1024, // 32 GB
            used_space: 32 * 1024 * 1024 * 1024, // 32 GB
            app_size: 100 * 1024 * 1024, // 100 MB
        }
    }
    
    pub fn cache_data(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let cache_file = format!("{}/{}.cache", self.cache_path, key);
        println!("🗂️ Caching data: {} ({} bytes)", cache_file, data.len());
        Ok(())
    }
    
    pub fn get_cached_data(&self, key: &str) -> Option<Vec<u8>> {
        let cache_file = format!("{}/{}.cache", self.cache_path, key);
        println!("📋 Retrieving cached data: {}", cache_file);
        Some(b"cached data".to_vec())
    }
    
    pub fn clear_cache(&self) -> Result<(), String> {
        println!("🧹 Clearing cache directory: {}", self.cache_path);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub app_size: u64,
}

impl StorageInfo {
    pub fn usage_percentage(&self) -> f32 {
        (self.used_space as f32 / self.total_space as f32) * 100.0
    }
    
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 📱 Device Info
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub platform: DevicePlatform,
    pub model: String,
    pub os_version: String,
    pub app_version: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub screen_density: f32,
    pub device_id: String,
    pub is_tablet: bool,
    pub has_notch: bool,
    pub supports_biometrics: bool,
}

impl DeviceInfo {
    pub fn get_device_info(platform: DevicePlatform) -> Self {
        match platform {
            DevicePlatform::iOS => Self {
                platform,
                model: "iPhone 14 Pro".to_string(),
                os_version: "iOS 17.0".to_string(),
                app_version: "1.0.0".to_string(),
                screen_width: 393,
                screen_height: 852,
                screen_density: 3.0,
                device_id: "iOS-DEVICE-ID-12345".to_string(),
                is_tablet: false,
                has_notch: true,
                supports_biometrics: true,
            },
            DevicePlatform::Android => Self {
                platform,
                model: "Samsung Galaxy S23".to_string(),
                os_version: "Android 13".to_string(),
                app_version: "1.0.0".to_string(),
                screen_width: 360,
                screen_height: 780,
                screen_density: 2.75,
                device_id: "ANDROID-DEVICE-ID-67890".to_string(),
                is_tablet: false,
                has_notch: false,
                supports_biometrics: true,
            },
            DevicePlatform::Unknown => Self {
                platform,
                model: "Unknown Device".to_string(),
                os_version: "Unknown OS".to_string(),
                app_version: "1.0.0".to_string(),
                screen_width: 375,
                screen_height: 667,
                screen_density: 2.0,
                device_id: "UNKNOWN-DEVICE-ID".to_string(),
                is_tablet: false,
                has_notch: false,
                supports_biometrics: false,
            },
        }
    }
    
    pub fn get_screen_size_category(&self) -> String {
        let diagonal = ((self.screen_width.pow(2) + self.screen_height.pow(2)) as f32).sqrt() / self.screen_density;
        
        if diagonal < 5.0 {
            "Small Phone".to_string()
        } else if diagonal < 6.5 {
            "Regular Phone".to_string()
        } else if diagonal < 8.0 {
            "Large Phone".to_string()
        } else {
            "Tablet".to_string()
        }
    }
}

/// 📲 สาธิตการใช้งาน Device Integration
pub fn demonstrate_device_integration() {
    println!("📲 === Device Integration Demo ===");
    
    // ตรวจสอบข้อมูลอุปกรณ์
    println!("\n📱 Device Information:");
    let ios_device = DeviceInfo::get_device_info(DevicePlatform::iOS);
    let android_device = DeviceInfo::get_device_info(DevicePlatform::Android);
    
    println!("   iOS Device: {} - {} ({})", ios_device.model, ios_device.os_version, ios_device.get_screen_size_category());
    println!("   Android Device: {} - {} ({})", android_device.model, android_device.os_version, android_device.get_screen_size_category());
    
    // Location Services
    println!("\n🧭 Location Services:");
    let mut location_manager = LocationManager::new(DevicePlatform::iOS);
    
    if let Ok(()) = location_manager.request_permission() {
        if let Ok(()) = location_manager.start_location_updates() {
            if let Some(location) = location_manager.get_current_location() {
                println!("   Current Location: {:.6}, {:.6}", location.latitude, location.longitude);
                println!("   Accuracy: {:.1}m, Speed: {:?} m/s", location.accuracy, location.speed);
                
                // Test distance calculation
                let destination = Location::new(13.7500, 100.5000); // Another point in Bangkok
                let distance = location.distance_to(&destination);
                println!("   Distance to destination: {:.2} km", distance);
            }
        }
    }
    
    // Camera Integration
    println!("\n📷 Camera Integration:");
    let mut camera_manager = CameraManager::new(DevicePlatform::Android);
    
    if let Ok(()) = camera_manager.request_permission() {
        // Configure camera
        let config = CameraConfig {
            resolution: CameraResolution::High,
            quality: 0.9,
            flash_mode: FlashMode::Auto,
            focus_mode: FocusMode::Auto,
            camera_facing: CameraFacing::Back,
        };
        camera_manager.configure_camera(config);
        
        // Capture photos
        if let Ok(photo1) = camera_manager.capture_photo() {
            println!("   📸 Captured: {}", photo1);
        }
        
        // Record video
        if let Ok(()) = camera_manager.start_video_recording() {
            std::thread::sleep(Duration::from_millis(100)); // Simulate recording
            if let Ok(video) = camera_manager.stop_video_recording() {
                println!("   🎥 Recorded: {}", video);
            }
        }
        
        println!("   📚 Total photos captured: {}", camera_manager.get_captured_photos().len());
    }
    
    // Sensor Integration
    println!("\n🔬 Sensor Integration:");
    let mut sensor_manager = SensorManager::new(DevicePlatform::Android);
    
    println!("   Available sensors: {:?}", sensor_manager.get_available_sensors());
    
    // Start sensors
    let _ = sensor_manager.start_sensor(SensorType::Accelerometer);
    let _ = sensor_manager.start_sensor(SensorType::Gyroscope);
    let _ = sensor_manager.start_sensor(SensorType::Battery);
    
    // Update and read sensor data
    sensor_manager.update_sensor_data();
    let sensor_data = sensor_manager.get_sensor_data();
    
    if let Some(ref accel) = sensor_data.accelerometer {
        println!("   📊 Accelerometer: x={:.2}, y={:.2}, z={:.2} m/s²", accel.x, accel.y, accel.z);
    }
    
    if let Some(ref gyro) = sensor_data.gyroscope {
        println!("   🌀 Gyroscope: x={:.3}, y={:.3}, z={:.3} rad/s", gyro.x, gyro.y, gyro.z);
    }
    
    if let Some(battery) = sensor_data.battery_level {
        println!("   🔋 Battery Level: {:.0}%", battery * 100.0);
    }
    
    // Storage Management
    println!("\n💾 Storage Management:");
    let storage_manager = StorageManager::new(DevicePlatform::iOS);
    
    // Save and load files
    let test_data = b"Hello, Mobile World!";
    if let Ok(file_path) = storage_manager.save_file("test.txt", test_data) {
        println!("   💾 Saved file: {}", file_path);
        
        if let Ok(loaded_data) = storage_manager.load_file("test.txt") {
            println!("   📂 Loaded {} bytes", loaded_data.len());
        }
    }
    
    // Cache management
    storage_manager.cache_data("user_preferences", b"{\"theme\": \"dark\"}").ok();
    if let Some(cached) = storage_manager.get_cached_data("user_preferences") {
        println!("   🗂️ Retrieved cached data: {} bytes", cached.len());
    }
    
    // Storage info
    let storage_info = storage_manager.get_storage_info();
    println!("   📊 Storage Usage: {:.1}%", storage_info.usage_percentage());
    println!("   💽 Available: {}", StorageInfo::format_size(storage_info.available_space));
    println!("   📱 App Size: {}", StorageInfo::format_size(storage_info.app_size));
    
    // Platform-specific features
    println!("\n🔧 Platform-specific Features:");
    show_platform_specific_features();
    
    // Best practices
    println!("\n💡 Device Integration Best Practices:");
    show_device_integration_best_practices();
}

/// 🔧 Platform-specific Features
fn show_platform_specific_features() {
    println!("   🍎 iOS Specific:");
    println!("      • Core Location for GPS");
    println!("      • AVFoundation for camera/audio");
    println!("      • Core Motion for sensors");
    println!("      • FileManager for storage");
    println!("      • UserDefaults for preferences");
    println!("      • Keychain for secure storage");
    println!("      • HealthKit for health data");
    println!("      • HomeKit for smart home");
    
    println!("\n   🤖 Android Specific:");
    println!("      • LocationManager for GPS");
    println!("      • Camera2 API for camera");
    println!("      • SensorManager for sensors");
    println!("      • File API for storage");
    println!("      • SharedPreferences for preferences");
    println!("      • Keystore for secure storage");
    println!("      • Google Fit for health data");
    println!("      • Google Assistant for voice");
    
    println!("\n   🌐 Cross-platform Solutions:");
    println!("      • Rust + JNI for Android");
    println!("      • Rust + Swift/ObjC for iOS");
    println!("      • Flutter + Rust FFI");
    println!("      • React Native + Rust modules");
    println!("      • Tauri for mobile apps");
}

/// 💡 Device Integration Best Practices
fn show_device_integration_best_practices() {
    let practices = vec![
        "🔐 Always request permissions before accessing device features",
        "⚡ Handle permission denials gracefully",
        "🔋 Minimize battery usage by stopping unused sensors",
        "📊 Implement proper error handling for device APIs",
        "💾 Use appropriate storage locations for different data types",
        "🗑️ Clean up resources when no longer needed",
        "📱 Test on various device models and OS versions",
        "🔒 Secure sensitive data with encryption",
        "⏰ Implement timeouts for device operations",
        "📈 Monitor performance impact of device integrations",
        "🔄 Handle device state changes (background/foreground)",
        "📋 Provide fallbacks when features are unavailable",
        "🎯 Follow platform-specific guidelines and best practices",
        "🧪 Test offline scenarios and edge cases",
        "📊 Implement proper logging for debugging device issues",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   🛠️ Recommended Crates:");
    println!("      • jni - Java Native Interface for Android");
    println!("      • objc - Objective-C runtime for iOS");
    println!("      • core-foundation - iOS Core Foundation bindings");
    println!("      • android-ndk - Android NDK bindings");
    println!("      • gps - GPS coordinate utilities");
    println!("      • image - Image processing");
    println!("      • serde - Data serialization");
    println!("      • tokio - Async runtime for device operations");
    
    println!("\n   📋 Permission Management:");
    println!("      • Request permissions at appropriate times");
    println!("      • Explain why permissions are needed");
    println!("      • Provide alternative flows for denied permissions");
    println!("      • Check permission status before each use");
    println!("      • Handle permission changes during app lifecycle");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_location_distance_calculation() {
        let bangkok = Location::new(13.7563, 100.5018);
        let chiang_mai = Location::new(18.7883, 98.9853);
        
        let distance = bangkok.distance_to(&chiang_mai);
        assert!(distance > 580.0 && distance < 590.0); // Approximately 582 km
    }
    
    #[test]
    fn test_location_manager() {
        let mut manager = LocationManager::new(DevicePlatform::iOS);
        assert!(!manager.is_enabled);
        
        assert!(manager.request_permission().is_ok());
        assert!(manager.is_enabled);
        
        assert!(manager.start_location_updates().is_ok());
        assert!(manager.get_current_location().is_some());
    }
    
    #[test]
    fn test_camera_manager() {
        let mut camera = CameraManager::new(DevicePlatform::Android);
        assert!(!camera.is_available);
        
        assert!(camera.request_permission().is_ok());
        assert!(camera.is_available);
        
        assert!(camera.capture_photo().is_ok());
        assert_eq!(camera.get_captured_photos().len(), 1);
    }
    
    #[test]
    fn test_sensor_manager() {
        let mut sensors = SensorManager::new(DevicePlatform::Android);
        assert!(sensors.get_available_sensors().len() > 0);
        
        assert!(sensors.start_sensor(SensorType::Accelerometer).is_ok());
        assert!(sensors.get_active_sensors().contains(&SensorType::Accelerometer));
        
        sensors.update_sensor_data();
        assert!(sensors.get_sensor_data().accelerometer.is_some());
    }
    
    #[test]
    fn test_storage_manager() {
        let storage = StorageManager::new(DevicePlatform::iOS);
        
        let test_data = b"test data";
        assert!(storage.save_file("test.txt", test_data).is_ok());
        assert!(storage.load_file("test.txt").is_ok());
        assert!(storage.delete_file("test.txt").is_ok());
    }
    
    #[test]
    fn test_storage_info() {
        let info = StorageInfo {
            total_space: 1000,
            used_space: 500,
            available_space: 500,
            app_size: 100,
        };
        
        assert_eq!(info.usage_percentage(), 50.0);
        assert_eq!(StorageInfo::format_size(1024), "1.0 KB");
        assert_eq!(StorageInfo::format_size(1024 * 1024), "1.0 MB");
    }
    
    #[test]
    fn test_device_info() {
        let ios_device = DeviceInfo::get_device_info(DevicePlatform::iOS);
        assert_eq!(ios_device.platform, DevicePlatform::iOS);
        assert!(ios_device.supports_biometrics);
        
        let android_device = DeviceInfo::get_device_info(DevicePlatform::Android);
        assert_eq!(android_device.platform, DevicePlatform::Android);
        assert!(!android_device.has_notch);
    }
}