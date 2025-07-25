//! Practice Functions - แบบฝึกหัด Functions และ Control Flow
//!
//! ไฟล์นี้มีแบบฝึกหัดต่างๆ เช่น เครื่องคิดเลข, ตรวจสอบจำนวนเฉพาะ,
//! ลำดับฟีโบนักชี, แปลงอุณหภูมิ และการหาตัวประกอบ
//! (ยิมแห่งการเขียนโค้ด: มาออกกำลังสมองกัน! 🏋️‍♂️💪)

/// ฟังก์ชันสำหรับฝึกฝน Functions และ Control Flow
/// มาทำแบบฝึกหัดกันเถอะ! 💪
/// (ยินดีต้อนรับสู่ยิมแห่งการเขียนโค้ด! 🏋️‍♂️)
pub fn practice_functions_and_control_flow() {
    println!("\n💪 === แบบฝึกหัด Functions และ Control Flow: ยิมสมอง! === 💪");

    // 1. เครื่องคิดเลขพื้นฐาน (เครื่องคิดเลขจอมยุทธ! 🧮)
    fn calculator(operation: char, a: f64, b: f64) -> f64 {  // นักคำนวณผู้เก่งกาจ! 🤓
        match operation {  // ดูว่าจะทำอะไร! 🤔
            '+' => a + b,  // บวกแบบสนุก! ➕
            '-' => a - b,  // ลบแบบเท่! ➖
            '*' => a * b,  // คูณแบบเจ๋ง! ✖️
            '/' => {  // หารแบบระวัง! ➗
                if b == 0.0 {  // ระวังศูนย์! 🚨
                    println!("⚠️ ไม่สามารถหารด้วยศูนย์ได้! (คณิตศาสตร์ไม่อนุญาต! 🚫)");
                    0.0  // คืนค่า 0! 🤷‍♂️
                } else {
                    a / b  // หารปกติ! ✅
                }
            }
            _ => {  // ไม่รู้จักคำสั่ง! 🤷‍♀️
                println!("⚠️ ไม่รู้จักการดำเนินการ: {operation} (มาจากดาวไหน? 👽)");
                0.0  // คืนค่า 0! 🤷‍♂️
            }
        }
    }

    println!("🧮 === เครื่องคิดเลข: ศูนย์คำนวณ! === 🧮");
    println!("➕ 10 + 5 = {} (บวกเก่ง! 🎯)", calculator('+', 10.0, 5.0));
    println!("➖ 10 - 5 = {} (ลบเท่! 😎)", calculator('-', 10.0, 5.0));
    println!("✖️ 10 * 5 = {} (คูณเจ๋ง! 🌟)", calculator('*', 10.0, 5.0));
    println!("➗ 10 / 5 = {} (หารสวย! ✨)", calculator('/', 10.0, 5.0));

    // 2. ตรวจสอบจำนวนเฉพาะ (นักสืบตัวเลข! 🕵️‍♂️)
    fn is_prime(n: u32) -> bool {  // ฟังก์ชันนักสืบ! 🔍
        if n < 2 {  // เล็กเกินไป! 👶
            return false;  // ไม่ใช่เฉพาะ! ❌
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        for i in 2..=f64::from(n).sqrt() as u32 {  // ตรวจสอบตัวหาร! 🔍
            if n % i == 0 {  // หารลงตัว! 😱
                return false;  // ไม่ใช่เฉพาะ! ❌
            }
        }
        true  // เป็นเฉพาะ! ✅
    }

    println!("\n🔍 === ตรวจสอบจำนวนเฉพาะ: สำนักงานสืบสวน! === 🔍");
    for num in 1..=10 {  // ตรวจสอบ 1-10! 📊
        if is_prime(num) {  // ถ้าเป็นเฉพาะ! ✨
            println!("✅ {num} เป็นจำนวนเฉพาะ (พิเศษมาก! 🌟)");
        } else {  // ถ้าไม่ใช่เฉพาะ! 😅
            println!("❌ {num} ไม่เป็นจำนวนเฉพาะ (ธรรมดา! 😊)");
        }
    }

    // 3. ลำดับฟีโบนักชี (ฟาร์มกระต่ายเวอร์ชันเร็ว! 🐰)
    fn fibonacci(n: u32) -> u32 {  // ฟังก์ชันกระต่ายเร็ว! 🏃‍♂️
        match n {  // ดูรุ่นที่เท่าไหร่! 👀
            0 => 0,  // รุ่นที่ 0: ไม่มี! 🚫
            1 => 1,  // รุ่นที่ 1: มี 1 ตัว! 🐰
            _ => {  // รุ่นอื่นๆ! 👨‍👩‍👧‍👦
                let mut a = 0;  // คู่แรก! 👫
                let mut b = 1;  // คู่แรก! 👫
                for _ in 2..=n {  // นับรุ่นต่อไป! 📈
                    let temp = a + b;  // ลูกใหม่! 👶
                    a = b;  // เลื่อนตำแหน่ง! 🔄
                    b = temp;  // เลื่อนตำแหน่ง! 🔄
                }
                b  // จำนวนกระต่าย! 🐰
            }
        }
    }

    println!("\n🌀 === ลำดับฟีโบนักชี: ฟาร์มกระต่ายเร็ว! === 🌀");
    for i in 0..10 {  // ดูกระต่าย 10 รุ่น! 📊
        println!("📊 F({}) = {} (รุ่นที่ {} มีกระต่าย! 🐰)", i, fibonacci(i), i);
    }

    // 4. แปลงอุณหภูมิ (เครื่องแปลงอุณหภูมิมหัศจรรย์! 🌡️)
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {  // แปลงเซลเซียสเป็นฟาเรนไฮต์! 🔥
        (celsius * 9.0 / 5.0) + 32.0  // สูตรมหัศจรรย์! ✨
    }

    fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {  // แปลงฟาเรนไฮต์เป็นเซลเซียส! ❄️
        (fahrenheit - 32.0) * 5.0 / 9.0  // สูตรกลับด้าน! 🔄
    }

    println!("\n🌡️ === แปลงอุณหภูมิ: ห้องแล็บอุณหภูมิ! === 🌡️");
    let temp_c = 25.0;  // อุณหภูมิเซลเซียส! 🌡️
    let temp_f = celsius_to_fahrenheit(temp_c);  // แปลงเป็นฟาเรนไฮต์! 🔄
    println!("🔥 {temp_c}°C = {temp_f:.1}°F (ร้อนขึ้น! 📈)");
    println!(
        "❄️ {:.1}°F = {:.1}°C (เย็นลง! 📉)",
        temp_f,
        fahrenheit_to_celsius(temp_f)
    );

    // 5. หาตัวประกอบ (นักสืบตัวประกอบ! 🕵️‍♀️)
    fn find_factors(n: u32) -> Vec<u32> {  // ฟังก์ชันนักสืบ! 🔍
        let mut factors = Vec::new();  // กล่องเก็บตัวประกอบ! 📦
        for i in 1..=n {  // ตรวจสอบทุกตัวเลข! 🔍
            if n % i == 0 {  // หารลงตัว! ✅
                factors.push(i);  // เก็บไว้! 📝
            }
        }
        factors  // คืนผลลัพธ์! 🎁
    }

    println!("\n🔢 === หาตัวประกอบ: สำนักงานสืบสวนตัวเลข! === 🔢");
    let number = 12;  // ตัวเลขผู้ต้องสงสัย! 🎯
    let factors = find_factors(number);  // เริ่มสืบสวน! 🔍
    println!("📊 ตัวประกอบของ {number}: {factors:?} (เจอหมดแล้ว! 🎊)");

    println!("\n✅ เสร็จสิ้นการฝึกฝน! (ออกจากยิมแล้ว! 🏋️‍♂️💪)");
}
