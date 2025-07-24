//! Functions และ Control Flow - ห้องแล็บฟังก์ชันมหัศจรรย์! 🧪✨
//!
//! โมดูลนี้เป็นโรงเรียนสอนเวทมนตร์ฟังก์ชัน! 🎓 เรียนรู้การสร้างฟังก์ชัน, closures มหัศจรรย์,
//! recursion แบบวนซ้ำ และ control flow ที่ควบคุมการทำงานได้อย่างเทพ! 🚀

mod advanced_functions;
mod basic_functions;
mod closures;
mod control_flow;
mod practice_functions;
mod recursion;

pub use advanced_functions::*;
pub use basic_functions::*;
pub use closures::*;
pub use control_flow::*;
pub use recursion::*;
// pub use practice_functions::*; // ใช้เฉพาะฟังก์ชันที่ต้องการ
pub use practice_functions::practice_functions_and_control_flow;

// ฟังก์ชันสำหรับสอนเรื่องฟังก์ชันพื้นฐาน (ถูกรวมเข้าใน run_functions_examples แล้ว)
// มาเรียนรู้การสร้างและใช้งานฟังก์ชันกันเถอะ! เป็นนักเวทย์ฟังก์ชัน! 🧙‍♂️✨
// pub fn learn_functions() {
//     learn_basic_functions();     // เรียนพื้นฐาน! 📚
//     learn_advanced_functions();  // ขั้นสูง! 🎯
//     learn_closures();           // เวทมนตร์ closures! 🪄
//     learn_recursion();          // การเรียกตัวเอง! 🔄
// }

/// ฟังก์ชันสำหรับรันตัวอย่างฟังก์ชัน (เรียกจาก main.rs) - โรงเรียนเวทมนตร์! 🏫✨
pub fn run_functions_examples() {
    println!("   🔧 Basic Functions (ฟังก์ชันพื้นฐาน: เรียนรู้เวทมนตร์เบื้องต้น!)");
    learn_basic_functions();

    println!("\n   🎛️ Control Flow (การควบคุมการทำงาน: เป็นผู้กำกับการแสดง!)");
    learn_control_flow();

    println!("\n   ⚙️ Advanced Functions (ฟังก์ชันขั้นสูง: เวทมนตร์ระดับมาสเตอร์!)");
    learn_advanced_functions();

    println!("\n   🎯 Closures (เวทมนตร์ปิดผนึก: จับตัวแปรมาใช้งาน!)");
    learn_closures();

    println!("\n   🔄 Recursion (การเรียกตัวเอง: เวทมนตร์วนซ้ำ!)");
    learn_recursion();

    println!("\n   💪 แบบฝึกหัด Functions และ Control Flow (ยิมฝึกเวทมนตร์!)");
    practice_functions_and_control_flow();
}
