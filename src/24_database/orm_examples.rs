//! 🔗 ORM Examples - การทำงานกับ Object-Relational Mapping
//! 
//! โมดูลนี้สาธิตการใช้งาน ORM และ Database Abstraction Layer
//! รวมถึง Diesel, SQLx, และ SeaORM patterns

use std::collections::HashMap;
use std::fmt;

/// 📊 โครงสร้างข้อมูล User Model
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
}

impl User {
    /// สร้าง User ใหม่
    pub fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            id: None,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: format!("hashed_{}", password), // จำลองการ hash
            created_at: "2024-01-15T10:30:00Z".to_string(),
            updated_at: "2024-01-15T10:30:00Z".to_string(),
            is_active: true,
        }
    }
    
    /// ตรวจสอบรหัสผ่าน
    pub fn verify_password(&self, password: &str) -> bool {
        self.password_hash == format!("hashed_{}", password)
    }
}

/// 📝 โครงสร้างข้อมูล Post Model
#[derive(Debug, Clone)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 🔍 Query Builder Pattern
#[derive(Debug)]
pub struct QueryBuilder<T> {
    table_name: String,
    conditions: Vec<String>,
    order_by: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    /// สร้าง Query Builder ใหม่
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            conditions: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// เพิ่มเงื่อนไข WHERE
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.conditions.push(format!("{} = '{}'", column, value));
        self
    }
    
    /// เพิ่มเงื่อนไข WHERE LIKE
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        self.conditions.push(format!("{} LIKE '%{}%'", column, pattern));
        self
    }
    
    /// เรียงลำดับ
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by.push(format!("{} {}", column, direction));
        self
    }
    
    /// จำกัดจำนวนผลลัพธ์
    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }
    
    /// ข้ามผลลัพธ์
    pub fn offset(mut self, count: usize) -> Self {
        self.offset = Some(count);
        self
    }
    
    /// สร้าง SQL Query
    pub fn to_sql(&self) -> String {
        let mut query = format!("SELECT * FROM {}", self.table_name);
        
        if !self.conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        
        if !self.order_by.is_empty() {
            query.push_str(&format!(" ORDER BY {}", self.order_by.join(", ")));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        query
    }
}

/// 🗄️ Repository Pattern
pub trait Repository<T> {
    type Error;
    
    fn find_by_id(&self, id: i32) -> Result<Option<T>, Self::Error>;
    fn find_all(&self) -> Result<Vec<T>, Self::Error>;
    fn create(&mut self, entity: &T) -> Result<T, Self::Error>;
    fn update(&mut self, entity: &T) -> Result<T, Self::Error>;
    fn delete(&mut self, id: i32) -> Result<bool, Self::Error>;
}

/// 👤 User Repository Implementation
pub struct UserRepository {
    users: HashMap<i32, User>,
    next_id: i32,
}

impl UserRepository {
    /// สร้าง Repository ใหม่
    pub fn new() -> Self {
        let mut repo = Self {
            users: HashMap::new(),
            next_id: 1,
        };
        
        // เพิ่มข้อมูลตัวอย่าง
        repo.seed_data();
        repo
    }
    
    /// เพิ่มข้อมูลตัวอย่าง
    fn seed_data(&mut self) {
        let users = vec![
            User {
                id: Some(1),
                username: "somchai".to_string(),
                email: "somchai@example.com".to_string(),
                password_hash: "hashed_password123".to_string(),
                created_at: "2024-01-15T10:30:00Z".to_string(),
                updated_at: "2024-01-15T10:30:00Z".to_string(),
                is_active: true,
            },
            User {
                id: Some(2),
                username: "somying".to_string(),
                email: "somying@example.com".to_string(),
                password_hash: "hashed_mypassword".to_string(),
                created_at: "2024-01-16T14:20:00Z".to_string(),
                updated_at: "2024-01-16T14:20:00Z".to_string(),
                is_active: true,
            },
        ];
        
        for user in users {
            if let Some(id) = user.id {
                self.users.insert(id, user);
                if id >= self.next_id {
                    self.next_id = id + 1;
                }
            }
        }
    }
    
    /// ค้นหาผู้ใช้ด้วย username
    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username)
    }
    
    /// ค้นหาผู้ใช้ด้วย email
    pub fn find_by_email(&self, email: &str) -> Option<&User> {
        self.users.values().find(|user| user.email == email)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    ValidationError(String),
    DatabaseError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "ไม่พบข้อมูล"),
            RepositoryError::ValidationError(msg) => write!(f, "ข้อมูลไม่ถูกต้อง: {}", msg),
            RepositoryError::DatabaseError(msg) => write!(f, "ข้อผิดพลาดฐานข้อมูล: {}", msg),
        }
    }
}

impl Repository<User> for UserRepository {
    type Error = RepositoryError;
    
    fn find_by_id(&self, id: i32) -> Result<Option<User>, Self::Error> {
        Ok(self.users.get(&id).cloned())
    }
    
    fn find_all(&self) -> Result<Vec<User>, Self::Error> {
        Ok(self.users.values().cloned().collect())
    }
    
    fn create(&mut self, entity: &User) -> Result<User, Self::Error> {
        // ตรวจสอบข้อมูล
        if entity.username.is_empty() {
            return Err(RepositoryError::ValidationError(
                "Username ไม่สามารถเป็นค่าว่างได้".to_string()
            ));
        }
        
        if entity.email.is_empty() || !entity.email.contains('@') {
            return Err(RepositoryError::ValidationError(
                "Email ไม่ถูกต้อง".to_string()
            ));
        }
        
        // ตรวจสอบว่า username หรือ email ซ้ำหรือไม่
        if self.find_by_username(&entity.username).is_some() {
            return Err(RepositoryError::ValidationError(
                "Username นี้มีอยู่แล้ว".to_string()
            ));
        }
        
        if self.find_by_email(&entity.email).is_some() {
            return Err(RepositoryError::ValidationError(
                "Email นี้มีอยู่แล้ว".to_string()
            ));
        }
        
        // สร้างผู้ใช้ใหม่
        let mut new_user = entity.clone();
        new_user.id = Some(self.next_id);
        
        self.users.insert(self.next_id, new_user.clone());
        self.next_id += 1;
        
        println!("✅ สร้างผู้ใช้ใหม่: {} (ID: {})", new_user.username, new_user.id.unwrap());
        Ok(new_user)
    }
    
    fn update(&mut self, entity: &User) -> Result<User, Self::Error> {
        let id = entity.id.ok_or(RepositoryError::ValidationError(
            "ID ไม่สามารถเป็นค่าว่างได้".to_string()
        ))?;
        
        if !self.users.contains_key(&id) {
            return Err(RepositoryError::NotFound);
        }
        
        let mut updated_user = entity.clone();
        updated_user.updated_at = "2024-01-17T15:45:00Z".to_string();
        
        self.users.insert(id, updated_user.clone());
        
        println!("✅ อัปเดตผู้ใช้: {} (ID: {})", updated_user.username, id);
        Ok(updated_user)
    }
    
    fn delete(&mut self, id: i32) -> Result<bool, Self::Error> {
        if let Some(user) = self.users.remove(&id) {
            println!("🗑️ ลบผู้ใช้: {} (ID: {})", user.username, id);
            Ok(true)
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}

/// 🔄 Migration System
#[derive(Debug)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

impl Migration {
    /// สร้าง Migration ใหม่
    pub fn new(version: &str, name: &str, up_sql: &str, down_sql: &str) -> Self {
        Self {
            version: version.to_string(),
            name: name.to_string(),
            up_sql: up_sql.to_string(),
            down_sql: down_sql.to_string(),
        }
    }
}

/// 🎯 สาธิตการทำงานกับ ORM
pub fn demonstrate_orm_examples() {
    println!("\n🔗 === ORM Examples Demo ===");
    
    // 1. Query Builder Pattern
    println!("\n1️⃣ Query Builder Pattern:");
    demonstrate_query_builder();
    
    // 2. Repository Pattern
    println!("\n2️⃣ Repository Pattern:");
    demonstrate_repository_pattern();
    
    // 3. Migration System
    println!("\n3️⃣ Migration System:");
    demonstrate_migrations();
    
    // 4. ORM Patterns Comparison
    println!("\n4️⃣ การเปรียบเทียบ ORM Patterns:");
    compare_orm_patterns();
    
    println!("\n✅ จบการสาธิต ORM Examples!");
}

/// 🔍 สาธิต Query Builder
fn demonstrate_query_builder() {
    println!("🔍 การใช้งาน Query Builder:");
    
    // สร้าง Query สำหรับค้นหาผู้ใช้
    let user_query = QueryBuilder::<User>::new("users")
        .where_eq("is_active", "true")
        .where_like("username", "som")
        .order_by("created_at", "DESC")
        .limit(10)
        .to_sql();
    
    println!("📝 User Query: {}", user_query);
    
    // สร้าง Query สำหรับค้นหาโพสต์
    let post_query = QueryBuilder::<Post>::new("posts")
        .where_eq("published", "true")
        .where_eq("author_id", "1")
        .order_by("updated_at", "DESC")
        .limit(5)
        .offset(0)
        .to_sql();
    
    println!("📝 Post Query: {}", post_query);
    
    // Query แบบซับซ้อน
    let complex_query = QueryBuilder::<User>::new("users")
        .where_eq("is_active", "true")
        .where_like("email", "@example.com")
        .order_by("username", "ASC")
        .order_by("created_at", "DESC")
        .limit(20)
        .to_sql();
    
    println!("📝 Complex Query: {}", complex_query);
}

/// 🗄️ สาธิต Repository Pattern
fn demonstrate_repository_pattern() {
    println!("🗄️ การใช้งาน Repository Pattern:");
    
    let mut user_repo = UserRepository::new();
    
    // แสดงผู้ใช้ทั้งหมด
    match user_repo.find_all() {
        Ok(users) => {
            println!("👥 ผู้ใช้ทั้งหมด ({} คน):", users.len());
            for user in &users {
                println!("   • {} ({}): {}", 
                        user.id.unwrap_or(0), user.username, user.email);
            }
        },
        Err(e) => println!("❌ ข้อผิดพลาด: {}", e),
    }
    
    // ค้นหาผู้ใช้ด้วย ID
    match user_repo.find_by_id(1) {
        Ok(Some(user)) => println!("\n🔍 พบผู้ใช้ ID 1: {}", user.username),
        Ok(None) => println!("\n🔍 ไม่พบผู้ใช้ ID 1"),
        Err(e) => println!("\n❌ ข้อผิดพลาด: {}", e),
    }
    
    // สร้างผู้ใช้ใหม่
    let new_user = User::new("wichai", "wichai@example.com", "mypassword");
    match user_repo.create(&new_user) {
        Ok(created_user) => {
            println!("\n✅ สร้างผู้ใช้สำเร็จ: {} (ID: {})", 
                    created_user.username, created_user.id.unwrap());
        },
        Err(e) => println!("\n❌ สร้างผู้ใช้ล้มเหลว: {}", e),
    }
    
    // ลองสร้างผู้ใช้ที่ username ซ้ำ
    let duplicate_user = User::new("somchai", "somchai2@example.com", "password");
    match user_repo.create(&duplicate_user) {
        Ok(_) => println!("\n✅ สร้างผู้ใช้สำเร็จ"),
        Err(e) => println!("\n❌ สร้างผู้ใช้ล้มเหลว: {}", e),
    }
}

/// 🔄 สาธิต Migration System
fn demonstrate_migrations() {
    println!("🔄 การใช้งาน Migration System:");
    
    let migrations = vec![
        Migration::new(
            "001",
            "create_users_table",
            "CREATE TABLE users (\n    id SERIAL PRIMARY KEY,\n    username VARCHAR(50) UNIQUE NOT NULL,\n    email VARCHAR(100) UNIQUE NOT NULL,\n    password_hash VARCHAR(255) NOT NULL,\n    created_at TIMESTAMP DEFAULT NOW(),\n    updated_at TIMESTAMP DEFAULT NOW(),\n    is_active BOOLEAN DEFAULT TRUE\n);",
            "DROP TABLE users;"
        ),
        Migration::new(
            "002",
            "create_posts_table",
            "CREATE TABLE posts (\n    id SERIAL PRIMARY KEY,\n    title VARCHAR(200) NOT NULL,\n    content TEXT NOT NULL,\n    author_id INTEGER REFERENCES users(id),\n    published BOOLEAN DEFAULT FALSE,\n    created_at TIMESTAMP DEFAULT NOW(),\n    updated_at TIMESTAMP DEFAULT NOW()\n);",
            "DROP TABLE posts;"
        ),
        Migration::new(
            "003",
            "add_user_profile_fields",
            "ALTER TABLE users ADD COLUMN first_name VARCHAR(50);\nALTER TABLE users ADD COLUMN last_name VARCHAR(50);\nALTER TABLE users ADD COLUMN bio TEXT;",
            "ALTER TABLE users DROP COLUMN first_name;\nALTER TABLE users DROP COLUMN last_name;\nALTER TABLE users DROP COLUMN bio;"
        ),
    ];
    
    println!("📋 Available Migrations:");
    for migration in &migrations {
        println!("   {} - {}", migration.version, migration.name);
    }
    
    // จำลองการรัน Migration
    println!("\n🔄 Running Migrations:");
    for migration in &migrations {
        println!("   ⬆️ Running migration {}: {}", migration.version, migration.name);
        println!("      SQL: {}", migration.up_sql.lines().next().unwrap_or(""));
    }
    
    println!("\n✅ All migrations completed successfully!");
}

/// 📊 เปรียบเทียบ ORM Patterns
fn compare_orm_patterns() {
    println!("📊 การเปรียบเทียบ ORM Patterns:");
    
    let patterns = vec![
        (
            "Active Record",
            "🎯",
            "Model มี method สำหรับ database operations",
            "ง่ายต่อการใช้งาน แต่ coupling สูง"
        ),
        (
            "Data Mapper",
            "🗺️",
            "แยก Model และ Database Logic",
            "ยืดหยุ่นสูง แต่ซับซ้อนกว่า"
        ),
        (
            "Repository Pattern",
            "🗄️",
            "Abstract layer สำหรับ data access",
            "ง่ายต่อการ test และ maintain"
        ),
        (
            "Query Builder",
            "🔍",
            "สร้าง SQL แบบ programmatic",
            "ยืดหยุ่นและ type-safe"
        ),
    ];
    
    for (name, icon, description, pros_cons) in patterns {
        println!("   {} {}", icon, name);
        println!("     คำอธิบาย: {}", description);
        println!("     ข้อดี/ข้อเสีย: {}", pros_cons);
        println!();
    }
    
    println!("🎯 การเลือกใช้ ORM Pattern:");
    println!("   • Active Record: สำหรับโปรเจกต์เล็กๆ ที่ต้องการความเร็ว");
    println!("   • Data Mapper: สำหรับโปรเจกต์ใหญ่ที่ต้องการความยืดหยุ่น");
    println!("   • Repository: สำหรับโปรเจกต์ที่ต้องการ testability สูง");
    println!("   • Query Builder: สำหรับ query ที่ซับซ้อนและต้องการ performance");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new("testuser", "test@example.com", "password123");
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.verify_password("password123"));
        assert!(!user.verify_password("wrongpassword"));
    }
    
    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::<User>::new("users")
            .where_eq("active", "true")
            .order_by("name", "ASC")
            .limit(10)
            .to_sql();
        
        assert!(query.contains("SELECT * FROM users"));
        assert!(query.contains("WHERE active = 'true'"));
        assert!(query.contains("ORDER BY name ASC"));
        assert!(query.contains("LIMIT 10"));
    }
    
    #[test]
    fn test_user_repository() {
        let mut repo = UserRepository::new();
        
        // Test find_all
        let users = repo.find_all().unwrap();
        assert!(!users.is_empty());
        
        // Test find_by_id
        let user = repo.find_by_id(1).unwrap();
        assert!(user.is_some());
        
        // Test create
        let new_user = User::new("newuser", "new@example.com", "password");
        let created = repo.create(&new_user).unwrap();
        assert!(created.id.is_some());
        
        // Test duplicate username
        let duplicate = User::new("newuser", "another@example.com", "password");
        assert!(repo.create(&duplicate).is_err());
    }
}