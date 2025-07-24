//! Closures - การใช้งาน Closures ใน Rust
//!
//! ไฟล์นี้สอนเรื่อง closures, การ capture environment, closure types (Fn, `FnMut`, `FnOnce`)
//! และการใช้งาน closures กับ iterator methods
//! (โรงละครแห่งฟังก์ชันนิรนาม: ศิลปะแห่งการจับตัว! 🎭✨)

/// ฟังก์ชันสำหรับเรียนรู้ Closures
/// ยินดีต้อนรับสู่โรงละครแห่งฟังก์ชันนิรนาม! 🎭
pub fn learn_closures() {
    println!("\n🎭 === Closures: ฟังก์ชันนิรนามสุดเจ๋ง! === 🎭");

    // Basic closure (ฟังก์ชันนิรนามมือใหม่! 👶)
    let add_one = |x| x + 1;  // ฟังก์ชันบวก 1 แบบเท่! 😎
    println!("🔢 add_one(5) = {} (ง่ายมาก! 🎯)", add_one(5));

    // Closure กับ type annotations (ฟังก์ชันนิรนามแบบชัดเจน! 📝)
    let multiply = |x: i32, y: i32| -> i32 { x * y };  // ระบุชนิดข้อมูลชัดๆ! 🎯
    println!("✖️ multiply(4, 5) = {} (คูณเก่ง! 🌟)", multiply(4, 5));

    // Closure ที่ capture environment (ฟังก์ชันนักลักขโมย! 🕵️‍♂️)
    let factor = 10;  // ตัวแปรเหยื่อ! 🎯
    let scale = |x| x * factor;  // จับตัวแปร factor มาใช้! 🪤
    println!("📏 scale(5) = {} (คูณด้วย {} ที่จับมาได้! 🎣)", scale(5), factor);

    // Closure กับ move keyword (ฟังก์ชันนักย้ายบ้าน! 📦)
    let name = String::from("รัสต์");  // ชื่อเดิม! 🏷️
    let greet = move |greeting| format!("{greeting} {name}!");  // ย้าย name เข้ามาในฟังก์ชัน! 🚚
    println!("👋 {} (ย้ายมาแล้ว! 📍)", greet("สวัสดี"));
    // println!("{}", name); // Error! name ถูก move ไปแล้ว (หายไปแล้ว! 👻)

    // Closure กับ Vec methods (สวนสนุกแห่งการประมวลผลข้อมูล! 🎢)
    let numbers = [1, 2, 3, 4, 5];  // ทีมตัวเลข! 🔢

    // map (เครื่องแปลงข้อมูล! 🔄)
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();  // คูณ 2 ทุกตัว! ✖️2️⃣
    println!("🔢 doubled: {doubled:?} (ใหญ่ขึ้น 2 เท่า! 📈)");

    // filter (เครื่องกรองข้อมูล! 🔍)
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();  // เอาแต่เลขคู่! 👫
    println!("🔢 evens: {evens:?} (เลขคู่เท่านั้น! 💑)");

    // fold/reduce (เครื่องรวมข้อมูล! 🔗)
    let sum = numbers.iter().sum::<i32>();  // รวมทุกตัว! ➕
    println!("➕ sum: {sum} (รวมแล้วได้เท่านี้! 🧮)");

    // find (นักสืบตัวเลข! 🕵️‍♀️)
    let found = numbers.iter().find(|&x| x > &3);  // หาตัวที่มากกว่า 3! 🔍
    match found {
        Some(x) => println!("🔍 พบตัวเลขที่มากกว่า 3: {x} (เจอแล้ว! 🎯)"),
        None => println!("🔍 ไม่พบตัวเลขที่มากกว่า 3 (หาไม่เจอ! 😅)"),
    }

    // any และ all (นักตรวจสอบ! 👮‍♂️)
    let has_even = numbers.iter().any(|&x| x % 2 == 0);  // มีเลขคู่มั้ย? 🤔
    let all_positive = numbers.iter().all(|&x| x > 0);  // เป็นบวกหมดมั้ย? 🤔
    println!("❓ มีเลขคู่หรือไม่: {has_even} (ตรวจแล้ว! 📋)");
    println!("❓ เป็นเลขบวกทั้งหมดหรือไม่: {all_positive} (เช็คแล้ว! ✅)");

    // Closure types: Fn, FnMut, FnOnce (ตระกูลฟังก์ชันนิรนาม! 👨‍👩‍👧‍👦)
    println!("\n🎭 Closure Types: ครอบครัวฟังก์ชันนิรนาม!");

    // Fn - สามารถเรียกได้หลายครั้ง, ไม่เปลี่ยนแปลง captured values (ฟังก์ชันสุภาพ! 🎩)
    let multiplier = 3;  // ตัวแปรที่ปลอดภัย! 🔒
    let multiply_by_three = |x| x * multiplier;  // ฟังก์ชันที่ไม่ทำร้ายใคร! 😇
    println!("🔢 multiply_by_three(4) = {} (เรียกครั้งแรก! 1️⃣)", multiply_by_three(4));
    println!("🔢 multiply_by_three(7) = {} (เรียกครั้งที่สอง! 2️⃣)", multiply_by_three(7)); // เรียกได้อีก (ไม่มีปัญหา! ✅)

    // FnMut - สามารถเรียกได้หลายครั้ง, เปลี่ยนแปลง captured values ได้ (ฟังก์ชันนักแก้ไข! ✏️)
    let mut counter = 0;  // ตัวนับที่เปลี่ยนได้! 🔄
    let mut increment = || {  // ฟังก์ชันนักปรับปรุง! 🔧
        counter += 1;  // เพิ่มค่า! ⬆️
        counter
    };
    println!("📊 increment() = {} (ปรับปรุงครั้งแรก! 🛠️)", increment());
    println!("📊 increment() = {} (ปรับปรุงครั้งที่สอง! 🔨)", increment()); // ยังแก้ไขได้! 🎨

    // FnOnce - เรียกได้ครั้งเดียว, อาจ move captured values (ฟังก์ชันครั้งเดียว! 💥)
    let message = String::from("Hello");  // ข้อความที่จะหายไป! 👻
    let consume = || {  // ฟังก์ชันแบบใช้แล้วทิ้ง! 🗑️
        println!("📝 {message} (ครั้งแรกและครั้งสุดท้าย! 🎬)");
        message // move message out (ย้ายออกไป! 📦)
    };
    let _consumed = consume();  // เรียกใช้เพียงครั้งเดียว! ⚡
    // consume(); // Error! ไม่สามารถเรียกอีกครั้งได้ (หมดแล้ว! 🚫)

    println!("\n🎉 จบการเรียนรู้ Closures! (จบการแสดงแล้ว! 🎭🎉)");
}
