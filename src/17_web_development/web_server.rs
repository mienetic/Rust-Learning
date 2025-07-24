//! 🖥️ Web Server Implementation - การสร้างเซิร์ฟเวอร์เว็บ
//! 
//! 🚀 ตัวอย่างการสร้าง web server พื้นฐานด้วย Rust ในเวิร์คช็อปพัฒนาเว็บ

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::{HttpRequest, HttpResponse, HttpStatus};

/// 🌐 Simple Web Server - เซิร์ฟเวอร์เว็บแบบง่าย
pub struct WebServer {
    routes: HashMap<String, Box<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>>,
    middleware: Vec<Box<dyn Fn(&HttpRequest, &HttpResponse) -> HttpResponse + Send + Sync>>,
}

impl Default for WebServer {
    fn default() -> Self {
        Self::new()
    }
}

impl WebServer {
    #[must_use] pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            middleware: Vec::new(),
        }
    }
    
    /// 🛤️ เพิ่ม route ใหม่ - เพิ่มเส้นทางการเข้าถึง
    pub fn route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
    
    /// 🔧 เพิ่ม middleware - เพิ่มตัวกลางประมวลผล
    pub fn use_middleware<F>(&mut self, middleware: F)
    where
        F: Fn(&HttpRequest, &HttpResponse) -> HttpResponse + Send + Sync + 'static,
    {
        self.middleware.push(Box::new(middleware));
    }
    
    /// 📨 จัดการ request - ประมวลผลคำขอ HTTP
    #[must_use] pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        let route_key = format!("{} {}", request.method, request.path);
        
        let mut response = if let Some(handler) = self.routes.get(&route_key) {
            handler(request)
        } else {
            HttpResponse::new(HttpStatus::NotFound)
                .with_body("Route not found")
        };
        
        // Apply middleware
        for middleware in &self.middleware {
            response = middleware(request, &response);
        }
        
        response
    }
}

/// 📁 Static File Server - เซิร์ฟเวอร์ไฟล์คงที่
pub struct StaticFileServer {
    root_dir: String,
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl StaticFileServer {
    #[must_use] pub fn new(root_dir: &str) -> Self {
        Self {
            root_dir: root_dir.to_string(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    #[must_use] pub fn serve_file(&self, path: &str) -> HttpResponse {
        // Simulate file serving
        let file_path = format!("{}/{}", self.root_dir, path.trim_start_matches('/'));
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some(content) = cache.get(&file_path) {
                return self.create_file_response(&file_path, content);
            }
        }
        
        // Simulate reading file
        let content = match path {
            "/index.html" => "<html><body><h1>Welcome to Rust Web Server!</h1></body></html>",
            "/style.css" => "body { font-family: Arial, sans-serif; }",
            "/script.js" => "console.log('Hello from Rust Web Server!');",
            _ => return HttpResponse::new(HttpStatus::NotFound).with_body("File not found"),
        };
        
        // Cache the content
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(file_path.clone(), content.to_string());
        }
        
        self.create_file_response(&file_path, content)
    }
    
    fn create_file_response(&self, file_path: &str, content: &str) -> HttpResponse {
        let content_type = if file_path.ends_with(".html") {
            "text/html"
        } else if file_path.ends_with(".css") {
            "text/css"
        } else if file_path.ends_with(".js") {
            "application/javascript"
        } else {
            "text/plain"
        };
        
        HttpResponse::new(HttpStatus::Ok)
            .with_header("Content-Type", content_type)
            .with_header("Cache-Control", "public, max-age=3600")
            .with_body(content)
    }
}

/// 🔐 Session Management - การจัดการเซสชัน
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub data: HashMap<String, String>,
    pub created_at: std::time::SystemTime,
}

impl Session {
    #[must_use] pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            data: HashMap::new(),
            created_at: std::time::SystemTime::now(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
    
    #[must_use] pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    #[must_use] pub fn is_expired(&self, max_age_secs: u64) -> bool {
        if let Ok(elapsed) = self.created_at.elapsed() {
            elapsed.as_secs() > max_age_secs
        } else {
            true
        }
    }
}

/// 🗂️ Session Manager - ตัวจัดการเซสชัน
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    max_age_secs: u64,
}

impl SessionManager {
    #[must_use] pub fn new(max_age_secs: u64) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            max_age_secs,
        }
    }
    
    #[must_use] pub fn generate_session_id() -> String {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_nanos().hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    #[must_use] pub fn create_session(&self) -> String {
        let session_id = format!("session_{}", Self::generate_session_id());
        let session = Session::new(&session_id);
        
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.insert(session_id.clone(), session);
        }
        
        session_id
    }
    
    #[must_use] pub fn get_session(&self, session_id: &str) -> Option<Session> {
        if let Ok(mut sessions) = self.sessions.lock() {
            if let Some(session) = sessions.get(session_id) {
                if session.is_expired(self.max_age_secs) {
                    sessions.remove(session_id);
                    None
                } else {
                    Some(session.clone())
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn update_session(&self, session_id: &str, session: Session) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.insert(session_id.to_string(), session);
        }
    }
    
    pub fn cleanup_expired_sessions(&self) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.retain(|_, session| !session.is_expired(self.max_age_secs));
        }
    }
}

// Mock UUID module for demonstration
mod uuid {
    pub struct Uuid;
    
    impl Uuid {
        pub const fn new_v4() -> Self {
            Self
        }
    }
}

/// 🎭 ฟังก์ชันสำหรับแสดงตัวอย่างการใช้งานในเวิร์คช็อปพัฒนาเว็บ
pub fn demonstrate_basic_server() {
    println!("🌐 Web Development Workshop - Basic Web Server Example");
    
    let mut server = WebServer::new();
    
    // 🛤️ เพิ่ม routes สำหรับเวิร์คช็อป
    server.route("GET /", |_req| {
        HttpResponse::html(HttpStatus::Ok, "<h1>🌐 Welcome to Web Development Workshop!</h1>")
    });
    
    server.route("GET /api/health", |_req| {
        HttpResponse::json(HttpStatus::Ok, "{\"status\": \"healthy\", \"workshop\": \"web-dev\"}")
    });
    
    server.route("POST /api/users", |req| {
        println!("🔨 Creating workshop user with data: {}", req.body);
        HttpResponse::json(HttpStatus::Created, "{\"id\": 1, \"message\": \"Workshop user created\", \"workshop\": \"web-dev\"}")
    });
    
    // 📝 เพิ่ม logging middleware สำหรับเวิร์คช็อป
    server.use_middleware(|req, res| {
        println!("🔍 [Workshop Log] [{}] {} - {}", req.method, req.path, res.status.as_str());
        res.clone()
    });
    
    // ทดสอบ requests
    let requests = vec![
        HttpRequest::new("GET", "/"),
        HttpRequest::new("GET", "/api/health"),
        HttpRequest::new("POST", "/api/users").with_body("{\"name\": \"John\"}"),
        HttpRequest::new("GET", "/not-found"),
    ];
    
    for request in requests {
        let response = server.handle_request(&request);
        println!("Response: {} - {}", response.status.as_str(), response.body);
    }
    
    // 📁 Static File Server Workshop Example
    println!("\n📁 Web Development Workshop - Static File Server Example");
    let static_server = StaticFileServer::new("./workshop-public");
    
    let file_requests = vec!["/index.html", "/style.css", "/script.js", "/not-found.txt"];
    
    for path in file_requests {
        let response = static_server.serve_file(path);
        println!("📄 Workshop File {}: {} - {} bytes", path, response.status.as_str(), response.body.len());
    }
    
    // 🔐 Session Management Workshop Example
    println!("\n🔐 Web Development Workshop - Session Management Example");
    let session_manager = SessionManager::new(3600); // 1 hour workshop session
    
    let session_id = session_manager.create_session();
    println!("🆔 Created workshop session: {session_id}");
    
    if let Some(mut session) = session_manager.get_session(&session_id) {
        session.set("user_id", "workshop_123");
        session.set("username", "workshop_developer");
        session.set("workshop_role", "web_developer");
        session_manager.update_session(&session_id, session);
        println!("✅ Updated workshop session with user data");
    }
    
    if let Some(session) = session_manager.get_session(&session_id) {
        if let Some(username) = session.get("username") {
            println!("👤 Retrieved workshop username from session: {username}");
        }
        if let Some(role) = session.get("workshop_role") {
            println!("🎭 Workshop role: {role}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_server_routing() {
        let mut server = WebServer::new();
        server.route("GET /test", |_| {
            HttpResponse::new(HttpStatus::Ok).with_body("Test response")
        });
        
        let request = HttpRequest::new("GET", "/test");
        let response = server.handle_request(&request);
        
        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.body, "Test response");
    }

    #[test]
    fn test_static_file_server() {
        let server = StaticFileServer::new("./public");
        let response = server.serve_file("/index.html");
        
        assert_eq!(response.status, HttpStatus::Ok);
        assert!(response.body.contains("Welcome to Rust Web Server!"));
    }

    #[test]
    fn test_session_management() {
        let manager = SessionManager::new(3600);
        let session_id = manager.create_session();
        
        assert!(!session_id.is_empty());
        
        if let Some(mut session) = manager.get_session(&session_id) {
            session.set("test_key", "test_value");
            manager.update_session(&session_id, session);
        }
        
        if let Some(session) = manager.get_session(&session_id) {
            assert_eq!(session.get("test_key"), Some(&"test_value".to_string()));
        }
    }

    #[test]
    fn test_demonstrate_basic_server() {
        // Test that the function runs without panicking
        demonstrate_basic_server();
    }
}