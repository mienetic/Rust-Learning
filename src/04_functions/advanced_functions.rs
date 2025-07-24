//! Advanced Functions - ฟังก์ชันขั้นสูงใน Rust
//!
//! ไฟล์นี้สอนเรื่องฟังก์ชันขั้นสูง เช่น multiple parameters, tuple returns,
//! Option/Result returns, higher-order functions และ function pointers
//! (มหาวิทยาลัยแห่งฟังก์ชัน: สำหรับนักเรียนขั้นสูง! 🎓🚀)

/// ฟังก์ชันสำหรับเรียนรู้ฟังก์ชันขั้นสูง
/// ยินดีต้อนรับสู่มหาวิทยาลัยแห่งฟังก์ชัน! 🎓
pub fn learn_advanced_functions() {
    println!("\n🚀 === ฟังก์ชันขั้นสูง: ระดับเซียน! === 🚀");

    // ฟังก์ชันที่มีหลาย parameters (ฟังก์ชันหิวข้อมูล! 🍽️)
    fn calculate_bmi(weight: f64, height: f64, name: &str) -> (f64, String) {  // หมอเช็คสุขภาพ! 👩‍⚕️
        let bmi = weight / (height * height);  // สูตรเวทมนตร์! ✨
        let category = if bmi < 18.5 {
            "น้ำหนักน้อย".to_string()  // ผอมไป! 🥢
        } else if bmi < 25.0 {
            "น้ำหนักปกติ".to_string()  // เพอร์เฟค! 👌
        } else if bmi < 30.0 {
            "น้ำหนักเกิน".to_string()  // อุ๊ปส์! 😅
        } else {
            "อ้วน".to_string()  // ต้องออกกำลังกาย! 🏃‍♂️
        };

        println!("👤 {name}: BMI = {bmi:.2} ({category}) (รายงานสุขภาพ! 📊)");
        (bmi, category)  // คืนผลตรวจ! 📋
    }

    calculate_bmi(70.0, 1.75, "สมชาย");  // ตรวจสุขภาพสมชาย! 🩺

    // ฟังก์ชันที่ return tuple (ฟังก์ชันแจกของรางวัล! 🎁)
    fn get_name_and_age() -> (String, u32) {  // เครื่องแจกข้อมูล! 📦
        ("น้องรัสต์".to_string(), 13)  // ข้อมูลส่วนตัว! 🆔
    }

    let (name, age) = get_name_and_age();  // แกะของรางวัล! 🎉
    println!("👤 ชื่อ: {name}, อายุ: {age} ปี (ข้อมูลส่วนตัว! 📝)");

    // ฟังก์ชันที่ return Option (ฟังก์ชันระวังอันตราย! ⚠️)
    fn divide(a: f64, b: f64) -> Option<f64> {  // เครื่องหารปลอดภัย! 🛡️
        if b == 0.0 { None } else { Some(a / b) }  // เช็คก่อนหาร! 🔍
    }

    match divide(10.0, 2.0) {  // ทดสอบหารปกติ! ✅
        Some(result) => println!("➗ 10 ÷ 2 = {result} (หารได้สบาย! 😌)"),
        None => println!("❌ ไม่สามารถหารได้ (เฮ้อ! 😮‍💨)"),
    }

    match divide(10.0, 0.0) {  // ทดสอบหารอันตราย! 💣
        Some(result) => println!("➗ 10 ÷ 0 = {result} (เป็นไปไม่ได้! 🤯)"),
        None => println!("❌ ไม่สามารถหารด้วยศูนย์ได้ (รอดแล้ว! 😅)"),
    }

    // ฟังก์ชันที่ return Result (ฟังก์ชันนักแปลภาษา! 🗣️)
    fn parse_number(s: &str) -> Result<i32, String> {  // เครื่องแปลข้อความ! 🔄
        match s.parse::<i32>() {  // พยายามแปล! 💪
            Ok(num) => Ok(num),  // แปลได้! 🎉
            Err(_) => Err(format!("ไม่สามารถแปลง '{s}' เป็นตัวเลขได้")),  // แปลไม่ได้! 😵
        }
    }

    match parse_number("42") {  // ทดสอบข้อความดี! ✨
        Ok(num) => println!("✅ แปลงสำเร็จ: {num} (เก่งมาก! 🌟)"),
        Err(e) => println!("❌ {e} (เสียใจด้วย! 😢)"),
    }

    match parse_number("abc") {  // ทดสอบข้อความแปลก! 🤔
        Ok(num) => println!("✅ แปลงสำเร็จ: {num} (เป็นไปไม่ได้! 🤯)"),
        Err(e) => println!("❌ {e} (คาดไว้แล้ว! 😏)"),
    }

    // Higher-order functions (ฟังก์ชันระดับเทพ! 🧙‍♂️)
    fn apply_operation<F>(a: i32, b: i32, op: F) -> i32  // เครื่องประมวลผลพิเศษ! 🎛️
    where
        F: Fn(i32, i32) -> i32,  // รับฟังก์ชันมาเป็นพารามิเตอร์! 🎭
    {
        op(a, b)  // เรียกใช้ฟังก์ชันที่ได้รับมา! 🎪
    }

    const fn add(a: i32, b: i32) -> i32 {  // ฟังก์ชันบวกคลาสสิค! ➕
        a + b  // บวกธรรมดา! 🔢
    }
    const fn multiply(a: i32, b: i32) -> i32 {  // ฟังก์ชันคูณเก่ง! ✖️
        a * b  // คูณง่ายๆ! 📊
    }

    println!("\n🔧 === Higher-Order Functions: ฟังก์ชันเวทมนตร์! === 🔧");
    println!(
        "➕ apply_operation(5, 3, add) = {} (ส่งฟังก์ชันบวกไป! 📤)",
        apply_operation(5, 3, add)
    );
    println!(
        "✖️ apply_operation(5, 3, multiply) = {} (ส่งฟังก์ชันคูณไป! 📦)",
        apply_operation(5, 3, multiply)
    );

    // ฟังก์ชันที่ return ฟังก์ชัน (โรงงานผลิตฟังก์ชัน! 🏭)
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {  // เครื่องสร้างฟังก์ชันบวก! 🎰
        move |y| x + y  // closure ที่จำค่า x ได้! 🧠
    }

    let add_5 = make_adder(5);  // สั่งทำฟังก์ชันบวก 5! 🛒
    println!("🔢 add_5(10) = {} (ฟังก์ชันที่เกิดใหม่ทำงาน! 🤖)", add_5(10));

    println!("\n🎉 จบการเรียนรู้ฟังก์ชันขั้นสูง: ขอแสดงความยินดี! 🎊");
    println!("🏆 คุณได้เรียนรู้ฟังก์ชันระดับเซียนแล้ว! (เก่งมาก! 👏)");
}
