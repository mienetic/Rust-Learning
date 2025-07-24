//! 🗄️ บทที่ 24: Database และ ORM
//! 
//! บทนี้จะสอนการทำงานกับฐานข้อมูลใน Rust
//! ครอบคลุมการใช้งาน SQL และ NoSQL databases
//! 
//! ## 📚 เนื้อหาในบทนี้:
//! - การเชื่อมต่อฐานข้อมูล
//! - การใช้งาน Diesel ORM
//! - การทำงานกับ SQLx
//! - MongoDB และ Redis
//! - Connection pooling
//! - Database migrations
//! - การจัดการ transactions

pub mod sql_databases;
pub mod nosql_databases;
pub mod orm_examples;
pub mod connection_pooling;

/// 🎯 ฟังก์ชันหลักสำหรับการเรียนรู้ Database
pub fn learn_database() {
    println!("\n{}", "=".repeat(50));
    println!("🗄️ Database และ ORM");
    println!("{}", "=".repeat(50));
    
    println!("📖 ยินดีต้อนรับสู่บทเรียน Database!");
    println!("🎯 ในบทนี้เราจะเรียนรู้:");
    println!("   • การเชื่อมต่อฐานข้อมูล");
    println!("   • การใช้งาน ORM และ Query Builder");
    println!("   • การทำงานกับ SQL และ NoSQL");
    println!("   • การจัดการ Connection Pool");
    println!("   • Database Migrations");
    println!();
    
    // เรียกใช้งานตัวอย่างต่างๆ
    sql_databases::demonstrate_sql_databases();
    nosql_databases::demonstrate_nosql_databases();
    orm_examples::demonstrate_orm_examples();
    connection_pooling::demonstrate_connection_pooling();
    
    println!("✅ จบบทเรียน Database และ ORM!");
    println!("🎉 ตอนนี้คุณสามารถทำงานกับฐานข้อมูลใน Rust ได้แล้ว!");
}

/// 🧪 ฟังก์ชันสำหรับรันตัวอย่าง Database
pub fn run_database_examples() {
    println!("🚀 กำลังรันตัวอย่าง Database...");
    learn_database();
}