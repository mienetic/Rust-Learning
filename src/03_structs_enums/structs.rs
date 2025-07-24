//! Structs - การใช้งาน Structs ใน Rust
//!
//! ไฟล์นี้สอนเรื่องการสร้างและใช้งาน Structs ในรูปแบบต่างๆ
//! รวมถึง Named Structs, Tuple Structs, Unit Structs และ Methods
//! (โรงงานผลิตข้อมูลแห่งอนาคต! 🏭)

/// ฟังก์ชันสำหรับสอนเรื่อง Structs
/// มาเรียนรู้การสร้างและใช้งาน Structs กันเถอะ! (สถาปนิกแห่งข้อมูล! 👷‍♂️)
pub fn learn_structs() {
    println!("🏗️ === Structs ใน Rust: โรงงานผลิตข้อมูล! === 🏗️");

    // Named Structs (บัตรประชาชนดิจิทัล! 🆔)
    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        username: String,    // ชื่อเล่นในโลกดิจิทัล! 🎮
        email: String,       // ที่อยู่ในไซเบอร์สเปซ! 📮
        sign_in_count: u64,  // จำนวนครั้งที่มาเยือน! 📊
        active: bool,        // สถานะออนไลน์! 🟢
    }

    println!("👤 === Named Structs: สำนักงานทะเบียนราษฎร์ดิจิทัล! === 👤");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("✅ ผู้ใช้: {user1:?} (ลงทะเบียนสำเร็จ! 🎉)");
    println!("📧 อีเมล: {} (ที่อยู่ในโลกดิจิทัล! 🌐)", user1.email);
    println!("👤 ชื่อผู้ใช้: {} (ชื่อเล่นสุดเท่! 😎)", user1.username);

    // Mutable struct (ศูนย์แก้ไขข้อมูลส่วนตัว! ✏️)
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com"); // เปลี่ยนที่อยู่ใหม่! 📬
    println!("✏️ อีเมลใหม่: {} (ย้านบ้านแล้ว! 🏠)", user2.email);

    // Struct update syntax (เครื่องถ่ายเอกสารอัจฉริยะ! 📄)
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // ใช้ค่าที่เหลือจาก user1 (คัดลอกส่วนที่เหลือ! 📋)
    };

    println!("🔄 ผู้ใช้ใหม่จาก update syntax: {user3:?} (โคลนสำเร็จ! 🧬)");

    // Tuple Structs (กล่องลึกลับไร้ป้ายชื่อ! 📦)
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Color(i32, i32, i32);  // พาเลทสีนักศิลปิน! 🎨
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point(i32, i32, i32);  // พิกัดในอวกาศ! 🚀

    println!("\n🎨 === Tuple Structs: กล่องลึกลับไร้ป้าย! === 🎨");
    let black = Color(0, 0, 0);   // สีดำสนิท! ⚫
    let origin = Point(0, 0, 0);  // จุดศูนย์กลางจักรวาล! 🌌

    println!("⚫ สีดำ: {black:?} (มืดมิดไร้แสง! 🌑)");
    println!("📍 จุดเริ่มต้น: {origin:?} (ศูนย์กลางจักรวาล! 🎯)");
    println!("🔴 ค่าสีแดง: {} (ช่องแรกของสี! 🎪)", black.0);

    // Unit Structs (กล่องว่างเปล่าแต่มีความหมาย! 📭)
    #[derive(Debug)]
    struct AlwaysEqual;  // สัญลักษณ์แห่งความเท่าเทียม! ⚖️

    println!("\n⚪ === Unit Structs: กล่องว่างที่ไม่ว่าง! === ⚪");
    let subject = AlwaysEqual;  // ตัวแทนแห่งความเท่าเทียม! 🤝
    println!("✅ Unit struct: {subject:?} (ว่างแต่มีค่า! 💎)");

    // Methods (ห้องแล็บของนักคณิตศาสตร์! 🧮)
    #[derive(Debug)]
    struct Rectangle {
        width: u32,   // ความกว้างของอาณาจักร! 📏
        height: u32,  // ความสูงของปราสาท! 🏰
    }

    impl Rectangle {
        // Method (เครื่องคิดเลขพกพา! 🔢)
        const fn area(&self) -> u32 {
            self.width * self.height  // คำนวณอาณาเขต! 🗺️
        }

        const fn can_hold(&self, other: &Self) -> bool {
            self.width > other.width && self.height > other.height  // เช็คว่าใส่ได้มั้ย! 📦
        }

        // Associated function (โรงงานผลิตสี่เหลี่ยม! 🏭)
        const fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,  // สี่เหลี่ยมจัตุรัสสมบูรณ์แบบ! ⬜
            }
        }
    }

    println!("\n📐 === Methods และ Associated Functions: ห้องแล็บคณิตศาสตร์! === 📐");
    let rect1 = Rectangle {
        width: 30,   // ปราสาทใหญ่! 🏰
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,   // กระท่อมเล็ก! 🏠
        height: 40,
    };

    let square = Rectangle::square(25);  // สั่งทำสี่เหลี่ยมจัตุรัส! 📐

    println!("📊 สี่เหลี่ยม 1: {rect1:?} (ปราสาทใหญ่! 🏰)");
    println!("📏 พื้นที่: {} ตารางหน่วย (อาณาเขตกว้างใหญ่! 🗺️)", rect1.area());
    println!(
        "🤔 rect1 สามารถใส่ rect2 ได้หรือไม่? {} (เทสต์เกมเตตริส! 🎮)",
        rect1.can_hold(&rect2)
    );
    println!("⬜ สี่เหลี่ยมจัตุรัส: {square:?} (สมมาตรสมบูรณ์! ✨)");
}
