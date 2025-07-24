//! 🔧 Middleware Implementation - การสร้างมิดเดิลแวร์
//! 
//! 🚀 ตัวอย่างการสร้าง middleware สำหรับ web applications ในเวิร์คช็อปพัฒนาเว็บ

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use super::{HttpRequest, HttpResponse, HttpStatus};

/// 🎭 Middleware Trait - เทรตมิดเดิลแวร์
pub trait Middleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse;
}

/// 📝 Logging Middleware - มิดเดิลแวร์บันทึกล็อก
pub struct LoggingMiddleware {
    pub enabled: bool,
    pub log_body: bool,
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl LoggingMiddleware {
    #[must_use] pub const fn new() -> Self {
        Self {
            enabled: true,
            log_body: false,
        }
    }
    
    #[must_use] pub const fn with_body_logging(mut self) -> Self {
        self.log_body = true;
        self
    }
    
    fn get_timestamp() -> String {
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            format!("{}", duration.as_secs())
        } else {
            "unknown".to_string()
        }
    }
}

impl Middleware for LoggingMiddleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse {
        if self.enabled {
            let timestamp = Self::get_timestamp();
            let user_agent = request.headers.get("User-Agent")
                .map_or("Unknown", std::string::String::as_str);
            
            println!(
                "[{}] {} {} - {} - User-Agent: {}",
                timestamp,
                request.method,
                request.path,
                response.status.as_str(),
                user_agent
            );
            
            if self.log_body && !request.body.is_empty() {
                println!("Request Body: {}", request.body);
            }
            
            if self.log_body && !response.body.is_empty() {
                println!("Response Body: {}", response.body);
            }
        }
        
        response.clone()
    }
}

/// 🌐 CORS Middleware - มิดเดิลแวร์ CORS
pub struct CorsMiddleware {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl CorsMiddleware {
    #[must_use] pub fn new() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
            allow_credentials: false,
        }
    }
    
    #[must_use] pub fn allow_origin(mut self, origin: &str) -> Self {
        if self.allowed_origins.contains(&"*".to_string()) {
            self.allowed_origins.clear();
        }
        self.allowed_origins.push(origin.to_string());
        self
    }
    
    #[must_use] pub const fn allow_credentials(mut self) -> Self {
        self.allow_credentials = true;
        self
    }
    
    fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) ||
        self.allowed_origins.contains(&origin.to_string())
    }
}

impl Middleware for CorsMiddleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse {
        let mut new_response = response.clone();
        
        // Handle preflight requests
        if request.method == "OPTIONS" {
            new_response = HttpResponse::new(HttpStatus::Ok);
        }
        
        // Add CORS headers
        if let Some(origin) = request.headers.get("Origin") {
            if self.is_origin_allowed(origin) {
                new_response = new_response.with_header("Access-Control-Allow-Origin", origin);
            }
        } else {
            new_response = new_response.with_header("Access-Control-Allow-Origin", "*");
        }
        
        new_response = new_response
            .with_header("Access-Control-Allow-Methods", &self.allowed_methods.join(", "))
            .with_header("Access-Control-Allow-Headers", &self.allowed_headers.join(", "));
        
        if self.allow_credentials {
            new_response = new_response.with_header("Access-Control-Allow-Credentials", "true");
        }
        
        new_response
    }
}

/// 🔐 Authentication Middleware - มิดเดิลแวร์การยืนยันตัวตน
pub struct AuthMiddleware {
    pub protected_paths: Vec<String>,
    pub api_keys: HashMap<String, String>, // API Key -> User ID
    pub jwt_secret: String,
}

impl AuthMiddleware {
    #[must_use] pub fn new(jwt_secret: &str) -> Self {
        let mut api_keys = HashMap::new();
        api_keys.insert("workshop_api_123".to_string(), "workshop_user_1".to_string());
        api_keys.insert("workshop_api_456".to_string(), "workshop_user_2".to_string());
        
        Self {
            protected_paths: vec![
                "/api/admin".to_string(),
                "/api/users".to_string(),
            ],
            api_keys,
            jwt_secret: jwt_secret.to_string(),
        }
    }
    
    #[must_use] pub fn add_protected_path(mut self, path: &str) -> Self {
        self.protected_paths.push(path.to_string());
        self
    }
    
    fn is_protected_path(&self, path: &str) -> bool {
        self.protected_paths.iter().any(|protected| path.starts_with(protected))
    }
    
    fn validate_api_key(&self, api_key: &str) -> Option<String> {
        self.api_keys.get(api_key).cloned()
    }
    
    fn validate_jwt(&self, token: &str) -> Option<String> {
        // Simplified JWT validation for demonstration
        // In real implementation, use a proper JWT library
        if token.starts_with("eyJ") && token.len() > 20 {
            Some("user_from_jwt".to_string())
        } else {
            None
        }
    }
    
    fn extract_bearer_token(&self, auth_header: &str) -> Option<String> {
        if auth_header.starts_with("Bearer ") {
            Some(auth_header[7..].to_string())
        } else {
            None
        }
    }
}

impl Middleware for AuthMiddleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse {
        // Skip authentication for non-protected paths
        if !self.is_protected_path(&request.path) {
            return response.clone();
        }
        
        // Check for API Key authentication
        if let Some(api_key) = request.headers.get("X-API-Key") {
            if let Some(_user_id) = self.validate_api_key(api_key) {
                return response.clone(); // Authentication successful
            }
        }
        
        // Check for JWT authentication
        if let Some(auth_header) = request.headers.get("Authorization") {
            if let Some(token) = self.extract_bearer_token(auth_header) {
                if let Some(_user_id) = self.validate_jwt(&token) {
                    return response.clone(); // Authentication successful
                }
            }
        }
        
        // Authentication failed
        HttpResponse::json(
            HttpStatus::BadRequest,
            "{\"error\": \"Authentication required\"}"
        )
    }
}

/// ⏱️ Rate Limiting Middleware - มิดเดิลแวร์จำกัดอัตรา
pub struct RateLimitMiddleware {
    pub requests_per_minute: u32,
    pub requests: HashMap<String, Vec<u64>>, // IP -> timestamps
}

impl RateLimitMiddleware {
    #[must_use] pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            requests: HashMap::new(),
        }
    }
    
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
    
    fn get_client_ip(&self, request: &HttpRequest) -> String {
        // Try to get real IP from headers
        if let Some(forwarded_for) = request.headers.get("X-Forwarded-For") {
            if let Some(ip) = forwarded_for.split(',').next() {
                return ip.trim().to_string();
            }
        }
        
        if let Some(real_ip) = request.headers.get("X-Real-IP") {
            return real_ip.clone();
        }
        
        // Fallback to a simulated IP
        "127.0.0.1".to_string()
    }
    
    fn is_rate_limited(&mut self, client_ip: &str) -> bool {
        let current_time = Self::get_current_timestamp();
        let minute_ago = current_time - 60;
        
        // Get or create request history for this IP
        let requests = self.requests.entry(client_ip.to_string()).or_default();
        
        // Remove old requests (older than 1 minute)
        requests.retain(|&timestamp| timestamp > minute_ago);
        
        // Check if rate limit is exceeded
        if requests.len() >= self.requests_per_minute as usize {
            return true;
        }
        
        // Add current request
        requests.push(current_time);
        false
    }
}

impl Middleware for RateLimitMiddleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse {
        let client_ip = self.get_client_ip(request);
        
        // Note: In real implementation, this should use Arc<Mutex<Self>> to allow mutation
        // For demonstration, we'll simulate the check
        let simulated_request_count = client_ip.len() % 10; // Simulate based on IP length
        
        if simulated_request_count > 7 { // Simulate rate limit exceeded
            return HttpResponse::json(
                HttpStatus::BadRequest,
                "{\"error\": \"Rate limit exceeded. Please try again later.\"}"
            ).with_header("Retry-After", "60");
        }
        
        // Add rate limit headers to response
        response.clone()
            .with_header("X-RateLimit-Limit", &self.requests_per_minute.to_string())
            .with_header("X-RateLimit-Remaining", &(self.requests_per_minute - simulated_request_count as u32).to_string())
    }
}

/// 🗜️ Compression Middleware - มิดเดิลแวร์บีบอัด
pub struct CompressionMiddleware {
    pub min_size: usize,
    pub compression_types: Vec<String>,
}

impl Default for CompressionMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressionMiddleware {
    #[must_use] pub fn new() -> Self {
        Self {
            min_size: 1024, // Only compress responses larger than 1KB
            compression_types: vec![
                "text/html".to_string(),
                "text/css".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
            ],
        }
    }
    
    fn should_compress(&self, response: &HttpResponse) -> bool {
        // Check response size
        if response.body.len() < self.min_size {
            return false;
        }
        
        // Check content type
        if let Some(content_type) = response.headers.get("Content-Type") {
            return self.compression_types.iter().any(|ct| content_type.contains(ct));
        }
        
        false
    }
    
    fn compress_response(&self, response: &HttpResponse) -> HttpResponse {
        // Simulate compression (in real implementation, use gzip/deflate)
        let compressed_body = format!("[COMPRESSED:{}]", response.body.len());
        
        response.clone()
            .with_body(&compressed_body)
            .with_header("Content-Encoding", "gzip")
            .with_header("Content-Length", &compressed_body.len().to_string())
    }
}

impl Middleware for CompressionMiddleware {
    fn process(&self, request: &HttpRequest, response: &HttpResponse) -> HttpResponse {
        // Check if client accepts compression
        if let Some(accept_encoding) = request.headers.get("Accept-Encoding") {
            if accept_encoding.contains("gzip") && self.should_compress(response) {
                return self.compress_response(response);
            }
        }
        
        response.clone()
    }
}

/// ⛓️ Middleware Chain - ห่วงโซ่มิดเดิลแวร์
pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware + Send + Sync>>,
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

impl MiddlewareChain {
    #[must_use] pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }
    
    pub fn add<M: Middleware + Send + Sync + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }
    
    #[must_use] pub fn process(&self, request: &HttpRequest, mut response: HttpResponse) -> HttpResponse {
        for middleware in &self.middlewares {
            response = middleware.process(request, &response);
        }
        response
    }
}

/// 🎭 ฟังก์ชันสำหรับแสดงตัวอย่างการใช้งานในเวิร์คช็อปพัฒนาเว็บ
pub fn demonstrate_middleware() {
    println!("🔧 Web Development Workshop - Middleware Example");
    
    // 🔗 สร้างห่วงโซ่มิดเดิลแวร์สำหรับเวิร์คช็อป
    let middleware_chain = MiddlewareChain::new()
        .add(LoggingMiddleware::new().with_body_logging())
        .add(CorsMiddleware::new().allow_origin("https://workshop.example.com").allow_credentials())
        .add(AuthMiddleware::new("workshop_secret_key_123"))
        .add(RateLimitMiddleware::new(60)) // 60 requests per minute
        .add(CompressionMiddleware::new());
    
    // 🧪 ทดสอบคำขอต่างๆในเวิร์คช็อป
    let test_requests = vec![
        // จุดสิ้นสุดสาธารณะ (ไม่ต้องยืนยันตัวตน)
        (HttpRequest::new("GET", "/public")
            .with_header("User-Agent", "Mozilla/5.0")
            .with_header("Accept-Encoding", "gzip, deflate"),
         HttpResponse::new(HttpStatus::Ok)
            .with_header("Content-Type", "text/html")
            .with_body("<html><body><h1>หน้าสาธารณะเวิร์คช็อป</h1><p>นี่คือหน้าสาธารณะที่ทุกคนสามารถเข้าถึงได้</p></body></html>")),
        
        // จุดสิ้นสุดที่ป้องกันด้วย API key ที่ถูกต้อง
        (HttpRequest::new("GET", "/api/users")
            .with_header("X-API-Key", "workshop_api_123")
            .with_header("User-Agent", "Workshop API Client 1.0")
            .with_header("Accept-Encoding", "gzip"),
         HttpResponse::json(HttpStatus::Ok, "{\"users\": [{\"id\": 1, \"name\": \"John Workshop\"}]}")),
        
        // จุดสิ้นสุดที่ป้องกันด้วย JWT ที่ถูกต้อง
        (HttpRequest::new("POST", "/api/users")
            .with_header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")
            .with_header("Content-Type", "application/json")
            .with_body("{\"name\": \"Alice Workshop\", \"email\": \"alice@workshop.example.com\"}"),
         HttpResponse::json(HttpStatus::Created, "{\"id\": 2, \"message\": \"สร้างผู้ใช้เวิร์คช็อปสำเร็จ\"}")),
        
        // จุดสิ้นสุดที่ป้องกันโดยไม่มีการยืนยันตัวตน
        (HttpRequest::new("DELETE", "/api/users/1")
            .with_header("User-Agent", "Unauthorized Workshop Client"),
         HttpResponse::new(HttpStatus::Ok)),
        
        // คำขอ CORS preflight
        (HttpRequest::new("OPTIONS", "/api/users")
            .with_header("Origin", "https://workshop.example.com")
            .with_header("Access-Control-Request-Method", "POST")
            .with_header("Access-Control-Request-Headers", "Content-Type"),
         HttpResponse::new(HttpStatus::Ok)),
    ];
    
    for (i, (request, initial_response)) in test_requests.into_iter().enumerate() {
        println!("\n--- Test {} ---", i + 1);
        println!("Request: {} {}", request.method, request.path);
        
        // Show request headers
        for (key, value) in &request.headers {
            println!("  {key}: {value}");
        }
        
        if !request.body.is_empty() {
            println!("  Body: {}", request.body);
        }
        
        // Process through middleware chain
        let final_response = middleware_chain.process(&request, initial_response);
        
        println!("Final Response: {}", final_response.status.as_str());
        
        // Show response headers
        for (key, value) in &final_response.headers {
            println!("  {key}: {value}");
        }
        
        if !final_response.body.is_empty() {
            println!("  Body: {}", final_response.body);
        }
    }
    
    // Demonstrate individual middleware
    println!("\n🔍 Individual Middleware Tests");
    
    // Test rate limiting
    println!("\n--- Rate Limiting Test ---");
    let rate_limiter = RateLimitMiddleware::new(5);
    let test_request = HttpRequest::new("GET", "/api/test")
        .with_header("X-Forwarded-For", "192.168.1.100");
    let test_response = HttpResponse::new(HttpStatus::Ok).with_body("Test response");
    
    let limited_response = rate_limiter.process(&test_request, &test_response);
    println!("Rate limit headers added: {:?}", limited_response.headers.get("X-RateLimit-Limit"));
    
    // Test compression
    println!("\n--- Compression Test ---");
    let compressor = CompressionMiddleware::new();
    let large_response = HttpResponse::new(HttpStatus::Ok)
        .with_header("Content-Type", "application/json")
        .with_body(&"x".repeat(2000)); // Large response
    
    let compressed_request = HttpRequest::new("GET", "/api/data")
        .with_header("Accept-Encoding", "gzip, deflate");
    
    let compressed_response = compressor.process(&compressed_request, &large_response);
    println!("Compression applied: {}", compressed_response.headers.get("Content-Encoding").unwrap_or(&"none".to_string()));
    println!("Original size: {} bytes, Compressed: {} bytes", large_response.body.len(), compressed_response.body.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_middleware() {
        let middleware = LoggingMiddleware::new();
        let request = HttpRequest::new("GET", "/test");
        let response = HttpResponse::new(HttpStatus::Ok);
        
        let result = middleware.process(&request, &response);
        assert_eq!(result.status, HttpStatus::Ok);
    }

    #[test]
    fn test_cors_middleware() {
        let middleware = CorsMiddleware::new().allow_origin("https://example.com");
        let request = HttpRequest::new("GET", "/api/test")
            .with_header("Origin", "https://example.com");
        let response = HttpResponse::new(HttpStatus::Ok);
        
        let result = middleware.process(&request, &response);
        assert!(result.headers.contains_key("Access-Control-Allow-Origin"));
    }

    #[test]
    fn test_auth_middleware() {
        let middleware = AuthMiddleware::new("secret");
        
        // Test protected path without auth
        let request = HttpRequest::new("GET", "/api/users");
        let response = HttpResponse::new(HttpStatus::Ok);
        let result = middleware.process(&request, &response);
        assert_eq!(result.status, HttpStatus::BadRequest);
        
        // Test protected path with valid API key
        let request = HttpRequest::new("GET", "/api/users")
            .with_header("X-API-Key", "workshop_api_123");
        let result = middleware.process(&request, &response);
        assert_eq!(result.status, HttpStatus::Ok);
    }

    #[test]
    fn test_middleware_chain() {
        let chain = MiddlewareChain::new()
            .add(LoggingMiddleware::new())
            .add(CorsMiddleware::new());
        
        let request = HttpRequest::new("GET", "/test");
        let response = HttpResponse::new(HttpStatus::Ok);
        
        let result = chain.process(&request, response);
        assert_eq!(result.status, HttpStatus::Ok);
    }

    #[test]
    fn test_demonstrate_middleware() {
        // Test that the function runs without panicking
        demonstrate_middleware();
    }
}