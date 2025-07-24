//! Type Conversion Module - การเรียนรู้เกี่ยวกับการแปลงชนิดข้อมูลใน Rust (เหมือนการแปลงร่างของ Transformer! 🤖)

/// ฟังก์ชันสำหรับเรียนรู้ Type Conversion (มาเรียนรู้การเปลี่ยนชุดของข้อมูล! 👗➡️👔)
///
/// # Panics
/// 
/// ฟังก์ชันนี้จะ panic หากไม่สามารถแปลง string "42" เป็น i32 ได้
pub fn learn_type_conversion() {
    println!("\n🔄 === Type Conversion: ศิลปะการแปลงร่างข้อมูล! === 🔄");

    // Explicit casting (as keyword) (การบังคับให้แปลงร่าง! 🎭)
    println!("🎯 === Explicit Casting: การแปลงร่างแบบบังคับ! === 🎯");

    let integer = 42;
    let float = f64::from(integer); // แปลงจากจำนวนเต็มเป็นทศนิยม (เหมือนเพิ่มจุดให้ตัวเลข! 📍)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let byte = integer as u8; // บีบให้เล็กลง (เหมือนใส่เสื้อไซส์ S! 👕)

    println!("🔢 integer (i32): {integer} (ตัวเลขเต็มๆ! 💪)");
    println!("🔢 float (f64): {float} (ตัวเลขที่มีจุดทศนิยม! 🎯)");
    println!("🔢 byte (u8): {byte} (ตัวเลขขนาดมินิ! 🐭)");

    // Casting ระหว่าง numeric types (การเปลี่ยนขนาดตัวเลข! 📏)
    let big_number: i64 = 1000;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let small_number = big_number as i32; // ลดขนาดลง (เหมือนการลดน้ำหนัก! 🏃‍♂️)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let unsigned_number = big_number as u32; // เปลี่ยนเป็นบวกเท่านั้น (เหมือนคิดบวก! 😊)

    println!("\n📊 big_number (i64): {big_number} (ตัวเลขยักษ์! 🦣)");
    println!("📊 small_number (i32): {small_number} (ตัวเลขขนาดกลาง! 🐘)");
    println!("📊 unsigned_number (u32): {unsigned_number} (ตัวเลขที่คิดบวกเสมอ! 🌞)");

    // Parsing strings (การแปลงข้อความเป็นตัวเลข! 🔤➡️🔢)
    println!("\n📝 === String Parsing: นักแปลภาษาข้อความ! === 📝");

    let number_str = "42";
    let parsed_int: i32 = number_str.parse().expect("ไม่สามารถแปลงเป็นตัวเลข (ข้อความนี้ไม่ใช่ตัวเลข! 😵)");

    println!("📄 string: '{number_str}' (ข้อความที่แฝงตัวเลข! 🕵️)");
    println!("🔢 parsed integer: {parsed_int} (ตัวเลขที่แปลงสำเร็จ! ✨)");

    // Parsing กับ error handling (การแปลงแบบระวังผิดพลาด! 🛡️)
    let float_str = "3.14";
    match float_str.parse::<f64>() {
        Ok(num) => println!("✅ แปลง '{float_str}' เป็น float: {num} (สำเร็จ! เหมือนได้ค่า Pi! 🥧)"),
        Err(e) => println!("❌ ไม่สามารถแปลง: {e} (ล้มเหลว! 😢)"),
    }

    let invalid_str = "abc";
    match invalid_str.parse::<i32>() {
        Ok(num) => println!("✅ แปลง '{invalid_str}' เป็น integer: {num} (มหัศจรรย์! 🎩✨)"),
        Err(e) => println!("❌ ไม่สามารถแปลง '{invalid_str}': {e} (ตัวอักษรไม่ใช่ตัวเลข! 🤷‍♂️)"),
    }

    // Converting to string (การแปลงทุกอย่างเป็นข้อความ! 🔢➡️📝)
    println!("\n📝 === Converting to String: นักเขียนข้อความ! === 📝");

    let num = 123;
    let num_string = num.to_string(); // แปลงตัวเลขเป็นข้อความ (เหมือนการเขียนตัวเลขบนกระดาษ! 📝)
    let formatted = format!("ตัวเลข: {num}"); // จัดรูปแบบข้อความ (เหมือนการแต่งหน้าข้อความ! 💄)

    println!("🔢 number: {num} (ตัวเลขดิบๆ! 🥩)");
    println!("📄 to_string(): '{num_string}' (ตัวเลขที่กลายเป็นข้อความ! 🎭)");
    println!("📄 format!(): '{formatted}' (ข้อความที่แต่งตัวแล้ว! 👗)");

    // Boolean conversion (การแปลงความจริง-เท็จ! ✅❌)
    let bool_val = true;
    let bool_string = bool_val.to_string(); // แปลงบูลีนเป็นข้อความ (เหมือนการบอกความจริง! 🗣️)
    println!("\n✅ boolean: {bool_val} (ความจริงแท้! 💎)");
    println!("📄 boolean as string: '{bool_string}' (ความจริงในรูปข้อความ! 📜)");

    println!("\n🎉 จบการเรียนรู้ Type Conversion! (ตอนนี้คุณเป็นนักแปลงร่างข้อมูลแล้ว! 🦸‍♂️✨)");
}
