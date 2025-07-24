//! # Repository Testing Examples - คลังข้อมูลนักสืบ! 🗃️🕵️‍♂️
//!
//! ตัวอย่างการทดสอบ repository pattern และการจัดการข้อมูล
//! เหมือนการจัดการคลังหลักฐานของนักสืบโค้ด! 📚🔍
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การทดสอบระบบจัดการข้อมูลอย่างมืออาชีพ!

use super::user_testing::User;
use std::collections::HashMap;

/// คลังข้อมูลผู้ใช้นักสืบ - เก็บหลักฐานผู้ต้องสงสัย! 🗃️🔍
pub struct UserRepository {
    users: HashMap<u32, User>,  // 📚 คลังหลักฐานผู้ใช้
    next_id: u32,               // 🔢 หมายเลขประจำตัวถัดไป
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository {
    /// สร้างคลังข้อมูลใหม่ - เริ่มต้นการสืบสวน! 🆕🗃️
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),  // 📚 คลังหลักฐานว่าง
            next_id: 1,             // 🔢 เริ่มต้นที่หมายเลข 1
        }
    }

    /// สร้างผู้ใช้ใหม่ - เพิ่มผู้ต้องสงสัยในคลัง! 👤➕
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หากข้อมูลผู้ใช้ไม่ถูกต้อง (ดูรายละเอียดใน `User::new`)
    pub fn create_user(&mut self, name: String, email: String, age: u32) -> Result<u32, String> {
        let user = User::new(self.next_id, name, email, age)?;  // 👤 สร้างผู้ใช้ใหม่
        let id = user.id;                                       // 🔢 เก็บ ID
        self.users.insert(id, user);                           // 📚 เก็บในคลัง
        self.next_id += 1;                                     // 🔢 เพิ่มหมายเลขถัดไป
        Ok(id)  // ✅ ส่งคืน ID ที่สร้าง
    }

    /// ค้นหาผู้ใช้ - สืบสวนหลักฐาน! 🔍👤
    #[must_use]
    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)  // 🔍 ค้นหาในคลังหลักฐาน
    }

    /// ค้นหาผู้ใช้แบบแก้ไขได้ - สืบสวนและแก้ไขหลักฐาน! 🔍✏️
    pub fn get_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.get_mut(&id)  // 🔍 ค้นหาและอนุญาตให้แก้ไข
    }

    /// อัปเดตผู้ใช้ - แก้ไขหลักฐาน! ✏️👤
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หากไม่พบผู้ใช้ที่มี ID ดังกล่าว
    pub fn update_user(&mut self, id: u32, user: User) -> Result<(), String> {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.users.entry(id) {
            e.insert(user);  // ✏️ แก้ไขหลักฐาน
            Ok(())  // ✅ อัปเดตสำเร็จ
        } else {
            Err("User not found".to_string())  // ❌ ไม่พบหลักฐาน
        }
    }

    /// ลบผู้ใช้ - ทำลายหลักฐาน! 🗑️👤
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หากไม่พบผู้ใช้ที่มี ID ดังกล่าว
    pub fn delete_user(&mut self, id: u32) -> Result<(), String> {
        if self.users.remove(&id).is_some() {
            Ok(())  // ✅ ลบสำเร็จ
        } else {
            Err("User not found".to_string())  // ❌ ไม่พบหลักฐานที่จะลบ
        }
    }

    /// ดูผู้ใช้ทั้งหมด - ตรวจสอบคลังหลักฐาน! 👥📋
    #[must_use]
    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()  // 📋 รวบรวมหลักฐานทั้งหมด
    }

    /// นับจำนวนผู้ใช้ - นับหลักฐาน! 🔢📊
    #[must_use]
    pub fn count(&self) -> usize {
        self.users.len()  // 🔢 นับจำนวนหลักฐาน
    }

    /// ค้นหาผู้ใช้ตามช่วงอายุ - สืบสวนตามเกณฑ์อายุ! 🔍📅
    #[must_use]
    pub fn find_users_by_age_range(&self, min_age: u32, max_age: u32) -> Vec<&User> {
        self.users
            .values()
            .filter(|user| user.is_valid_age_range(min_age, max_age))  // 🔍 กรองตามอายุ
            .collect()  // 📋 รวบรวมผลลัพธ์
    }

    /// ค้นหาผู้ใช้ตามชื่อ - สืบสวนตามชื่อ! 🔍📝
    #[must_use]
    pub fn find_users_by_name_contains(&self, search_term: &str) -> Vec<&User> {
        self.users
            .values()
            .filter(|user| {
                user.name
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())  // 🔍 ค้นหาในชื่อ
            })
            .collect()  // 📋 รวบรวมผลลัพธ์
    }

    /// ค้นหาผู้ใหญ่ - สืบสวนผู้ที่บรรลุนิติภาวะ! 🔍🔞
    #[must_use]
    pub fn get_adult_users(&self) -> Vec<&User> {
        self.users.values().filter(|user| user.is_adult()).collect()  // 🔞 กรองผู้ใหญ่
    }

    /// ล้างคลังข้อมูล - เริ่มการสืบสวนใหม่! 🧹🔄
    pub fn clear(&mut self) {
        self.users.clear();  // 🧹 ล้างหลักฐานทั้งหมด
        self.next_id = 1;    // 🔢 รีเซ็ตหมายเลข
    }
}

/// ตัวอย่างการใช้งาน Repository testing
pub fn repository_testing_examples() {
    println!("📚 === Repository Testing Examples ===");

    // สร้าง UserRepository
    let mut repo = UserRepository::new();
    println!("🆕 สร้าง Repository ใหม่: จำนวนผู้ใช้ = {}", repo.count());

    // เพิ่มผู้ใช้
    match repo.create_user("สมหญิง".to_string(), "somying@example.com".to_string(), 30)
    {
        Ok(id) => {
            println!("✅ สร้าง User ID: {id}");
            if let Some(user) = repo.get_user(id) {
                println!("📖 ข้อมูล User: {user:?}");
            }
        }
        Err(e) => println!("❌ ไม่สามารถสร้าง User: {e}"),
    }

    // เพิ่มผู้ใช้หลายคน
    let _ = repo.create_user("สมชาย".to_string(), "somchai@example.com".to_string(), 25);
    let _ = repo.create_user("สมศรี".to_string(), "somsri@example.com".to_string(), 35);
    let _ = repo.create_user("สมพงษ์".to_string(), "sompong@example.com".to_string(), 16);

    println!("👥 จำนวนผู้ใช้ทั้งหมด: {}", repo.count());

    // ค้นหาผู้ใช้ตามช่วงอายุ
    let adults_20_30 = repo.find_users_by_age_range(20, 30);
    println!("🔍 ผู้ใช้อายุ 20-30 ปี: {} คน", adults_20_30.len());
    for user in adults_20_30 {
        println!("   - {} (อายุ {})", user.name, user.age);
    }

    // ค้นหาผู้ใช้ตามชื่อ
    let som_users = repo.find_users_by_name_contains("สม");
    println!("🔍 ผู้ใช้ที่มีชื่อมี 'สม': {} คน", som_users.len());

    // ค้นหาผู้ใหญ่
    let adults = repo.get_adult_users();
    println!("🔞 ผู้ใหญ่: {} คน", adults.len());

    // ทดสอบการอัปเดต
    if let Some(user_id) = repo.get_all_users().first().map(|u| u.id) {
        if let Ok(updated_user) = User::new(
            user_id,
            "ชื่อใหม่".to_string(),
            "newemail@example.com".to_string(),
            28,
        ) {
            match repo.update_user(user_id, updated_user) {
                Ok(()) => println!("✅ อัปเดต User ID {user_id} สำเร็จ"),
                Err(e) => println!("❌ ไม่สามารถอัปเดต User: {e}"),
            }
        }
    }

    // ทดสอบการลบ
    if let Some(user_id) = repo.get_all_users().last().map(|u| u.id) {
        match repo.delete_user(user_id) {
            Ok(()) => println!("🗑️ ลบ User ID {user_id} สำเร็จ"),
            Err(e) => println!("❌ ไม่สามารถลบ User: {e}"),
        }
        println!("👥 จำนวนผู้ใช้หลังลบ: {}", repo.count());
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // ทดสอบ UserRepository
    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_new() {
        let repo = UserRepository::new();
        assert_eq!(repo.count(), 0);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_create_user() {
        let mut repo = UserRepository::new();

        let id = repo.create_user("John".to_string(), "john@example.com".to_string(), 25);
        assert!(id.is_ok());

        let id = id.unwrap();
        assert_eq!(id, 1);
        assert_eq!(repo.count(), 1);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_create_invalid_user() {
        let mut repo = UserRepository::new();

        let result = repo.create_user(String::new(), "john@example.com".to_string(), 25);
        assert!(result.is_err());
        assert_eq!(repo.count(), 0);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_get_user() {
        let mut repo = UserRepository::new();
        let id = repo
            .create_user("John".to_string(), "john@example.com".to_string(), 25)
            .unwrap();

        let user = repo.get_user(id);
        assert!(user.is_some());

        let user = user.unwrap();
        assert_eq!(user.name, "John");
        assert_eq!(user.email, "john@example.com");

        // ทดสอบ user ที่ไม่มี
        assert!(repo.get_user(999).is_none());
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_update_user() {
        let mut repo = UserRepository::new();
        let id = repo
            .create_user("John".to_string(), "john@example.com".to_string(), 25)
            .unwrap();

        let updated_user = User::new(
            id,
            "John Updated".to_string(),
            "john.updated@example.com".to_string(),
            26,
        )
        .unwrap();

        assert!(repo.update_user(id, updated_user).is_ok());

        let user = repo.get_user(id).unwrap();
        assert_eq!(user.name, "John Updated");
        assert_eq!(user.email, "john.updated@example.com");
        assert_eq!(user.age, 26);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_update_nonexistent_user() {
        let mut repo = UserRepository::new();
        let user = User::new(
            999,
            "Ghost".to_string(),
            "ghost@example.com".to_string(),
            25,
        )
        .unwrap();

        let result = repo.update_user(999, user);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_delete_user() {
        let mut repo = UserRepository::new();
        let id = repo
            .create_user("John".to_string(), "john@example.com".to_string(), 25)
            .unwrap();

        assert_eq!(repo.count(), 1);
        assert!(repo.delete_user(id).is_ok());
        assert_eq!(repo.count(), 0);
        assert!(repo.get_user(id).is_none());
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_delete_nonexistent_user() {
        let mut repo = UserRepository::new();

        let result = repo.delete_user(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User not found");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_get_all_users() {
        let mut repo = UserRepository::new();

        repo.create_user("John".to_string(), "john@example.com".to_string(), 25)
            .unwrap();
        repo.create_user("Jane".to_string(), "jane@example.com".to_string(), 30)
            .unwrap();

        let users = repo.get_all_users();
        assert_eq!(users.len(), 2);

        let names: Vec<&String> = users.iter().map(|u| &u.name).collect();
        assert!(names.contains(&&"John".to_string()));
        assert!(names.contains(&&"Jane".to_string()));
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_find_by_age_range() {
        let mut repo = UserRepository::new();

        repo.create_user("Young".to_string(), "young@example.com".to_string(), 20)
            .unwrap();
        repo.create_user("Middle".to_string(), "middle@example.com".to_string(), 30)
            .unwrap();
        repo.create_user("Old".to_string(), "old@example.com".to_string(), 50)
            .unwrap();

        let young_adults = repo.find_users_by_age_range(18, 35);
        assert_eq!(young_adults.len(), 2);

        let seniors = repo.find_users_by_age_range(45, 60);
        assert_eq!(seniors.len(), 1);
        assert_eq!(seniors[0].name, "Old");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_find_by_name() {
        let mut repo = UserRepository::new();

        repo.create_user("John Smith".to_string(), "john@example.com".to_string(), 25)
            .unwrap();
        repo.create_user("Jane Doe".to_string(), "jane@example.com".to_string(), 30)
            .unwrap();
        repo.create_user(
            "Johnny Cash".to_string(),
            "johnny@example.com".to_string(),
            35,
        )
        .unwrap();

        let john_users = repo.find_users_by_name_contains("john");
        assert_eq!(john_users.len(), 2);

        let smith_users = repo.find_users_by_name_contains("Smith");
        assert_eq!(smith_users.len(), 1);
        assert_eq!(smith_users[0].name, "John Smith");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_get_adults() {
        let mut repo = UserRepository::new();

        repo.create_user("Adult1".to_string(), "adult1@example.com".to_string(), 25)
            .unwrap();
        repo.create_user("Minor".to_string(), "minor@example.com".to_string(), 16)
            .unwrap();
        repo.create_user("Adult2".to_string(), "adult2@example.com".to_string(), 30)
            .unwrap();

        let adults = repo.get_adult_users();
        assert_eq!(adults.len(), 2);

        for adult in adults {
            assert!(adult.is_adult());
        }
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_repository_clear() {
        let mut repo = UserRepository::new();

        repo.create_user("User1".to_string(), "user1@example.com".to_string(), 25)
            .unwrap();
        repo.create_user("User2".to_string(), "user2@example.com".to_string(), 30)
            .unwrap();

        assert_eq!(repo.count(), 2);

        repo.clear();
        assert_eq!(repo.count(), 0);

        // ทดสอบว่า next_id รีเซ็ตแล้ว
        let new_id = repo
            .create_user("New User".to_string(), "new@example.com".to_string(), 25)
            .unwrap();
        assert_eq!(new_id, 1);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_repository_testing_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        repository_testing_examples();
    }
}