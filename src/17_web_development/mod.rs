//! 🌐 บทที่ 17: Web Development Workshop - เวิร์คช็อปพัฒนาเว็บ
//! 
//! 🚀 บทนี้จะสอนการพัฒนาเว็บแอปพลิเคชันด้วย Rust
//! 📡 ครอบคลุมการสร้าง web server, REST API, และการจัดการ HTTP requests

pub mod web_server;
pub mod rest_api;
pub mod middleware;
pub mod templating;

use std::collections::HashMap;

/// 🎭 ฟังก์ชันหลักสำหรับรันตัวอย่างทั้งหมดในเวิร์คช็อปพัฒนาเว็บ
pub fn run_web_development_examples() {
    println!("\n🌐 === Web Development Workshop - เวิร์คช็อปพัฒนาเว็บ === 🌐");
    
    // Basic Web Server
    println!("\n🖥️ --- Basic Web Server - เซิร์ฟเวอร์เว็บพื้นฐาน ---");
    web_server::demonstrate_basic_server();
    
    // REST API
    println!("\n📡 --- REST API - API แบบ RESTful ---");
    rest_api::demonstrate_rest_api();
    
    // Middleware
    println!("\n🔧 --- Middleware - มิดเดิลแวร์ ---");
    middleware::demonstrate_middleware();
    
    // Templating
    println!("\n📄 --- Templating - ระบบเทมเพลต ---");
    templating::demonstrate_templating();
    
    println!("\n✅ Web Development Workshop เสร็จสมบูรณ์! 🎉");
}

/// 📊 HTTP Status Codes - รหัสสถานะ HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl HttpStatus {
    #[must_use] pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "200 OK",
            Self::Created => "201 Created",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
            Self::InternalServerError => "500 Internal Server Error",
        }
    }
}

/// 📨 HTTP Request Structure - โครงสร้างคำขอ HTTP
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    #[must_use] pub fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    
    #[must_use] pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    #[must_use] pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }
}

/// 📤 HTTP Response Structure - โครงสร้างการตอบกลับ HTTP
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    #[must_use] pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    
    #[must_use] pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    #[must_use] pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }
    
    #[must_use] pub fn json(status: HttpStatus, data: &str) -> Self {
        Self::new(status)
            .with_header("Content-Type", "application/json")
            .with_body(data)
    }
    
    #[must_use] pub fn html(status: HttpStatus, html: &str) -> Self {
        Self::new(status)
            .with_header("Content-Type", "text/html")
            .with_body(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status() {
        assert_eq!(HttpStatus::Ok.as_str(), "200 OK");
        assert_eq!(HttpStatus::NotFound.as_str(), "404 Not Found");
    }

    #[test]
    fn test_http_request() {
        let request = HttpRequest::new("GET", "/api/users")
            .with_header("Authorization", "Bearer token")
            .with_body("{\"name\": \"John\"}");
        
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/api/users");
        assert_eq!(request.headers.get("Authorization"), Some(&"Bearer token".to_string()));
        assert_eq!(request.body, "{\"name\": \"John\"}");
    }

    #[test]
    fn test_http_response() {
        let response = HttpResponse::json(HttpStatus::Ok, "{\"message\": \"success\"}");
        
        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(response.body, "{\"message\": \"success\"}");
    }

    #[test]
    fn test_run_web_development_examples() {
        // Test that the function runs without panicking
        run_web_development_examples();
    }
}