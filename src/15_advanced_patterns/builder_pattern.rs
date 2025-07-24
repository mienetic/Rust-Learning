//! 🏗️ Configuration Architect Workshop - Builder Pattern Implementation
//!
//! ยินดีต้อนรับสู่ Configuration Architect Workshop! 🎯
//! Builder Pattern ช่วยในการสร้าง object ที่ซับซ้อนแบบ step-by-step
//! เหมาะสำหรับ struct ที่มี field จำนวนมากหรือมี optional parameters
//! เหมือนการออกแบบสถาปัตยกรรมระบบแบบมืออาชีพ! 🏛️

use std::collections::HashMap;

/// 📡 HTTP Client Configuration Blueprint - แบบแปลนการเชื่อมต่อเครือข่าย
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub headers: HashMap<String, String>,
    pub user_agent: String,
    pub follow_redirects: bool,
    pub verify_ssl: bool,
}

/// 🔧 HTTP Client Configuration Architect - สถาปนิกการกำหนดค่าเครือข่าย
#[derive(Debug, Default)]
pub struct HttpClientConfigBuilder {
    base_url: Option<String>,
    timeout_seconds: Option<u64>,
    max_retries: Option<u32>,
    headers: HashMap<String, String>,
    user_agent: Option<String>,
    follow_redirects: Option<bool>,
    verify_ssl: Option<bool>,
}

impl HttpClientConfigBuilder {
    /// 🎯 เริ่มต้นการออกแบบ Configuration - Start architectural design
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌐 กำหนด Base URL - วางรากฐานการเชื่อมต่อ
    #[must_use]
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// ⏱️ กำหนด Timeout - ตั้งเวลาการรอคอย
    #[must_use]
    pub const fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// 🔄 กำหนดจำนวน Retry - วางแผนการลองใหม่
    #[must_use]
    pub const fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// 📋 เพิ่ม Header - ติดป้ายข้อมูลเพิ่มเติม
    #[must_use]
    pub fn header<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// 🤖 กำหนด User Agent - ระบุตัวตนของ Client
    #[must_use]
    pub fn user_agent<S: Into<String>>(mut self, agent: S) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// 🔀 กำหนดการ Follow Redirects - ติดตามการเปลี่ยนเส้นทาง
    #[must_use]
    pub const fn follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = Some(follow);
        self
    }

    /// 🔒 กำหนดการตรวจสอบ SSL - รักษาความปลอดภัย
    #[must_use]
    pub const fn verify_ssl(mut self, verify: bool) -> Self {
        self.verify_ssl = Some(verify);
        self
    }

    /// 🏗️ สร้าง `HttpClientConfig` จาก Blueprint - Build from architectural design
    /// 
    /// # Errors
    /// 
    /// Returns error ถ้า required fields ไม่ได้ถูกกำหนด
    pub fn build(self) -> Result<HttpClientConfig, String> {
        let base_url = self.base_url.ok_or("base_url is required")?;
        
        Ok(HttpClientConfig {
            base_url,
            timeout_seconds: self.timeout_seconds.unwrap_or(30),
            max_retries: self.max_retries.unwrap_or(3),
            headers: self.headers,
            user_agent: self.user_agent.unwrap_or_else(|| "RustClient/1.0".to_string()),
            follow_redirects: self.follow_redirects.unwrap_or(true),
            verify_ssl: self.verify_ssl.unwrap_or(true),
        })
    }
}

/// 🗄️ Database Connection Blueprint - แบบแปลนการเชื่อมต่อฐานข้อมูล
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub pool_size: u32,
    pub connection_timeout: u64,
    pub ssl_mode: SslMode,
}

/// 🔐 SSL Security Modes - โหมดความปลอดภัย SSL
#[derive(Debug, Clone)]
pub enum SslMode {
    Disable,  // 🚫 ปิดการใช้งาน SSL
    Prefer,   // 🤔 ใช้ SSL หากเป็นไปได้
    Require,  // ✅ บังคับใช้ SSL
}

/// 🏗️ Database Configuration Architect - สถาปนิกการกำหนดค่าฐานข้อมูล
#[derive(Debug)]
pub struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    database: Option<String>,
    username: Option<String>,
    password: Option<String>,
    pool_size: Option<u32>,
    connection_timeout: Option<u64>,
    ssl_mode: Option<SslMode>,
}

impl DatabaseConfigBuilder {
    /// 🎯 เริ่มต้นการออกแบบ Database Configuration
    #[must_use]
    pub const fn new() -> Self {
        Self {
            host: None,
            port: None,
            database: None,
            username: None,
            password: None,
            pool_size: None,
            connection_timeout: None,
            ssl_mode: None,
        }
    }

    /// 🏠 กำหนด Host - ระบุที่อยู่เซิร์ฟเวอร์
    #[must_use]
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    /// 🚪 กำหนด Port - ระบุประตูการเชื่อมต่อ
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// 📚 กำหนด Database - เลือกฐานข้อมูลที่ต้องการ
    #[must_use]
    pub fn database<S: Into<String>>(mut self, db: S) -> Self {
        self.database = Some(db.into());
        self
    }

    /// 🔑 กำหนด Credentials - ระบุข้อมูลการเข้าสู่ระบบ
    #[must_use]
    pub fn credentials<U: Into<String>, P: Into<String>>(mut self, username: U, password: P) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// 🏊 กำหนด Pool Size - ขนาดสระการเชื่อมต่อ
    #[must_use]
    pub const fn pool_size(mut self, size: u32) -> Self {
        self.pool_size = Some(size);
        self
    }

    /// ⏰ กำหนด Connection Timeout - เวลาหมดการเชื่อมต่อ
    #[must_use]
    pub const fn connection_timeout(mut self, timeout: u64) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    /// 🔐 กำหนด SSL Mode - โหมดความปลอดภัย
    #[must_use]
    pub const fn ssl_mode(mut self, mode: SslMode) -> Self {
        self.ssl_mode = Some(mode);
        self
    }

    /// 🏗️ สร้าง `DatabaseConfig` จาก Blueprint - Build from database design
    /// 
    /// # Errors
    /// 
    /// Returns error ถ้า required fields ไม่ได้ถูกกำหนด
    pub fn build(self) -> Result<DatabaseConfig, String> {
        Ok(DatabaseConfig {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(5432),
            database: self.database.ok_or("database name is required")?,
            username: self.username.ok_or("username is required")?,
            password: self.password.ok_or("password is required")?,
            pool_size: self.pool_size.unwrap_or(10),
            connection_timeout: self.connection_timeout.unwrap_or(30),
            ssl_mode: self.ssl_mode.unwrap_or(SslMode::Prefer),
        })
    }
}

impl Default for DatabaseConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 🎭 สาธิต Configuration Architect Workshop - Builder Pattern Demonstration
///
/// # Panics
/// 
/// ฟังก์ชันนี้จะ panic หากไม่สามารถสร้าง HTTP config หรือ DB config ได้
pub fn demonstrate_builder() {
    println!("🏗️ Configuration Architect Workshop - Builder Pattern Examples:");
    
    // HTTP Client Configuration
    println!("\n📡 HTTP Client Configuration Blueprint:");
    
    let http_config = HttpClientConfigBuilder::new()
        .base_url("https://api.example.com")
        .timeout(60)
        .max_retries(5)
        .header("Authorization", "Bearer token123")
        .header("Content-Type", "application/json")
        .user_agent("MyApp/2.0")
        .follow_redirects(true)
        .verify_ssl(true)
        .build()
        .expect("Failed to build HTTP config");
    
    println!("📋 HTTP Configuration Blueprint: {http_config:#?}");
    
    // Database Configuration
    println!("\n🗄️ Database Configuration Blueprint:");
    
    let db_config = DatabaseConfigBuilder::new()
        .host("db.example.com")
        .port(5432)
        .database("myapp_production")
        .credentials("admin", "secure_password")
        .pool_size(20)
        .connection_timeout(45)
        .ssl_mode(SslMode::Require)
        .build()
        .expect("Failed to build DB config");
    
    println!("📊 Database Configuration Blueprint: {db_config:#?}");
    
    // Error handling example
    println!("\n❌ Architectural Validation Example:");
    
    let invalid_config = HttpClientConfigBuilder::new()
        .timeout(30)
        .build();
    
    match invalid_config {
        Ok(config) => println!("✅ Configuration Blueprint Created: {config:?}"),
        Err(e) => println!("🚫 Blueprint Validation Failed: {e}"),
    }
    
    println!("\n💡 Configuration Architect Workshop Benefits:");
    println!("  • 📖 Readable และ maintainable architectural design");
    println!("  • 🛡️ Type-safe configuration blueprints");
    println!("  • 🔄 Flexible parameter ordering system");
    println!("  • ⚙️ Default values support architecture");
    println!("  • ✅ Compile-time validation framework");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_architect() {
        // 🧪 ทดสอบการสร้าง HTTP Configuration Blueprint
        let config = HttpClientConfigBuilder::new()
            .base_url("https://test.com")
            .timeout(10)
            .build()
            .unwrap();
        
        assert_eq!(config.base_url, "https://test.com");
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.max_retries, 3); // default value
    }

    #[test]
    fn test_database_architect() {
        // 🧪 ทดสอบการสร้าง Database Configuration Blueprint
        let config = DatabaseConfigBuilder::new()
            .database("test_db")
            .credentials("user", "pass")
            .build()
            .unwrap();
        
        assert_eq!(config.database, "test_db");
        assert_eq!(config.username, "user");
        assert_eq!(config.host, "localhost"); // default value
    }

    #[test]
    fn test_architectural_validation() {
        // 🧪 ทดสอบการตรวจสอบความถูกต้องของ Blueprint
        let result = HttpClientConfigBuilder::new()
            .timeout(30)
            .build();
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("base_url is required"));
    }
}