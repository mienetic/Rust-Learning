//! User Testing Examples - ห้องทดลองนักสืบผู้ใช้! 🕵️‍♂️👤
//!
//! ตัวอย่างการทดสอบ struct ที่มี validation และ error handling
//! สืบสวนและตรวจสอบข้อมูลผู้ใช้อย่างละเอียด! 🔍📋

/// ตัวอย่าง struct สำหรับจัดการผู้ใช้ - บัตรประจำตัวนักสืบ! 🆔🕵️‍♂️
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u32,        // 🆔 หมายเลขประจำตัวนักสืบ
    pub name: String,    // 📛 ชื่อนักสืบ
    pub email: String,   // 📧 ช่องทางติดต่อ
    pub age: u32,        // 📅 อายุของนักสืบ
}

impl User {
    /// สร้างนักสืบใหม่ - ออกบัตรประจำตัว! 🆔✨
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หาก:
    /// - ชื่อว่างเปล่า (นักสืบต้องมีชื่อ!)
    /// - รูปแบบอีเมลไม่ถูกต้อง (ไม่มี '@')
    /// - อายุมากกว่า 150 ปี (นักสืบแก่เกินไป!)
    pub fn new(id: u32, name: String, email: String, age: u32) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());  // ❌ นักสืบต้องมีชื่อ
        }

        if !email.contains('@') {
            return Err("Invalid email format".to_string());  // ❌ อีเมลไม่ถูกต้อง
        }

        if age > 150 {
            return Err("Age is too high".to_string());  // ❌ อายุสูงเกินไป
        }

        Ok(Self {
            id,     // ✅ หมายเลขประจำตัว
            name,   // ✅ ชื่อนักสืบ
            email,  // ✅ ช่องทางติดต่อ
            age,    // ✅ อายุ
        })
    }

    /// ตรวจสอบว่าเป็นนักสืบผู้ใหญ่หรือไม่ - ตรวจสอบอายุ! 🔞🕵️‍♂️
    #[must_use]
    pub const fn is_adult(&self) -> bool {
        self.age >= 18  // 🔞 ตรวจสอบอายุ 18 ปีขึ้นไป
    }

    /// อัปเดตอีเมลนักสืบ - เปลี่ยนช่องทางติดต่อ! 📧✏️
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หากรูปแบบอีเมลไม่ถูกต้อง (ไม่มี '@')
    pub fn update_email(&mut self, new_email: String) -> Result<(), String> {
        if !new_email.contains('@') {
            return Err("Invalid email format".to_string());  // ❌ อีเมลไม่ถูกต้อง
        }
        self.email = new_email;  // ✏️ อัปเดตช่องทางติดต่อ
        Ok(())  // ✅ อัปเดตสำเร็จ
    }

    /// แสดงชื่อและข้อมูลนักสืบ - ป้ายชื่อ! 📛🆔
    #[must_use]
    pub fn get_display_name(&self) -> String {
        format!("{} ({})", self.name, self.email)  // 📛 ชื่อและอีเมล
    }

    /// ตรวจสอบช่วงอายุ - ตรวจสอบเกณฑ์อายุ! 📅🔍
    #[must_use]
    pub const fn is_valid_age_range(&self, min_age: u32, max_age: u32) -> bool {
        self.age >= min_age && self.age <= max_age  // 📅 ตรวจสอบช่วงอายุ
    }
}

/// ตัวอย่างการใช้งาน User testing - การทดสอบนักสืบ! 🕵️‍♂️🧪
pub fn user_testing_examples() {
    println!("🕵️‍♂️ === User Detective Lab Testing === 🧪👤");

    // ทดสอบการสร้าง User สำเร็จ
    match User::new(
        1,
        "สมชาย".to_string(),
        "somchai@example.com".to_string(),
        25,
    ) {
        Ok(user) => {
            println!("✅ User สร้างสำเร็จ: {user:?}");
            println!("🔞 เป็นผู้ใหญ่: {}", user.is_adult());
            println!("📛 ชื่อแสดง: {}", user.get_display_name());
            println!("📊 อายุในช่วง 20-30: {}", user.is_valid_age_range(20, 30));
        }
        Err(e) => println!("❌ ไม่สามารถสร้าง User: {e}"),
    }

    // ทดสอบการสร้าง User ล้มเหลว - ชื่อว่าง
    match User::new(2, String::new(), "test@example.com".to_string(), 25) {
        Ok(user) => println!("✅ User สร้างสำเร็จ: {user:?}"),
        Err(e) => println!("❌ ไม่สามารถสร้าง User (ชื่อว่าง): {e}"),
    }

    // ทดสอบการสร้าง User ล้มเหลว - อีเมลไม่ถูกต้อง
    match User::new(3, "สมหญิง".to_string(), "invalid-email".to_string(), 30) {
        Ok(user) => println!("✅ User สร้างสำเร็จ: {user:?}"),
        Err(e) => println!("❌ ไม่สามารถสร้าง User (อีเมลไม่ถูกต้อง): {e}"),
    }

    // ทดสอบการสร้าง User ล้มเหลว - อายุสูงเกินไป
    match User::new(4, "คนแก่".to_string(), "old@example.com".to_string(), 200) {
        Ok(user) => println!("✅ User สร้างสำเร็จ: {user:?}"),
        Err(e) => println!("❌ ไม่สามารถสร้าง User (อายุสูงเกินไป): {e}"),
    }

    // ทดสอบการอัปเดตอีเมล
    if let Ok(mut user) = User::new(5, "ทดสอบ".to_string(), "test@example.com".to_string(), 25)
    {
        println!("📧 อีเมลเดิม: {}", user.email);

        match user.update_email("newemail@example.com".to_string()) {
            Ok(()) => println!("✅ อัปเดตอีเมลสำเร็จ: {}", user.email),
            Err(e) => println!("❌ ไม่สามารถอัปเดตอีเมล: {e}"),
        }

        match user.update_email("invalid-email".to_string()) {
            Ok(()) => println!("✅ อัปเดตอีเมลสำเร็จ: {}", user.email),
            Err(e) => println!("❌ ไม่สามารถอัปเดตอีเมล (รูปแบบไม่ถูกต้อง): {e}"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // ทดสอบ User
    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_creation_success() {
        let user = User::new(1, "John".to_string(), "john@example.com".to_string(), 25);
        assert!(user.is_ok());

        let user = user.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.age, 25);
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_creation_empty_name() {
        let user = User::new(1, String::new(), "test@example.com".to_string(), 25);
        assert!(user.is_err());
        assert_eq!(user.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_creation_invalid_email() {
        let user = User::new(1, "John".to_string(), "invalid-email".to_string(), 25);
        assert!(user.is_err());
        assert_eq!(user.unwrap_err(), "Invalid email format");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_creation_invalid_age() {
        let user = User::new(1, "John".to_string(), "john@example.com".to_string(), 200);
        assert!(user.is_err());
        assert_eq!(user.unwrap_err(), "Age is too high");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_is_adult() {
        let adult = User::new(1, "Adult".to_string(), "adult@example.com".to_string(), 25).unwrap();
        let minor = User::new(2, "Minor".to_string(), "minor@example.com".to_string(), 16).unwrap();
        let exactly_18 = User::new(
            3,
            "Eighteen".to_string(),
            "eighteen@example.com".to_string(),
            18,
        )
        .unwrap();

        assert!(adult.is_adult());
        assert!(!minor.is_adult());
        assert!(exactly_18.is_adult());
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_update_email() {
        let mut user =
            User::new(1, "John".to_string(), "john@example.com".to_string(), 25).unwrap();

        // อัปเดตอีเมลสำเร็จ
        assert!(
            user.update_email("newemail@example.com".to_string())
                .is_ok()
        );
        assert_eq!(user.email, "newemail@example.com");

        // อัปเดตอีเมลล้มเหลว
        assert!(user.update_email("invalid-email".to_string()).is_err());
        assert_eq!(user.email, "newemail@example.com"); // ค่าไม่เปลี่ยน
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_get_display_name() {
        let user = User::new(
            1,
            "John Doe".to_string(),
            "john@example.com".to_string(),
            25,
        )
        .unwrap();
        assert_eq!(user.get_display_name(), "John Doe (john@example.com)");
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_is_valid_age_range() {
        let user = User::new(1, "John".to_string(), "john@example.com".to_string(), 25).unwrap();

        assert!(user.is_valid_age_range(20, 30));
        assert!(user.is_valid_age_range(25, 25));
        assert!(!user.is_valid_age_range(30, 40));
        assert!(!user.is_valid_age_range(10, 20));
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_edge_cases() {
        // ทดสอบอายุ 0
        let baby = User::new(1, "Baby".to_string(), "baby@example.com".to_string(), 0);
        assert!(baby.is_ok());
        assert!(!baby.unwrap().is_adult());

        // ทดสอบอายุ 150 (ขีดจำกัด)
        let old = User::new(2, "Old".to_string(), "old@example.com".to_string(), 150);
        assert!(old.is_ok());
        assert!(old.unwrap().is_adult());

        // ทดสอบอีเมลที่มี @ หลายตัว
        let multi_at = User::new(
            3,
            "Multi".to_string(),
            "test@sub@example.com".to_string(),
            25,
        );
        assert!(multi_at.is_ok());
    }

    #[test]
    #[allow(clippy::missing_panics_doc)]
    pub fn test_user_testing_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        user_testing_examples();
    }
}