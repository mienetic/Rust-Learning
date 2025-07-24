//! Type State Pattern Implementation
//!
//! การใช้งาน Type State Pattern ใน Rust
//! ครอบคลุม State Machines, Protocol Implementation, และ Compile-time Safety

use std::marker::PhantomData;
// use std::fmt;

/// Connection states
#[derive(Debug)]
pub struct Disconnected;

#[derive(Debug)]
pub struct Connected;

#[derive(Debug)]
pub struct Authenticated;

/// Database connection with type state
#[derive(Debug)]
pub struct DatabaseConnection<State> {
    host: String,
    port: u16,
    username: Option<String>,
    _state: PhantomData<State>,
}

/// Implementation for Disconnected state
impl DatabaseConnection<Disconnected> {
    #[must_use] pub fn new(host: String, port: u16) -> Self {
        println!("🔌 Creating new database connection to {host}:{port}");
        Self {
            host,
            port,
            username: None,
            _state: PhantomData,
        }
    }

    pub fn connect(self) -> Result<DatabaseConnection<Connected>, String> {
        println!("🔗 Connecting to {}:{}...", self.host, self.port);
        
        // Simulate connection logic
        if self.host.is_empty() {
            return Err("Invalid host".to_string());
        }
        
        println!("✅ Connected successfully!");
        Ok(DatabaseConnection {
            host: self.host,
            port: self.port,
            username: None,
            _state: PhantomData,
        })
    }
}

/// Implementation for Connected state
impl DatabaseConnection<Connected> {
    pub fn authenticate(self, username: String, password: String) -> Result<DatabaseConnection<Authenticated>, String> {
        println!("🔐 Authenticating user: {username}");
        
        // Simulate authentication logic
        if username.is_empty() || password.len() < 6 {
            return Err("Invalid credentials".to_string());
        }
        
        println!("✅ Authentication successful!");
        Ok(DatabaseConnection {
            host: self.host,
            port: self.port,
            username: Some(username),
            _state: PhantomData,
        })
    }

    #[must_use] pub fn disconnect(self) -> DatabaseConnection<Disconnected> {
        println!("🔌 Disconnecting from {}:{}", self.host, self.port);
        DatabaseConnection {
            host: self.host,
            port: self.port,
            username: None,
            _state: PhantomData,
        }
    }
}

/// Implementation for Authenticated state
impl DatabaseConnection<Authenticated> {
    pub fn execute_query(&self, query: &str) -> Result<QueryResult, String> {
        println!("📊 Executing query: {query}");
        
        if query.trim().is_empty() {
            return Err("Empty query".to_string());
        }
        
        // Simulate query execution
        let rows_affected = query.len() % 10; // Mock result
        println!("✅ Query executed successfully! Rows affected: {rows_affected}");
        
        Ok(QueryResult {
            rows_affected,
            execution_time_ms: 42,
        })
    }

    #[must_use] pub fn begin_transaction(&self) -> Transaction {
        println!("🔄 Beginning transaction");
        Transaction {
            id: "txn_123".to_string(),
            is_active: true,
        }
    }

    #[must_use] pub fn get_username(&self) -> &str {
        self.username.as_ref().unwrap()
    }

    #[must_use] pub fn logout(self) -> DatabaseConnection<Connected> {
        println!("👋 Logging out user: {}", self.username.as_ref().unwrap());
        DatabaseConnection {
            host: self.host,
            port: self.port,
            username: None,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn disconnect(self) -> DatabaseConnection<Disconnected> {
        println!("🔌 Disconnecting authenticated user: {}", self.username.as_ref().unwrap());
        DatabaseConnection {
            host: self.host,
            port: self.port,
            username: None,
            _state: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows_affected: usize,
    pub execution_time_ms: u64,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub is_active: bool,
}

/// File states
#[derive(Debug)]
pub struct Closed;

#[derive(Debug)]
pub struct Open;

#[derive(Debug)]
pub struct Locked;

/// File handle with type state
#[derive(Debug)]
pub struct FileHandle<State> {
    path: String,
    size: u64,
    _state: PhantomData<State>,
}

/// Implementation for Closed state
impl FileHandle<Closed> {
    #[must_use] pub fn new(path: String) -> Self {
        println!("📁 Creating file handle for: {path}");
        Self {
            path,
            size: 0,
            _state: PhantomData,
        }
    }

    pub fn open(self) -> Result<FileHandle<Open>, String> {
        println!("📂 Opening file: {}", self.path);
        
        if !self.path.ends_with(".txt") && !self.path.ends_with(".log") {
            return Err("Unsupported file type".to_string());
        }
        
        println!("✅ File opened successfully!");
        Ok(FileHandle {
            path: self.path,
            size: 1024, // Mock file size
            _state: PhantomData,
        })
    }
}

/// Implementation for Open state
impl FileHandle<Open> {
    pub fn read(&self) -> Result<String, String> {
        println!("📖 Reading from file: {}", self.path);
        Ok(format!("Content of {}", self.path))
    }

    pub fn write(&mut self, data: &str) -> Result<(), String> {
        println!("✏️  Writing to file: {} (data: {})", self.path, data);
        self.size += data.len() as u64;
        Ok(())
    }

    #[must_use] pub fn lock(self) -> FileHandle<Locked> {
        println!("🔒 Locking file: {}", self.path);
        FileHandle {
            path: self.path,
            size: self.size,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn close(self) -> FileHandle<Closed> {
        println!("📁 Closing file: {}", self.path);
        FileHandle {
            path: self.path,
            size: 0,
            _state: PhantomData,
        }
    }

    #[must_use] pub const fn get_size(&self) -> u64 {
        self.size
    }
}

/// Implementation for Locked state
impl FileHandle<Locked> {
    pub fn read(&self) -> Result<String, String> {
        println!("📖 Reading from locked file: {}", self.path);
        Ok(format!("Locked content of {}", self.path))
    }

    #[must_use] pub fn unlock(self) -> FileHandle<Open> {
        println!("🔓 Unlocking file: {}", self.path);
        FileHandle {
            path: self.path,
            size: self.size,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn force_close(self) -> FileHandle<Closed> {
        println!("🔒❌ Force closing locked file: {}", self.path);
        FileHandle {
            path: self.path,
            size: 0,
            _state: PhantomData,
        }
    }
}

/// HTTP Request Builder with type state
#[derive(Debug)]
pub struct NoMethod;

#[derive(Debug)]
pub struct WithMethod;

#[derive(Debug)]
pub struct WithUrl;

#[derive(Debug)]
pub struct Ready;

#[derive(Debug)]
pub struct HttpRequestBuilder<State> {
    method: Option<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
    _state: PhantomData<State>,
}

/// Implementation for `NoMethod` state
impl Default for HttpRequestBuilder<NoMethod> {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequestBuilder<NoMethod> {
    #[must_use] pub const fn new() -> Self {
        Self {
            method: None,
            url: None,
            headers: Vec::new(),
            body: None,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn method(self, method: &str) -> HttpRequestBuilder<WithMethod> {
        HttpRequestBuilder {
            method: Some(method.to_string()),
            url: self.url,
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

/// Implementation for `WithMethod` state
impl HttpRequestBuilder<WithMethod> {
    #[must_use] pub fn url(self, url: &str) -> HttpRequestBuilder<WithUrl> {
        HttpRequestBuilder {
            method: self.method,
            url: Some(url.to_string()),
            headers: self.headers,
            body: self.body,
            _state: PhantomData,
        }
    }
}

/// Implementation for `WithUrl` state
impl HttpRequestBuilder<WithUrl> {
    #[must_use] pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    #[must_use] pub fn body(self, body: &str) -> Self {
        Self {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: Some(body.to_string()),
            _state: PhantomData,
        }
    }

    #[must_use] pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: self.headers,
            body: self.body,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl HttpRequest {
    #[must_use] pub fn execute(&self) -> HttpResponse {
        println!("🌐 Executing {} request to {}", self.method, self.url);
        
        for (key, value) in &self.headers {
            println!("📋 Header: {key}: {value}");
        }
        
        if let Some(body) = &self.body {
            println!("📄 Body: {body}");
        }
        
        // Mock response
        HttpResponse {
            status_code: 200,
            body: "Success".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub body: String,
}

/// State machine for order processing
#[derive(Debug)]
pub struct Pending;

#[derive(Debug)]
pub struct Confirmed;

#[derive(Debug)]
pub struct Shipped;

#[derive(Debug)]
pub struct Delivered;

#[derive(Debug)]
pub struct Cancelled;

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Order<State> {
    id: String,
    items: Vec<OrderItem>,
    total: f64,
    customer_email: String,
    _state: PhantomData<State>,
}

/// Implementation for Pending state
impl Order<Pending> {
    #[must_use] pub fn new(id: String, customer_email: String) -> Self {
        println!("📦 Creating new order: {id} for {customer_email}");
        Self {
            id,
            items: Vec::new(),
            total: 0.0,
            customer_email,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn add_item(mut self, item: OrderItem) -> Self {
        println!("➕ Adding item: {} x{} @ ${:.2}", item.name, item.quantity, item.price);
        self.total += item.price * f64::from(item.quantity);
        self.items.push(item);
        self
    }

    pub fn confirm(self) -> Result<Order<Confirmed>, String> {
        if self.items.is_empty() {
            return Err("Cannot confirm empty order".to_string());
        }
        
        println!("✅ Confirming order: {} (Total: ${:.2})", self.id, self.total);
        Ok(Order {
            id: self.id,
            items: self.items,
            total: self.total,
            customer_email: self.customer_email,
            _state: PhantomData,
        })
    }

    #[must_use] pub fn cancel(self) -> Order<Cancelled> {
        println!("❌ Cancelling pending order: {}", self.id);
        Order {
            id: self.id,
            items: self.items,
            total: self.total,
            customer_email: self.customer_email,
            _state: PhantomData,
        }
    }
}

/// Implementation for Confirmed state
impl Order<Confirmed> {
    #[must_use] pub fn ship(self, tracking_number: String) -> Order<Shipped> {
        println!("🚚 Shipping order: {} (Tracking: {})", self.id, tracking_number);
        Order {
            id: self.id,
            items: self.items,
            total: self.total,
            customer_email: self.customer_email,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn cancel(self) -> Order<Cancelled> {
        println!("❌ Cancelling confirmed order: {} (Refund: ${:.2})", self.id, self.total);
        Order {
            id: self.id,
            items: self.items,
            total: self.total,
            customer_email: self.customer_email,
            _state: PhantomData,
        }
    }

    #[must_use] pub const fn get_total(&self) -> f64 {
        self.total
    }

    #[must_use] pub fn get_items(&self) -> &[OrderItem] {
        &self.items
    }
}

/// Implementation for Shipped state
impl Order<Shipped> {
    #[must_use] pub fn deliver(self) -> Order<Delivered> {
        println!("📬 Order delivered: {}", self.id);
        Order {
            id: self.id,
            items: self.items,
            total: self.total,
            customer_email: self.customer_email,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn get_tracking_info(&self) -> String {
        format!("Order {} is in transit", self.id)
    }
}

/// Implementation for Delivered state
impl Order<Delivered> {
    #[must_use] pub fn get_delivery_confirmation(&self) -> String {
        format!("Order {} has been delivered to {}", self.id, self.customer_email)
    }
}

/// Implementation for Cancelled state
impl Order<Cancelled> {
    #[must_use] pub fn get_cancellation_reason(&self) -> String {
        "Order was cancelled by customer".to_string()
    }
}

/// Common implementations for all order states
impl<State> Order<State> {
    #[must_use] pub fn get_id(&self) -> &str {
        &self.id
    }

    #[must_use] pub fn get_customer_email(&self) -> &str {
        &self.customer_email
    }
}

/// สาธิตการใช้งาน Type State Pattern
pub fn demonstrate_type_state() {
    println!("🔄 Type State Pattern Examples:");
    
    // Database Connection Example
    println!("\n🗄️  Database Connection State Machine:");
    
    let db = DatabaseConnection::new("localhost".to_string(), 5432);
    
    // Can only connect from disconnected state
    let db = match db.connect() {
        Ok(connected_db) => {
            println!("Database connected successfully!");
            connected_db
        }
        Err(e) => {
            println!("Failed to connect: {e}");
            return;
        }
    };
    
    // Can only authenticate from connected state
    let db = match db.authenticate("admin".to_string(), "password123".to_string()) {
        Ok(auth_db) => {
            println!("Authentication successful!");
            auth_db
        }
        Err(e) => {
            println!("Authentication failed: {e}");
            return;
        }
    };
    
    // Can only execute queries from authenticated state
    match db.execute_query("SELECT * FROM users") {
        Ok(result) => {
            println!("Query result: {result:?}");
        }
        Err(e) => {
            println!("Query failed: {e}");
        }
    }
    
    let _transaction = db.begin_transaction();
    println!("Current user: {}", db.get_username());
    
    // Logout and disconnect
    let db = db.logout();
    let _db = db.disconnect();
    
    // File Handle Example
    println!("\n📁 File Handle State Machine:");
    
    let file = FileHandle::new("data.txt".to_string());
    
    let mut file = match file.open() {
        Ok(open_file) => {
            println!("File opened successfully!");
            open_file
        }
        Err(e) => {
            println!("Failed to open file: {e}");
            return;
        }
    };
    
    // Read and write operations
    match file.read() {
        Ok(content) => println!("File content: {content}"),
        Err(e) => println!("Read error: {e}"),
    }
    
    if let Err(e) = file.write("Hello, World!") {
        println!("Write error: {e}");
    }
    
    println!("File size: {} bytes", file.get_size());
    
    // Lock the file
    let locked_file = file.lock();
    
    // Can still read from locked file
    match locked_file.read() {
        Ok(content) => println!("Locked file content: {content}"),
        Err(e) => println!("Read error: {e}"),
    }
    
    // Unlock and close
    let file = locked_file.unlock();
    let _file = file.close();
    
    // HTTP Request Builder Example
    println!("\n🌐 HTTP Request Builder:");
    
    let request = HttpRequestBuilder::new()
        .method("POST")
        .url("https://api.example.com/users")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "John Doe", "email": "john@example.com"}"#)
        .build();
    
    println!("Built request: {request:?}");
    let response = request.execute();
    println!("Response: {response:?}");
    
    // Order Processing Example
    println!("\n📦 Order Processing State Machine:");
    
    let order = Order::new("ORD-001".to_string(), "customer@example.com".to_string())
        .add_item(OrderItem {
            name: "Laptop".to_string(),
            price: 999.99,
            quantity: 1,
        })
        .add_item(OrderItem {
            name: "Mouse".to_string(),
            price: 29.99,
            quantity: 2,
        });
    
    println!("Order ID: {}", order.get_id());
    println!("Customer: {}", order.get_customer_email());
    
    let order = match order.confirm() {
        Ok(confirmed_order) => {
            println!("Order confirmed! Total: ${:.2}", confirmed_order.get_total());
            confirmed_order
        }
        Err(e) => {
            println!("Failed to confirm order: {e}");
            return;
        }
    };
    
    let order = order.ship("TRACK123456".to_string());
    println!("Tracking info: {}", order.get_tracking_info());
    
    let order = order.deliver();
    println!("Delivery confirmation: {}", order.get_delivery_confirmation());
    
    // Example of cancelled order
    println!("\n❌ Cancelled Order Example:");
    let cancelled_order = Order::new("ORD-002".to_string(), "another@example.com".to_string())
        .add_item(OrderItem {
            name: "Book".to_string(),
            price: 19.99,
            quantity: 1,
        })
        .cancel();
    
    println!("Cancellation reason: {}", cancelled_order.get_cancellation_reason());
    
    println!("\n✅ Type state pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_connection_flow() {
        let db = DatabaseConnection::new("localhost".to_string(), 5432);
        let db = db.connect().unwrap();
        let db = db.authenticate("user".to_string(), "password".to_string()).unwrap();
        
        let result = db.execute_query("SELECT 1").unwrap();
        assert_eq!(result.rows_affected, 8); // "SELECT 1".len() % 10
    }

    #[test]
    fn test_file_handle_operations() {
        let file = FileHandle::new("test.txt".to_string());
        let mut file = file.open().unwrap();
        
        let content = file.read().unwrap();
        assert!(content.contains("test.txt"));
        
        file.write("test data").unwrap();
        assert!(file.get_size() > 1024);
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequestBuilder::new()
            .method("GET")
            .url("https://example.com")
            .header("Accept", "application/json")
            .build();
        
        assert_eq!(request.method, "GET");
        assert_eq!(request.url, "https://example.com");
        assert_eq!(request.headers.len(), 1);
    }

    #[test]
    fn test_order_processing() {
        let order = Order::new("TEST-001".to_string(), "test@example.com".to_string())
            .add_item(OrderItem {
                name: "Item".to_string(),
                price: 10.0,
                quantity: 2,
            });
        
        let order = order.confirm().unwrap();
        assert_eq!(order.get_total(), 20.0);
        
        let order = order.ship("TRACK001".to_string());
        assert!(order.get_tracking_info().contains("TEST-001"));
    }

    #[test]
    fn test_empty_order_confirmation_fails() {
        let order = Order::new("EMPTY-001".to_string(), "test@example.com".to_string());
        assert!(order.confirm().is_err());
    }

    #[test]
    fn test_invalid_file_type() {
        let file = FileHandle::new("invalid.exe".to_string());
        assert!(file.open().is_err());
    }

    #[test]
    fn test_database_authentication_failure() {
        let db = DatabaseConnection::new("localhost".to_string(), 5432);
        let db = db.connect().unwrap();
        
        // Short password should fail
        assert!(db.authenticate("user".to_string(), "123".to_string()).is_err());
    }
}