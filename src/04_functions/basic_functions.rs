//! Basic Functions - ฟังก์ชันพื้นฐานใน Rust
//!
//! ไฟล์นี้สอนเรื่องการสร้างและใช้งานฟังก์ชันพื้นฐานใน Rust
//! รวมถึง parameters, return values, statements และ expressions
//! (โรงเรียนสอนทำฟังก์ชัน: จากมือใหม่สู่เซียน! 🎓✨)

/// ฟังก์ชันสำหรับเรียนรู้ฟังก์ชันพื้นฐาน
/// ยินดีต้อนรับสู่โรงเรียนสอนทำฟังก์ชัน! 🏫
pub fn learn_basic_functions() {
    println!("🎯 === ฟังก์ชันใน Rust: ศิลปะแห่งการแบ่งงาน! === 🎯");

    // ฟังก์ชันพื้นฐาน (ฟังก์ชันมือใหม่! 👶)
    fn greet() {
        println!("👋 สวัสดีจากฟังก์ชัน! (ฉันเป็นฟังก์ชันตัวแรก! 🎉)");
    }

    // ฟังก์ชันที่มี parameter (ฟังก์ชันที่รับของฝาก! 📦)
    fn greet_person(name: &str) {  // รับชื่อมาทักทาย! 👋
        println!("👋 สวัสดี {name}! (ยินดีที่ได้รู้จัก! 😊)");
    }

    // ฟังก์ชันที่ return ค่า (ฟังก์ชันนักคิดเลข! 🧮)
    const fn add(a: i32, b: i32) -> i32 {  // เครื่องบวกจิ๋ว! ➕
        a + b // expression ไม่ต้องใส่ semicolon (กฎทอง! ✨)
    }

    // ฟังก์ชันที่มี statement และ expression (ฟังก์ชันสถาปนิก! 🏗️)
    fn calculate_area(width: f64, height: f64) -> f64 {  // วัดพื้นที่เก่ง! 📐
        // statement (คำสั่งเตรียมการ! 📝)
        width * height // expression (คำตอบสุดท้าย! 🎯)
    }

    // เรียกใช้ฟังก์ชัน (เวลาโชว์ความสามารถ! 🎪)
    greet();  // ทักทายแบบธรรมดา! 👋
    greet_person("น้องรัสต์");  // ทักทายแบบส่วนตัว! 💖

    let sum = add(5, 3);  // ให้เครื่องบวกทำงาน! 🔢
    println!("🔢 5 + 3 = {sum} (เก่งมาก! 🌟)");

    let room_area = calculate_area(4.5, 3.2);  // วัดห้องนอน! 🏠
    println!("🏠 พื้นที่ห้อง: {room_area:.2} ตารางเมตร (กว้างขวางดี! 📏)");

    // Statements vs Expressions (ศึกแห่งความแตกต่าง! ⚔️)
    println!("\n📝 === Statements vs Expressions: ศิลปะแห่งการเขียนโค้ด! === 📝");
    let x = 5; // statement (คำสั่งธรรมดา! 📝)
    let y = {  // block expression (บล็อกเวทมนตร์! ✨)
        let x = 3;  // statement ใน scope ใหม่! 🔄
        x + 1 // expression (ไม่มี semicolon = คืนค่า! 🎯)
    }; // y = 4 (ผลลัพธ์สุดเจ๋ง! 🎉)
    println!("🔢 x = {x}, y = {y} (เห็นความแตกต่างมั้ย? 🤔)");
}
