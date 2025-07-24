//! 🗄️ SQL Databases - การทำงานกับฐานข้อมูล SQL
//! 
//! โมดูลนี้สาธิตการทำงานกับฐานข้อมูล SQL ต่างๆ
//! รวมถึง PostgreSQL, MySQL, และ SQLite
//! 
//! 💡 Fun Fact: SQL ย่อมาจาก "Structured Query Language" 
//! แต่บางคนเรียกว่า "Squeal" เพราะเสียงร้องเวลาเจอ bug! 😂

use std::collections::HashMap;

/// 📊 โครงสร้างข้อมูลผู้ใช้
/// เหมือนบัตรประชาชนแต่เวอร์ชันดิจิทัล ไม่มีรูปหน้าตาเศร้า 😅
#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub created_at: String,
}

/// 🔗 โครงสร้างการเชื่อมต่อฐานข้อมูล
/// เหมือนสายโทรศัพท์สมัยก่อน แต่เชื่อมต่อกับฐานข้อมูลแทน 📞
#[derive(Debug)]
pub struct DatabaseConnection {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub connected: bool,
}

impl DatabaseConnection {
    /// สร้างการเชื่อมต่อใหม่
    /// เหมือนการจับมือทักทาย แต่กับฐานข้อมูล 🤝
    pub fn new(host: &str, port: u16, database: &str, username: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            connected: false,
        }
    }
    
    /// เชื่อมต่อฐานข้อมูล
    /// ขั้นตอนนี้เหมือนการเคาะประตูบ้านเพื่อน หวังว่าจะเปิดให้ 🚪
    pub fn connect(&mut self) -> Result<(), String> {
        println!("🔌 กำลังเชื่อมต่อ {}:{}...", self.host, self.port);
        
        // จำลองการเชื่อมต่อ (แบบจำลองนะ ไม่ใช่ของจริง!)
        if self.host.is_empty() {
            return Err("Host ไม่สามารถเป็นค่าว่างได้ - เหมือนไปหาเพื่อนแต่ไม่รู้ที่อยู่! 🏠❓".to_string());
        }
        
        self.connected = true;
        println!("✅ เชื่อมต่อฐานข้อมูล {} สำเร็จ!", self.database);
        Ok(())
    }
    
    /// ตัดการเชื่อมต่อ
    /// เหมือนการบอกลาเพื่อน "ไปก่อนนะ จะกลับมาใหม่!" 👋
    pub fn disconnect(&mut self) {
        if self.connected {
            self.connected = false;
            println!("🔌 ตัดการเชื่อมต่อฐานข้อมูล {} แล้ว", self.database);
        }
    }
}

/// 📝 SQL Query Builder
/// เหมือนเชฟที่ปรุงอาหาร แต่ปรุง SQL Query แทน! 👨‍🍳✨
#[derive(Debug)]
pub struct QueryBuilder {
    query_type: String,
    table: String,
    fields: Vec<String>,
    conditions: Vec<String>,
    values: HashMap<String, String>,
}

impl QueryBuilder {
    /// สร้าง Query Builder ใหม่
    /// เริ่มต้นด้วยจานเปล่า พร้อมปรุงเมนูพิเศษ! 🍽️
    pub fn new() -> Self {
        Self {
            query_type: String::new(),
            table: String::new(),
            fields: Vec::new(),
            conditions: Vec::new(),
            values: HashMap::new(),
        }
    }
    
    /// SELECT query
    /// เลือกข้อมูลเหมือนเลือกของในซุปเปอร์มาร์เก็ต 🛒
    pub fn select(mut self, fields: &[&str]) -> Self {
        self.query_type = "SELECT".to_string();
        self.fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// FROM clause  
    /// บอกว่าจะไปเอาข้อมูลจากโต๊ะไหน เหมือนบอกแผนกในห้าง 🏪
    pub fn from(mut self, table: &str) -> Self {
        self.table = table.to_string();
        self
    }
    
    /// WHERE clause
    /// ใส่เงื่อนไขเหมือนการกรองกาแฟ เอาแต่เมล็ดดีๆ ☕
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }
    
    /// INSERT query
    /// เพิ่มข้อมูลใหม่เหมือนใส่ของลงในกล่อง 📦
    pub fn insert_into(mut self, table: &str) -> Self {
        self.query_type = "INSERT".to_string();
        self.table = table.to_string();
        self
    }
    
    /// VALUES clause
    /// ใส่ค่าข้อมูลเหมือนใส่ของขวัญในกล่อง 🎁
    pub fn values(mut self, values: HashMap<String, String>) -> Self {
        self.values = values;
        self
    }
    
    /// สร้าง SQL query
    /// ขั้นตอนสุดท้าย! เหมือนการห่อของขวัญให้สวยงาม 🎀
    pub fn build(&self) -> String {
        match self.query_type.as_str() {
            "SELECT" => {
                let fields = if self.fields.is_empty() {
                    "*".to_string()
                } else {
                    self.fields.join(", ")
                };
                
                let mut query = format!("SELECT {} FROM {}", fields, self.table);
                
                if !self.conditions.is_empty() {
                    query.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
                }
                
                query
            },
            "INSERT" => {
                let fields: Vec<String> = self.values.keys().cloned().collect();
                let values: Vec<String> = self.values.values().map(|v| format!("'{}'", v)).collect();
                
                format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    self.table,
                    fields.join(", "),
                    values.join(", ")
                )
            },
            _ => "Invalid query type".to_string(),
        }
    }
}

/// 🎯 สาธิตการทำงานกับ SQL Databases
/// เหมือนการแสดงมายากล แต่กับฐานข้อมูล! 🎩✨
pub fn demonstrate_sql_databases() {
    println!("\n🗄️ === SQL Databases Demo ===");
    
    // 1. การเชื่อมต่อฐานข้อมูล (ขั้นตอนสำคัญที่สุด!)
    println!("\n1️⃣ การเชื่อมต่อฐานข้อมูล:");
    let mut postgres_conn = DatabaseConnection::new(
        "localhost", 
        5432, 
        "myapp_db", 
        "postgres"
    );
    
    match postgres_conn.connect() {
        Ok(_) => println!("✅ เชื่อมต่อ PostgreSQL สำเร็จ - ยินดีต้อนรับสู่โลกของข้อมูล! 🎉"),
        Err(e) => println!("❌ เชื่อมต่อล้มเหลว: {} - อย่าเศร้านะ ลองใหม่ได้! 😅", e),
    }
    
    // 2. การสร้าง SQL Queries (เหมือนการเขียนจดหมายถึงฐานข้อมูล)
    println!("\n2️⃣ การสร้าง SQL Queries:");
    
    // SELECT query
    let select_query = QueryBuilder::new()
        .select(&["id", "name", "email"])
        .from("users")
        .where_clause("age > 18")
        .where_clause("active = true")
        .build();
    
    println!("📝 SELECT Query: {} - เลือกข้อมูลแบบมีสไตล์! 😎", select_query);
    
    // INSERT query
    let mut insert_values = HashMap::new();
    insert_values.insert("name".to_string(), "สมชาย ใจดี".to_string());
    insert_values.insert("email".to_string(), "somchai@example.com".to_string());
    insert_values.insert("age".to_string(), "25".to_string());
    
    let insert_query = QueryBuilder::new()
        .insert_into("users")
        .values(insert_values)
        .build();
    
    println!("📝 INSERT Query: {} - เพิ่มข้อมูลใหม่เข้าไปในครอบครัว! 👨‍👩‍👧‍👦", insert_query);
    
    // 3. การจำลองการดึงข้อมูล (แบบจำลองนะ ไม่ใช่ของจริง!)
    println!("\n3️⃣ การจำลองการดึงข้อมูล:");
    let users = simulate_fetch_users();
    
    println!("👥 ผู้ใช้ในระบบ ({} คน) - ครอบครัวใหญ่เลยนะ! 👨‍👩‍👧‍👦:", users.len());
    for user in &users {
        println!("   • {} ({}): {} - อายุ {} ปี 🎂", 
                user.id, user.name, user.email, user.age);
    }
    
    // 4. การจัดการ Database Types (รู้จักเพื่อนๆ ฐานข้อมูล)
    println!("\n4️⃣ ประเภทฐานข้อมูล SQL:");
    demonstrate_database_types();
    
    // 5. การปิดการเชื่อมต่อ
    postgres_conn.disconnect();
    
    println!("\n✅ จบการสาธิต SQL Databases! - หวังว่าจะสนุกนะ! 🎉");
}

/// 📊 จำลองการดึงข้อมูลผู้ใช้
/// สร้างข้อมูลปลอมๆ เหมือนนักแสดงในละคร! 🎭
fn simulate_fetch_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "สมชาย ใจดี".to_string(),
            email: "somchai@example.com".to_string(),
            age: 25,
            created_at: "2024-01-15".to_string(),
        },
        User {
            id: 2,
            name: "สมหญิง สวยงาม".to_string(),
            email: "somying@example.com".to_string(),
            age: 28,
            created_at: "2024-01-20".to_string(),
        },
        User {
            id: 3,
            name: "วิชัย เก่งมาก".to_string(),
            email: "wichai@example.com".to_string(),
            age: 32,
            created_at: "2024-01-25".to_string(),
        },
    ]
}

/// 🗃️ สาธิตประเภทฐานข้อมูล SQL ต่างๆ
/// แนะนำเพื่อนๆ ฐานข้อมูลที่น่ารัก! 🤗
fn demonstrate_database_types() {
    println!("📋 ประเภทฐานข้อมูล SQL ที่นิยม - เหมือนการเลือกสัตว์เลี้ยง! 🐾:");
    
    let databases = vec![
        ("PostgreSQL", "🐘", "ฐานข้อมูลแบบ Object-Relational ที่มีฟีเจอร์ครบครัน - เหมือนช้างที่จำได้ทุกอย่าง!"),
        ("MySQL", "🐬", "ฐานข้อมูลแบบ Relational ที่เร็วและเสถียร - ว่ายน้ำเร็วเหมือนโลมา!"),
        ("SQLite", "🪶", "ฐานข้อมูลแบบ Embedded ที่เบาและใช้งานง่าย - เบาเหมือนขนนก!"),
        ("MariaDB", "🦭", "ฐานข้อมูลแบบ Open Source ที่พัฒนาจาก MySQL - น้องสาวของ MySQL!"),
    ];
    
    for (name, icon, description) in databases {
        println!("   {} {}: {}", icon, name, description);
    }
    
    println!("\n🔧 การเลือกใช้ฐานข้อมูล - เหมือนการเลือกรถใช้! 🚗:");
    println!("   • PostgreSQL: สำหรับแอปพลิเคชันที่ซับซ้อน - เหมือนรถหรู มีฟีเจอร์เยอะ! 🚙");
    println!("   • MySQL: สำหรับเว็บแอปพลิเคชันทั่วไป - เหมือนรถเก๋ง ใช้ง่าย! 🚘");
    println!("   • SQLite: สำหรับแอปพลิเคชันขนาดเล็กหรือ Prototype - เหมือนจักรยาน เบาๆ! 🚲");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_connection() {
        let mut conn = DatabaseConnection::new("localhost", 5432, "test_db", "user");
        assert!(!conn.connected);
        
        conn.connect().unwrap();
        assert!(conn.connected);
        
        conn.disconnect();
        assert!(!conn.connected);
    }
    
    #[test]
    fn test_query_builder_select() {
        let query = QueryBuilder::new()
            .select(&["name", "email"])
            .from("users")
            .where_clause("age > 18")
            .build();
        
        assert_eq!(query, "SELECT name, email FROM users WHERE age > 18");
    }
    
    #[test]
    fn test_query_builder_insert() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Test User".to_string());
        values.insert("email".to_string(), "test@example.com".to_string());
        
        let query = QueryBuilder::new()
            .insert_into("users")
            .values(values)
            .build();
        
        assert!(query.contains("INSERT INTO users"));
        assert!(query.contains("Test User"));
        assert!(query.contains("test@example.com"));
    }
}