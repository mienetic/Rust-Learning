//! Practice Collections - ฝึกฝนเป็นนักสะสมมืออาชีพแบบ Pokemon master! 💪🏆🎮
//!
//! ไฟล์นี้มีตัวอย่างการใช้งาน Collections ในสถานการณ์จริง
//! เช่น ระบบจัดการเกรด, นับความถี่คำ, ติดตามผู้เยี่ยมชม และตะกร้าสินค้า
//! เหมือนเป็นโค้ชส่วนตัวที่สอนให้เป็นมืออาชีพแบบ personal trainer! 🎯✨💪

use std::collections::{HashMap, HashSet};

/// ฟังก์ชันสำหรับฝึกฝน Collections
/// มาฝึกฝนเป็นนักสะสมมืออาชีพกันเถอะ! เป็น data scientist! 💪🏆📊
pub fn practice_collections() {
    println!("\n💪 === แบบฝึกหัด Collections: ฝึกฝนเป็นมืออาชีพแบบ bootcamp! === 💪");

    // 1. Student Grade Manager - ระบบจัดการเกรดแบบครูใจดีกว่า Google Classroom! 🎓
    println!("🎓 === ระบบจัดการเกรดนักเรียน: ครูใจดีจัดการคะแนนแบบ AI teacher! === 🎓");

    let mut student_grades: HashMap<String, Vec<f64>> = HashMap::new();  // สมุดคะแนนดิจิทัลแบบ smart gradebook! 📚

    // เพิ่มคะแนนนักเรียน - บันทึกผลงานเด็กๆแบบ data entry! 📝
    student_grades.insert(String::from("สมชาย"), vec![85.5, 92.0, 78.5, 88.0]);    // นักเรียนขยันแบบ study hard! 📖
    student_grades.insert(String::from("สมหญิง"), vec![90.0, 87.5, 95.0, 89.5]);   // นักเรียนเก่งแบบ honor student! 🌟
    student_grades.insert(String::from("สมศักดิ์"), vec![75.0, 82.5, 79.0, 85.5]); // นักเรียนพยายามแบบ never give up! 💪

    // คำนวณเกรดเฉลี่ย - ครูคิดเลขให้แบบ calculator pro! 🧮
    for (name, grades) in &student_grades {
        let average: f64 = grades.iter().sum::<f64>() / grades.len() as f64;  // รวมคะแนนหารด้วยจำนวนครั้งแบบ math wizard! ➗
        let grade = match average {  // ให้เกรดตามมาตรฐานแบบ grading system! 📊
            90.0..=100.0 => "A",  // เก่งมากแบบ genius level! 🌟
            80.0..=89.9 => "B",   // เก่งแบบ good job! 👍
            70.0..=79.9 => "C",   // ปานกลางแบบ average! 😊
            60.0..=69.9 => "D",   // ต้องพยายามแบบ need improvement! 💪
            _ => "F",             // ต้องตั้งใจมากขึ้นแบบ study harder! 📚
        };
        println!("👤 {name}: คะแนนเฉลี่ย {average:.1} (เกรด {grade}) - ผลการเรียนแบบ report card!");
    }

    // 2. Word Frequency Counter - เครื่องนับคำอัจฉริยะแบบ Google Analytics! 📊
    println!("\n📊 === นับความถี่ของคำ: เครื่องนับคำอัจฉริยะแบบ text analyzer! === 📊");

    let text = "rust is great rust is fast rust is safe programming in rust is fun";  // ข้อความทดสอบแบบ sample data! 📝
    let mut word_count = HashMap::new();  // ตู้นับคำแบบ word counter! 🗃️

    for word in text.split_whitespace() {  // แยกคำทีละคำแบบ tokenizer! ✂️
        *word_count.entry(word).or_insert(0) += 1;  // นับคำ +1 แบบ increment! ➕
    }

    // เรียงลำดับตามความถี่ - จัดอันดับคำแบบ ranking system! 🏆
    let mut count_vec: Vec<_> = word_count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));  // เรียงจากมากไปน้อยแบบ descending order! 📉

    println!("📝 ความถี่ของคำ (เรียงจากมากไปน้อย - อันดับคำยอดนิยมแบบ trending words!):");
    for (word, count) in count_vec {
        println!("  '{word}': {count} ครั้ง (คำนี้ได้อันดับแบบ popular term!)");
    }

    // 3. Visitor Tracking System - ระบบติดตามผู้เยี่ยมชมแบบ Google Analytics! 👥
    println!("\n👥 === ระบบติดตามผู้เยี่ยมชม: ระบบ analytics แบบ web tracking! === 👥");

    let mut daily_visitors: HashMap<String, HashSet<String>> = HashMap::new();  // สมุดเซ็นชื่อดิจิทัลแบบ visitor database! 📋

    // จำลองข้อมูลผู้เยี่ยมชม - สร้างข้อมูลแบบ visitor simulation! 🎭
    let visits = vec![
        ("2024-01-01", "user1"),  // ผู้เยี่ยมชมคนแรกแบบ first visitor! 🥇
        ("2024-01-01", "user2"),  // ผู้เยี่ยมชมคนสองแบบ second visitor! 🥈
        ("2024-01-01", "user1"),  // ซ้ำ - ผู้เยี่ยมชมกลับมาอีกแบบ returning visitor! 🔄
        ("2024-01-02", "user2"),  // ผู้เยี่ยมชมมาอีกวันแบบ daily visitor! 📅
        ("2024-01-02", "user3"),  // ผู้เยี่ยมชมใหม่แบบ new visitor! 🆕
        ("2024-01-02", "user4"),  // ผู้เยี่ยมชมใหม่อีกคนแบบ another new visitor! 🎉
        ("2024-01-03", "user1"),  // ผู้เยี่ยมชมยังมาแบบ loyal visitor! 💪
        ("2024-01-03", "user3"),  // ผู้เยี่ยมชมกลับมาแบบ repeat visitor! 🔙
    ];

    for (date, user) in visits {  // บันทึกการเยี่ยมชมแบบ tracking visits! 📝
        daily_visitors
            .entry(date.to_string())
            .or_default()  // สร้างวันใหม่ถ้าไม่มีแบบ create new date! 🆕
            .insert(user.to_string());  // เซ็นชื่อในสมุดแบบ register visitor! ✍️
    }

    println!("📅 รายงานผู้เยี่ยมชมรายวันแบบ daily analytics report:");
    for (date, visitors) in &daily_visitors {
        println!("  {}: {} คนไม่ซ้ำ ({:?}) - ยอดเยี่ยมชมวันนี้แบบ unique visitors today!", date, visitors.len(), visitors);
    }

    // หาผู้ใช้ที่เยี่ยมชมทุกวัน - หาแฟนตัวจริง! 🔍
    let _all_dates: HashSet<_> = daily_visitors.keys().collect();
    let mut frequent_visitors = HashSet::new();  // ตู้เก็บแฟนตัวจริง! 🏆

    // รวบรวมผู้ใช้ทั้งหมด - เก็บรายชื่อแฟนทั้งหมด! 📋
    let mut all_users = HashSet::new();
    for visitors in daily_visitors.values() {
        for user in visitors {
            all_users.insert(user.clone());  // เก็บชื่อแฟน! 📝
        }
    }

    // ตรวจสอบว่าผู้ใช้คนไหนมาทุกวัน - หาแฟนซื่อสัตย์! 🕵️
    for user in &all_users {
        let mut visit_count = 0;
        for visitors in daily_visitors.values() {
            if visitors.contains(user) {
                visit_count += 1;  // นับวันที่มา! 📊
            }
        }
        if visit_count == daily_visitors.len() {  // มาทุกวัน! 💯
            frequent_visitors.insert(user.clone());  // แฟนตัวจริง! 🌟
        }
    }

    println!("🏆 ผู้เยี่ยมชมที่มาทุกวัน: {frequent_visitors:?} (แฟนตัวจริงของเว็บไซต์!)");

    // 4. Shopping Cart System - ระบบตะกร้าสินค้าอัจฉริยะแบบ e-commerce! 🛒
    println!("\n🛒 === ระบบตะกร้าสินค้า: ระบบตะกร้าสินค้าอัจฉริยะแบบ online shopping! === 🛒");

    let mut cart: HashMap<String, (f64, u32)> = HashMap::new(); // (ราคา, จำนวน) - ตะกร้าดิจิทัลแบบ digital cart! 💻

    // เพิ่มสินค้า - ใส่ของลงตะกร้าแบบ add to cart! 🍎
    cart.insert(String::from("แอปเปิ้ล"), (25.0, 5));  // แอปเปิ้ลสีแดงสวยแบบ fresh red apple! 🍎
    cart.insert(String::from("กล้วย"), (15.0, 3));     // กล้วยหวานๆแบบ sweet banana! 🍌
    cart.insert(String::from("ส้ม"), (30.0, 2));       // ส้มสดใหม่แบบ fresh orange! 🍊

    let mut total = 0.0;  // เครื่องคิดเงินแบบ calculator! 🧮
    println!("🧾 รายการสินค้าในตะกร้าแบบ shopping cart summary (ใบเสร็จสวยๆ!):");
    for (item, (price, quantity)) in &cart {
        let subtotal = price * f64::from(*quantity);  // คิดเงินรายการแบบ calculate item total! 💰
        total += subtotal;  // รวมเงินทั้งหมดแบบ sum total! ➕
        println!("  {item} x{quantity} @ {price:.0}฿ = {subtotal:.0}฿ (รายการนี้แบบ line item!)");
    }

    println!("💰 รวมทั้งหมด: {total:.0}฿ (ยอดชำระทั้งหมดแบบ grand total!)");

    println!("\n🎉 จบแบบฝึกหัด Collections! (เป็นนักสะสมมืออาชีพแล้วแบบ professional collector! 🏆💪)");
}
