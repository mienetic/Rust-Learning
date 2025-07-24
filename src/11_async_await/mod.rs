//! # Async/Await Programming - โรงงานผลิตความเร็วแสง! 🚀⚡
//!
//! ยินดีต้อนรับสู่โลกแห่งการทำงานแบบ "ไม่ต้องรอ"! 🎭✨
//! ที่นี่เราจะเรียนรู้ว่าทำไม Rust ถึงเร็วกว่าแสง (เกือบๆ)! 💫
//!
//! 🎪 **สิ่งมหัศจรรย์ที่จะได้เรียนรู้:**
//! - 🤹‍♂️ **Async/Await**: เหมือนมีหลายมือทำงานพร้อมกัน! (ซุปเปอร์ฮีโร่!)
//! - 🔮 **Future Trait**: เวทมนตร์แห่งอนาคตที่ยังไม่เกิดขึ้น! (นาฬิกาเวลา!)
//! - 🏎️ **Tokio Runtime**: เครื่องยนต์ที่เร็วกว่า Formula 1! (รถแข่งแห่งโค้ด!)
//! - 🎬 **Concurrent Tasks**: ดิเรกเตอร์หนังแอ็คชั่นระดับฮอลลีวูด! (ผู้กำกับมืออาชีพ!)
//! - 🚒 **Error Handling**: ทีมกู้ภัยที่ไม่เคยหลับ! (ฮีโร่กู้ภัย!)
//! - 🧙‍♂️ **Advanced Patterns**: เทคนิคขั้นเทพที่แม้แต่ Gandalf ยังต้องอิจฉา! (เวทมนตร์ขั้นเทพ!)
//!
//! 🎯 **เป้าหมาย**: ทำให้คุณเป็นนินจา async ที่เก่งที่สุดในจักรวาล! 🥷✨

// โมดูลย่อย - ห้องแล็บแต่ละสาขา! 🧪🔬
pub mod basic_async;        // พื้นฐาน async - เริ่มต้นที่นี่! 🌱 (โรงเรียนอนุบาล!)
pub mod channels;           // ช่องทางสื่อสาร - โทรศัพท์แห่งอนาคต! 📞 (ไปรษณีย์ดิจิทัล!)
pub mod custom_futures;     // Future ที่สร้างเอง - DIY แห่งอนาคต! 🔧 (ช่างฝีมือ!)
pub mod error_handling;     // จัดการ error - ทีมกู้ภัย! 🚑 (หน่วยกู้ภัยมืออาชีพ!)
pub mod advanced_async;     // เทคนิคขั้นสูง - โรงเรียนนินจา! 🥷 (มหาวิทยาลัยเทพ!)

// Re-export สำหรับการใช้งานง่าย - ประตูเดียวเข้าได้ทุกที่! 🚪✨
pub use basic_async::*;
pub use channels::*;
pub use custom_futures::*;
pub use error_handling::*;
pub use advanced_async::*;  // เพิ่ม advanced_async! 🧙‍♂️

// ตัวอย่าง basic async/await ถูกย้ายไปที่ basic_async.rs แล้ว

// ตัวอย่าง concurrent tasks และ spawn ถูกย้ายไปที่ basic_async.rs แล้ว
// ตัวอย่าง error handling ถูกย้ายไปที่ error_handling.rs แล้ว

// ตัวอย่าง custom Future ถูกย้ายไปที่ custom_futures.rs แล้ว
// ตัวอย่าง channels ถูกย้ายไปที่ channels.rs แล้ว

/// ฟังก์ชันหลักสำหรับรันตัวอย่างทั้งหมด - เปิดโรงละครใหญ่! 🎭🎪
pub async fn run_async_examples() {
    println!("\n🦀✨ === บทที่ 11: Async/Await Programming === ✨🦀");
    println!("📚🚀 เรียนรู้การเขียนโปรแกรมแบบ asynchronous ใน Rust (เตรียมตัวให้พร้อม!)\n");

    println!("🎬 === เริ่มการแสดง! === 🎬");
    basic_async::basic_async_example().await;                    // 🌱 พื้นฐาน
    basic_async::concurrent_tasks_example().await;              // 🦸‍♂️ ซุปเปอร์ฮีโร่
    basic_async::spawn_tasks_example().await;                   // 👻 ผีช่วยงาน
    error_handling::async_error_handling().await;               // 🚑 ทีมกู้ภัย
    error_handling::timeout_example().await;                    // ⏰ นาฬิกาจับเวลา
    custom_futures::custom_future_example().await;              // 🔧 ช่างฝีมือ
    channels::channels_example().await;                         // 📞 ไปรษณีย์
    advanced_async::demonstrate_advanced_async().await;         // 🧙‍♂️ เวทมนตร์ขั้นเทพ!

    println!("\n🎉🏆 === จบบทที่ 11: Async/Await Programming! === 🏆🎉");
    println!("💡🌟 คุณได้เรียนรู้ (และกลายเป็นนินจา async แล้ว!):");
    println!("   🤹‍♂️ การใช้ async/await syntax (เวทมนตร์พื้นฐาน!)");
    println!("   🦸‍♂️ การรัน tasks แบบ concurrent (ซุปเปอร์ฮีโร่ทีมงาน!)");
    println!("   👻 การใช้ tokio::spawn (ผีช่วยงานเบื้องหลัง!)");
    println!("   🚑 Error handling ใน async context (ทีมกู้ภัยมืออาชีพ!)");
    println!("   🔧 การสร้าง custom Future (ช่างฝีมือระดับเทพ!)");
    println!("   📞 การใช้ async channels (ไปรษณีย์แห่งอนาคต!)");
    println!("   🧙‍♂️ เทคนิคขั้นสูง (เวทมนตร์ขั้นเทพ!)");
    println!("\n🥷✨ ยินดีด้วย! คุณเป็นนินจา async แล้ว! ✨🥷");
}

// Tests are now in individual sub-modules
