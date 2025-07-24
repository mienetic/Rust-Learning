//! Control Flow - การควบคุมการทำงานของโปรแกรม
//!
//! ไฟล์นี้สอนเรื่อง if expressions, loops (loop, while, for), match expressions
//! และการใช้งาน control flow ต่างๆ ใน Rust
//! (ศูนย์ควบคุมการบิน: ผู้กำกับการแสดงโค้ด! 🎬🎮)

/// ฟังก์ชันสำหรับสอน Control Flow
/// มาเรียนรู้การควบคุมการทำงานของโปรแกรมกันเถอะ! 🎮
/// (ยินดีต้อนรับสู่ศูนย์ควบคุมการบิน! 🛩️)
pub fn learn_control_flow() {
    println!("\n🎮 === Control Flow: ศูนย์ควบคุมการบิน! === 🎮");

    // if expressions (นักตัดสินใจ! ⚖️)
    let number = 6;  // ตัวเลขผู้ถูกพิพากษา! 🎯
    if number % 4 == 0 {  // เงื่อนไขแรก! 1️⃣
        println!("🔢 {number} หารด้วย 4 ลงตัว (เป๊ะมาก! 🎯)");
    } else if number % 3 == 0 {  // เงื่อนไขที่สอง! 2️⃣
        println!("🔢 {number} หารด้วย 3 ลงตัว (ใกล้แล้ว! 🎪)");
    } else if number % 2 == 0 {  // เงื่อนไขที่สาม! 3️⃣
        println!("🔢 {number} เป็นเลขคู่ (คู่ใจ! 💑)");
    } else {  // กรณีอื่นๆ! 🤷‍♂️
        println!("🔢 {number} เป็นเลขคี่ (เดี่ยวดาย! 🕺)");
    }

    // if ใน let statement (การตัดสินใจแบบเร็ว! ⚡)
    let condition = true;  // เงื่อนไขความจริง! ✅
    let number = if condition { 5 } else { 6 };  // เลือกแบบฉับไว! 🏃‍♂️
    println!("🎯 ค่าที่ได้: {number} (ตัดสินใจแล้ว! 🎪)");

    // loop - วนลูปไม่สิ้นสุด (เครื่องจักรนิรันดร์! 🔄)
    println!("\n🔄 === Loops: สวนสนุกแห่งการวนซ้ำ! === 🔄");
    let mut counter = 0;  // ตัวนับผู้กล้าหาญ! 🧮
    let result = loop {  // ลูปอนันต์! ♾️
        counter += 1;  // นับไปเรื่อยๆ! ⬆️
        if counter == 3 {  // เมื่อถึงเป้าหมาย! 🎯
            break counter * 2; // หลุดออกมาพร้อมของรางวัล! 🎁
        }
    };
    println!("🔢 ผลลัพธ์จาก loop: {result} (หลุดออกมาแล้ว! 🏃‍♂️💨)");

    // while loop (ลูปนักรอ! ⏳)
    let mut number = 3;  // เครื่องนับถอยหลัง! ⏰
    while number != 0 {  // รอจนกว่าจะเป็น 0! 🎯
        println!("⏰ {number} (นับถอยหลัง! 📉)");
        number -= 1;  // ลดลงทีละ 1! ⬇️
    }
    println!("🚀 ปล่อยจรวด! (บินแล้ว! 🌌)");

    // for loop กับ array (ทัวร์ชมสวนผลไม้! 🍓)
    let fruits = ["แอปเปิ้ล", "กล้วย", "ส้ม"];  // ตะกร้าผลไม้! 🧺
    for fruit in fruits {  // เดินชมทีละผล! 🚶‍♂️
        println!("🍎 ผลไม้: {fruit} (อร่อยจัง! 😋)");
    }

    // for loop กับ range (ทัวร์ชมตัวเลข! 🔢)
    for number in 1..4 {  // เดินทางจาก 1 ถึง 3! 🚶‍♂️
        println!("🔢 ตัวเลข: {number} (สวยงาม! ✨)");
    }

    // for loop กับ index (ทัวร์แบบมีไกด์! 🗺️)
    for (index, value) in fruits.iter().enumerate() {  // เดินชมพร้อมหมายเลข! 📋
        println!("📍 ลำดับที่ {}: {} (จดบันทึกแล้ว! 📝)", index + 1, value);
    }

    // match expression (ศาลยุติธรรมแห่งการจับคู่! ⚖️)
    println!("\n🎯 === Match Expression: ศาลยุติธรรม! === 🎯");
    let dice_roll = 4;  // ลูกเต๋าผู้ถูกพิพากษา! 🎲
    match dice_roll {  // เปิดศาล! 👨‍⚖️
        1 => println!("🎲 ได้ 1 - เริ่มต้นใหม่! (โชคร้าย! 😅)"),
        2 | 3 => println!("🎲 ได้ {dice_roll} - ลองอีกครั้ง! (เกือบแล้ว! 🤏)"),
        4..=6 => println!("🎲 ได้ {dice_roll} - เก่งมาก! (ชนะแล้ว! 🏆)"),
        _ => println!("🎲 ลูกเต๋าแปลกๆ (มาจากมิติไหน? 🛸)"),
    }

    // match กับ Option (ตู้เซฟลึกลับ! 🔐)
    let maybe_number = Some(5);  // กล่องที่อาจมีของ! 📦
    match maybe_number {  // เปิดกล่อง! 🎁
        Some(x) => println!("🎁 มีค่า: {x} (โชคดี! 🍀)"),
        None => println!("📭 ไม่มีค่า (กล่องว่าง! 😢)"),
    }

    println!("✅ เรียนรู้ Control Flow เสร็จแล้ว! (ปิดศูนย์ควบคุม! 🎬🎉)");
}
