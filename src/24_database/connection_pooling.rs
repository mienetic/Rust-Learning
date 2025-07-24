//! 🏊 Connection Pooling - การจัดการ Connection Pool
//! 
//! โมดูลนี้สาธิตการจัดการ Database Connection Pool
//! เพื่อเพิ่มประสิทธิภาพและจัดการ resource ให้ดีขึ้น

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::time::{Duration, Instant};
use std::thread;

/// 🔗 โครงสร้างการเชื่อมต่อฐานข้อมูล
#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    pub id: usize,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub created_at: Instant,
    pub last_used: Instant,
    pub is_active: bool,
    pub query_count: usize,
}

impl DatabaseConnection {
    /// สร้างการเชื่อมต่อใหม่
    pub fn new(id: usize, host: &str, port: u16, database: &str) -> Self {
        let now = Instant::now();
        Self {
            id,
            host: host.to_string(),
            port,
            database: database.to_string(),
            created_at: now,
            last_used: now,
            is_active: true,
            query_count: 0,
        }
    }
    
    /// จำลองการรัน query
    pub fn execute_query(&mut self, query: &str) -> Result<String, String> {
        if !self.is_active {
            return Err("Connection is not active".to_string());
        }
        
        self.last_used = Instant::now();
        self.query_count += 1;
        
        // จำลองเวลาในการประมวลผล
        thread::sleep(Duration::from_millis(10));
        
        Ok(format!("Query executed: {} (Connection {})", query, self.id))
    }
    
    /// ตรวจสอบว่าการเชื่อมต่อยังใช้งานได้หรือไม่
    pub fn is_healthy(&self) -> bool {
        self.is_active && self.last_used.elapsed() < Duration::from_secs(300) // 5 นาที
    }
    
    /// ปิดการเชื่อมต่อ
    pub fn close(&mut self) {
        self.is_active = false;
        println!("🔌 ปิดการเชื่อมต่อ {} (ใช้งาน {} queries)", self.id, self.query_count);
    }
}

/// 📊 สถิติของ Connection Pool
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub total_queries: usize,
    pub pool_hits: usize,
    pub pool_misses: usize,
    pub created_at: Instant,
}

impl PoolStats {
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            total_queries: 0,
            pool_hits: 0,
            pool_misses: 0,
            created_at: Instant::now(),
        }
    }
    
    /// คำนวณ hit rate
    pub fn hit_rate(&self) -> f64 {
        if self.pool_hits + self.pool_misses == 0 {
            return 0.0;
        }
        self.pool_hits as f64 / (self.pool_hits + self.pool_misses) as f64 * 100.0
    }
    
    /// แสดงสถิติ
    pub fn display(&self) {
        println!("📊 Pool Statistics:");
        println!("   Total Connections: {}", self.total_connections);
        println!("   Active: {} | Idle: {}", self.active_connections, self.idle_connections);
        println!("   Total Queries: {}", self.total_queries);
        println!("   Pool Hits: {} | Misses: {}", self.pool_hits, self.pool_misses);
        println!("   Hit Rate: {:.2}%", self.hit_rate());
        println!("   Uptime: {:.2}s", self.created_at.elapsed().as_secs_f64());
    }
}

/// 🏊 Connection Pool Configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: usize,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub health_check_interval: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 นาที
            max_lifetime: Duration::from_secs(3600), // 1 ชั่วโมง
            health_check_interval: Duration::from_secs(60), // 1 นาที
        }
    }
}

/// 🏊 Connection Pool Implementation
pub struct ConnectionPool {
    config: PoolConfig,
    available: Arc<Mutex<VecDeque<DatabaseConnection>>>,
    in_use: Arc<Mutex<Vec<DatabaseConnection>>>,
    stats: Arc<Mutex<PoolStats>>,
    condvar: Arc<Condvar>,
    next_id: Arc<Mutex<usize>>,
    host: String,
    port: u16,
    database: String,
}

impl ConnectionPool {
    /// สร้าง Connection Pool ใหม่
    pub fn new(host: &str, port: u16, database: &str, config: PoolConfig) -> Self {
        let pool = Self {
            config,
            available: Arc::new(Mutex::new(VecDeque::new())),
            in_use: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(PoolStats::new())),
            condvar: Arc::new(Condvar::new()),
            next_id: Arc::new(Mutex::new(1)),
            host: host.to_string(),
            port,
            database: database.to_string(),
        };
        
        // สร้างการเชื่อมต่อเริ่มต้น
        pool.initialize_connections();
        
        pool
    }
    
    /// สร้างการเชื่อมต่อเริ่มต้น
    fn initialize_connections(&self) {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();
        
        for _ in 0..self.config.min_connections {
            let conn = DatabaseConnection::new(
                *next_id,
                &self.host,
                self.port,
                &self.database,
            );
            
            available.push_back(conn);
            stats.total_connections += 1;
            stats.idle_connections += 1;
            *next_id += 1;
        }
        
        println!("🏊 สร้าง Connection Pool: {} connections", self.config.min_connections);
    }
    
    /// ขอการเชื่อมต่อจาก Pool
    pub fn get_connection(&self) -> Result<DatabaseConnection, String> {
        let mut available = self.available.lock().unwrap();
        let mut in_use = self.in_use.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // ลองหาการเชื่อมต่อที่ว่าง
        if let Some(mut conn) = available.pop_front() {
            // ตรวจสอบสุขภาพของการเชื่อมต่อ
            if conn.is_healthy() {
                conn.last_used = Instant::now();
                in_use.push(conn.clone());
                stats.active_connections += 1;
                stats.idle_connections -= 1;
                stats.pool_hits += 1;
                
                println!("✅ ได้การเชื่อมต่อจาก Pool: {}", conn.id);
                return Ok(conn);
            } else {
                // การเชื่อมต่อไม่ดี ต้องสร้างใหม่
                println!("⚠️ การเชื่อมต่อ {} ไม่ดี กำลังสร้างใหม่", conn.id);
                stats.total_connections -= 1;
            }
        }
        
        // ถ้าไม่มีการเชื่อมต่อว่าง ลองสร้างใหม่
        if stats.total_connections < self.config.max_connections {
            let mut next_id = self.next_id.lock().unwrap();
            let conn = DatabaseConnection::new(
                *next_id,
                &self.host,
                self.port,
                &self.database,
            );
            
            in_use.push(conn.clone());
            stats.total_connections += 1;
            stats.active_connections += 1;
            stats.pool_misses += 1;
            *next_id += 1;
            
            println!("🆕 สร้างการเชื่อมต่อใหม่: {}", conn.id);
            return Ok(conn);
        }
        
        // ถ้าถึงขีดจำกัดแล้ว รอให้มีการเชื่อมต่อว่าง
        println!("⏳ รอการเชื่อมต่อว่าง...");
        
        let timeout_result = self.condvar.wait_timeout(
            available,
            self.config.connection_timeout,
        ).unwrap();
        
        if timeout_result.1.timed_out() {
            return Err("Connection timeout".to_string());
        }
        
        // ลองอีกครั้งหลังจากรอ
        self.get_connection()
    }
    
    /// คืนการเชื่อมต่อกลับไปยัง Pool
    pub fn return_connection(&self, conn: DatabaseConnection) {
        let mut available = self.available.lock().unwrap();
        let mut in_use = self.in_use.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // ลบจาก in_use list
        in_use.retain(|c| c.id != conn.id);
        
        // เก็บ connection id ก่อนที่จะ move
        let conn_id = conn.id;
        
        // ตรวจสอบว่าการเชื่อมต่อยังใช้งานได้หรือไม่
        if conn.is_healthy() && 
           conn.created_at.elapsed() < self.config.max_lifetime {
            available.push_back(conn);
            stats.active_connections -= 1;
            stats.idle_connections += 1;
            
            println!("🔄 คืนการเชื่อมต่อ {} กลับไปยัง Pool", conn_id);
        } else {
            stats.total_connections -= 1;
            stats.active_connections -= 1;
            
            println!("🗑️ ทิ้งการเชื่อมต่อ {} (หมดอายุหรือไม่ดี)", conn_id);
        }
        
        // แจ้งให้ thread ที่รออยู่
        self.condvar.notify_one();
    }
    
    /// ทำความสะอาด Pool
    pub fn cleanup(&self) {
        let mut available = self.available.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let initial_count = available.len();
        
        // ลบการเชื่อมต่อที่หมดอายุหรือไม่ดี
        available.retain(|conn| {
            let is_valid = conn.is_healthy() && 
                          conn.created_at.elapsed() < self.config.max_lifetime;
            
            if !is_valid {
                stats.total_connections -= 1;
                stats.idle_connections -= 1;
            }
            
            is_valid
        });
        
        let removed_count = initial_count - available.len();
        if removed_count > 0 {
            println!("🧹 ทำความสะอาด Pool: ลบ {} การเชื่อมต่อ", removed_count);
        }
        
        // เพิ่มการเชื่อมต่อใหม่ถ้าต่ำกว่าขั้นต่ำ
        let current_total = stats.total_connections;
        if current_total < self.config.min_connections {
            let mut next_id = self.next_id.lock().unwrap();
            let needed = self.config.min_connections - current_total;
            
            for _ in 0..needed {
                let conn = DatabaseConnection::new(
                    *next_id,
                    &self.host,
                    self.port,
                    &self.database,
                );
                
                available.push_back(conn);
                stats.total_connections += 1;
                stats.idle_connections += 1;
                *next_id += 1;
            }
            
            println!("➕ เพิ่มการเชื่อมต่อใหม่: {} connections", needed);
        }
    }
    
    /// แสดงสถิติของ Pool
    pub fn show_stats(&self) {
        let stats = self.stats.lock().unwrap();
        stats.display();
    }
    
    /// รันการทำงานแบบ async กับ Pool
    pub fn execute_query(&self, query: &str) -> Result<String, String> {
        let mut conn = self.get_connection()?;
        
        // อัปเดตสถิติ
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_queries += 1;
        }
        
        let result = conn.execute_query(query);
        self.return_connection(conn);
        
        result
    }
}

/// 🎯 สาธิตการทำงานกับ Connection Pooling
pub fn demonstrate_connection_pooling() {
    println!("\n🏊 === Connection Pooling Demo ===");
    
    // 1. สร้าง Connection Pool
    println!("\n1️⃣ การสร้าง Connection Pool:");
    let config = PoolConfig {
        min_connections: 3,
        max_connections: 8,
        connection_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(300),
        max_lifetime: Duration::from_secs(1800),
        health_check_interval: Duration::from_secs(30),
    };
    
    let pool = ConnectionPool::new("localhost", 5432, "myapp_db", config);
    pool.show_stats();
    
    // 2. การใช้งาน Connection Pool
    println!("\n2️⃣ การใช้งาน Connection Pool:");
    demonstrate_pool_usage(&pool);
    
    // 3. การทดสอบ Load
    println!("\n3️⃣ การทดสอบ Load:");
    demonstrate_load_testing(&pool);
    
    // 4. การทำความสะอาด Pool
    println!("\n4️⃣ การทำความสะอาด Pool:");
    pool.cleanup();
    pool.show_stats();
    
    // 5. Best Practices
    println!("\n5️⃣ Best Practices สำหรับ Connection Pooling:");
    show_best_practices();
    
    println!("\n✅ จบการสาธิต Connection Pooling!");
}

/// 🔧 สาธิตการใช้งาน Pool
fn demonstrate_pool_usage(pool: &ConnectionPool) {
    println!("🔧 การใช้งาน Pool:");
    
    // รัน queries หลายๆ ตัว
    let queries = vec![
        "SELECT * FROM users WHERE active = true",
        "SELECT COUNT(*) FROM posts",
        "INSERT INTO logs (message) VALUES ('Test log')",
        "UPDATE users SET last_login = NOW() WHERE id = 1",
        "SELECT * FROM products ORDER BY created_at DESC LIMIT 10",
    ];
    
    for (i, query) in queries.iter().enumerate() {
        match pool.execute_query(query) {
            Ok(result) => println!("   ✅ Query {}: {}", i + 1, result),
            Err(e) => println!("   ❌ Query {}: Error - {}", i + 1, e),
        }
    }
    
    println!();
    pool.show_stats();
}

/// 📈 สาธิตการทดสอบ Load
fn demonstrate_load_testing(pool: &ConnectionPool) {
    println!("📈 การทดสอบ Load:");
    
    // จำลองการใช้งานหนัก
    let queries = vec![
        "SELECT * FROM users",
        "SELECT * FROM posts",
        "SELECT * FROM comments",
    ];
    
    println!("🚀 รัน {} queries พร้อมกัน...", queries.len() * 5);
    
    for round in 1..=5 {
        println!("   Round {}: รัน {} queries", round, queries.len());
        
        for query in &queries {
            match pool.execute_query(query) {
                Ok(_) => print!("✅ "),
                Err(_) => print!("❌ "),
            }
        }
        println!();
        
        // รอสักครู่
        thread::sleep(Duration::from_millis(100));
    }
    
    println!();
    pool.show_stats();
}

/// 📋 แสดง Best Practices
fn show_best_practices() {
    println!("📋 Best Practices สำหรับ Connection Pooling:");
    
    let practices = vec![
        ("🎯", "Pool Size", "ตั้งค่า min/max connections ให้เหมาะสมกับ workload"),
        ("⏰", "Timeout", "ตั้งค่า timeout ที่เหมาะสมเพื่อป้องกัน deadlock"),
        ("🏥", "Health Check", "ตรวจสอบสุขภาพของ connection เป็นระยะ"),
        ("🧹", "Cleanup", "ทำความสะอาด expired connections เป็นประจำ"),
        ("📊", "Monitoring", "ติดตามสถิติการใช้งาน pool"),
        ("🔄", "Retry Logic", "มี retry mechanism สำหรับ connection failures"),
        ("🛡️", "Error Handling", "จัดการ error ให้เหมาะสมและ graceful"),
        ("⚡", "Performance", "ใช้ async/await สำหรับ non-blocking operations"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎯 การตั้งค่าที่แนะนำ:");
    println!("   • Min Connections: 5-10 (ขึ้นอยู่กับ base load)");
    println!("   • Max Connections: 20-50 (ขึ้นอยู่กับ peak load)");
    println!("   • Connection Timeout: 30s");
    println!("   • Idle Timeout: 10 นาที");
    println!("   • Max Lifetime: 1 ชั่วโมง");
    println!("   • Health Check: ทุก 1 นาที");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_connection() {
        let mut conn = DatabaseConnection::new(1, "localhost", 5432, "test_db");
        assert!(conn.is_healthy());
        
        let result = conn.execute_query("SELECT 1");
        assert!(result.is_ok());
        assert_eq!(conn.query_count, 1);
    }
    
    #[test]
    fn test_pool_stats() {
        let mut stats = PoolStats::new();
        stats.pool_hits = 80;
        stats.pool_misses = 20;
        
        assert_eq!(stats.hit_rate(), 80.0);
    }
    
    #[test]
    fn test_connection_pool() {
        let config = PoolConfig {
            min_connections: 2,
            max_connections: 5,
            ..Default::default()
        };
        
        let pool = ConnectionPool::new("localhost", 5432, "test_db", config);
        
        // Test getting connection
        let conn = pool.get_connection();
        assert!(conn.is_ok());
        
        // Test returning connection
        pool.return_connection(conn.unwrap());
        
        // Test executing query
        let result = pool.execute_query("SELECT 1");
        assert!(result.is_ok());
    }
}