//! 📊 Monitoring & Observability - การติดตามและสังเกตการณ์ระบบ
//! 
//! โมดูลนี้สาธิตการสร้างระบบ monitoring และ observability สำหรับ Rust applications
//! รวมถึง metrics, logging, tracing, และ health checks

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fmt;

/// 📊 ประเภทของ Metrics
#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
            MetricType::Summary => write!(f, "summary"),
        }
    }
}

/// 📈 Metric Value
#[derive(Debug, Clone)]
pub struct MetricValue {
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

impl MetricValue {
    /// สร้าง MetricValue ใหม่
    pub fn new(value: f64) -> Self {
        Self {
            value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            labels: HashMap::new(),
        }
    }
    
    /// เพิ่ม label
    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }
}

/// 📊 Metric Definition
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub unit: String,
    pub values: Vec<MetricValue>,
}

impl Metric {
    /// สร้าง Metric ใหม่
    pub fn new(name: &str, metric_type: MetricType, description: &str) -> Self {
        Self {
            name: name.to_string(),
            metric_type,
            description: description.to_string(),
            unit: String::new(),
            values: Vec::new(),
        }
    }
    
    /// เพิ่มค่า metric
    pub fn record(&mut self, value: f64) {
        self.values.push(MetricValue::new(value));
    }
    
    /// เพิ่มค่า metric พร้อม labels
    pub fn record_with_labels(&mut self, value: f64, labels: HashMap<String, String>) {
        let mut metric_value = MetricValue::new(value);
        metric_value.labels = labels;
        self.values.push(metric_value);
    }
    
    /// คำนวณค่าเฉลี่ย
    pub fn average(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().map(|v| v.value).sum::<f64>() / self.values.len() as f64
        }
    }
    
    /// หาค่าสูงสุด
    pub fn max(&self) -> f64 {
        self.values.iter().map(|v| v.value).fold(f64::NEG_INFINITY, f64::max)
    }
    
    /// หาค่าต่ำสุด
    pub fn min(&self) -> f64 {
        self.values.iter().map(|v| v.value).fold(f64::INFINITY, f64::min)
    }
}

/// 📊 Metrics Registry
#[derive(Debug)]
pub struct MetricsRegistry {
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
}

impl MetricsRegistry {
    /// สร้าง MetricsRegistry ใหม่
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// ลงทะเบียน metric ใหม่
    pub fn register(&self, metric: Metric) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(metric.name.clone(), metric);
    }
    
    /// บันทึกค่า metric
    pub fn record(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(metric) = metrics.get_mut(name) {
            metric.record(value);
        }
    }
    
    /// บันทึกค่า metric พร้อม labels
    pub fn record_with_labels(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(metric) = metrics.get_mut(name) {
            metric.record_with_labels(value, labels);
        }
    }
    
    /// ดึงข้อมูล metric
    pub fn get_metric(&self, name: &str) -> Option<Metric> {
        let metrics = self.metrics.lock().unwrap();
        metrics.get(name).cloned()
    }
    
    /// ดึงรายชื่อ metrics ทั้งหมด
    pub fn list_metrics(&self) -> Vec<String> {
        let metrics = self.metrics.lock().unwrap();
        metrics.keys().cloned().collect()
    }
    
    /// Export metrics ในรูปแบบ Prometheus
    pub fn export_prometheus(&self) -> String {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();
        
        for metric in metrics.values() {
            // Help comment
            output.push_str(&format!("# HELP {} {}\n", metric.name, metric.description));
            
            // Type comment
            output.push_str(&format!("# TYPE {} {}\n", metric.name, metric.metric_type));
            
            // Values
            for value in &metric.values {
                if value.labels.is_empty() {
                    output.push_str(&format!("{} {} {}\n", 
                        metric.name, value.value, value.timestamp));
                } else {
                    let labels: Vec<String> = value.labels.iter()
                        .map(|(k, v)| format!("{}=\"{}\"", k, v))
                        .collect();
                    output.push_str(&format!("{}{{{}}} {} {}\n", 
                        metric.name, labels.join(","), value.value, value.timestamp));
                }
            }
            output.push_str("\n");
        }
        
        output
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 🏥 Health Check Status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
            HealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// 🏥 Health Check Result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub duration: Duration,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

impl HealthCheckResult {
    /// สร้าง HealthCheckResult ใหม่
    pub fn new(name: &str, status: HealthStatus, message: &str, duration: Duration) -> Self {
        Self {
            name: name.to_string(),
            status,
            message: message.to_string(),
            duration,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: HashMap::new(),
        }
    }
    
    /// เพิ่ม metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// 🏥 Health Check Trait
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self) -> HealthCheckResult;
}

/// 🏥 Database Health Check
#[derive(Debug)]
pub struct DatabaseHealthCheck {
    name: String,
    connection_string: String,
}

impl DatabaseHealthCheck {
    pub fn new(name: &str, connection_string: &str) -> Self {
        Self {
            name: name.to_string(),
            connection_string: connection_string.to_string(),
        }
    }
}

impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        // Simulate database connection check
        let is_connected = !self.connection_string.is_empty();
        let duration = start.elapsed();
        
        if is_connected {
            HealthCheckResult::new(
                &self.name,
                HealthStatus::Healthy,
                "Database connection successful",
                duration,
            ).with_metadata("connection_string", &self.connection_string)
        } else {
            HealthCheckResult::new(
                &self.name,
                HealthStatus::Unhealthy,
                "Database connection failed",
                duration,
            )
        }
    }
}

/// 🏥 Memory Health Check
#[derive(Debug)]
pub struct MemoryHealthCheck {
    name: String,
    threshold_mb: u64,
}

impl MemoryHealthCheck {
    pub fn new(name: &str, threshold_mb: u64) -> Self {
        Self {
            name: name.to_string(),
            threshold_mb,
        }
    }
    
    fn get_memory_usage(&self) -> u64 {
        // Simulate memory usage check
        // In real implementation, you would use system APIs
        512 // MB
    }
}

impl HealthCheck for MemoryHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        let memory_usage = self.get_memory_usage();
        let duration = start.elapsed();
        
        let status = if memory_usage < self.threshold_mb {
            HealthStatus::Healthy
        } else if memory_usage < self.threshold_mb * 2 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        
        let message = format!("Memory usage: {} MB (threshold: {} MB)", 
            memory_usage, self.threshold_mb);
        
        HealthCheckResult::new(&self.name, status, &message, duration)
            .with_metadata("memory_usage_mb", &memory_usage.to_string())
            .with_metadata("threshold_mb", &self.threshold_mb.to_string())
    }
}

/// 🏥 Health Monitor
pub struct HealthMonitor {
    checks: Vec<Box<dyn HealthCheck>>,
    results: Arc<Mutex<Vec<HealthCheckResult>>>,
}

impl HealthMonitor {
    /// สร้าง HealthMonitor ใหม่
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// เพิ่ม health check
    pub fn add_check(&mut self, check: Box<dyn HealthCheck>) {
        self.checks.push(check);
    }
    
    /// รัน health checks ทั้งหมด
    pub fn run_checks(&self) -> Vec<HealthCheckResult> {
        let mut results = Vec::new();
        
        for check in &self.checks {
            let result = check.check();
            results.push(result);
        }
        
        // เก็บผลลัพธ์
        {
            let mut stored_results = self.results.lock().unwrap();
            stored_results.extend(results.clone());
            
            // เก็บเฉพาะ 100 ผลลัพธ์ล่าสุด
            if stored_results.len() > 100 {
                let keep_from = stored_results.len() - 100;
                *stored_results = stored_results.split_off(keep_from);
            }
        }
        
        results
    }
    
    /// ดึงสถานะรวม
    pub fn overall_status(&self) -> HealthStatus {
        let results = self.run_checks();
        
        if results.is_empty() {
            return HealthStatus::Unknown;
        }
        
        let unhealthy_count = results.iter()
            .filter(|r| r.status == HealthStatus::Unhealthy)
            .count();
        
        let degraded_count = results.iter()
            .filter(|r| r.status == HealthStatus::Degraded)
            .count();
        
        if unhealthy_count > 0 {
            HealthStatus::Unhealthy
        } else if degraded_count > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }
    
    /// Export health status เป็น JSON
    pub fn export_json(&self) -> String {
        let results = self.run_checks();
        let overall = self.overall_status();
        
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"status\": \"{}\",\n", overall));
        json.push_str(&format!("  \"timestamp\": {},\n", 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
        json.push_str("  \"checks\": [\n");
        
        for (i, result) in results.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"name\": \"{}\",\n", result.name));
            json.push_str(&format!("      \"status\": \"{}\",\n", result.status));
            json.push_str(&format!("      \"message\": \"{}\",\n", result.message));
            json.push_str(&format!("      \"duration_ms\": {},\n", result.duration.as_millis()));
            json.push_str(&format!("      \"timestamp\": {}\n", result.timestamp));
            json.push_str("    }");
            
            if i < results.len() - 1 {
                json.push_str(",");
            }
            json.push_str("\n");
        }
        
        json.push_str("  ]\n");
        json.push_str("}\n");
        
        json
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 📊 Performance Monitor
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics_registry: MetricsRegistry,
    start_time: Instant,
}

impl PerformanceMonitor {
    /// สร้าง PerformanceMonitor ใหม่
    pub fn new() -> Self {
        let mut monitor = Self {
            metrics_registry: MetricsRegistry::new(),
            start_time: Instant::now(),
        };
        
        // ลงทะเบียน metrics พื้นฐาน
        monitor.register_default_metrics();
        monitor
    }
    
    /// ลงทะเบียน metrics พื้นฐาน
    fn register_default_metrics(&mut self) {
        let metrics = vec![
            Metric::new("http_requests_total", MetricType::Counter, "Total HTTP requests"),
            Metric::new("http_request_duration_seconds", MetricType::Histogram, "HTTP request duration"),
            Metric::new("memory_usage_bytes", MetricType::Gauge, "Memory usage in bytes"),
            Metric::new("cpu_usage_percent", MetricType::Gauge, "CPU usage percentage"),
            Metric::new("active_connections", MetricType::Gauge, "Number of active connections"),
        ];
        
        for metric in metrics {
            self.metrics_registry.register(metric);
        }
    }
    
    /// บันทึก HTTP request
    pub fn record_http_request(&self, method: &str, path: &str, status_code: u16, duration: Duration) {
        let mut labels = HashMap::new();
        labels.insert("method".to_string(), method.to_string());
        labels.insert("path".to_string(), path.to_string());
        labels.insert("status_code".to_string(), status_code.to_string());
        
        self.metrics_registry.record_with_labels("http_requests_total", 1.0, labels.clone());
        self.metrics_registry.record_with_labels(
            "http_request_duration_seconds", 
            duration.as_secs_f64(), 
            labels
        );
    }
    
    /// บันทึกการใช้ memory
    pub fn record_memory_usage(&self, bytes: u64) {
        self.metrics_registry.record("memory_usage_bytes", bytes as f64);
    }
    
    /// บันทึกการใช้ CPU
    pub fn record_cpu_usage(&self, percent: f64) {
        self.metrics_registry.record("cpu_usage_percent", percent);
    }
    
    /// บันทึกจำนวน active connections
    pub fn record_active_connections(&self, count: u32) {
        self.metrics_registry.record("active_connections", count as f64);
    }
    
    /// ดึง uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Export metrics
    pub fn export_metrics(&self) -> String {
        self.metrics_registry.export_prometheus()
    }
    
    /// ดึงสถิติ performance
    pub fn get_performance_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        // Uptime
        stats.insert("uptime_seconds".to_string(), self.uptime().as_secs_f64());
        
        // HTTP request stats
        if let Some(metric) = self.metrics_registry.get_metric("http_requests_total") {
            stats.insert("total_requests".to_string(), metric.values.len() as f64);
        }
        
        if let Some(metric) = self.metrics_registry.get_metric("http_request_duration_seconds") {
            if !metric.values.is_empty() {
                stats.insert("avg_response_time".to_string(), metric.average());
                stats.insert("max_response_time".to_string(), metric.max());
                stats.insert("min_response_time".to_string(), metric.min());
            }
        }
        
        // Memory stats
        if let Some(metric) = self.metrics_registry.get_metric("memory_usage_bytes") {
            if !metric.values.is_empty() {
                stats.insert("current_memory_usage".to_string(), 
                    metric.values.last().unwrap().value);
                stats.insert("avg_memory_usage".to_string(), metric.average());
                stats.insert("max_memory_usage".to_string(), metric.max());
            }
        }
        
        stats
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 🎯 สาธิตการทำงานกับ Monitoring & Observability
pub fn demonstrate_monitoring_observability() {
    println!("\n📊 === Monitoring & Observability Demo ===");
    
    // 1. Metrics Collection
    println!("\n1️⃣ Metrics Collection:");
    demonstrate_metrics();
    
    // 2. Health Checks
    println!("\n2️⃣ Health Checks:");
    demonstrate_health_checks();
    
    // 3. Performance Monitoring
    println!("\n3️⃣ Performance Monitoring:");
    demonstrate_performance_monitoring();
    
    // 4. Observability Best Practices
    println!("\n4️⃣ Observability Best Practices:");
    show_observability_best_practices();
    
    println!("\n✅ จบการสาธิต Monitoring & Observability!");
}

/// 📊 สาธิต Metrics
fn demonstrate_metrics() {
    println!("📊 การจัดเก็บ Metrics:");
    
    let registry = MetricsRegistry::new();
    
    // ลงทะเบียน metrics
    let mut counter = Metric::new("requests_total", MetricType::Counter, "Total requests");
    let mut response_time = Metric::new("response_time_seconds", MetricType::Histogram, "Response time");
    
    // บันทึกข้อมูล
    for i in 1..=10 {
        counter.record(i as f64);
        response_time.record(0.1 + (i as f64 * 0.05));
    }
    
    registry.register(counter.clone());
    registry.register(response_time.clone());
    
    println!("\n📈 Metrics Statistics:");
    println!("   • Total requests: {}", counter.values.len());
    println!("   • Avg response time: {:.3}s", response_time.average());
    println!("   • Max response time: {:.3}s", response_time.max());
    println!("   • Min response time: {:.3}s", response_time.min());
    
    // Export Prometheus format
    println!("\n📤 Prometheus Export (ตัวอย่าง):");
    let prometheus_output = registry.export_prometheus();
    let lines: Vec<&str> = prometheus_output.lines().take(5).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
}

/// 🏥 สาธิต Health Checks
fn demonstrate_health_checks() {
    println!("🏥 การตรวจสอบสุขภาพระบบ:");
    
    let mut monitor = HealthMonitor::new();
    
    // เพิ่ม health checks
    monitor.add_check(Box::new(DatabaseHealthCheck::new(
        "database", 
        "postgresql://localhost:5432/mydb"
    )));
    
    monitor.add_check(Box::new(MemoryHealthCheck::new(
        "memory", 
        1024 // 1GB threshold
    )));
    
    // รัน health checks
    let results = monitor.run_checks();
    let overall_status = monitor.overall_status();
    
    println!("\n🎯 Health Check Results:");
    println!("   • Overall Status: {}", overall_status);
    
    for result in &results {
        println!("   • {}: {} - {} ({:.2}ms)", 
            result.name, 
            result.status, 
            result.message,
            result.duration.as_millis()
        );
        
        if !result.metadata.is_empty() {
            for (key, value) in &result.metadata {
                println!("     - {}: {}", key, value);
            }
        }
    }
    
    // Export JSON
    println!("\n📤 Health Status JSON (ตัวอย่าง):");
    let json_output = monitor.export_json();
    let lines: Vec<&str> = json_output.lines().take(8).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
}

/// 📊 สาธิต Performance Monitoring
fn demonstrate_performance_monitoring() {
    println!("📊 การติดตาม Performance:");
    
    let monitor = PerformanceMonitor::new();
    
    // จำลองการใช้งาน
    println!("\n🔄 จำลองการใช้งานระบบ:");
    
    // HTTP requests
    monitor.record_http_request("GET", "/api/users", 200, Duration::from_millis(150));
    monitor.record_http_request("POST", "/api/users", 201, Duration::from_millis(300));
    monitor.record_http_request("GET", "/api/users/123", 200, Duration::from_millis(80));
    monitor.record_http_request("DELETE", "/api/users/123", 204, Duration::from_millis(120));
    
    // System metrics
    monitor.record_memory_usage(512 * 1024 * 1024); // 512 MB
    monitor.record_cpu_usage(45.5);
    monitor.record_active_connections(25);
    
    println!("   • บันทึก HTTP requests: 4 requests");
    println!("   • บันทึก Memory usage: 512 MB");
    println!("   • บันทึก CPU usage: 45.5%");
    println!("   • บันทึก Active connections: 25");
    
    // ดึงสถิติ
    let stats = monitor.get_performance_stats();
    
    println!("\n📈 Performance Statistics:");
    for (key, value) in &stats {
        if key.contains("time") {
            println!("   • {}: {:.3}s", key, value);
        } else if key.contains("memory") {
            println!("   • {}: {:.0} bytes", key, value);
        } else {
            println!("   • {}: {:.2}", key, value);
        }
    }
    
    println!("\n📤 Metrics Export (ตัวอย่าง):");
    let metrics_output = monitor.export_metrics();
    let lines: Vec<&str> = metrics_output.lines().take(6).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
}

/// 📋 แสดง Observability Best Practices
fn show_observability_best_practices() {
    println!("📋 Observability Best Practices:");
    
    let practices = vec![
        ("📊", "Metrics", "เก็บ metrics ที่สำคัญ: latency, throughput, error rate"),
        ("📝", "Logging", "ใช้ structured logging และ correlation IDs"),
        ("🔍", "Tracing", "ติดตาม request flow ข้าม services"),
        ("🏥", "Health Checks", "ตรวจสอบสุขภาพระบบอย่างสม่ำเสมอ"),
        ("🚨", "Alerting", "ตั้ง alerts สำหรับ critical metrics"),
        ("📈", "Dashboards", "สร้าง dashboards สำหรับ visualization"),
        ("🔄", "SLI/SLO", "กำหนด Service Level Indicators และ Objectives"),
        ("📊", "Cardinality", "ระวัง high cardinality labels ใน metrics"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎯 Rust-specific Observability:");
    println!("   • ใช้ tracing crate สำหรับ structured logging");
    println!("   • ใช้ metrics crate สำหรับ metrics collection");
    println!("   • ใช้ tokio-console สำหรับ async debugging");
    println!("   • ใช้ criterion สำหรับ benchmarking");
    println!("   • ใช้ flamegraph สำหรับ profiling");
    
    println!("\n🔧 การ Setup Observability Stack:");
    println!("   • Prometheus + Grafana สำหรับ metrics");
    println!("   • ELK Stack หรือ Loki สำหรับ logs");
    println!("   • Jaeger หรือ Zipkin สำหรับ tracing");
    println!("   • AlertManager สำหรับ alerting");
    println!("   • OpenTelemetry สำหรับ unified observability");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metric_creation() {
        let mut metric = Metric::new("test_counter", MetricType::Counter, "Test counter");
        metric.record(1.0);
        metric.record(2.0);
        metric.record(3.0);
        
        assert_eq!(metric.values.len(), 3);
        assert_eq!(metric.average(), 2.0);
        assert_eq!(metric.max(), 3.0);
        assert_eq!(metric.min(), 1.0);
    }
    
    #[test]
    fn test_metrics_registry() {
        let registry = MetricsRegistry::new();
        let metric = Metric::new("test_metric", MetricType::Gauge, "Test metric");
        
        registry.register(metric);
        registry.record("test_metric", 42.0);
        
        let retrieved = registry.get_metric("test_metric").unwrap();
        assert_eq!(retrieved.values.len(), 1);
        assert_eq!(retrieved.values[0].value, 42.0);
    }
    
    #[test]
    fn test_health_check() {
        let db_check = DatabaseHealthCheck::new("test_db", "postgresql://localhost:5432/test");
        let result = db_check.check();
        
        assert_eq!(result.name, "test_db");
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(result.duration.as_millis() >= 0);
    }
    
    #[test]
    fn test_health_monitor() {
        let mut monitor = HealthMonitor::new();
        monitor.add_check(Box::new(DatabaseHealthCheck::new("db", "test://connection")));
        
        let results = monitor.run_checks();
        assert_eq!(results.len(), 1);
        
        let overall = monitor.overall_status();
        assert_eq!(overall, HealthStatus::Healthy);
    }
    
    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        
        monitor.record_http_request("GET", "/test", 200, Duration::from_millis(100));
        monitor.record_memory_usage(1024);
        monitor.record_cpu_usage(50.0);
        
        let stats = monitor.get_performance_stats();
        assert!(stats.contains_key("uptime_seconds"));
        assert!(stats.get("uptime_seconds").unwrap() > &0.0);
    }
}