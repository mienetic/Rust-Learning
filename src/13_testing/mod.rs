//! # บทที่ 13: Testing - ห้องทดลองนักสืบโค้ด! 🔍🧪
//!
//! ยินดีต้อนรับสู่ห้องทดลองนักสืบโค้ด! ที่นี่เราจะเรียนรู้การสืบสวนและทดสอบโค้ด
//! เหมือนนักสืบที่ต้องหาหลักฐานและตรวจสอบความจริง! 🕵️‍♂️🔬
//!
//! 🎯 **เป้าหมายการเรียนรู้**:
//! - 🧪 Unit testing ใน Rust - การทดสอบชิ้นส่วนเล็กๆ
//! - 🔗 Integration testing - การทดสอบการทำงานร่วมกัน
//! - 📋 Test organization และ best practices - การจัดระเบียบการทดสอบ
//! - 🎭 Mocking และ test doubles - การจำลองสถานการณ์
//! - 🎲 Property-based testing - การทดสอบแบบสุ่ม
//! - ⚡ Benchmark testing - การวัดประสิทธิภาพ
//!
//! 🔧 **หมายเหตุเทคนิค**:
//! - `use std::collections::HashMap` อาจแสดง warning "unused import" 🚨
//! - เหตุผล: import นี้เหลือจากการ refactor โค้ดไปยังโมดูลย่อย 🔄
//! - `HashMap` ถูกใช้งานในโมดูลย่อยแล้ว แต่ import หลักยังคงอยู่ 📦

// 🧪 โมดูลย่อยสำหรับการทดสอบ - ห้องทดลองต่างๆ!
pub mod basic_testing;        // 🔬 ห้องทดลองพื้นฐาน
pub mod calculator_testing;   // 🧮 ห้องทดลองเครื่องคิดเลข
pub mod performance_testing;  // ⚡ ห้องทดลองประสิทธิภาพ
pub mod repository_testing;   // 📚 ห้องทดลองคลังข้อมูล
pub mod user_testing;         // 👤 ห้องทดลองผู้ใช้

// 📤 Re-export สำหรับการใช้งานง่าย - ส่งออกเครื่องมือนักสืบ!
pub use basic_testing::*;        // 🔬 เครื่องมือพื้นฐาน
pub use calculator_testing::*;   // 🧮 เครื่องมือคำนวณ
pub use performance_testing::*;  // ⚡ เครื่องมือวัดประสิทธิภาพ
pub use repository_testing::*;   // 📚 เครื่องมือจัดการข้อมูล
pub use user_testing::*;         // 👤 เครื่องมือทดสอบผู้ใช้

// 🚨 **คำเตือนเทคนิค**: HashMap import อาจแสดง warning "unused import"
// 🔄 **เหตุผล**: เหลือจากการ refactor โค้ดไปยังโมดูลย่อย
// 📦 **สถานะ**: HashMap ถูกใช้งานในโมดูลย่อยแล้ว (repository_testing.rs, performance_testing.rs)

// 📋 **การจัดระเบียบโค้ด - Code Organization**:
// 🔬 ตัวอย่างฟังก์ชันพื้นฐานและ Calculator ➜ basic_testing.rs และ calculator_testing.rs
// 👤 ตัวอย่าง User struct ➜ user_testing.rs
// 📚 ตัวอย่าง UserRepository ➜ repository_testing.rs
// ⚡ ตัวอย่าง Performance testing ➜ performance_testing.rs

/// ตัวอย่างการใช้งาน testing - เริ่มการสืบสวนโค้ด! 🕵️‍♂️🔍
pub fn run_testing_examples() {
    println!("\n🕵️‍♂️✨ === บทที่ 13: Testing - ห้องทดลองนักสืบโค้ด! === ✨🕵️‍♂️");
    println!("🔬 ยินดีต้อนรับสู่ห้องทดลองนักสืบโค้ด! เตรียมพร้อมสืบสวนและทดสอบ! 🧪");
    println!("🎯 เป้าหมาย: เรียนรู้การเขียน tests ใน Rust แบบมืออาชีพ! 📚\n");

    // 🔬 เรียกใช้ตัวอย่างจากห้องทดลองต่างๆ - เริ่มการสืบสวน!
    println!("🔬 === เข้าสู่ห้องทดลองพื้นฐาน === 🔬");
    basic_testing::basic_testing_examples();
    println!();

    println!("🧮 === เข้าสู่ห้องทดลองเครื่องคิดเลข === 🧮");
    calculator_testing::calculator_testing_examples();
    println!();

    println!("👤 === เข้าสู่ห้องทดลองผู้ใช้ === 👤");
    user_testing::user_testing_examples();
    println!();

    println!("📚 === เข้าสู่ห้องทดลองคลังข้อมูล === 📚");
    repository_testing::repository_testing_examples();
    println!();

    println!("⚡ === เข้าสู่ห้องทดลองประสิทธิภาพ === ⚡");
    performance_testing::performance_testing_examples();

    println!("\n🎉✨ จบบทที่ 13: Testing - การสืบสวนสำเร็จ! ✨🎉");
    println!("🏆 ยินดีด้วย! คุณได้เป็นนักสืบโค้ดมืออาชีพแล้ว! 🕵️‍♂️🎓");
    println!("\n💡🔍 **สิ่งที่คุณได้เรียนรู้**:");
    println!("   🧪 การเขียน unit tests - ทดสอบชิ้นส่วนเล็กๆ");
    println!("   ❌ การทดสอบ error cases - จัดการกับข้อผิดพลาด");
    println!("   📋 การจัดระเบียบ tests - จัดระเบียบการทดสอบ");
    println!("   ✅ การใช้ assertions ต่างๆ - ตรวจสอบความถูกต้อง");
    println!("   🏗️ การทดสอบ structs และ methods - ทดสอบโครงสร้างข้อมูล");
    println!("\n🎯 คุณพร้อมแล้วที่จะสืบสวนและทดสอบโค้ดใดๆ! 🔬✨");
}

// Tests are now in individual sub-modules
