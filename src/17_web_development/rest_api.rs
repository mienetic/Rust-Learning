//! 📡 REST API Implementation - การสร้าง REST API
//! 
//! 🚀 ตัวอย่างการสร้าง REST API ด้วย Rust ในเวิร์คช็อปพัฒนาเว็บ

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::{HttpRequest, HttpResponse, HttpStatus};

// Mock serde for demonstration
mod serde {
    
    pub use self::ser::*;
    
    pub mod de {
        pub trait Deserialize<'de> {}
        pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
    }
    
    pub mod ser {
        pub trait Serialize {}
    }
}

// Mock derive macros
macro_rules! mock_derive {
    ($name:ident for $type:ident) => {
        impl serde::ser::Serialize for $type {}
        impl<'de> serde::de::Deserialize<'de> for $type {}
        impl serde::de::DeserializeOwned for $type {}
    };
    ($name:ident for $type:ident<$generic:ident>) => {
        impl<$generic> serde::ser::Serialize for $type<$generic> {}
        impl<'de, $generic> serde::de::Deserialize<'de> for $type<$generic> {}
        impl<$generic> serde::de::DeserializeOwned for $type<$generic> {}
    };
}

/// 👤 User Model for Workshop API - โมเดลผู้ใช้สำหรับ API
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: u32,
}
mock_derive!(User for User);

impl User {
    #[must_use] pub fn new(id: u32, name: &str, email: &str, age: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }
}

/// 📦 API Response Wrapper - ตัวห่อหุ้มการตอบกลับ API
#[derive(Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}
mock_derive!(ApiResponse for ApiResponse<T>);

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }
    
    #[must_use] pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
}

/// 🗄️ User Repository (Workshop In-memory Database) - คลังข้อมูลผู้ใช้
pub struct UserRepository {
    users: Arc<Mutex<HashMap<u32, User>>>,
    next_id: Arc<Mutex<u32>>,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository {
    #[must_use] pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // 📊 Add workshop sample data
        users.insert(1, User::new(1, "Workshop John", "john@workshop.dev", 30));
        users.insert(2, User::new(2, "Workshop Jane", "jane@workshop.dev", 25));
        users.insert(3, User::new(3, "Workshop Bob", "bob@workshop.dev", 35));
        
        Self {
            users: Arc::new(Mutex::new(users)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
    
    #[must_use] pub fn get_all(&self) -> Vec<User> {
        if let Ok(users) = self.users.lock() {
            users.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    #[must_use] pub fn get_by_id(&self, id: u32) -> Option<User> {
        if let Ok(users) = self.users.lock() {
            users.get(&id).cloned()
        } else {
            None
        }
    }
    
    pub fn create(&self, mut user: User) -> Result<User, String> {
        if let (Ok(mut users), Ok(mut next_id)) = (self.users.lock(), self.next_id.lock()) {
            user.id = *next_id;
            *next_id += 1;
            
            // Validate user data
            if user.name.is_empty() {
                return Err("Name cannot be empty".to_string());
            }
            if user.email.is_empty() || !user.email.contains('@') {
                return Err("Invalid email address".to_string());
            }
            if user.age == 0 || user.age > 150 {
                return Err("Invalid age".to_string());
            }
            
            users.insert(user.id, user.clone());
            Ok(user)
        } else {
            Err("Database error".to_string())
        }
    }
    
    pub fn update(&self, id: u32, updated_user: User) -> Result<User, String> {
        if let Ok(mut users) = self.users.lock() {
            if let std::collections::hash_map::Entry::Occupied(mut e) = users.entry(id) {
                let mut user = updated_user;
                user.id = id; // Ensure ID doesn't change
                
                // Validate user data
                if user.name.is_empty() {
                    return Err("Name cannot be empty".to_string());
                }
                if user.email.is_empty() || !user.email.contains('@') {
                    return Err("Invalid email address".to_string());
                }
                if user.age == 0 || user.age > 150 {
                    return Err("Invalid age".to_string());
                }
                
                e.insert(user.clone());
                Ok(user)
            } else {
                Err("User not found".to_string())
            }
        } else {
            Err("Database error".to_string())
        }
    }
    
    pub fn delete(&self, id: u32) -> Result<(), String> {
        if let Ok(mut users) = self.users.lock() {
            if users.remove(&id).is_some() {
                Ok(())
            } else {
                Err("User not found".to_string())
            }
        } else {
            Err("Database error".to_string())
        }
    }
    
    #[must_use] pub fn search(&self, query: &str) -> Vec<User> {
        if let Ok(users) = self.users.lock() {
            users.values()
                .filter(|user| {
                    user.name.to_lowercase().contains(&query.to_lowercase()) ||
                    user.email.to_lowercase().contains(&query.to_lowercase())
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// 🎮 REST API Controller - ตัวควบคุม REST API
pub struct UserController {
    repository: UserRepository,
}

impl Default for UserController {
    fn default() -> Self {
        Self::new()
    }
}

impl UserController {
    #[must_use] pub fn new() -> Self {
        Self {
            repository: UserRepository::new(),
        }
    }
    
    /// GET /api/users
    #[must_use] pub fn get_users(&self, request: &HttpRequest) -> HttpResponse {
        // Check for search query parameter
        if let Some(query) = self.extract_query_param(&request.path, "search") {
            let users = self.repository.search(&query);
            let response = ApiResponse::success(users);
            return self.json_response(HttpStatus::Ok, &response);
        }
        
        let users = self.repository.get_all();
        let response = ApiResponse::success(users);
        self.json_response(HttpStatus::Ok, &response)
    }
    
    /// GET /api/users/{id}
    #[must_use] pub fn get_user(&self, id: u32) -> HttpResponse {
        if let Some(user) = self.repository.get_by_id(id) {
            let response = ApiResponse::success(user);
            self.json_response(HttpStatus::Ok, &response)
        } else {
            let response: ApiResponse<()> = ApiResponse::error("User not found");
            self.json_response(HttpStatus::NotFound, &response)
        }
    }
    
    /// POST /api/users
    #[must_use] pub fn create_user(&self, request: &HttpRequest) -> HttpResponse {
        match self.parse_json::<User>(&request.body) {
            Ok(user) => {
                match self.repository.create(user) {
                    Ok(created_user) => {
                        let response = ApiResponse::success(created_user);
                        self.json_response(HttpStatus::Created, &response)
                    }
                    Err(error) => {
                        let response: ApiResponse<()> = ApiResponse::error(&error);
                        self.json_response(HttpStatus::BadRequest, &response)
                    }
                }
            }
            Err(error) => {
                let response: ApiResponse<()> = ApiResponse::error(&error);
                self.json_response(HttpStatus::BadRequest, &response)
            }
        }
    }
    
    /// PUT /api/users/{id}
    #[must_use] pub fn update_user(&self, id: u32, request: &HttpRequest) -> HttpResponse {
        match self.parse_json::<User>(&request.body) {
            Ok(user) => {
                match self.repository.update(id, user) {
                    Ok(updated_user) => {
                        let response = ApiResponse::success(updated_user);
                        self.json_response(HttpStatus::Ok, &response)
                    }
                    Err(error) => {
                        let status = if error == "User not found" {
                            HttpStatus::NotFound
                        } else {
                            HttpStatus::BadRequest
                        };
                        let response: ApiResponse<()> = ApiResponse::error(&error);
                        self.json_response(status, &response)
                    }
                }
            }
            Err(error) => {
                let response: ApiResponse<()> = ApiResponse::error(&error);
                self.json_response(HttpStatus::BadRequest, &response)
            }
        }
    }
    
    /// DELETE /api/users/{id}
    #[must_use] pub fn delete_user(&self, id: u32) -> HttpResponse {
        match self.repository.delete(id) {
            Ok(()) => {
                let response: ApiResponse<()> = ApiResponse::success(());
                self.json_response(HttpStatus::Ok, &response)
            }
            Err(error) => {
                let response: ApiResponse<()> = ApiResponse::error(&error);
                self.json_response(HttpStatus::NotFound, &response)
            }
        }
    }
    
    // Helper methods
    fn json_response<T: serde::Serialize>(&self, status: HttpStatus, data: &T) -> HttpResponse {
        let json = self.serialize_json(data);
        HttpResponse::json(status, &json)
    }
    
    fn parse_json<T: serde::de::DeserializeOwned>(&self, json_str: &str) -> Result<T, String> {
        // Simplified JSON parsing for demonstration
        // In real implementation, use serde_json
        if json_str.contains("\"name\": \"John\"") {
            let user = User::new(0, "John", "john@example.com", 30);
            // This is a hack for demonstration - normally use serde_json::from_str
            Ok(unsafe { std::mem::transmute_copy(&user) })
        } else if json_str.contains("\"name\": \"Alice\"") {
            let user = User::new(0, "Alice", "alice@example.com", 28);
            Ok(unsafe { std::mem::transmute_copy(&user) })
        } else {
            Err("Invalid JSON format".to_string())
        }
    }
    
    fn serialize_json<T: serde::Serialize>(&self, data: &T) -> String {
        // Simplified JSON serialization for demonstration
        // In real implementation, use serde_json
        "{\"serialized\": \"data\"}".to_string() // Placeholder
    }
    
    fn extract_query_param(&self, path: &str, param: &str) -> Option<String> {
        if let Some(query_start) = path.find('?') {
            let query_string = &path[query_start + 1..];
            for pair in query_string.split('&') {
                if let Some(eq_pos) = pair.find('=') {
                    let key = &pair[..eq_pos];
                    let value = &pair[eq_pos + 1..];
                    if key == param {
                        return Some(value.to_string());
                    }
                }
            }
        }
        None
    }
}

/// 🛤️ API Router - ตัวจัดเส้นทาง API
pub struct ApiRouter {
    controller: UserController,
}

impl Default for ApiRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRouter {
    #[must_use] pub fn new() -> Self {
        Self {
            controller: UserController::new(),
        }
    }
    
    #[must_use] pub fn route(&self, request: &HttpRequest) -> HttpResponse {
        let path_parts: Vec<&str> = request.path.split('/').collect();
        
        match (request.method.as_str(), path_parts.as_slice()) {
            ("GET", ["", "api", "users"]) => self.controller.get_users(request),
            ("GET", ["", "api", "users", id_str]) => {
                if let Ok(id) = id_str.parse::<u32>() {
                    self.controller.get_user(id)
                } else {
                    let response: ApiResponse<()> = ApiResponse::error("Invalid user ID");
                    HttpResponse::json(HttpStatus::BadRequest, "{\"error\": \"Invalid user ID\"}")
                }
            }
            ("POST", ["", "api", "users"]) => self.controller.create_user(request),
            ("PUT", ["", "api", "users", id_str]) => {
                if let Ok(id) = id_str.parse::<u32>() {
                    self.controller.update_user(id, request)
                } else {
                    let response: ApiResponse<()> = ApiResponse::error("Invalid user ID");
                    HttpResponse::json(HttpStatus::BadRequest, "{\"error\": \"Invalid user ID\"}")
                }
            }
            ("DELETE", ["", "api", "users", id_str]) => {
                if let Ok(id) = id_str.parse::<u32>() {
                    self.controller.delete_user(id)
                } else {
                    let response: ApiResponse<()> = ApiResponse::error("Invalid user ID");
                    HttpResponse::json(HttpStatus::BadRequest, "{\"error\": \"Invalid user ID\"}")
                }
            }
            _ => {
                let response: ApiResponse<()> = ApiResponse::error("Endpoint not found");
                HttpResponse::json(HttpStatus::NotFound, "{\"error\": \"Endpoint not found\"}")
            }
        }
    }
}



/// 🎭 ฟังก์ชันสำหรับแสดงตัวอย่างการใช้งานในเวิร์คช็อปพัฒนาเว็บ
pub fn demonstrate_rest_api() {
    println!("📡 Web Development Workshop - REST API Example");
    
    let router = ApiRouter::new();
    
    // Test API endpoints
    let test_requests = vec![
        // GET all users
        HttpRequest::new("GET", "/api/users"),
        
        // GET user by ID
        HttpRequest::new("GET", "/api/users/1"),
        
        // Search users
        HttpRequest::new("GET", "/api/users?search=john"),
        
        // 🔨 CREATE new workshop user
        HttpRequest::new("POST", "/api/users")
            .with_header("Content-Type", "application/json")
            .with_body("{\"name\": \"Workshop John\", \"email\": \"john@workshop.dev\", \"age\": 30}"),
        
        // ✏️ UPDATE workshop user
        HttpRequest::new("PUT", "/api/users/1")
            .with_header("Content-Type", "application/json")
            .with_body("{\"name\": \"Workshop Alice\", \"email\": \"alice@workshop.dev\", \"age\": 28}"),
        
        // DELETE user
        HttpRequest::new("DELETE", "/api/users/2"),
        
        // Invalid endpoint
        HttpRequest::new("GET", "/api/invalid"),
    ];
    
    for (i, request) in test_requests.iter().enumerate() {
        println!("\n--- Request {} ---", i + 1);
        println!("{} {}", request.method, request.path);
        if !request.body.is_empty() {
            println!("Body: {}", request.body);
        }
        
        let response = router.route(request);
        println!("Response: {}", response.status.as_str());
        if !response.body.is_empty() {
            println!("Body: {}", response.body);
        }
    }
    
    // 🗄️ Demonstrate workshop repository operations directly
    println!("\n📊 Workshop Repository Operations");
    let repo = UserRepository::new();
    
    println!("🔢 Total workshop users: {}", repo.get_all().len());
    
    if let Some(user) = repo.get_by_id(1) {
        println!("👤 Workshop User 1: {} ({})", user.name, user.email);
    }
    
    let search_results = repo.search("workshop");
    println!("🔍 Search results for 'workshop': {} users", search_results.len());
    
    // 🧪 Test workshop user creation
    let new_user = User::new(0, "Workshop Test User", "test@workshop.dev", 25);
    match repo.create(new_user) {
        Ok(created) => println!("✅ Created workshop user with ID: {}", created.id),
        Err(error) => println!("❌ Error creating workshop user: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_repository() {
        let repo = UserRepository::new();
        
        // Test get all
        let users = repo.get_all();
        assert!(!users.is_empty());
        
        // Test get by ID
        let user = repo.get_by_id(1);
        assert!(user.is_some());
        
        // Test create
        let new_user = User::new(0, "Test", "test@example.com", 25);
        let created = repo.create(new_user);
        assert!(created.is_ok());
    }

    #[test]
    fn test_api_router() {
        let router = ApiRouter::new();
        
        // Test GET users
        let request = HttpRequest::new("GET", "/api/users");
        let response = router.route(&request);
        assert_eq!(response.status, HttpStatus::Ok);
        
        // Test invalid endpoint
        let request = HttpRequest::new("GET", "/api/invalid");
        let response = router.route(&request);
        assert_eq!(response.status, HttpStatus::NotFound);
    }

    #[test]
    fn test_user_validation() {
        let repo = UserRepository::new();
        
        // Test invalid user creation
        let invalid_user = User::new(0, "", "invalid-email", 0);
        let result = repo.create(invalid_user);
        assert!(result.is_err());
    }

    #[test]
    fn test_demonstrate_rest_api() {
        // Test that the function runs without panicking
        demonstrate_rest_api();
    }
}