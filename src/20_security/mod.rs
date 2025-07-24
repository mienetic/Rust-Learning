//! 🔒 Security & Cryptography Workshop - ความปลอดภัยและการเข้ารหัส! 🛡️
//!
//! ยินดีต้อนรับสู่ Workshop การเรียนรู้ความปลอดภัยและการเข้ารหัส! 🎉
//! เหมือนการสร้างกุญแจและตู้เซฟดิจิทัล! 🔐
//!
//! 🎯 สิ่งที่จะได้เรียนรู้:
//! - 🔐 Encryption - การเข้ารหัสข้อมูล
//! - #️⃣ Hashing - การสร้างลายนิ้วมือดิจิทัล
//! - 🔑 Authentication - การยืนยันตัวตน
//! - 🛡️ Secure Coding Practices
//!
//! หมายเหตุ: นี่คือการจำลองเพื่อการศึกษา! 📚

pub mod encryption;
pub mod hashing;
pub mod authentication;

pub use encryption::*;
pub use hashing::*;
pub use authentication::*;

/// 🚀 รันตัวอย่าง Security & Cryptography Workshop!
/// เหมือนการเปิดห้องเรียนความปลอดภัยดิจิทัล! 🏫🔒
pub fn run_security_examples() {
    println!("🎉 ยินดีต้อนรับสู่ Security & Cryptography Workshop! 🔒");
    println!("เหมือนการเรียนรู้วิธีปกป้องข้อมูลแบบมืออาชีพ! 🛡️\n");

    // Encryption
    println!("🔐 การเข้ารหัสข้อมูล - เหมือนการใส่ข้อมูลในตู้เซฟ!");
    demonstrate_encryption();
    
    println!();
    
    // Hashing
    println!("#️⃣ การสร้างลายนิ้วมือดิจิทัล - เหมือนการทำบัตรประชาชนข้อมูล!");
    demonstrate_hashing();
    
    println!();
    
    // Authentication
    println!("🔑 การยืนยันตัวตน - เหมือนการเช็คบัตรก่อนเข้าอาคาร!");
    demonstrate_authentication();
    
    println!("\n🎉 ยินดีด้วย! คุณได้เรียนรู้ Security & Cryptography เรียบร้อยแล้ว!");
    println!("💡 ตอนนี้คุณรู้วิธีปกป้องข้อมูลแบบมืออาชีพแล้ว! 🛡️");
}