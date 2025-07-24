//! Enums - การใช้งาน Enums ใน Rust
//!
//! ไฟล์นี้สอนเรื่องการสร้างและใช้งาน Enums ในรูปแบบต่างๆ
//! รวมถึง Basic Enums, Enums with Data, Option, Result และ Pattern Matching
//! (ร้านขายตัวเลือกแห่งเดียวในจักรวาล! 🛒)

/// ฟังก์ชันสำหรับสอนเรื่อง Enums
/// มาเรียนรู้การสร้างและใช้งาน Enums กันเถอะ! (เมนูอาหารแห่งโปรแกรม! 📋)
pub fn learn_enums() {
    println!("\n🎯 === Enums ใน Rust: ร้านขายตัวเลือก! === 🎯");

    // Basic Enums (เมนูอาหารพื้นฐาน! 🍽️)
    #[derive(Debug)]
    enum IpAddrKind {
        V4,  // รุ่นคลาสสิค! 📻
        V6,  // รุ่นใหม่ล่าสุด! 🚀
    }

    println!("🌐 === Basic Enums: เมนูที่อยู่อินเทอร์เน็ต! === 🌐");
    let four = IpAddrKind::V4;  // สั่งรุ่นคลาสสิค! 📞
    let six = IpAddrKind::V6;   // สั่งรุ่นใหม่! 📱

    println!("✅ IPv4: {four:?} (รุ่นคลาสสิคที่ทุกคนรู้จัก! 👴)");
    println!("✅ IPv6: {six:?} (รุ่นใหม่สำหรับอนาคต! 🔮)");

    // Enums with data (เมนูพิเศษมีของแถม! 🎁)
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),  // เซ็ตสี่ชิ้น! 🍱
        V6(String),          // เมนูพิเศษแบบยาว! 🍜
    }

    // ฟังก์ชันสำหรับใช้งาน IpAddr enum (พนักงานเสิร์ฟมืออาชีพ! 👨‍🍳)
    fn display_ip_info(addr: &IpAddr) {
        match addr {
            IpAddr::V4(a, b, c, d) => println!("🌐 IPv4 Address: {a}.{b}.{c}.{d} (เซ็ตสี่ชิ้นพร้อมเสิร์ฟ! 🍱)"),
            IpAddr::V6(addr) => println!("🌐 IPv6 Address: {addr} (เมนูพิเศษแบบยาว! 🍜)"),
        }
    }

    println!("\n📦 === Enums with Data: เมนูพิเศษมีของแถม! === 📦");
    let home = IpAddr::V4(127, 0, 0, 1);           // ที่อยู่บ้านหวาน! 🏠
    let loopback = IpAddr::V6(String::from("::1")); // ทางลัดกลับบ้าน! 🔄

    println!("🏠 ที่อยู่บ้าน: {home:?} (บ้านหวานบ้าน! 💕)");
    println!("🔄 Loopback: {loopback:?} (ทางลัดมหัศจรรย์! ✨)");
    
    // ใช้งาน enum ผ่านฟังก์ชัน (เรียกพนักงานเสิร์ฟ! 🔔)
    display_ip_info(&home);     // เสิร์ฟที่อยู่บ้าน! 🏠
    display_ip_info(&loopback); // เสิร์ฟทางลัด! 🛤️

    // Complex enum (เมนูครบครันสำหรับนักผจญภัย! 🗺️)
    #[derive(Debug)]
    enum Message {
        Quit,                        // ปุ่มหนีฉุกเฉิน! 🚨
        Move { x: i32, y: i32 },     // คำสั่งเทเลพอร์ต! ⚡
        Write(String),               // ปากกาวิเศษ! ✒️
        ChangeColor(i32, i32, i32),  // พู่กันสีรุ้ง! 🌈
    }

    impl Message {
        fn call(&self) {  // ตัวแปลภาษาข้อความ! 🗣️
            match self {
                Self::Quit => println!("🚪 ออกจากโปรแกรม (บายบาย! 👋)"),
                Self::Move { x, y } => println!("📍 เคลื่อนที่ไป ({x}, {y}) (วูช! เทเลพอร์ต! ⚡)"),
                Self::Write(text) => println!("✍️ เขียน: {text} (ปากกาวิเศษทำงาน! ✨)"),
                Self::ChangeColor(r, g, b) => println!("🎨 เปลี่ยนสีเป็น RGB({r}, {g}, {b}) (พู่กันสีรุ้งเริ่มทำงาน! 🌈)"),
            }
        }
    }

    println!("\n📨 === Complex Enums: ศูนย์รับส่งข้อความ! === 📨");
    let messages = vec![
        Message::Write(String::from("สวัสดี")),  // จดหมายรัก! 💌
        Message::Move { x: 10, y: 20 },          // ตั๋วเครื่องบิน! ✈️
        Message::ChangeColor(255, 0, 0),         // คำสั่งแต่งหน้า! 💄
        Message::Quit,                           // จดหมายลาออก! 📄
    ];

    for message in messages {  // ส่งข้อความทีละฉบับ! 📮
        message.call();
    }

    // Option enum (กล่องลุ้นโชคแห่งโปรแกรม! 🎰)
    println!("\n🎁 === Option Enum: กล่องลุ้นโชค! === 🎁");
    let some_number = Some(5);                    // ถูกรางวัลตัวเลข! 🎊
    let some_string = Some("a string");           // ถูกรางวัลข้อความ! 📜
    let absent_number: Option<i32> = None;        // ไม่ถูกรางวัล! 😢

    println!("✅ มีตัวเลข: {some_number:?} (โชคดี! 🍀)");
    println!("✅ มีข้อความ: {some_string:?} (แจ็คพอต! 💰)");
    println!("❌ ไม่มีตัวเลข: {absent_number:?} (เสียใจด้วย! 💔)");

    // Pattern matching with Option (เครื่องคิดเลขวิเศษ! 🧙‍♂️)
    fn plus_one(x: Option<i32>) -> Option<i32> {
        x.map(|i| i + 1)  // เวทมนตร์บวกหนึ่ง! ✨
    }

    let five = Some(5);              // เลข 5 ในกล่อง! 📦
    let six = plus_one(five);        // เวทมนตร์เปลี่ยน 5 เป็น 6! 🎩
    let none = plus_one(None);       // เวทมนตร์กับกล่องว่าง! 🪄

    println!("🔢 5 + 1 = {six:?} (เวทมนตร์สำเร็จ! ✨)");
    println!("❌ None + 1 = {none:?} (เวทมนตร์ไม่ได้ผล! 🚫)");

    // Result enum (ศาลยุติธรรมแห่งการคำนวณ! ⚖️)
    println!("\n✅ === Result Enum: ศาลยุติธรรม! === ✅");
    fn divide(a: f64, b: f64) -> Result<f64, String> {  // ผู้พิพากษาคณิตศาสตร์! 👨‍⚖️
        if b == 0.0 {
            Err(String::from("ไม่สามารถหารด้วยศูนย์ได้"))  // คำพิพากษา: ผิดกฎหมาย! 🚫
        } else {
            Ok(a / b)  // คำพิพากษา: ถูกต้อง! ✅
        }
    }

    let result1 = divide(10.0, 2.0);  // คดีที่ 1: การหารปกติ! 📋
    let result2 = divide(10.0, 0.0);  // คดีที่ 2: การหารต้องห้าม! ⚠️

    match result1 {  // อ่านคำพิพากษาคดีที่ 1! 📜
        Ok(value) => println!("✅ ผลลัพธ์: {value} (ชนะคดี! 🏆)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (แพ้คดี! 💸)"),
    }

    match result2 {  // อ่านคำพิพากษาคดีที่ 2! 📜
        Ok(value) => println!("✅ ผลลัพธ์: {value} (ชนะคดี! 🏆)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (แพ้คดี! 💸)"),
    }

    // Advanced pattern matching (นักสืบเฉพาะทาง! 🕵️‍♂️)
    println!("\n🎯 === Advanced Pattern Matching: สำนักงานสืบสวน! === 🎯");
    let config_max = Some(3u8);  // หลักฐานชิ้นที่ 1! 🔍

    if let Some(max) = config_max {  // ตรวจสอบหลักฐาน! 🔎
        println!("🔧 ค่าสูงสุดที่กำหนดไว้: {max} (พบหลักฐาน! 📋)");
    }

    let mut count = 0;        // ตู้เซฟส่วนตัว! 🏦
    let coin = Some(25);      // เหรียญลึกลับ! 🪙

    if let Some(value) = coin {  // ตรวจสอบเหรียญ! 🔍
        count += value;
        println!("💰 เพิ่มเหรียญ {value} เหรียญ, รวม: {count} (รวยขึ้นแล้ว! 💸)");
    } else {
        println!("🚫 ไม่มีเหรียญ (ยากจนเหมือนเดิม! 😢)");
    }
}
