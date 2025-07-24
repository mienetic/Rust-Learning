//! 🌐 Mobile Networking
//! 
//! สาธิตการจัดการเครือข่ายในแอปพลิเคชันมือถือ
//! รวมถึง HTTP Requests, WebSocket, Offline Support, และ Network Optimization

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::fmt;

/// 🌐 Network Connection Type
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    WiFi,
    Cellular4G,
    Cellular5G,
    Cellular3G,
    Cellular2G,
    Ethernet,
    Offline,
    Unknown,
}

/// 📶 Network Quality
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkQuality {
    Excellent, // > 50 Mbps
    Good,      // 10-50 Mbps
    Fair,      // 1-10 Mbps
    Poor,      // < 1 Mbps
    Offline,
}

/// 🔄 Request Method
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// 📊 Request Priority
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RequestPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}

/// 🌐 HTTP Request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub id: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Duration,
    pub retry_count: u32,
    pub priority: RequestPriority,
    pub cache_policy: CachePolicy,
    pub created_at: u64,
}

/// 📥 HTTP Response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub request_id: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub response_time: Duration,
    pub from_cache: bool,
    pub received_at: u64,
}

/// 🗂️ Cache Policy
#[derive(Debug, Clone, PartialEq)]
pub enum CachePolicy {
    NoCache,           // Never cache
    CacheFirst,        // Use cache if available
    NetworkFirst,      // Try network first, fallback to cache
    CacheOnly,         // Only use cache
    NetworkOnly,       // Only use network
    StaleWhileRevalidate, // Use cache, update in background
}

/// 📱 Network Manager
#[derive(Debug)]
pub struct NetworkManager {
    connection_type: ConnectionType,
    network_quality: NetworkQuality,
    is_online: bool,
    request_queue: VecDeque<HttpRequest>,
    response_cache: HashMap<String, CachedResponse>,
    retry_queue: VecDeque<HttpRequest>,
    max_concurrent_requests: usize,
    active_requests: HashMap<String, HttpRequest>,
    network_stats: NetworkStats,
    offline_queue: VecDeque<HttpRequest>,
    bandwidth_limit: Option<u64>, // bytes per second
}

#[derive(Debug, Clone)]
pub struct CachedResponse {
    pub response: HttpResponse,
    pub expires_at: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub cached_responses: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub average_response_time: Duration,
    pub cache_hit_ratio: f32,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            connection_type: ConnectionType::WiFi,
            network_quality: NetworkQuality::Good,
            is_online: true,
            request_queue: VecDeque::new(),
            response_cache: HashMap::new(),
            retry_queue: VecDeque::new(),
            max_concurrent_requests: 6,
            active_requests: HashMap::new(),
            network_stats: NetworkStats::default(),
            offline_queue: VecDeque::new(),
            bandwidth_limit: None,
        }
    }
    
    pub fn update_connection_status(&mut self, connection_type: ConnectionType, is_online: bool) {
        self.connection_type = connection_type.clone();
        self.is_online = is_online;
        
        // Update network quality based on connection type
        self.network_quality = match connection_type {
            ConnectionType::WiFi => NetworkQuality::Excellent,
            ConnectionType::Cellular5G => NetworkQuality::Excellent,
            ConnectionType::Cellular4G => NetworkQuality::Good,
            ConnectionType::Cellular3G => NetworkQuality::Fair,
            ConnectionType::Cellular2G => NetworkQuality::Poor,
            ConnectionType::Ethernet => NetworkQuality::Excellent,
            ConnectionType::Offline => NetworkQuality::Offline,
            ConnectionType::Unknown => NetworkQuality::Fair,
        };
        
        println!("🌐 Network status updated: {:?} ({:?})", connection_type, self.network_quality);
        
        // Process offline queue if back online
        if is_online && !self.offline_queue.is_empty() {
            self.process_offline_queue();
        }
    }
    
    pub fn send_request(&mut self, mut request: HttpRequest) -> Result<String, NetworkError> {
        // Check if online
        if !self.is_online {
            if request.priority <= RequestPriority::Normal {
                self.offline_queue.push_back(request.clone());
                println!("📴 Request queued for offline: {}", request.id);
                return Ok(request.id);
            } else {
                return Err(NetworkError::Offline);
            }
        }
        
        // Check cache first
        if let Some(cached) = self.check_cache(&request) {
            println!("🚀 Serving from cache: {}", request.id);
            self.network_stats.cached_responses += 1;
            return Ok(request.id);
        }
        
        // Adjust request based on network quality
        self.optimize_request_for_network(&mut request);
        
        // Check concurrent request limit
        if self.active_requests.len() >= self.max_concurrent_requests {
            self.request_queue.push_back(request.clone());
            println!("⏳ Request queued: {} (queue size: {})", request.id, self.request_queue.len());
            return Ok(request.id);
        }
        
        // Execute request
        self.execute_request(request)
    }
    
    fn check_cache(&self, request: &HttpRequest) -> Option<&CachedResponse> {
        let cache_key = self.generate_cache_key(request);
        
        if let Some(cached) = self.response_cache.get(&cache_key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            match request.cache_policy {
                CachePolicy::NoCache | CachePolicy::NetworkOnly => None,
                CachePolicy::CacheOnly | CachePolicy::CacheFirst => {
                    if cached.expires_at > now {
                        Some(cached)
                    } else {
                        None
                    }
                }
                CachePolicy::NetworkFirst => {
                    // Try network first, this is just for demonstration
                    None
                }
                CachePolicy::StaleWhileRevalidate => {
                    // Return stale cache, revalidate in background
                    Some(cached)
                }
            }
        } else {
            None
        }
    }
    
    fn optimize_request_for_network(&self, request: &mut HttpRequest) {
        match self.network_quality {
            NetworkQuality::Poor => {
                // Reduce timeout and increase retry count for poor connections
                request.timeout = Duration::from_secs(30);
                request.retry_count = 3;
                println!("📶 Optimized request for poor network: {}", request.id);
            }
            NetworkQuality::Fair => {
                request.timeout = Duration::from_secs(20);
                request.retry_count = 2;
            }
            NetworkQuality::Good | NetworkQuality::Excellent => {
                request.timeout = Duration::from_secs(10);
                request.retry_count = 1;
            }
            NetworkQuality::Offline => {
                // This shouldn't happen as we check is_online first
            }
        }
        
        // Add compression headers for cellular connections
        if matches!(self.connection_type, ConnectionType::Cellular2G | ConnectionType::Cellular3G | ConnectionType::Cellular4G) {
            request.headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
        }
    }
    
    fn execute_request(&mut self, request: HttpRequest) -> Result<String, NetworkError> {
        println!("🚀 Executing request: {} {}", request.method.to_string(), request.url);
        
        // Add to active requests
        self.active_requests.insert(request.id.clone(), request.clone());
        
        // Simulate request execution
        let response = self.simulate_http_request(&request)?;
        
        // Update statistics
        self.update_network_stats(&request, &response);
        
        // Cache response if applicable
        self.cache_response(&request, &response);
        
        // Remove from active requests
        self.active_requests.remove(&request.id);
        
        // Process next request in queue
        self.process_request_queue();
        
        println!("✅ Request completed: {} ({}ms)", request.id, response.response_time.as_millis());
        
        Ok(request.id)
    }
    
    fn simulate_http_request(&self, request: &HttpRequest) -> Result<HttpResponse, NetworkError> {
        // Simulate network delay based on connection quality
        let base_delay = match self.network_quality {
            NetworkQuality::Excellent => 50,
            NetworkQuality::Good => 100,
            NetworkQuality::Fair => 300,
            NetworkQuality::Poor => 1000,
            NetworkQuality::Offline => return Err(NetworkError::Offline),
        };
        
        let response_time = Duration::from_millis(base_delay + (request.url.len() as u64 * 2));
        
        // Simulate different response codes
        let status_code = match request.priority {
            RequestPriority::Critical => 200,
            RequestPriority::High => if base_delay > 500 { 408 } else { 200 }, // Timeout for slow connections
            _ => 200,
        };
        
        if status_code != 200 {
            return Err(NetworkError::HttpError(status_code));
        }
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Cache-Control".to_string(), "max-age=3600".to_string());
        
        let body = format!("{{\"success\": true, \"data\": \"Response for {}\"}}", request.id).into_bytes();
        
        Ok(HttpResponse {
            request_id: request.id.clone(),
            status_code,
            headers,
            body,
            response_time,
            from_cache: false,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }
    
    fn cache_response(&mut self, request: &HttpRequest, response: &HttpResponse) {
        if matches!(request.cache_policy, CachePolicy::NoCache | CachePolicy::NetworkOnly) {
            return;
        }
        
        let cache_key = self.generate_cache_key(request);
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() + 3600; // 1 hour default
        
        let cached_response = CachedResponse {
            response: response.clone(),
            expires_at,
            etag: response.headers.get("ETag").cloned(),
            last_modified: response.headers.get("Last-Modified").cloned(),
            size_bytes: response.body.len(),
        };
        
        self.response_cache.insert(cache_key, cached_response);
        println!("💾 Cached response: {}", request.id);
    }
    
    fn generate_cache_key(&self, request: &HttpRequest) -> String {
        format!("{}_{}_{}", request.method.to_string(), request.url, 
               request.headers.get("Authorization").unwrap_or(&String::new()))
    }
    
    fn update_network_stats(&mut self, request: &HttpRequest, response: &HttpResponse) {
        self.network_stats.total_requests += 1;
        
        if response.status_code >= 200 && response.status_code < 300 {
            self.network_stats.successful_requests += 1;
        } else {
            self.network_stats.failed_requests += 1;
        }
        
        self.network_stats.total_bytes_sent += request.body.as_ref().map_or(0, |b| b.len()) as u64;
        self.network_stats.total_bytes_received += response.body.len() as u64;
        
        // Update average response time
        let total_time = self.network_stats.average_response_time.as_millis() as u64 * (self.network_stats.total_requests - 1);
        let new_average = (total_time + response.response_time.as_millis() as u64) / self.network_stats.total_requests;
        self.network_stats.average_response_time = Duration::from_millis(new_average);
        
        // Update cache hit ratio
        self.network_stats.cache_hit_ratio = 
            self.network_stats.cached_responses as f32 / self.network_stats.total_requests as f32;
    }
    
    fn process_request_queue(&mut self) {
        while self.active_requests.len() < self.max_concurrent_requests {
            if let Some(request) = self.request_queue.pop_front() {
                let _ = self.execute_request(request);
            } else {
                break;
            }
        }
    }
    
    fn process_offline_queue(&mut self) {
        println!("📶 Processing offline queue ({} requests)", self.offline_queue.len());
        
        let mut requests_to_process = Vec::new();
        while let Some(request) = self.offline_queue.pop_front() {
            requests_to_process.push(request);
        }
        
        for request in requests_to_process {
            let _ = self.send_request(request);
        }
    }
    
    pub fn retry_failed_requests(&mut self) {
        println!("🔄 Retrying failed requests ({} in queue)", self.retry_queue.len());
        
        let mut requests_to_retry = Vec::new();
        while let Some(mut request) = self.retry_queue.pop_front() {
            if request.retry_count > 0 {
                request.retry_count -= 1;
                requests_to_retry.push(request);
            }
        }
        
        for request in requests_to_retry {
            let _ = self.send_request(request);
        }
    }
    
    pub fn clear_cache(&mut self) {
        let cache_size = self.response_cache.len();
        self.response_cache.clear();
        println!("🧹 Cleared cache ({} entries)", cache_size);
    }
    
    pub fn get_network_stats(&self) -> &NetworkStats {
        &self.network_stats
    }
    
    pub fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            connection_type: self.connection_type.clone(),
            network_quality: self.network_quality.clone(),
            is_online: self.is_online,
            active_requests: self.active_requests.len(),
            queued_requests: self.request_queue.len(),
            offline_requests: self.offline_queue.len(),
            cache_size: self.response_cache.len(),
        }
    }
    
    pub fn set_bandwidth_limit(&mut self, bytes_per_second: Option<u64>) {
        self.bandwidth_limit = bytes_per_second;
        if let Some(limit) = bytes_per_second {
            println!("🚦 Bandwidth limit set: {} KB/s", limit / 1024);
        } else {
            println!("🚦 Bandwidth limit removed");
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub connection_type: ConnectionType,
    pub network_quality: NetworkQuality,
    pub is_online: bool,
    pub active_requests: usize,
    pub queued_requests: usize,
    pub offline_requests: usize,
    pub cache_size: usize,
}

/// ❌ Network Errors
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkError {
    Offline,
    Timeout,
    HttpError(u16),
    InvalidUrl,
    InvalidRequest,
    TooManyRequests,
    NetworkUnavailable,
    SSLError,
    DNSError,
    ConnectionRefused,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Offline => write!(f, "Device is offline"),
            NetworkError::Timeout => write!(f, "Request timeout"),
            NetworkError::HttpError(code) => write!(f, "HTTP error: {}", code),
            NetworkError::InvalidUrl => write!(f, "Invalid URL"),
            NetworkError::InvalidRequest => write!(f, "Invalid request"),
            NetworkError::TooManyRequests => write!(f, "Too many requests"),
            NetworkError::NetworkUnavailable => write!(f, "Network unavailable"),
            NetworkError::SSLError => write!(f, "SSL/TLS error"),
            NetworkError::DNSError => write!(f, "DNS resolution error"),
            NetworkError::ConnectionRefused => write!(f, "Connection refused"),
        }
    }
}

impl std::error::Error for NetworkError {}

impl HttpMethod {
    fn to_string(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

/// 🌐 สาธิตการใช้งาน Mobile Networking
pub fn demonstrate_mobile_networking() {
    println!("🌐 === Mobile Networking Demo ===");
    
    // Network Manager
    println!("\n📱 Network Manager:");
    demonstrate_network_manager();
    
    // Best Practices
    println!("\n💡 Mobile Networking Best Practices:");
    show_mobile_networking_best_practices();
}

/// 📱 สาธิต Network Manager
fn demonstrate_network_manager() {
    let mut network_manager = NetworkManager::new();
    
    // Test different connection types
    let connection_scenarios = vec![
        (ConnectionType::WiFi, true),
        (ConnectionType::Cellular4G, true),
        (ConnectionType::Cellular3G, true),
        (ConnectionType::Offline, false),
        (ConnectionType::WiFi, true), // Back online
    ];
    
    for (connection_type, is_online) in connection_scenarios {
        network_manager.update_connection_status(connection_type.clone(), is_online);
        
        if is_online {
            // Send different types of requests
            let requests = vec![
                HttpRequest {
                    id: "critical_request".to_string(),
                    method: HttpMethod::GET,
                    url: "https://api.example.com/critical".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    timeout: Duration::from_secs(5),
                    retry_count: 3,
                    priority: RequestPriority::Critical,
                    cache_policy: CachePolicy::NetworkFirst,
                    created_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                },
                HttpRequest {
                    id: "normal_request".to_string(),
                    method: HttpMethod::POST,
                    url: "https://api.example.com/data".to_string(),
                    headers: HashMap::new(),
                    body: Some(b"{\"data\": \"test\"}".to_vec()),
                    timeout: Duration::from_secs(10),
                    retry_count: 1,
                    priority: RequestPriority::Normal,
                    cache_policy: CachePolicy::CacheFirst,
                    created_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                },
                HttpRequest {
                    id: "background_request".to_string(),
                    method: HttpMethod::GET,
                    url: "https://api.example.com/analytics".to_string(),
                    headers: HashMap::new(),
                    body: None,
                    timeout: Duration::from_secs(30),
                    retry_count: 0,
                    priority: RequestPriority::Background,
                    cache_policy: CachePolicy::CacheOnly,
                    created_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                },
            ];
            
            for request in requests {
                let _ = network_manager.send_request(request);
            }
        }
        
        // Show connection info
        let info = network_manager.get_connection_info();
        println!("   📊 Connection Info:");
        println!("      • Type: {:?}", info.connection_type);
        println!("      • Quality: {:?}", info.network_quality);
        println!("      • Online: {}", info.is_online);
        println!("      • Active requests: {}", info.active_requests);
        println!("      • Queued requests: {}", info.queued_requests);
        println!("      • Offline requests: {}", info.offline_requests);
        println!("      • Cache size: {}", info.cache_size);
        
        std::thread::sleep(Duration::from_millis(200));
    }
    
    // Show network statistics
    let stats = network_manager.get_network_stats();
    println!("\n   📈 Network Statistics:");
    println!("      • Total requests: {}", stats.total_requests);
    println!("      • Successful: {}", stats.successful_requests);
    println!("      • Failed: {}", stats.failed_requests);
    println!("      • Cached responses: {}", stats.cached_responses);
    println!("      • Bytes sent: {:.1} KB", stats.total_bytes_sent as f64 / 1024.0);
    println!("      • Bytes received: {:.1} KB", stats.total_bytes_received as f64 / 1024.0);
    println!("      • Average response time: {:?}", stats.average_response_time);
    println!("      • Cache hit ratio: {:.1}%", stats.cache_hit_ratio * 100.0);
}

/// 💡 Mobile Networking Best Practices
fn show_mobile_networking_best_practices() {
    let practices = vec![
        "🌐 Design for offline-first architecture",
        "📶 Adapt to different network conditions",
        "🔄 Implement proper retry mechanisms",
        "💾 Use intelligent caching strategies",
        "⚡ Optimize for battery and data usage",
        "🔐 Always use HTTPS for secure communication",
        "📊 Monitor network performance and errors",
        "🚦 Implement rate limiting and throttling",
        "🔌 Use WebSockets for real-time communication",
        "📱 Handle network state changes gracefully",
        "🗜️ Compress requests and responses",
        "⏰ Set appropriate timeouts",
        "🎯 Prioritize critical requests",
        "📈 Use analytics to understand usage patterns",
        "🛡️ Implement proper error handling",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   📱 Platform-Specific Considerations:");
    println!("      🍎 iOS:");
    println!("         • Use URLSession for HTTP requests");
    println!("         • Leverage Network.framework for connection monitoring");
    println!("         • Implement background app refresh wisely");
    println!("         • Use NSURLCache for response caching");
    
    println!("\n      🤖 Android:");
    println!("         • Use OkHttp or Retrofit for networking");
    println!("         • Monitor connectivity with ConnectivityManager");
    println!("         • Implement JobScheduler for background tasks");
    println!("         • Use Volley or similar for request queuing");
    
    println!("\n   🛠️ Network Optimization Techniques:");
    println!("      • Bundle multiple requests when possible");
    println!("      • Use HTTP/2 for multiplexing");
    println!("      • Implement request deduplication");
    println!("      • Use CDN for static content");
    println!("      • Implement progressive data loading");
    println!("      • Use WebP or AVIF for images");
    println!("      • Implement connection pooling");
    println!("      • Use gRPC for efficient API communication");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_manager() {
        let mut manager = NetworkManager::new();
        
        assert!(manager.is_online);
        assert_eq!(manager.connection_type, ConnectionType::WiFi);
        
        manager.update_connection_status(ConnectionType::Offline, false);
        assert!(!manager.is_online);
        assert_eq!(manager.network_quality, NetworkQuality::Offline);
    }
    
    #[test]
    fn test_http_request() {
        let request = HttpRequest {
            id: "test".to_string(),
            method: HttpMethod::GET,
            url: "https://example.com".to_string(),
            headers: HashMap::new(),
            body: None,
            timeout: Duration::from_secs(10),
            retry_count: 1,
            priority: RequestPriority::Normal,
            cache_policy: CachePolicy::NetworkFirst,
            created_at: 0,
        };
        
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.priority, RequestPriority::Normal);
    }
}