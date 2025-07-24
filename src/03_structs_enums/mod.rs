//! Structs and Enums Module - การใช้งาน Structs และ Enums ใน Rust
//!
//! โมดูลนี้รวบรวมการเรียนรู้เกี่ยวกับ Structs และ Enums ใน Rust
//! แบ่งออกเป็นหมวดหมู่ตามประเภทของโครงสร้างข้อมูล
//! (ศูนย์รวมความรู้แห่งการจัดระเบียบข้อมูล! 🏗️📋)

// Module declarations (ประกาศโมดูลสุดเท่! 📦)
mod enums;                    // โลกของ Enums! 🌍
mod practice_structs_enums;   // สนามฝึกซ้อม! 🏟️
mod structs;                  // อาณาจักร Structs! 🏰

// Re-exports (ส่งออกความรู้! 📤)
pub use enums::*;                    // ส่งออก Enums! 📋
pub use practice_structs_enums::*;   // ส่งออกแบบฝึกหัด! 🎯
pub use structs::*;                  // ส่งออก Structs! 🏗️

/// ฟังก์ชันสำหรับรันตัวอย่าง structs และ enums (เรียกจาก main.rs)
/// ศูนย์รวมความสนุกแห่งการเรียนรู้! 🎪
pub fn run_structs_enums_examples() {
    println!("   🏗️ Structs: อาณาจักรแห่งการจัดระเบียบ!");
    learn_structs();  // เรียนรู้ Structs! 📚

    println!("\n   🔧 Methods และ Associated Functions: เครื่องมือเวทมนตร์!");
    // เรียกใช้ฟังก์ชันที่เกี่ยวข้องกับ methods ถ้ามี
    println!("      - impl blocks และ method syntax (มนต์คาถาแห่งการเขียนโค้ด! ✨)");

    println!("\n   📋 Enums: โลกแห่งตัวเลือก!");
    learn_enums();  // เรียนรู้ Enums! 🌟

    println!("\n   🎯 Pattern Matching กับ match: ศิลปะแห่งการจับคู่!");
    // เรียกใช้ฟังก์ชันที่เกี่ยวข้องกับ pattern matching ถ้ามี
    println!("      - match expressions และ if let (นักสืบแห่งโค้ด! 🕵️‍♂️)");

    println!("\n   🎮 Practice Examples: สนามเด็กเล่นโปรแกรมเมอร์!");
    practice_structs_and_enums();  // ฝึกฝนกันเถอะ! 💪
}
