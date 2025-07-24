//! 🏗️ Blockchain Module - เวิร์กช็อปสร้างระบบบล็อกเชนแบบมืออาชีพ! ⛓️💎
//! โมดูลสำหรับเรียนรู้ Blockchain และ Cryptocurrency - เหมือนการสร้างธนาคารดิจิทัลในเวิร์กช็อป! 🏦
//!
//! 🎯 สิ่งที่จะได้เรียนรู้ในเวิร์กช็อปนี้:
//! - 🔗 Blockchain basics - พื้นฐานการสร้างโซ่บล็อคแบบมืออาชีพ
//! - 💰 Cryptocurrency - การออกแบบเงินดิจิทัลสำหรับเวิร์กช็อป
//! - ⛏️ Mining และ consensus - ระบบขุดและการตกลงร่วมแบบเวิร์กช็อป
//! - 📜 Smart contracts - การเขียนสัญญาอัจฉริยะสำหรับเว็บแอป
//! - 🌐 Decentralization - การกระจายอำนาจในระบบเว็บแอปพลิเคชัน

pub mod blockchain;

pub use blockchain::*;

/// 🎪 เรียกใช้ตัวอย่าง Blockchain และ Cryptocurrency - เวิร์กช็อปสร้างระบบบล็อกเชน! 🏗️
pub fn run_blockchain_examples() {
    println!("\n⛓️ === เวิร์กช็อป Web Development: Blockchain === 🏗️");
    println!("🎯 สร้างระบบ Blockchain และ Cryptocurrency สำหรับเว็บแอป - เหมือนการสร้างธนาคารดิจิทัลในเวิร์กช็อป!\n");
    
    blockchain::demonstrate_blockchain();
    
    println!("\n🎉 === เวิร์กช็อป Blockchain เสร็จสิ้น! === 🏆");
    println!("🎯 คุณได้เรียนรู้การสร้างระบบบล็อกเชนสำหรับเว็บแอปเหล่านี้:");
    println!("   🔗 Blockchain basics (การสร้างโซ่บล็อคแบบมืออาชีพ!)");
    println!("   💰 Cryptocurrency (การออกแบบเงินดิจิทัลสำหรับเวิร์กช็อป!)");
    println!("   ⛏️ Mining และ consensus (ระบบขุดและการตกลงร่วมแบบเวิร์กช็อป!)");
    println!("   📜 Smart contracts (การเขียนสัญญาอัจฉริยะสำหรับเว็บแอป!)");
    println!("   🌐 Decentralization (การกระจายอำนาจในระบบเว็บแอปพลิเคชัน!)");
}