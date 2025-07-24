//! Machine Learning and AI Module
//!
//! โมดูลสำหรับการเรียนรู้ของเครื่อง (Machine Learning) และปัญญาประดิษฐ์ (AI)

pub mod machine_learning;

pub use machine_learning::*;

/// เรียกใช้ตัวอย่าง Machine Learning และ AI
pub fn run_machine_learning_examples() {
    println!("🤖 Running Machine Learning and AI Examples");
    println!("{}", "=".repeat(60));
    
    machine_learning::demonstrate_machine_learning();
    
    println!("{}", "\n".repeat(2));
    println!("🎯 Machine Learning examples completed!");
}