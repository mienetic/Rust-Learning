//! # Macros Module - ห้องแล็บ Macros มหัศจรรย์! 🪄✨
//! โมดูลสำหรับเรียนรู้เรื่อง Macros ใน Rust - เวทมนตร์สร้างโค้ดอัตโนมัติ! 🎩
//!
//! ยินดีต้อนรับสู่โรงเรียนเวทมนตร์แห่ง Rust! ที่นี่คุณจะได้เรียนรู้ศิลปะการสร้างโค้ดด้วยเวทมนตร์! 🧙‍♂️✨
//!
//! ## 🎯 บทนี้จะสอนเกี่ยวกับ:
//! - 🔮 **Declarative macros** (`macro_rules`!) - เวทมนตร์ประกาศแบบ spell casting!
//! - 🧙‍♂️ **Procedural macros** - เวทมนตร์ขั้นสูงแบบ advanced wizardry!
//! - ✨ **การสร้าง custom macros** - สร้างเวทมนตร์ของตัวเองแบบ DIY magic!
//! - 🧹 **Macro hygiene และ scope** - ความสะอาดของเวทมนตร์แบบ clean magic!
//! - 🔍 **การ debug macros** - ตรวจสอบเวทมนตร์ที่ผิดพลาดแบบ magic debugging!
//! - 📝 **Code generation** - เวทมนตร์สร้างโค้ดแบบ automatic code summoning!
//! - 🧪 **Testing macros** - เวทมนตร์ทดสอบแบบ magical testing spells!
//!
//! 🎭 **เป้าหมาย**: ทำให้คุณเป็นพ่อมดแม่มดแห่งการเขียนโค้ดที่เก่งที่สุดในจักรวาล! 🌟

// โมดูลย่อยสำหรับ Macros - ห้องเก็บเวทมนตร์แต่ละประเภทแบบ magic storage! 📚🔮
pub mod code_generation;     // 🏗️ เวทมนตร์สร้างโค้ด (Code Summoning Spells!)
pub mod declarative_macros;  // 🔮 เวทมนตร์ประกาศ (Declaration Magic!)
pub mod logging_macros;      // 📝 เวทมนตร์บันทึก (Logging Spells!)
pub mod macro_hygiene;       // 🧹 เวทมนตร์ความสะอาด (Clean Magic!)
pub mod practice_macros;     // 💪 ยิมฝึกเวทมนตร์ (Magic Training Gym!)
pub mod testing_macros;      // 🧪 เวทมนตร์ทดสอบ (Testing Spells!)

// Re-export ฟังก์ชันสำคัญ - เปิดประตูเวทมนตร์ให้ใช้งานแบบ magic gateway! 🚪✨
pub use code_generation::*;     // 🏗️ นำเข้าเวทมนตร์สร้างโค้ด!
pub use declarative_macros::*;  // 🔮 นำเข้าเวทมนตร์ประกาศ!
pub use logging_macros::*;      // 📝 นำเข้าเวทมนตร์บันทึก!
pub use macro_hygiene::*;       // 🧹 นำเข้าเวทมนตร์ความสะอาด!
pub use practice_macros::*;     // 💪 นำเข้าการฝึกเวทมนตร์!
pub use testing_macros::*;      // 🧪 นำเข้าเวทมนตร์ทดสอบ!

// ตัวอย่าง declarative macros ถูกย้ายไปที่ declarative_macros.rs แล้ว

// ตัวอย่าง struct generation ถูกย้ายไปที่ code_generation.rs แล้ว
// ตัวอย่าง testing macros ถูกย้ายไปที่ testing_macros.rs แล้ว
// ตัวอย่าง configuration macros ถูกย้ายไปที่ code_generation.rs แล้ว
// ตัวอย่าง logging macros ถูกย้ายไปที่ logging_macros.rs แล้ว
// ตัวอย่าง macro hygiene ถูกย้ายไปที่ macro_hygiene.rs แล้ว

/// ฟังก์ชันหลักสำหรับรันตัวอย่างทั้งหมด - มาสเตอร์คลาสเวทมนตร์แบบ magic masterclass! 🎭🪄
pub fn run_macros_examples() {
    println!("\n🪄✨ === บทที่ 12: Macros - โรงเรียนเวทมนตร์แห่ง Rust! === ✨🪄");
    println!("📚🧙‍♂️ เรียนรู้การสร้างและใช้งาน macros ใน Rust - เหมือนเรียนเวทมนตร์ที่ Hogwarts แบบ coding wizardry!\n");
    println!("🎩 === เริ่มการเรียนเวทมนตร์! === 🎩");

    // รันตัวอย่างจากโมดูลย่อยต่างๆ - เปิดตำราเวทมนตร์ทีละเล่มแบบ spellbook collection! 📖🔮
    declarative_macros::declarative_macros_examples();      // 🔮 เวทมนตร์ประกาศ
    declarative_macros::variadic_macros_examples();         // 🌟 เวทมนตร์พารามิเตอร์ไม่จำกัด
    code_generation::struct_generation_macros();            // 🏗️ เวทมนตร์สร้าง struct
    code_generation::configuration_macros();                // ⚙️ เวทมนตร์ configuration
    testing_macros::testing_macros_examples();              // 🧪 เวทมนตร์ทดสอบ
    testing_macros::performance_macros_examples();          // 🏃‍♂️ เวทมนตร์วัดประสิทธิภาพ
    logging_macros::logging_macros();                       // 📝 เวทมนตร์บันทึก
    logging_macros::debugging_macros();                     // 🔍 เวทมนตร์ debug
    logging_macros::conditional_macros();                   // 🎯 เวทมนตร์เงื่อนไข
    macro_hygiene::macro_hygiene_examples();                // 🧹 เวทมนตร์ความสะอาด
    macro_hygiene::advanced_macro_examples();               // 🧙‍♂️ เวทมนตร์ขั้นสูง
    macro_hygiene::recursive_macro_examples();              // 🔄 เวทมนตร์เรียกตัวเอง
    
    println!("\n💪🏋️‍♂️ === แบบฝึกหัด Macros - ยิมฝึกเวทมนตร์! === 🏋️‍♂️💪");
    practice_macros::practice_macros();                     // 💪 ฝึกเวทมนตร์พื้นฐาน
    practice_macros::api_endpoints_example();               // 🌐 เวทมนตร์ API endpoints
    practice_macros::configuration_example();               // ⚙️ เวทมนตร์ configuration

    println!("\n🎉🎓 === จบบทที่ 12: Macros - จบการเรียนเวทมนตร์แล้ว! === 🎓🎉");
    println!("💡🌟 คุณได้เรียนรู้เวทมนตร์เหล่านี้ (และกลายเป็นพ่อมดแม่มดแล้ว!):");
    println!("   🔮 การสร้าง declarative macros ด้วย macro_rules! (เวทมนตร์พื้นฐานแบบ basic spells!)");
    println!("   🎯 การใช้ pattern matching ใน macros (การจับรูปแบบแบบ pattern wizard!)");
    println!("   🌟 การสร้าง variadic macros (เวทมนตร์รับพารามิเตอร์ไม่จำกัดแบบ unlimited args magic!)");
    println!("   🏗️ การใช้ macros สำหรับ code generation (เวทมนตร์สร้างโค้ดแบบ code summoning!)");
    println!("   🧹 Macro hygiene และการจัดการ scope (ความสะอาดเวทมนตร์แบบ clean magic!)");
    println!("   📝 การใช้ macros สำหรับ logging และ testing (เวทมนตร์ตรวจสอบแบบ debugging spells!)");
    println!("\n🧙‍♂️✨ ยินดีด้วย! คุณเป็นพ่อมดแม่มดแห่งการเขียนโค้ดแล้ว! ✨🧙‍♀️");
}

// Tests are now in individual sub-modules - การทดสอบเวทมนตร์แยกตามประเภทแบบ specialized magic testing! 🧪✨
// แต่ละโมดูลมีการทดสอบเวทมนตร์เฉพาะทาง เหมือนห้องปฏิบัติการเวทมนตร์แยกตามสาขา! 🔬🪄
