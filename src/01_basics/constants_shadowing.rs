//! Constants and Shadowing Module - การเรียนรู้เกี่ยวกับ Constants และ Shadowing ใน Rust (เมื่อตัวแปรเล่นซ่อนหา! 🙈)

/// ฟังก์ชันสำหรับเรียนรู้ Constants และ Shadowing (เรื่องราวของค่าคงที่และการปลอมตัว! 🎭)
pub fn learn_constants_and_shadowing() {
    // Constants - ประกาศที่ต้นฟังก์ชัน (เหมือนกฎหมายที่เปลี่ยนไม่ได้! ⚖️)
    const MAX_POINTS: u32 = 100_000;
    // ใช้ค่าคงที่ PI จาก standard library เพื่อความแม่นยำ (ไม่ใช่ 3.14 แบบคร่าวๆ! 🥧)
    const PI: f64 = std::f64::consts::PI;
    const APP_NAME: &str = "Rust Learning App";
    
    println!("\n🔒 === Constants และ Shadowing: เมื่อตัวแปรเล่นเกมซ่อนหา! === 🔒");

    println!("📌 Constants (ค่าคงที่ที่ไม่เปลี่ยนใจ! 💎):");
    println!("   MAX_POINTS: {MAX_POINTS} (คะแนนสูงสุดที่ไม่มีใครทำลายได้! 🏆)");
    println!("   PI: {PI} (ค่าคงที่ที่มีชื่อเสียงที่สุดในโลก! 🌍)");
    println!("   APP_NAME: {APP_NAME} (ชื่อแอปที่ไม่เปลี่ยนแปลง! 📱)");

    // Shadowing (การปลอมตัวของตัวแปร! 🎭)
    println!("\n👥 === Variable Shadowing: เมื่อตัวแปรเล่นเกมปลอมตัว! === 👥");

    let x = 5;
    println!("🔢 x เริ่มต้น: {x} (ตัวตนแรกของ x! 👶)");

    let x = x + 1; // Shadowing (x ปลอมตัวเป็นคนใหม่! 🎭)
    println!("🔢 x หลัง shadow (x + 1): {x} (x เวอร์ชัน 2.0! 🆕)");

    let x = x * 2; // Shadowing อีกครั้ง (x ปลอมตัวอีกแล้ว! 🔄)
    println!("🔢 x หลัง shadow (x * 2): {x} (x เวอร์ชัน 3.0 ทวีคูณ! ✨)");

    // Shadowing กับการเปลี่ยน type (เหมือนการแปลงร่างของ Transformer! 🤖)
    let spaces = "   ";
    println!(
        "\n📝 spaces เป็น string: '{}' (ความยาว: {}) (ช่องว่างที่มองไม่เห็น! 👻)",
        spaces,
        spaces.len()
    );

    let spaces = spaces.len(); // Shadow ด้วย type ใหม่ (แปลงร่างจาก string เป็น number! 🔄)
    println!("🔢 spaces เป็น number: {spaces} (จากช่องว่างกลายเป็นตัวเลข! ✨)");

    // Scope และ Shadowing (เหมือนการเข้าออกห้องที่มีคนชื่อเดียวกัน! 🚪)
    let y = 10;
    println!("\n🌍 y ใน outer scope: {y} (y ตัวจริงในโลกภายนอก! 🌎)");

    {
        let y = 20; // Shadow ใน inner scope (y ปลอมในห้องลับ! 🏠)
        println!("🏠 y ใน inner scope: {y} (y เวอร์ชันห้องส่วนตัว! 🔒)");

        let y = "hello"; // Shadow ด้วย type ใหม่ (y แปลงร่างเป็นคำทักทาย! 👋)
        println!("🏠 y ใน inner scope (string): {y} (y พูดได้แล้ว! 🗣️)");
    }

    println!("🌍 y กลับมาที่ outer scope: {y} (y ตัวจริงกลับมาแล้ว! 🔄)");

    println!("\n🎉 จบการเรียนรู้ Constants และ Shadowing! (ตอนนี้คุณเป็นนักสืบตัวแปรแล้ว! 🕵️‍♂️)");
}
