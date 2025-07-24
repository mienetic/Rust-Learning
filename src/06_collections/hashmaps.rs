//! `HashMap` - พจนานุกรมดิจิทัลสุดเจ๋งกว่า Google Translate! 📚✨🌐
//!
//! ไฟล์นี้สอนเรื่องการสร้าง, เข้าถึง, แก้ไข และใช้งาน `HashMap` methods ต่างๆ
//! รวมถึงการจัดการ key-value pairs และการวนลูป
//! เหมือนมีผู้ช่วยส่วนตัวที่จำทุกอย่างได้แบบ Siri แต่ฉลาดกว่า! 🧠💫🤖

use std::collections::HashMap;

/// ฟังก์ชันสำหรับสอนเรื่อง `HashMap`
/// มาเรียนรู้การใช้งานพจนานุกรมดิจิทัลกันเถอะ! เป็น Wikipedia ส่วนตัว! 🗺️📖🌍
pub fn learn_hashmaps() {
    println!("🗺️ === HashMap: พจนานุกรมดิจิทัลสุดเจ๋งกว่า Oxford Dictionary! === 🗺️");

    // การสร้าง HashMap - สร้างพจนานุกรมส่วนตัวแบบ startup! 📚
    println!("\n🆕 === การสร้าง HashMap: เปิดร้านพจนานุกรมแบบ Amazon! === 🆕");

    let mut scores = HashMap::new();  // เปิดร้านใหม่แบบ grand opening! 🏪

    // เพิ่มข้อมูล - ใส่ของลงในตู้เก็บของแบบ inventory management! 📦
    scores.insert(String::from("Blue"), 10);    // ทีมน้ำเงิน: 10 คะแนนแบบ rookie score! 💙
    scores.insert(String::from("Yellow"), 50);  // ทีมเหลือง: 50 คะแนนแบบ pro level! 💛
    scores.insert(String::from("Red"), 25);     // ทีมแดง: 25 คะแนนแบบ average joe! ❤️

    println!("🏆 คะแนนทีม: {scores:?} (สมุดคะแนนพร้อมแล้วแบบ leaderboard!)");

    // การเข้าถึงข้อมูล - ค้นหาในพจนานุกรมแบบ Google search! 🔍
    println!("\n🔍 === การเข้าถึงข้อมูล: เปิดพจนานุกรมหาคำแบบ Ctrl+F! === 🔍");

    let team_name = String::from("Blue");  // ค้นหาทีมน้ำเงินแบบ detective! 🔎
    match scores.get(&team_name) {
        Some(score) => println!("💙 ทีม {team_name} ได้ {score} คะแนน (เจอแล้วแบบ eureka moment!)"),
        None => println!("❌ ไม่พบทีม {team_name} (หาไม่เจอแบบ 404 error!)"),
    }

    // การวนลูป - อ่านพจนานุกรมทั้งเล่มแบบ full scan! 🔄
    println!("\n🔄 === การวนลูป HashMap: อ่านสมุดคะแนนทั้งหมดแบบ data iteration! === 🔄");

    println!("📊 คะแนนทุกทีม (รายงานผลการแข่งขันแบบ live scoreboard!):");
    for (key, value) in &scores {  // อ่านทีละหน้าแบบ page by page! 📖
        println!("  🏅 {key}: {value} คะแนน (บันทึกแล้วแบบ saved data!)");
    }

    // การอัพเดทข้อมูล - แก้ไขพจนานุกรมแบบ software update! ✏️
    println!("\n✏️ === การอัพเดทข้อมูล: แก้ไขสมุดคะแนนแบบ patch notes! === ✏️");

    // เขียนทับ - แก้ไขคะแนนใหม่แบบ hot fix! 🔄
    scores.insert(String::from("Blue"), 100);  // ทีมน้ำเงินได้คะแนนเพิ่มแบบ level up! 🚀
    println!("🔄 หลังอัพเดท Blue: {scores:?} (ทีมน้ำเงินขึ้นอันดับแบบ ranking boost!)");

    // เพิ่มเฉพาะเมื่อไม่มี key - เพิ่มทีมใหม่แบบ new player! 🆕
    scores.entry(String::from("Green")).or_insert(30);  // ทีมเขียวเข้าร่วมแบบ fresh recruit! 💚
    scores.entry(String::from("Blue")).or_insert(999);  // จะไม่เปลี่ยนแบบ existing user (มีอยู่แล้ว!) 🚫
    println!("➕ หลังเพิ่ม Green: {scores:?} (ทีมใหม่เข้าร่วมแบบ welcome aboard!)");

    // อัปเดตตามค่าเดิม - เพิ่มคะแนนบนฐานเดิมแบบ accumulator! 📈
    let count = scores.entry(String::from("Yellow")).or_insert(0);  // หาทีมเหลืองแบบ search query! 🔍
    *count += 10;  // เพิ่มคะแนน 10 แบบ bonus points! ➕
    println!("📈 หลังเพิ่ม Yellow: {scores:?} (ทีมเหลืองได้โบนัสแบบ reward system!)");

    // HashMap methods ที่มีประโยชน์ - เครื่องมือจัดการพจนานุกรมแบบ Swiss Army knife! 🛠️
    println!("\n🛠️ === HashMap Methods: เครื่องมือจัดการแบบ toolkit! === 🛠️");

    println!("📏 จำนวนทีม: {} ทีม (นับแล้วแบบ count function!)", scores.len());
    println!("❓ ว่างหรือไม่: {} (ตรวจสอบสมุดคะแนนแบบ empty check!)", scores.is_empty());

    // ตรวจสอบว่ามี key หรือไม่ - หาทีมในสมุดคะแนนแบบ search operation! 🔍
    println!("🔍 มีทีม Red หรือไม่: {} (ค้นหาทีมแดงแบบ existence check!)", scores.contains_key("Red"));
    println!("🔍 มีทีม Purple หรือไม่: {} (ค้นหาทีมม่วงแบบ lookup!)", scores.contains_key("Purple"));

    // ลบข้อมูล - ไล่ทีมออกจากการแข่งขันแบบ elimination! 🗑️
    if let Some(removed_score) = scores.remove("Red") {
        println!("🗑️ ลบทีม Red (คะแนน {removed_score}) - ทีมแดงถูกไล่ออกแบบ disqualified!");
    }
    println!("📊 หลังลบ Red: {scores:?} (อัพเดทสมุดคะแนนแล้วแบบ real-time update!)");

    // รวบรวม keys และ values - สรุปข้อมูลทั้งหมดแบบ data aggregation! 📋
    let keys: Vec<_> = scores.keys().collect();    // รายชื่อทีมทั้งหมดแบบ team roster! 📝
    let values: Vec<_> = scores.values().collect(); // คะแนนทั้งหมดแบบ score summary! 💯
    println!("🔑 Keys: {keys:?} (รายชื่อทีมแบบ team list!)");
    println!("💎 Values: {values:?} (คะแนนทั้งหมดแบบ score board!)");

    println!("\n🎉 === สรุป HashMap แบบ conclusion! === 🎉");
    println!("✅ HashMap เป็นพจนานุกรมดิจิทัลที่เก็บ key-value pairs แบบ database!");
    println!("✅ ใช้ได้กับการค้นหา, เพิ่ม, แก้ไข, ลบข้อมูลอย่างรวดเร็วแบบ lightning fast!");
    println!("✅ Entry API ช่วยจัดการข้อมูลอย่างปลอดภัยแบบ secure access!");
    println!("✅ เหมาะสำหรับงานที่ต้องการความเร็วในการเข้าถึงข้อมูลแบบ high performance! 🚀");
    println!("\n🎉 จบการเรียนรู้ HashMap! (เป็นนักจัดการพจนานุกรมมืออาชีพแล้วแบบ expert level! 📚🏆)");
}
