//! 🌐 Cross-platform Mobile Frameworks
//! 
//! สาธิตการใช้ Rust ในการพัฒนาแอปมือถือแบบ cross-platform
//! รวมถึง Tauri, Dioxus, และ integration กับ Flutter/React Native

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 🌐 Cross-platform Framework Types
#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkType {
    Tauri,
    Dioxus,
    Flutter,
    ReactNative,
    Slint,
    Egui,
    Native,
}

/// 📱 Platform Target
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum PlatformTarget {
    iOS,
    Android,
    Windows,
    MacOS,
    Linux,
    Web,
}

/// 🎯 App Configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub bundle_id: String,
    pub framework: FrameworkType,
    pub targets: Vec<PlatformTarget>,
    pub features: Vec<String>,
    pub permissions: Vec<String>,
}

impl AppConfig {
    pub fn new(name: &str, framework: FrameworkType) -> Self {
        Self {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            bundle_id: format!("com.example.{}", name.to_lowercase()),
            framework,
            targets: vec![PlatformTarget::iOS, PlatformTarget::Android],
            features: Vec::new(),
            permissions: Vec::new(),
        }
    }
    
    pub fn add_target(&mut self, target: PlatformTarget) {
        if !self.targets.contains(&target) {
            self.targets.push(target);
        }
    }
    
    pub fn add_feature(&mut self, feature: &str) {
        self.features.push(feature.to_string());
    }
    
    pub fn add_permission(&mut self, permission: &str) {
        self.permissions.push(permission.to_string());
    }
    
    pub fn generate_manifest(&self) -> String {
        match self.framework {
            FrameworkType::Tauri => self.generate_tauri_config(),
            FrameworkType::Flutter => self.generate_flutter_config(),
            FrameworkType::ReactNative => self.generate_rn_config(),
            _ => self.generate_generic_config(),
        }
    }
    
    fn generate_tauri_config(&self) -> String {
        format!(
            r#"{{
  "package": {{
    "productName": "{}",
    "version": "{}"
  }},
  "build": {{
    "distDir": "../dist",
    "devPath": "http://localhost:1420",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  }},
  "tauri": {{
    "bundle": {{
      "identifier": "{}",
      "targets": {:?}
    }},
    "allowlist": {{
      "all": false,
      "shell": {{
        "all": false,
        "open": true
      }}
    }},
    "windows": [
      {{
        "fullscreen": false,
        "height": 600,
        "resizable": true,
        "title": "{}",
        "width": 800
      }}
    ]
  }}
}}
"#,
            self.name, self.version, self.bundle_id, self.targets, self.name
        )
    }
    
    fn generate_flutter_config(&self) -> String {
        format!(
            r#"name: {}
description: A Flutter application with Rust backend.
version: {}

environment:
  sdk: '>=2.17.0 <4.0.0'
  flutter: ">=3.0.0"

dependencies:
  flutter:
    sdk: flutter
  ffi: ^2.0.1
  path: ^1.8.2

dev_dependencies:
  flutter_test:
    sdk: flutter
  ffigen: ^6.1.2

flutter:
  uses-material-design: true
  
  # Rust library integration
  plugin:
    platforms:
      android:
        package: {}
        pluginClass: RustPlugin
      ios:
        pluginClass: RustPlugin
"#,
            self.name.to_lowercase(),
            self.version,
            self.bundle_id
        )
    }
    
    fn generate_rn_config(&self) -> String {
        format!(
            r#"{{
  "name": "{}",
  "version": "{}",
  "private": true,
  "scripts": {{
    "android": "react-native run-android",
    "ios": "react-native run-ios",
    "start": "react-native start",
    "test": "jest",
    "lint": "eslint ."
  }},
  "dependencies": {{
    "react": "18.2.0",
    "react-native": "0.72.0",
    "react-native-rust-bridge": "^1.0.0"
  }},
  "devDependencies": {{
    "@babel/core": "^7.20.0",
    "@babel/preset-env": "^7.20.0",
    "@babel/runtime": "^7.20.0",
    "metro-react-native-babel-preset": "0.76.0"
  }},
  "jest": {{
    "preset": "react-native"
  }}
}}
"#,
            self.name.to_lowercase(),
            self.version
        )
    }
    
    fn generate_generic_config(&self) -> String {
        format!(
            "App: {}\nVersion: {}\nFramework: {:?}\nTargets: {:?}\nFeatures: {:?}",
            self.name, self.version, self.framework, self.targets, self.features
        )
    }
}

/// 🔧 Cross-platform Builder
pub struct CrossPlatformBuilder {
    configs: HashMap<String, AppConfig>,
    build_cache: Arc<Mutex<HashMap<String, BuildResult>>>,
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub target: PlatformTarget,
    pub output_path: String,
    pub build_time: Duration,
    pub size_bytes: u64,
    pub errors: Vec<String>,
}

impl CrossPlatformBuilder {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            build_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn add_config(&mut self, config: AppConfig) {
        self.configs.insert(config.name.clone(), config);
    }
    
    pub fn build_for_target(&self, app_name: &str, target: PlatformTarget) -> Result<BuildResult, String> {
        let config = self.configs.get(app_name)
            .ok_or_else(|| format!("App config not found: {}", app_name))?;
        
        if !config.targets.contains(&target) {
            return Err(format!("Target {:?} not supported for app {}", target, app_name));
        }
        
        let start_time = Instant::now();
        
        // Simulate build process
        let result = match (&config.framework, &target) {
            (FrameworkType::Tauri, PlatformTarget::iOS) => self.build_tauri_ios(config),
            (FrameworkType::Tauri, PlatformTarget::Android) => self.build_tauri_android(config),
            (FrameworkType::Flutter, PlatformTarget::iOS) => self.build_flutter_ios(config),
            (FrameworkType::Flutter, PlatformTarget::Android) => self.build_flutter_android(config),
            (FrameworkType::ReactNative, PlatformTarget::iOS) => self.build_rn_ios(config),
            (FrameworkType::ReactNative, PlatformTarget::Android) => self.build_rn_android(config),
            _ => self.build_generic(config, &target),
        };
        
        let build_time = start_time.elapsed();
        
        let build_result = BuildResult {
            success: result.is_ok(),
            target: target.clone(),
            output_path: result.as_ref().map(|s| s.clone()).unwrap_or_else(|_| "build/failed".to_string()),
            build_time,
            size_bytes: 1024 * 1024 * 10, // Simulated 10MB
            errors: if let Err(e) = result { vec![e] } else { Vec::new() },
        };
        
        // Cache result
        let cache_key = format!("{}_{:?}", app_name, target);
        if let Ok(mut cache) = self.build_cache.lock() {
            cache.insert(cache_key, build_result.clone());
        }
        
        Ok(build_result)
    }
    
    pub fn build_all_targets(&self, app_name: &str) -> Vec<BuildResult> {
        let mut results = Vec::new();
        
        if let Some(config) = self.configs.get(app_name) {
            for target in &config.targets {
                match self.build_for_target(app_name, target.clone()) {
                    Ok(result) => results.push(result),
                    Err(error) => {
                        results.push(BuildResult {
                            success: false,
                            target: target.clone(),
                            output_path: String::new(),
                            build_time: Duration::from_millis(0),
                            size_bytes: 0,
                            errors: vec![error],
                        });
                    }
                }
            }
        }
        
        results
    }
    
    fn build_tauri_ios(&self, config: &AppConfig) -> Result<String, String> {
        println!("🍎 Building Tauri app for iOS...");
        println!("   • Compiling Rust core to aarch64-apple-ios");
        println!("   • Generating Swift bindings");
        println!("   • Building iOS app bundle");
        Ok(format!("build/ios/{}.app", config.name))
    }
    
    fn build_tauri_android(&self, config: &AppConfig) -> Result<String, String> {
        println!("🤖 Building Tauri app for Android...");
        println!("   • Compiling Rust core to aarch64-linux-android");
        println!("   • Generating JNI bindings");
        println!("   • Building Android APK");
        Ok(format!("build/android/{}.apk", config.name))
    }
    
    fn build_flutter_ios(&self, config: &AppConfig) -> Result<String, String> {
        println!("🍎 Building Flutter app with Rust for iOS...");
        println!("   • Compiling Rust library to iOS framework");
        println!("   • Generating Dart FFI bindings");
        println!("   • Building Flutter iOS app");
        Ok(format!("build/ios/Runner.app"))
    }
    
    fn build_flutter_android(&self, config: &AppConfig) -> Result<String, String> {
        println!("🤖 Building Flutter app with Rust for Android...");
        println!("   • Compiling Rust library to Android .so");
        println!("   • Generating Dart FFI bindings");
        println!("   • Building Flutter Android APK");
        Ok(format!("build/app/outputs/flutter-apk/app-release.apk"))
    }
    
    fn build_rn_ios(&self, config: &AppConfig) -> Result<String, String> {
        println!("🍎 Building React Native app with Rust for iOS...");
        println!("   • Compiling Rust library to iOS framework");
        println!("   • Generating JavaScript bridge");
        println!("   • Building React Native iOS app");
        Ok(format!("ios/build/Build/Products/Release-iphoneos/{}.app", config.name))
    }
    
    fn build_rn_android(&self, config: &AppConfig) -> Result<String, String> {
        println!("🤖 Building React Native app with Rust for Android...");
        println!("   • Compiling Rust library to Android .so");
        println!("   • Generating JNI bridge");
        println!("   • Building React Native Android APK");
        Ok(format!("android/app/build/outputs/apk/release/app-release.apk"))
    }
    
    fn build_generic(&self, config: &AppConfig, target: &PlatformTarget) -> Result<String, String> {
        println!("🔧 Building {:?} app for {:?}...", config.framework, target);
        Ok(format!("build/{:?}/{}", target, config.name))
    }
    
    pub fn get_build_stats(&self) -> HashMap<String, BuildResult> {
        self.build_cache.lock().unwrap().clone()
    }
    
    pub fn clean_cache(&self) {
        if let Ok(mut cache) = self.build_cache.lock() {
            cache.clear();
        }
    }
}

/// 🎨 UI Component System
pub trait CrossPlatformComponent {
    fn render(&self, platform: &PlatformTarget) -> String;
    fn get_properties(&self) -> HashMap<String, String>;
    fn handle_event(&mut self, event: &str, data: &str);
}

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub style: HashMap<String, String>,
    pub enabled: bool,
}

impl Button {
    pub fn new(text: &str) -> Self {
        let mut style = HashMap::new();
        style.insert("background".to_string(), "#007AFF".to_string());
        style.insert("color".to_string(), "white".to_string());
        style.insert("padding".to_string(), "12px 24px".to_string());
        style.insert("border-radius".to_string(), "8px".to_string());
        
        Self {
            text: text.to_string(),
            style,
            enabled: true,
        }
    }
}

impl CrossPlatformComponent for Button {
    fn render(&self, platform: &PlatformTarget) -> String {
        match platform {
            PlatformTarget::iOS => {
                format!(
                    "UIButton(title: \"{}\", enabled: {}, style: {:?})",
                    self.text, self.enabled, self.style
                )
            }
            PlatformTarget::Android => {
                format!(
                    "<Button android:text=\"{}\" android:enabled=\"{}\" style=\"{:?}\" />",
                    self.text, self.enabled, self.style
                )
            }
            PlatformTarget::Web => {
                let style_str = self.style.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!(
                    "<button style=\"{}\" {}>{}</button>",
                    style_str,
                    if self.enabled { "" } else { "disabled" },
                    self.text
                )
            }
            _ => format!("Button: {}", self.text),
        }
    }
    
    fn get_properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("text".to_string(), self.text.clone());
        props.insert("enabled".to_string(), self.enabled.to_string());
        props.extend(self.style.clone());
        props
    }
    
    fn handle_event(&mut self, event: &str, _data: &str) {
        match event {
            "click" | "tap" => {
                if self.enabled {
                    println!("Button '{}' was clicked!", self.text);
                }
            }
            "enable" => self.enabled = true,
            "disable" => self.enabled = false,
            _ => {}
        }
    }
}

/// 📱 Cross-platform App Template
pub struct CrossPlatformApp {
    pub config: AppConfig,
    pub components: Vec<Box<dyn CrossPlatformComponent>>,
    pub state: HashMap<String, String>,
}

impl CrossPlatformApp {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            components: Vec::new(),
            state: HashMap::new(),
        }
    }
    
    pub fn add_component(&mut self, component: Box<dyn CrossPlatformComponent>) {
        self.components.push(component);
    }
    
    pub fn render_for_platform(&self, platform: &PlatformTarget) -> String {
        let mut output = format!("=== {} for {:?} ===\n", self.config.name, platform);
        
        for (i, component) in self.components.iter().enumerate() {
            output.push_str(&format!("Component {}: {}\n", i, component.render(platform)));
        }
        
        output
    }
    
    pub fn set_state(&mut self, key: &str, value: &str) {
        self.state.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_state(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }
}

/// 🌐 สาธิตการใช้งาน Cross-platform Frameworks
pub fn demonstrate_cross_platform_frameworks() {
    println!("🌐 === Cross-platform Mobile Frameworks Demo ===");
    
    // สร้าง App Configurations
    let mut tauri_config = AppConfig::new("MyTauriApp", FrameworkType::Tauri);
    tauri_config.add_feature("camera");
    tauri_config.add_feature("geolocation");
    tauri_config.add_permission("camera");
    tauri_config.add_permission("location");
    
    let mut flutter_config = AppConfig::new("MyFlutterApp", FrameworkType::Flutter);
    flutter_config.add_target(PlatformTarget::Web);
    flutter_config.add_feature("offline_storage");
    
    let mut rn_config = AppConfig::new("MyReactNativeApp", FrameworkType::ReactNative);
    rn_config.add_feature("push_notifications");
    
    println!("\n📋 App Configurations:");
    println!("Tauri Config:\n{}", tauri_config.generate_manifest());
    println!("\nFlutter Config:\n{}", flutter_config.generate_manifest());
    println!("\nReact Native Config:\n{}", rn_config.generate_manifest());
    
    // Cross-platform Builder
    let mut builder = CrossPlatformBuilder::new();
    builder.add_config(tauri_config.clone());
    builder.add_config(flutter_config.clone());
    builder.add_config(rn_config.clone());
    
    println!("\n🔨 Building Apps:");
    
    // Build Tauri app for multiple platforms
    println!("\n📱 Building Tauri App:");
    let tauri_results = builder.build_all_targets("MyTauriApp");
    for result in tauri_results {
        println!("   {:?}: {} ({}ms, {} bytes)", 
                result.target, 
                if result.success { "✅ Success" } else { "❌ Failed" },
                result.build_time.as_millis(),
                result.size_bytes);
        if !result.errors.is_empty() {
            println!("     Errors: {:?}", result.errors);
        }
    }
    
    // Build Flutter app
    println!("\n📱 Building Flutter App:");
    let flutter_results = builder.build_all_targets("MyFlutterApp");
    for result in flutter_results {
        println!("   {:?}: {} ({}ms, {} bytes)", 
                result.target, 
                if result.success { "✅ Success" } else { "❌ Failed" },
                result.build_time.as_millis(),
                result.size_bytes);
    }
    
    // Cross-platform UI Components
    println!("\n🎨 Cross-platform UI Components:");
    
    let mut app = CrossPlatformApp::new(tauri_config);
    
    let mut login_button = Button::new("Login");
    login_button.style.insert("background".to_string(), "#34C759".to_string());
    
    let mut cancel_button = Button::new("Cancel");
    cancel_button.style.insert("background".to_string(), "#FF3B30".to_string());
    
    app.add_component(Box::new(login_button));
    app.add_component(Box::new(cancel_button));
    
    // Render for different platforms
    let platforms = vec![PlatformTarget::iOS, PlatformTarget::Android, PlatformTarget::Web];
    
    for platform in platforms {
        println!("\n{}", app.render_for_platform(&platform));
    }
    
    // Performance Comparison
    println!("\n📊 Framework Performance Comparison:");
    show_framework_comparison();
    
    // Best Practices
    println!("\n💡 Cross-platform Development Best Practices:");
    show_cross_platform_best_practices();
}

/// 📊 Framework Performance Comparison
fn show_framework_comparison() {
    let frameworks = vec![
        ("Tauri", "⚡ Fast", "🔋 Low", "📦 Small", "🛠️ Medium"),
        ("Flutter + Rust", "⚡ Fast", "🔋 Medium", "📦 Large", "🛠️ High"),
        ("React Native + Rust", "⚡ Medium", "🔋 Medium", "📦 Medium", "🛠️ Medium"),
        ("Dioxus", "⚡ Fast", "🔋 Low", "📦 Small", "🛠️ Low"),
        ("Slint", "⚡ Fast", "🔋 Low", "📦 Small", "🛠️ Medium"),
    ];
    
    println!("   Framework          Performance  Battery  Size     Complexity");
    println!("   ─────────────────  ───────────  ───────  ───────  ──────────");
    
    for (name, perf, battery, size, complexity) in frameworks {
        println!("   {:<17}  {:<11}  {:<7}  {:<7}  {}", name, perf, battery, size, complexity);
    }
}

/// 💡 Cross-platform Best Practices
fn show_cross_platform_best_practices() {
    let practices = vec![
        "🎯 Define clear separation between core logic and UI",
        "📱 Use platform-specific UI guidelines when needed",
        "⚡ Optimize for the lowest common denominator",
        "🔧 Implement platform-specific optimizations",
        "🧪 Test on real devices for each platform",
        "📊 Monitor performance metrics across platforms",
        "🔄 Use consistent state management patterns",
        "🛡️ Implement proper error handling for each platform",
        "📦 Optimize bundle size for mobile constraints",
        "🔋 Consider battery usage in design decisions",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_config_creation() {
        let config = AppConfig::new("TestApp", FrameworkType::Tauri);
        assert_eq!(config.name, "TestApp");
        assert_eq!(config.framework, FrameworkType::Tauri);
        assert_eq!(config.bundle_id, "com.example.testapp");
    }
    
    #[test]
    fn test_cross_platform_builder() {
        let mut builder = CrossPlatformBuilder::new();
        let config = AppConfig::new("TestApp", FrameworkType::Tauri);
        builder.add_config(config);
        
        let result = builder.build_for_target("TestApp", PlatformTarget::iOS);
        assert!(result.is_ok());
        
        let build_result = result.unwrap();
        assert!(build_result.success);
        assert_eq!(build_result.target, PlatformTarget::iOS);
    }
    
    #[test]
    fn test_button_component() {
        let mut button = Button::new("Test Button");
        assert_eq!(button.text, "Test Button");
        assert!(button.enabled);
        
        button.handle_event("disable", "");
        assert!(!button.enabled);
        
        let ios_render = button.render(&PlatformTarget::iOS);
        assert!(ios_render.contains("UIButton"));
        assert!(ios_render.contains("Test Button"));
    }
    
    #[test]
    fn test_cross_platform_app() {
        let config = AppConfig::new("TestApp", FrameworkType::Tauri);
        let mut app = CrossPlatformApp::new(config);
        
        app.set_state("user_id", "123");
        assert_eq!(app.get_state("user_id"), Some(&"123".to_string()));
        
        let button = Button::new("Test");
        app.add_component(Box::new(button));
        
        let render_output = app.render_for_platform(&PlatformTarget::iOS);
        assert!(render_output.contains("TestApp"));
        assert!(render_output.contains("iOS"));
    }
}