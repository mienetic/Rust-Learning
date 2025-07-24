//! Database Integration Example
//! ตัวอย่างการทำงานกับฐานข้อมูล `SQLite` ใน Rust
//!
//! การรันตัวอย่างนี้:
//! ```bash
//! cargo run --example database_example
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// โมเดลข้อมูลผู้ใช้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    #[must_use]
    pub fn new(name: String, email: String, age: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            age,
            created_at: chrono::Utc::now(),
        }
    }
}

/// Database connection และ operations
pub struct Database {
    file_path: std::path::PathBuf,
    users: Vec<User>,
}

impl Database {
    /// สร้าง database connection ใหม่
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            file_path: file_path.as_ref().to_path_buf(),
            users: Vec::new(),
        }
    }

    /// เชื่อมต่อและสร้างตารางถ้ายังไม่มี
    pub fn connect(&mut self) -> Result<()> {
        // ในตัวอย่างนี้เราจะใช้ JSON file แทน SQLite เพื่อความง่าย
        // ในโปรเจกต์จริงควรใช้ database จริงๆ เช่น SQLite, PostgreSQL

        if self.file_path.exists() {
            self.load_from_file()
                .context("Failed to load existing database")?;
        } else {
            // สร้างไฟล์ใหม่
            self.save_to_file()
                .context("Failed to create new database file")?;
        }

        println!("✅ Connected to database: {:?}", self.file_path);
        Ok(())
    }

    /// เพิ่มผู้ใช้ใหม่
    pub fn create_user(&mut self, name: String, email: String, age: u32) -> Result<Uuid> {
        // ตรวจสอบว่า email ไม่ซ้ำ
        if self.users.iter().any(|u| u.email == email) {
            anyhow::bail!("Email already exists: {}", email);
        }

        let user = User::new(name, email, age);
        let user_id = user.id;
        self.users.push(user);

        self.save_to_file()
            .context("Failed to save user to database")?;

        println!("✅ Created user with ID: {user_id}");
        Ok(user_id)
    }

    /// ค้นหาผู้ใช้ตาม ID
    #[must_use]
    pub fn get_user(&self, id: &Uuid) -> Option<&User> {
        self.users.iter().find(|u| u.id == *id)
    }

    /// ค้นหาผู้ใช้ตาม email
    #[must_use]
    pub fn get_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|u| u.email == email)
    }

    /// อัปเดตข้อมูลผู้ใช้
    pub fn update_user(
        &mut self,
        id: &Uuid,
        name: Option<String>,
        email: Option<String>,
        age: Option<u32>,
    ) -> Result<()> {
        // ตรวจสอบว่า email ใหม่ไม่ซ้ำกับคนอื่น (ถ้ามี)
        if let Some(ref new_email) = email {
            if self
                .users
                .iter()
                .any(|u| u.id != *id && u.email == *new_email)
            {
                anyhow::bail!("Email already exists: {}", new_email);
            }
        }

        // หาและอัปเดตผู้ใช้
        let user = self
            .users
            .iter_mut()
            .find(|u| u.id == *id)
            .ok_or_else(|| anyhow::anyhow!("User not found: {}", id))?;

        if let Some(new_name) = name {
            user.name = new_name;
        }

        if let Some(new_email) = email {
            user.email = new_email;
        }

        if let Some(new_age) = age {
            user.age = new_age;
        }

        self.save_to_file().context("Failed to save updated user")?;

        println!("✅ Updated user: {id}");
        Ok(())
    }

    /// ลบผู้ใช้
    pub fn delete_user(&mut self, id: &Uuid) -> Result<()> {
        let initial_len = self.users.len();
        self.users.retain(|u| u.id != *id);

        if self.users.len() == initial_len {
            anyhow::bail!("User not found: {}", id);
        }

        self.save_to_file()
            .context("Failed to save after deleting user")?;

        println!("✅ Deleted user: {id}");
        Ok(())
    }

    /// ดึงรายการผู้ใช้ทั้งหมด
    #[must_use]
    pub fn list_users(&self) -> &[User] {
        &self.users
    }

    /// ค้นหาผู้ใช้ตามเงื่อนไข
    #[must_use]
    pub fn search_users(&self, query: &str) -> Vec<&User> {
        let query_lower = query.to_lowercase();
        self.users
            .iter()
            .filter(|u| {
                u.name.to_lowercase().contains(&query_lower)
                    || u.email.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// สถิติผู้ใช้
    #[must_use]
    pub fn get_statistics(&self) -> UserStatistics {
        let total_users = self.users.len();
        let average_age = if total_users > 0 {
            self.users.iter().map(|u| f64::from(u.age)).sum::<f64>() / total_users as f64
        } else {
            0.0
        };

        let oldest_user = self.users.iter().max_by_key(|u| u.age);
        let youngest_user = self.users.iter().min_by_key(|u| u.age);

        UserStatistics {
            total_users,
            average_age,
            oldest_user_age: oldest_user.map(|u| u.age),
            youngest_user_age: youngest_user.map(|u| u.age),
        }
    }

    /// บันทึกข้อมูลลงไฟล์
    fn save_to_file(&self) -> Result<()> {
        let json_data =
            serde_json::to_string_pretty(&self.users).context("Failed to serialize users")?;

        std::fs::write(&self.file_path, json_data).context("Failed to write to file")?;

        Ok(())
    }

    /// โหลดข้อมูลจากไฟล์
    fn load_from_file(&mut self) -> Result<()> {
        let json_data =
            std::fs::read_to_string(&self.file_path).context("Failed to read from file")?;

        self.users = serde_json::from_str(&json_data).context("Failed to deserialize users")?;

        Ok(())
    }
}

/// สถิติผู้ใช้
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatistics {
    pub total_users: usize,
    pub average_age: f64,
    pub oldest_user_age: Option<u32>,
    pub youngest_user_age: Option<u32>,
}

/// Transaction wrapper สำหรับการทำงานแบบ atomic
pub struct Transaction<'a> {
    database: &'a mut Database,
    backup_users: Vec<User>,
}

impl<'a> Transaction<'a> {
    pub fn new(database: &'a mut Database) -> Self {
        let backup_users = database.users.clone();
        Self {
            database,
            backup_users,
        }
    }

    /// Commit การเปลี่ยนแปลง
    pub fn commit(self) -> Result<()> {
        self.database
            .save_to_file()
            .context("Failed to commit transaction")?;
        println!("✅ Transaction committed");
        Ok(())
    }

    /// Rollback การเปลี่ยนแปลง
    pub fn rollback(self) {
        self.database.users = self.backup_users;
        println!("⚠️ Transaction rolled back");
    }

    /// เข้าถึง database ภายใน transaction
    pub const fn database(&mut self) -> &mut Database {
        self.database
    }
}

/// ตัวอย่างการใช้งาน
fn main() -> Result<()> {
    println!("🚀 Database Integration Example");
    println!("=================================\n");

    // สร้าง database connection
    let db_path = "users.db.json";
    let mut db = Database::new(db_path);

    // เชื่อมต่อ database
    db.connect()?;

    // ตัวอย่างการใช้งาน CRUD operations
    println!("\n📝 Creating users...");
    let user1_id = db.create_user(
        "Alice Johnson".to_string(),
        "alice@example.com".to_string(),
        28,
    )?;

    let user2_id = db.create_user("Bob Smith".to_string(), "bob@example.com".to_string(), 35)?;

    let user3_id = db.create_user(
        "Charlie Brown".to_string(),
        "charlie@example.com".to_string(),
        22,
    )?;

    // อ่านข้อมูลผู้ใช้
    println!("\n📖 Reading users...");
    if let Some(user) = db.get_user(&user1_id) {
        println!("Found user: {} ({})", user.name, user.email);
    }

    // ค้นหาผู้ใช้ตาม email
    if let Some(user) = db.get_user_by_email("bob@example.com") {
        println!("Found user by email: {} (age: {})", user.name, user.age);
    }

    // อัปเดตข้อมูลผู้ใช้
    println!("\n✏️ Updating user...");
    db.update_user(&user2_id, Some("Robert Smith".to_string()), None, Some(36))?;

    // แสดงรายการผู้ใช้ทั้งหมด
    println!("\n👥 All users:");
    for user in db.list_users() {
        println!(
            "  - {} ({}) - Age: {}, Created: {}",
            user.name,
            user.email,
            user.age,
            user.created_at.format("%Y-%m-%d %H:%M:%S")
        );
    }

    // ค้นหาผู้ใช้
    println!("\n🔍 Searching for 'Smith':");
    let search_results = db.search_users("Smith");
    for user in search_results {
        println!("  - Found: {} ({})", user.name, user.email);
    }

    // แสดงสถิติ
    println!("\n📊 User Statistics:");
    let stats = db.get_statistics();
    println!("  - Total users: {}", stats.total_users);
    println!("  - Average age: {:.1}", stats.average_age);
    if let Some(oldest) = stats.oldest_user_age {
        println!("  - Oldest user age: {oldest}");
    }
    if let Some(youngest) = stats.youngest_user_age {
        println!("  - Youngest user age: {youngest}");
    }

    // ตัวอย่างการใช้ Transaction
    println!("\n💾 Transaction example...");
    {
        let mut transaction = Transaction::new(&mut db);

        // ทำการเปลี่ยนแปลงใน transaction
        let temp_user_id = transaction.database().create_user(
            "Temporary User".to_string(),
            "temp@example.com".to_string(),
            25,
        )?;

        println!("Created temporary user in transaction");

        // จำลองข้อผิดพลาด - rollback transaction
        if !temp_user_id.to_string().is_empty() {
            transaction.rollback();
            println!("Transaction was rolled back due to simulated error");
        }
    }

    // ตัวอย่าง successful transaction
    {
        let mut transaction = Transaction::new(&mut db);

        transaction.database().create_user(
            "Permanent User".to_string(),
            "permanent@example.com".to_string(),
            30,
        )?;

        // Commit transaction
        transaction.commit()?;
    }

    // ลบผู้ใช้
    println!("\n🗑️ Deleting user...");
    db.delete_user(&user3_id)?;

    // แสดงรายการสุดท้าย
    println!("\n👥 Final user list:");
    for user in db.list_users() {
        println!("  - {} ({}) - Age: {}", user.name, user.email, user.age);
    }

    // ทดสอบ error handling
    println!("\n❌ Testing error handling...");

    // ลองสร้างผู้ใช้ที่มี email ซ้ำ
    match db.create_user("Duplicate".to_string(), "alice@example.com".to_string(), 25) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {e}"),
    }

    // ลองอัปเดตผู้ใช้ที่ไม่มีอยู่
    let fake_id = Uuid::new_v4();
    match db.update_user(&fake_id, Some("Fake".to_string()), None, None) {
        Ok(()) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {e}"),
    }

    println!("\n✅ Database example completed successfully!");
    println!("📁 Database file saved as: {db_path}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> (Database, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db.json");
        let mut db = Database::new(db_path);
        db.connect().unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_create_and_get_user() {
        let (mut db, _temp_dir) = create_test_db();

        let user_id = db
            .create_user("Test User".to_string(), "test@example.com".to_string(), 25)
            .unwrap();

        let user = db.get_user(&user_id).unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_duplicate_email() {
        let (mut db, _temp_dir) = create_test_db();

        db.create_user("User 1".to_string(), "same@example.com".to_string(), 25)
            .unwrap();

        let result = db.create_user("User 2".to_string(), "same@example.com".to_string(), 30);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_user() {
        let (mut db, _temp_dir) = create_test_db();

        let user_id = db
            .create_user(
                "Original Name".to_string(),
                "original@example.com".to_string(),
                25,
            )
            .unwrap();

        db.update_user(&user_id, Some("Updated Name".to_string()), None, Some(26))
            .unwrap();

        let user = db.get_user(&user_id).unwrap();
        assert_eq!(user.name, "Updated Name");
        assert_eq!(user.age, 26);
        assert_eq!(user.email, "original@example.com"); // ไม่เปลี่ยน
    }

    #[test]
    fn test_delete_user() {
        let (mut db, _temp_dir) = create_test_db();

        let user_id = db
            .create_user(
                "To Delete".to_string(),
                "delete@example.com".to_string(),
                25,
            )
            .unwrap();

        assert!(db.get_user(&user_id).is_some());

        db.delete_user(&user_id).unwrap();

        assert!(db.get_user(&user_id).is_none());
    }

    #[test]
    fn test_search_users() {
        let (mut db, _temp_dir) = create_test_db();

        db.create_user(
            "Alice Smith".to_string(),
            "alice@example.com".to_string(),
            25,
        )
        .unwrap();
        db.create_user("Bob Johnson".to_string(), "bob@example.com".to_string(), 30)
            .unwrap();
        db.create_user(
            "Charlie Smith".to_string(),
            "charlie@example.com".to_string(),
            35,
        )
        .unwrap();

        let results = db.search_users("Smith");
        assert_eq!(results.len(), 2);

        let results = db.search_users("alice");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }

    #[test]
    fn test_statistics() {
        let (mut db, _temp_dir) = create_test_db();

        db.create_user("User 1".to_string(), "user1@example.com".to_string(), 20)
            .unwrap();
        db.create_user("User 2".to_string(), "user2@example.com".to_string(), 30)
            .unwrap();
        db.create_user("User 3".to_string(), "user3@example.com".to_string(), 40)
            .unwrap();

        let stats = db.get_statistics();
        assert_eq!(stats.total_users, 3);
        assert_eq!(stats.average_age, 30.0);
        assert_eq!(stats.oldest_user_age, Some(40));
        assert_eq!(stats.youngest_user_age, Some(20));
    }

    #[test]
    fn test_transaction_rollback() {
        let (mut db, _temp_dir) = create_test_db();

        let initial_count = db.list_users().len();

        {
            let mut transaction = Transaction::new(&mut db);
            transaction
                .database()
                .create_user("Temp User".to_string(), "temp@example.com".to_string(), 25)
                .unwrap();

            // Rollback instead of commit
            transaction.rollback();
        }

        // ตรวจสอบว่าจำนวนผู้ใช้ไม่เปลี่ยน
        assert_eq!(db.list_users().len(), initial_count);
    }

    #[test]
    fn test_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("persistence_test.db.json");

        let user_id = {
            let mut db = Database::new(&db_path);
            db.connect().unwrap();

            db.create_user(
                "Persistent User".to_string(),
                "persistent@example.com".to_string(),
                25,
            )
            .unwrap()
        };

        // สร้าง database instance ใหม่และโหลดข้อมูล
        {
            let mut db = Database::new(&db_path);
            db.connect().unwrap();

            let user = db.get_user(&user_id).unwrap();
            assert_eq!(user.name, "Persistent User");
            assert_eq!(user.email, "persistent@example.com");
        }
    }
}
