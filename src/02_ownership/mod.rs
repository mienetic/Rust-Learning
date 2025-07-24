//! Ownership และ Borrowing Module
//!
//! โมดูลนี้สอนเกี่ยวกับระบบ Ownership และ Borrowing ใน Rust
//! ซึ่งเป็นหัวใจสำคัญของภาษา Rust

mod borrowing;
mod ownership_basics;
mod practice_ownership;

pub use ownership_basics::learn_ownership;
pub use borrowing::learn_borrowing;
pub use practice_ownership::practice_ownership_and_borrowing;

/// ฟังก์ชันสำหรับรันตัวอย่าง ownership (เรียกจาก main.rs)
pub fn run_ownership_examples() {
    println!("   🏠 Ownership Basics");
    learn_ownership();

    println!("\n   🤝 Borrowing และ References");
    learn_borrowing();

    println!("\n   ✂️ Slices");
    // เพิ่มการเรียกใช้ slice examples ถ้ามี
    println!("      - String slices และ array slices");

    println!("\n   🎯 Practice Examples");
    practice_ownership_and_borrowing();
}
