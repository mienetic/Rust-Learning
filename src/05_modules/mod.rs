//! Module System - ห้องสมุดโมดูลมหัศจรรย์ที่ Harry Potter ต้องอิจฉา! 📚✨🪄
//!
//! โมดูลใน Rust เป็นเหมือนตู้เก็บของวิเศษที่ Doraemon ต้องขอยืม! 🗃️🤖 ช่วยจัดระเบียบโค้ดให้เป็นระบบ,
//! แบ่งแยกฟังก์ชันการทำงานอย่างชาญฉลาดกว่า Sherlock Holmes และควบคุมการเข้าถึงแบบนักสืบ CIA! 🕵️‍♀️🔍

mod basic_modules;
mod practice_modules;
mod reexporting;
mod use_statements;
mod visibility_privacy;

pub use basic_modules::*;
pub use practice_modules::*;
pub use reexporting::*;
pub use use_statements::*;
pub use visibility_privacy::*;

/// ฟังก์ชันสำหรับรันตัวอย่าง modules (เรียกจาก main.rs) - ทัวร์ห้องสมุดโมดูลแบบ VIP! 📚🎫👑
pub fn run_modules_examples() {
    println!("   📦 Basic Modules (โมดูลพื้นฐาน: เรียนรู้การจัดระเบียบแบบ Marie Kondo!)");
    learn_basic_modules();

    println!("\n   📥 Use Statements (คำสั่ง use: เรียกใช้โมดูลอย่างชาญฉลาดกว่า Einstein!)");
    learn_use_statements();

    println!("\n   🔒 Visibility และ Privacy (การมองเห็นและความเป็นส่วนตัว: ระบบรักษาความปลอดภัยแบบ Fort Knox!)");
    learn_visibility_privacy();

    println!("\n   🔄 Re-exporting (การส่งออกซ้ำ: แชร์โมดูลอย่างเทพแบบ influencer!)");
    learn_reexporting();

    println!("\n   💪 แบบฝึกหัด Modules (ยิมฝึกจัดระเบียบโมดูลที่แกร่งกว่า Fitness First!)");
    practice_modules();
}
