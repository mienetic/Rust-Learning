//! Rust Concepts Learning Library
//!
//! โปรเจคนี้แสดงแนวคิดต่างๆ ของ Rust programming
//! จัดระเบียบเป็นโมดูลเพื่อการเรียนรู้อย่างเป็นระบบ
//!
//! # ตัวอย่างการใช้งาน
//!
//! ```rust
//! use rust_concepts::*;
//!
//! // เรียนรู้พื้นฐาน
//! basics::run_basics_examples();
//!
//! // เรียนรู้ functions
//! functions::run_functions_examples();
//! ```

// การประกาศโมดูล (เรียงตามลำดับการเรียนรู้)
#[path = "01_basics/mod.rs"]
pub mod basics; // พื้นฐาน Rust

#[path = "02_ownership/mod.rs"]
pub mod ownership; // ระบบ ownership

#[path = "03_structs_enums/mod.rs"]
pub mod structs_enums; // struct และ enum

#[path = "04_functions/mod.rs"]
pub mod functions; // ฟังก์ชัน

#[path = "05_modules/mod.rs"]
pub mod modules; // ระบบโมดูล

#[path = "06_collections/mod.rs"]
pub mod collections; // collection types

#[path = "07_error_handling/mod.rs"]
pub mod error_handling; // การจัดการ error

#[path = "08_generics/mod.rs"]
pub mod generics; // generic programming

#[path = "09_traits/mod.rs"]
pub mod traits; // traits และ shared behavior

#[path = "10_lifetimes/mod.rs"]
pub mod lifetimes; // lifetimes

#[path = "11_async_await/mod.rs"]
pub mod async_await; // async/await programming

#[path = "12_macros/mod.rs"]
pub mod macros; // macros

#[path = "13_testing/mod.rs"]
pub mod testing; // testing

#[path = "14_unsafe_rust/mod.rs"]
pub mod unsafe_rust; // unsafe rust

#[path = "15_advanced_patterns/mod.rs"]
pub mod advanced_patterns; // advanced patterns

#[path = "16_concurrency/mod.rs"]
pub mod concurrency; // concurrency and parallelism

#[path = "17_web_development/mod.rs"]
pub mod web_development; // web development with Rust

// โมดูลที่มีอยู่จริงและใช้งานได้ - จัดเรียงตามลำดับใหม่! 🎯✨
#[path = "18_networking/mod.rs"]
pub mod networking; // networking programming - เครือข่ายแบบเทพ! 🌐

#[path = "19_performance/mod.rs"]
pub mod performance; // performance optimization - เร็วแรงเหมือนสายฟ้า! ⚡

#[path = "20_security/mod.rs"]
pub mod security; // security programming - ป้องกันแบบ Fort Knox! 🔐

#[path = "21_advanced_topics/mod.rs"]
pub mod advanced_topics; // advanced topics - เทคนิคขั้นเทพ! 🧙‍♂️

#[path = "22_machine_learning/mod.rs"]
pub mod machine_learning; // machine learning and AI

#[path = "23_blockchain/mod.rs"]
pub mod blockchain; // blockchain and cryptocurrency

#[path = "24_database/mod.rs"]
pub mod database; // database programming - ฐานข้อมูลแบบเทพ! 🗄️

#[path = "25_devops/mod.rs"]
pub mod devops; // devops and deployment - DevOps แบบโปร! 🚀

#[path = "26_game_development/mod.rs"]
pub mod game_development; // game development - เกมส์แบบมันส์! 🎮

#[path = "27_mobile_development/mod.rs"]
pub mod mobile_development; // mobile development - มือถือแบบเจ๋ง! 📱

// Re-exports เพื่อความสะดวก
pub use async_await::*;
pub use basics::*;
pub use collections::*;
pub use error_handling::*;
pub use functions::*;
pub use generics::*;
pub use lifetimes::*;
pub use macros::*;
pub use modules::*;
pub use ownership::*;
pub use structs_enums::*;
pub use testing::*;
pub use traits::*;
pub use unsafe_rust::*;

// Re-export types for examples
pub use serde::{Deserialize, Serialize};

/// ฟังก์ชันสำหรับรันตัวอย่างทั้งหมด (sync version)
fn run_all_examples_sync() {
    println!("🦀 ยินดีต้อนรับสู่ Rust Concepts Learning Project! 🦀");
    println!("{}", "=".repeat(50));

    println!("🔥 === บทที่ 1: พื้นฐาน Rust === 🔥");
    basics::run_basics_examples();

    println!("\n\n🔒 === บทที่ 2: Ownership และ Borrowing === 🔒");
    ownership::run_ownership_examples();

    println!("\n\n📊 === บทที่ 3: Structs และ Enums === 📊");
    structs_enums::run_structs_enums_examples();

    println!("\n\n🚀 === บทที่ 4: Functions และ Control Flow === 🚀");
    functions::run_functions_examples();

    println!("\n\n📦 === บทที่ 5: Modules === 📦");
    modules::run_modules_examples();

    println!("\n\n📚 === บทที่ 6: Collections === 📚");
    collections::run_collections_examples();

    println!("\n\n⚠️ === บทที่ 7: Error Handling === ⚠️");
    error_handling::run_error_handling_examples();

    println!("\n\n🔧 === บทที่ 8: Generics === 🔧");
    generics::run_generics_examples();

    println!("\n\n🎯 === บทที่ 9: Traits === 🎯");
    traits::run_traits_examples();

    println!("\n\n⏰ === บทที่ 10: Lifetimes === ⏰");
    lifetimes::run_lifetimes_examples();

    println!("\n\n🎭 === บทที่ 12: Macros === 🎭");
    macros::run_macros_examples();

    println!("\n\n🧪 === บทที่ 13: Testing === 🧪");
    testing::run_testing_examples();

    println!("\n\n⚡ === บทที่ 14: Unsafe Rust === ⚡");
    unsafe_rust::run_unsafe_examples();

    println!("\n\n🎯 === บทที่ 15: Advanced Patterns === 🎯");
    advanced_patterns::run_advanced_patterns_examples();

    println!("\n\n🔀 === บทที่ 16: Concurrency === 🔀");
    concurrency::run_concurrency_examples();

    println!("\n\n🌐 === บทที่ 17: Web Development === 🌐");
    web_development::run_web_development_examples();

    println!("\n\n🌐 === บทที่ 18: Networking === 🌐");
    networking::run_networking_examples();

    println!("\n\n⚡ === บทที่ 19: Performance === ⚡");
    performance::run_performance_examples();

    println!("\n\n🔒 === บทที่ 20: Security === 🔒");
    security::run_security_examples();

    println!("\n\n🧙‍♂️ === บทที่ 21: Advanced Topics === 🧙‍♂️");
    advanced_topics::run_advanced_topics_examples();

    println!("\n\n🤖 === บทที่ 22: Machine Learning === 🤖");
    machine_learning::run_machine_learning_examples();

    println!("\n\n⛓️ === บทที่ 23: Blockchain === ⛓️");
    blockchain::run_blockchain_examples();

    println!("\n\n🗄️ === บทที่ 24: Database === 🗄️");
    database::run_database_examples();

    println!("\n\n🚀 === บทที่ 25: DevOps === 🚀");
    devops::run_devops_examples();

    println!("\n\n🎮 === บทที่ 26: Game Development === 🎮");
    game_development::run_game_development_examples();

    println!("\n\n📱 === บทที่ 27: Mobile Development === 📱");
    mobile_development::run_mobile_development_examples();

    println!("\n🎊 สำเร็จ! คุณได้เรียนรู้แนวคิดสำคัญของ Rust ครบถ้วนแล้ว! 🎊");
    println!("🚀 ตอนนี้คุณพร้อมที่จะสร้างแอปพลิเคชัน Rust ของตัวเองแล้ว!");
    println!("💡 คุณได้เรียนรู้ทั้งหมด 27 บท ครอบคลุมตั้งแต่พื้นฐานจนถึงหัวข้อขั้นสูง!");
    println!("⚠️ หมายเหตุ: บทที่ 11 (Async Programming) ต้องใช้ tokio runtime");
}
pub use anyhow::{Context, Result};
pub use chrono::{DateTime, Utc};
pub use std::path::PathBuf;
pub use uuid::Uuid;

/// Task struct for CLI example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub priority: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Task {
    #[must_use]
    pub fn new(title: String, priority: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            priority,
            completed: false,
            created_at: Utc::now(),
        }
    }
}

/// `TaskManager` for CLI example
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

impl TaskManager {
    #[must_use]
    pub const fn new(file_path: PathBuf) -> Self {
        Self {
            tasks: Vec::new(),
            file_path,
        }
    }

    pub fn add_task(&mut self, title: String, priority: String) -> Uuid {
        let task = Task::new(title, priority);
        let task_id = task.id;
        self.tasks.push(task);
        task_id
    }

    #[must_use]
    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn complete_task(&mut self, task_id: &Uuid) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == *task_id)
            .ok_or_else(|| anyhow::anyhow!("Task not found"))?;
        task.completed = true;
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn remove_task(&mut self, task_id: &Uuid) -> Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == *task_id)
            .ok_or_else(|| anyhow::anyhow!("Task not found"))?;
        self.tasks.remove(index);
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn save_to_file(&self) -> Result<()> {
        let json =
            serde_json::to_string_pretty(&self.tasks).context("Failed to serialize tasks")?;
        std::fs::write(&self.file_path, json).context("Failed to write to file")?;
        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn load_from_file(&mut self) -> Result<()> {
        if !self.file_path.exists() {
            return Ok(());
        }
        let content = std::fs::read_to_string(&self.file_path).context("Failed to read file")?;
        self.tasks = serde_json::from_str(&content).context("Failed to parse JSON")?;
        Ok(())
    }
}

/// ฟังก์ชันสำหรับรันตัวอย่างทั้งหมด (async version)
pub async fn run_all_examples_async() {
    run_all_examples_internal().await;
}

/// ฟังก์ชันสำหรับรันตัวอย่างทั้งหมด (sync version)
pub fn run_all_examples() {
    // สำหรับ sync version เราจะข้าม async examples
    run_all_examples_sync();
}

/// ฟังก์ชันภายในสำหรับรันตัวอย่างทั้งหมด (async)
async fn run_all_examples_internal() {
    println!("🦀 ยินดีต้อนรับสู่ Rust Concepts Learning Project! 🦀");
    println!("{}", "=".repeat(50));

    println!("🔥 === บทที่ 1: พื้นฐาน Rust === 🔥");
    basics::run_basics_examples();

    println!("\n\n🔒 === บทที่ 2: Ownership และ Borrowing === 🔒");
    ownership::run_ownership_examples();

    println!("\n\n📊 === บทที่ 3: Structs และ Enums === 📊");
    structs_enums::run_structs_enums_examples();

    println!("\n\n🚀 === บทที่ 4: Functions และ Control Flow === 🚀");
    functions::run_functions_examples();

    println!("\n\n📦 === บทที่ 5: Modules === 📦");
    modules::run_modules_examples();

    println!("\n\n📚 === บทที่ 6: Collections === 📚");
    collections::run_collections_examples();

    println!("\n\n⚠️ === บทที่ 7: Error Handling === ⚠️");
    error_handling::run_error_handling_examples();

    println!("\n\n🔧 === บทที่ 8: Generics === 🔧");
    generics::run_generics_examples();

    println!("\n\n🎯 === บทที่ 9: Traits === 🎯");
    traits::run_traits_examples();

    println!("\n\n⏰ === บทที่ 10: Lifetimes === ⏰");
    lifetimes::run_lifetimes_examples();

    println!("\n\n🔄 === บทที่ 11: Async/Await === 🔄");
    async_await::run_async_examples().await;

    println!("\n\n🎭 === บทที่ 12: Macros === 🎭");
    macros::run_macros_examples();

    println!("\n\n🧪 === บทที่ 13: Testing === 🧪");
    testing::run_testing_examples();

    println!("\n\n⚡ === บทที่ 14: Unsafe Rust === ⚡");
    unsafe_rust::run_unsafe_examples();

    println!("\n\n🎯 === บทที่ 15: Advanced Patterns === 🎯");
    advanced_patterns::run_advanced_patterns_examples();

    println!("\n\n🔀 === บทที่ 16: Concurrency === 🔀");
    concurrency::run_concurrency_examples();

    println!("\n\n🌐 === บทที่ 17: Web Development === 🌐");
    web_development::run_web_development_examples();

    println!("\n\n🌐 === บทที่ 18: Networking === 🌐");
    networking::run_networking_examples();

    println!("\n\n⚡ === บทที่ 19: Performance === ⚡");
    performance::run_performance_examples();

    println!("\n\n🔒 === บทที่ 20: Security === 🔒");
    security::run_security_examples();

    println!("\n\n🧙‍♂️ === บทที่ 21: Advanced Topics === 🧙‍♂️");
    advanced_topics::run_advanced_topics_examples();

    println!("\n\n🤖 === บทที่ 22: Machine Learning === 🤖");
    machine_learning::run_machine_learning_examples();

    println!("\n\n⛓️ === บทที่ 23: Blockchain === ⛓️");
    blockchain::run_blockchain_examples();

    println!("\n\n🗄️ === บทที่ 24: Database === 🗄️");
    database::run_database_examples();

    println!("\n\n🚀 === บทที่ 25: DevOps === 🚀");
    devops::run_devops_examples();

    println!("\n\n🎮 === บทที่ 26: Game Development === 🎮");
    game_development::run_game_development_examples();

    println!("\n\n📱 === บทที่ 27: Mobile Development === 📱");
    mobile_development::run_mobile_development_examples();

    println!("\n🎊 สำเร็จ! คุณได้เรียนรู้แนวคิดสำคัญของ Rust ครบถ้วนแล้ว! 🎊");
    println!("🚀 ตอนนี้คุณพร้อมที่จะสร้างแอปพลิเคชัน Rust ของตัวเองแล้ว!");
    println!("💡 คุณได้เรียนรู้ทั้งหมด 27 บท ครอบคลุมตั้งแต่พื้นฐานจนถึงหัวข้อขั้นสูง!");
}

// Tests are now in individual modules