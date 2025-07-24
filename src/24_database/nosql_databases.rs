//! 🍃 NoSQL Databases - การทำงานกับฐานข้อมูล NoSQL
//! 
//! โมดูลนี้สาธิตการทำงานกับฐานข้อมูล NoSQL ต่างๆ
//! รวมถึง MongoDB, Redis, และ Cassandra
//! 
//! 💡 Fun Fact: NoSQL ย่อมาจาก "Not Only SQL" 
//! ไม่ใช่ "No SQL" นะ! เหมือนบอกว่า "ไม่ใช่แค่ SQL" 😄

use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Deserialize};

/// 📄 โครงสร้างเอกสาร JSON สำหรับ MongoDB
/// เหมือนแฟ้มเอกสารในตู้เก็บเอกสาร แต่เป็นดิจิทัล! 📁
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub _id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 🔑 โครงสร้างข้อมูล Key-Value สำหรับ Redis
/// เหมือนกล่องใส่ของที่มีป้ายชื่อติดไว้ 📦🏷️
#[derive(Debug, Clone)]
pub struct KeyValueStore {
    data: HashMap<String, String>,
    expiry: HashMap<String, u64>, // timestamp
}

impl KeyValueStore {
    /// สร้าง Key-Value Store ใหม่
    /// เริ่มต้นด้วยกล่องเปล่าๆ พร้อมใส่ของ! 📦
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            expiry: HashMap::new(),
        }
    }
    
    /// เก็บข้อมูล
    /// ใส่ของลงกล่องแล้วติดป้ายชื่อ 🏷️
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        println!("✅ เก็บข้อมูล: {} = {} - เก็บเรียบร้อย! 📦", key, value);
    }
    
    /// เก็บข้อมูลพร้อม TTL (Time To Live)
    /// เหมือนใส่ของในตู้เย็น มีวันหมดอายุ! ❄️⏰
    pub fn set_with_ttl(&mut self, key: &str, value: &str, ttl_seconds: u64) {
        let expiry_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + ttl_seconds;
        
        self.data.insert(key.to_string(), value.to_string());
        self.expiry.insert(key.to_string(), expiry_time);
        
        println!("⏰ เก็บข้อมูลพร้อม TTL: {} = {} (หมดอายุใน {} วินาที) - เหมือนนมในตู้เย็น! 🥛", 
                key, value, ttl_seconds);
    }
    
    /// ดึงข้อมูล
    /// เปิดกล่องดูว่าข้างในมีอะไร 👀
    pub fn get(&self, key: &str) -> Option<&String> {
        // ตรวจสอบว่าหมดอายุหรือไม่
        if let Some(&expiry_time) = self.expiry.get(key) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if current_time > expiry_time {
                println!("⏰ ข้อมูล {} หมดอายุแล้ว - เหมือนนมเสีย! 🥛💔", key);
                return None;
            }
        }
        
        self.data.get(key)
    }
    
    /// ลบข้อมูล
    /// โยนกล่องทิ้งถังขยะ 🗑️
    pub fn delete(&mut self, key: &str) -> bool {
        let removed = self.data.remove(key).is_some();
        self.expiry.remove(key);
        
        if removed {
            println!("🗑️ ลบข้อมูล: {} - ลาก่อนนะ! 👋", key);
        }
        
        removed
    }
    
    /// แสดงข้อมูลทั้งหมด
    /// ดูตารางทั้งหมด เหมือนเปิดสเปรดชีต! 📊
    /// เปิดดูในกล่องทุกใบ 📦📦📦
    pub fn list_all(&self) -> Vec<(String, String)> {
        self.data.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

/// 🏛️ โครงสร้างข้อมูล Column Family สำหรับ Cassandra
/// เหมือนตารางในสเปรดชีต แต่ยืดหยุ่นกว่า! 📊
#[derive(Debug, Clone)]
pub struct ColumnFamily {
    name: String,
    rows: BTreeMap<String, BTreeMap<String, String>>, // row_key -> column_name -> value
}

impl ColumnFamily {
    /// สร้าง Column Family ใหม่
    /// เริ่มต้นด้วยสเปรดชีตเปล่าๆ 📋
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rows: BTreeMap::new(),
        }
    }
    
    /// เพิ่มข้อมูลในแถว
    /// เขียนข้อมูลลงในช่องตาราง ✏️
    pub fn insert(&mut self, row_key: &str, column: &str, value: &str) {
        self.rows
            .entry(row_key.to_string())
            .or_insert_with(BTreeMap::new)
            .insert(column.to_string(), value.to_string());
        
        println!("📝 เพิ่มข้อมูล: {}[{}] = {} - เขียนลงตารางแล้ว! ✅", row_key, column, value);
    }
    
    /// ดึงข้อมูลจากแถว
    /// อ่านข้อมูลจากช่องเฉพาะ 👀
    pub fn get(&self, row_key: &str, column: &str) -> Option<&String> {
        self.rows.get(row_key)?.get(column)
    }
    
    /// ดึงข้อมูลทั้งแถว
    /// อ่านข้อมูลทั้งแถวเลย! 📖
    pub fn get_row(&self, row_key: &str) -> Option<&BTreeMap<String, String>> {
        self.rows.get(row_key)
    }
    
    /// แสดงข้อมูลทั้งหมด
    pub fn list_all_rows(&self) -> Vec<(String, BTreeMap<String, String>)> {
        self.rows.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

/// 🎯 สาธิตการทำงานกับ NoSQL Databases
/// มาดูโลกของฐานข้อมูลที่ไม่ใช่แค่ SQL กัน! 🌍
pub fn demonstrate_nosql_databases() {
    println!("\n🍃 === NoSQL Databases Demo ===");
    
    // 1. Document Database (MongoDB style) - เหมือนแฟ้มเอกสารดิจิทัล!
    println!("\n1️⃣ Document Database (MongoDB):");
    demonstrate_document_database();
    
    // 2. Key-Value Store (Redis style) - เหมือนกล่องใส่ของมีป้ายชื่อ!
    println!("\n2️⃣ Key-Value Store (Redis):");
    demonstrate_key_value_store();
    
    // 3. Column Family (Cassandra style) - เหมือนสเปรดชีตยืดหยุ่น!
    println!("\n3️⃣ Column Family (Cassandra):");
    demonstrate_column_family();
    
    // 4. การเปรียบเทียบ NoSQL Types - มาดูว่าใครเก่งอะไร!
    println!("\n4️⃣ การเปรียบเทียบประเภท NoSQL:");
    compare_nosql_types();
    
    println!("\n✅ จบการสาธิต NoSQL Databases! - หวังว่าจะชอบโลกของ NoSQL นะ! 🌟");
}

/// 📄 สาธิต Document Database
/// เหมือนการจัดเก็บเอกสารในแฟ้ม แต่เป็นดิจิทัล! 📁✨
fn demonstrate_document_database() {
    println!("📄 การทำงานกับ Document Database - เหมือนห้องสมุดดิจิทัล! 📚:");
    
    // สร้างเอกสาร
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "สมชาย ใจดี".to_string());
    metadata.insert("category".to_string(), "เทคโนโลยี".to_string());
    
    let document = Document {
        _id: "doc_001".to_string(),
        title: "การเรียนรู้ Rust Programming".to_string(),
        content: "Rust เป็นภาษาโปรแกรมมิ่งที่ปลอดภัยและรวดเร็ว...".to_string(),
        tags: vec!["rust".to_string(), "programming".to_string(), "tutorial".to_string()],
        metadata,
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
    };
    
    println!("📝 เอกสาร: {} - น่าอ่านมาก! 📖", document.title);
    println!("   ID: {} - รหัสประจำตัวเอกสาร 🆔", document._id);
    println!("   แท็ก: {:?} - ป้ายกำกับสำหรับค้นหา 🏷️", document.tags);
    println!("   ผู้เขียน: {:?} - คนเก่งที่เขียน! ✍️", document.metadata.get("author"));
    
    // จำลองการค้นหา
    let search_results = simulate_document_search("rust");
    println!("\n🔍 ผลการค้นหา 'rust': {} เอกสาร - เจอเยอะเลย! 🎯", search_results.len());
    for doc in search_results {
        println!("   • {} - น่าสนใจมาก! 📖", doc.title);
    }
}

/// 🔑 สาธิต Key-Value Store
/// เหมือนการใส่ของในกล่องแล้วติดป้ายชื่อ! 📦🏷️
fn demonstrate_key_value_store() {
    println!("🔑 การทำงานกับ Key-Value Store - เหมือนตู้เก็บของส่วนตัว! 🗄️:");
    
    let mut kv_store = KeyValueStore::new();
    
    // เก็บข้อมูลปกติ (ไม่มีวันหมดอายุ)
    kv_store.set("user:1001:name", "สมชาย ใจดี");
    kv_store.set("user:1001:email", "somchai@example.com");
    
    // เก็บข้อมูลพร้อม TTL (มีวันหมดอายุ เหมือนนมในตู้เย็น!)
    kv_store.set_with_ttl("session:abc123", "active", 3600); // 1 ชั่วโมง
    kv_store.set_with_ttl("cache:popular_posts", "[1,2,3,4,5]", 300); // 5 นาที
    
    // ดึงข้อมูล
    if let Some(name) = kv_store.get("user:1001:name") {
        println!("👤 ชื่อผู้ใช้: {} - สวัสดีครับ! 👋", name);
    }
    
    if let Some(session) = kv_store.get("session:abc123") {
        println!("🔐 สถานะ Session: {} - ยังใช้งานได้! ✅", session);
    }
    
    // แสดงข้อมูลทั้งหมด
    println!("\n📋 ข้อมูลทั้งหมดใน Store - มาดูว่ามีอะไรบ้าง! 👀:");
    for (key, value) in kv_store.list_all() {
        println!("   {} = {} 📦", key, value);
    }
}

/// 🏛️ สาธิต Column Family
fn demonstrate_column_family() {
    println!("🏛️ การทำงานกับ Column Family:");
    
    let mut users_cf = ColumnFamily::new("users");
    
    // เพิ่มข้อมูลผู้ใช้
    users_cf.insert("user:1001", "name", "สมชาย ใจดี");
    users_cf.insert("user:1001", "email", "somchai@example.com");
    users_cf.insert("user:1001", "age", "25");
    users_cf.insert("user:1001", "city", "กรุงเทพฯ");
    
    users_cf.insert("user:1002", "name", "สมหญิง รักดี");
    users_cf.insert("user:1002", "email", "somying@example.com");
    users_cf.insert("user:1002", "department", "IT"); // คอลัมน์เพิ่มเติม - ยืดหยุ่นได้!
    
    // ดึงข้อมูลเฉพาะคอลัมน์ (เลือกดูแค่ที่สนใจ!)
    if let Some(name) = users_cf.get("user:1001", "name") {
        println!("👤 ชื่อ: {} - ยินดีที่ได้รู้จัก! 🤝", name);
    }
    
    // ดึงข้อมูลทั้งแถว (เอาทุกอย่างมาเลย!)
    if let Some(user_data) = users_cf.get_row("user:1002") {
        println!("\n👥 ข้อมูลผู้ใช้ 1002 - ข้อมูลครบเซ็ต! 📋:");
        for (column, value) in user_data {
            println!("   {}: {} 📝", column, value);
        }
    }
    
    // แสดงข้อมูลทั้งหมด (มาดูสมบัติทั้งหมด!)
    println!("\n📊 ข้อมูลทั้งหมดใน Column Family - ครบครันมาก! 🎯:");
    for (row_key, columns) in users_cf.list_all_rows() {
        println!("🔑 {} - แถวข้อมูล! 📋", row_key);
        for (column, value) in columns {
            println!("   └─ {}: {} ✨", column, value);
        }
    }
}

/// 📊 เปรียบเทียบประเภท NoSQL
/// มาดูกันว่าแต่ละประเภทเหมาะกับงานไหน! 🎯
fn compare_nosql_types() {
    println!("📊 การเปรียบเทียบประเภท NoSQL - แต่ละตัวมีจุดเด่นต่างกัน! 🌟:");
    
    let nosql_types = vec![
        (
            "Document Store",
            "📄",
            "MongoDB, CouchDB",
            "เหมาะสำหรับข้อมูลที่มีโครงสร้างซับซ้อน - ราชาแห่งความยืดหยุ่น! 👑"
        ),
        (
            "Key-Value Store",
            "🔑",
            "Redis, DynamoDB",
            "เหมาะสำหรับ Cache และ Session Storage - นักวิ่งระยะสั้น! 🏃‍♂️💨"
        ),
        (
            "Column Family",
            "🏛️",
            "Cassandra, HBase",
            "เหมาะสำหรับข้อมูลขนาดใหญ่และ Time Series - นักกีฬาทนทาน! 💪"
        ),
        (
            "Graph Database",
            "🕸️",
            "Neo4j, ArangoDB",
            "เหมาะสำหรับข้อมูลที่มีความสัมพันธ์ซับซ้อน - นักสืบความสัมพันธ์! 🕵️‍♂️"
        ),
    ];
    
    for (name, icon, examples, use_case) in nosql_types {
        println!("   {} {}", icon, name);
        println!("     ตัวอย่าง: {}", examples);
        println!("     การใช้งาน: {}", use_case);
        println!();
    }
    
    println!("🎯 การเลือกใช้ NoSQL - เลือกให้เหมาะกับงาน! 🎪:");
    println!("   • Document: สำหรับ CMS, E-commerce - งานที่ข้อมูลซับซ้อน! 🛒");
    println!("   • Key-Value: สำหรับ Caching, Real-time - งานที่ต้องเร็ว! ⚡");
    println!("   • Column: สำหรับ Analytics, IoT Data - ข้อมูลเยอะๆ! 📊");
    println!("   • Graph: สำหรับ Social Networks, Recommendations - งานที่เชื่อมโยง! 🔗");
}

/// 🔍 จำลองการค้นหาเอกสาร
/// เหมือนการใช้ Google แต่ในฐานข้อมูลเรา! 🔍✨
fn simulate_document_search(query: &str) -> Vec<Document> {
    let mut results = Vec::new();
    
    // จำลองเอกสารในฐานข้อมูล (ห้องสมุดดิจิทัล!)
    let documents = vec![
        Document {
            _id: "doc_001".to_string(),
            title: "การเรียนรู้ Rust Programming".to_string(),
            content: "Rust เป็นภาษาโปรแกรมมิ่งที่ปลอดภัย...".to_string(),
            tags: vec!["rust".to_string(), "programming".to_string()],
            metadata: HashMap::new(),
            created_at: "2024-01-15T10:30:00Z".to_string(),
            updated_at: "2024-01-15T10:30:00Z".to_string(),
        },
        Document {
            _id: "doc_002".to_string(),
            title: "Rust vs Other Languages".to_string(),
            content: "เปรียบเทียบ Rust กับภาษาอื่นๆ...".to_string(),
            tags: vec!["rust".to_string(), "comparison".to_string()],
            metadata: HashMap::new(),
            created_at: "2024-01-16T14:20:00Z".to_string(),
            updated_at: "2024-01-16T14:20:00Z".to_string(),
        },
    ];
    
    // ค้นหาในแท็กและชื่อเรื่อง
    for doc in documents {
        if doc.tags.contains(&query.to_string()) || 
           doc.title.to_lowercase().contains(&query.to_lowercase()) {
            results.push(doc);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// 🧪 ทดสอบ Key-Value Store - มาดูว่าทำงานได้จริงมั้ย! 🔬
    #[test]
    fn test_key_value_store() {
        let mut store = KeyValueStore::new();
        
        // ทดสอบการเก็บและดึงข้อมูล (เหมือนเก็บของในกล่อง!)
        store.set("test_key", "test_value");
        assert_eq!(store.get("test_key"), Some(&"test_value".to_string()));
        
        // ทดสอบการลบข้อมูล (ลาก่อนข้อมูล! 👋)
        assert!(store.delete("test_key"));
        assert_eq!(store.get("test_key"), None);
    }
    
    /// 🧪 ทดสอบ Column Family - ตารางมหัศจรรย์! 🎭
    #[test]
    fn test_column_family() {
        let mut cf = ColumnFamily::new("test_cf");
        
        // ทดสอบการเพิ่มและดึงข้อมูล (เติมข้อมูลในตาราง!)
        cf.insert("row1", "col1", "value1");
        assert_eq!(cf.get("row1", "col1"), Some(&"value1".to_string()));
        
        // ทดสอบการดึงข้อมูลทั้งแถว (เอาทุกอย่างมาเลย!)
        let row = cf.get_row("row1").unwrap();
        assert_eq!(row.get("col1"), Some(&"value1".to_string()));
    }
    
    /// 🧪 ทดสอบการค้นหาเอกสาร - เหมือน Google ในบ้าน! 🏠🔍
    #[test]
    fn test_document_search() {
        let results = simulate_document_search("rust");
        assert!(!results.is_empty());
        
        // ตรวจสอบว่าผลลัพธ์มีคำว่า "rust" (ต้องเจอสิ!)
        for doc in results {
            assert!(doc.tags.contains(&"rust".to_string()) || 
                   doc.title.to_lowercase().contains("rust"));
        }
    }
}