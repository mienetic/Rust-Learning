//! Vectors - กล่องเก็บของแบบยืดหยุ่นกว่า Elastic Man! 📦🎯🦸‍♂️
//!
//! ไฟล์นี้เป็นคู่มือการใช้งาน Vector - กล่องวิเศษที่ขยายได้แบบ TARDIS! 🪄📺
//! สอนการสร้าง, เข้าถึง, แก้ไข และใช้งาน Vector methods แบบมืออาชีพกว่า Marie Kondo! 🎪✨
//! รวมถึงการวนลูปและการจัดการข้อมูลแบบนักเก็บของระดับ Tetris master! 🗃️🎮

/// ฟังก์ชันสำหรับสอนเรื่อง Vector - เป็นครูสอนจัดระเบียบแบบ Hermione Granger! 👨‍🏫⚡
/// มาเรียนรู้การใช้งาน Vec กันเถอะ! เป็นนักเก็บของมืออาชีพกว่า Amazon warehouse! 📚🎯📦
pub fn learn_vectors() {
    println!("📚 === Vectors ใน Rust: กล่องเก็บของแบบยืดหยุ่นกว่ายางยืดของ Mr. Fantastic! === 📚");

    // การสร้าง Vector - เปิดโรงงานผลิตกล่องแบบ Willy Wonka! 🏭🍫
    println!("\n🆕 === การสร้าง Vector: โรงงานผลิตกล่องวิเศษแบบ Santa's workshop! === 🆕");

    // Vector ว่าง - กล่องเปล่าพร้อมใช้แบบ unboxing video! 📦
    let mut v1: Vec<i32> = Vec::new();
    println!("📦 Vector ว่าง: {v1:?} (กล่องเปล่าใหม่เอี่ยมกว่า iPhone ใหม่!)");

    // Vector ด้วย vec! macro - กล่องมีของแล้วแบบ surprise box! 🎁
    let v2 = vec![1, 2, 3, 4, 5];
    println!("📦 Vector ด้วย macro: {v2:?} (กล่องพร้อมของแบบ Happy Meal!)");
    
    // แสดงการใช้งาน Vec methods - ตรวจสอบกล่องแบบ quality control! 🔍
    println!("📊 ขนาดของ Vector: {} (นับของในกล่องแบบ inventory check!)", v2.len());
    println!("🔍 ตัวแรก: {:?} (ของชิ้นแรกแบบ first impression!)", v2.first());
    println!("🔍 ตัวสุดท้าย: {:?} (ของชิ้นสุดท้ายแบบ grand finale!)", v2.last());

    // การเพิ่มข้อมูล - ใส่ของเข้ากล่องแบบ Tetris! 📥
    v1.push(10);  // ใส่ของชิ้นที่ 1 แบบ perfect fit! 🎯
    v1.push(20);  // ใส่ของชิ้นที่ 2 แบบ combo! 🎯
    v1.push(30);  // ใส่ของชิ้นที่ 3 แบบ triple score! 🎯
    println!("➕ หลังจากเพิ่มข้อมูล: {v1:?} (กล่องเต็มแล้วแบบ full house!)");

    // การเข้าถึงข้อมูล - เปิดกล่องดูของแบบ treasure hunt! 👀
    println!("\n🔍 === การเข้าถึงข้อมูล: เปิดกล่องดูของแบบ unboxing channel! === 🔍");

    let numbers = [10, 20, 30, 40, 50];  // ชั้นวางของแบบ IKEA! 📚

    // การใช้ index (อาจ panic) - เปิดกล่องแบบเสี่ยงกว่า extreme sports! ⚡
    let third = numbers[2];
    println!("🔢 ตัวที่ 3: {third} (เปิดกล่องแบบเสี่ยงกว่า Russian roulette!)");

    // การใช้ get (ปลอดภัย) - เปิดกล่องแบบระวังกว่า bomb disposal! 🛡️
    match numbers.get(2) {
        Some(value) => println!("✅ ตัวที่ 3 (ปลอดภัย): {value} (เจอของแล้วแบบ jackpot!)"),
        None => println!("❌ ไม่พบข้อมูล (กล่องว่างแบบ empty wallet!)"),
    }

    match numbers.get(10) {
        Some(value) => println!("✅ ตัวที่ 11: {value} (เจอของแล้วแบบ miracle!)"),
        None => println!("❌ ไม่พบข้อมูลที่ตำแหน่ง 10 (กล่องไม่มีแบบ 404 not found!)"),
    }

    // การวนลูป - ตรวจสอบของทุกชิ้นแบบ security check! 🔄
    println!("\n🔄 === การวนลูป Vector: ตรวจสอบของทุกชิ้นแบบ customs inspection! === 🔄");

    println!("📋 รายการตัวเลข: (สำรวจกล่องทุกช่องแบบ treasure map!)");
    for (index, value) in numbers.iter().enumerate() {
        println!("  [{index}] = {value} (ช่องที่ {index} มี {value} แบบ bingo!)");
    }

    // การแก้ไขข้อมูล - ปรับปรุงของในกล่องแบบ makeover! ✏️
    let mut mutable_numbers = [1, 2, 3, 4, 5];  // กล่องที่แก้ไขได้แบบ customizable! 🔧
    println!("\n✏️ ก่อนแก้ไข: {mutable_numbers:?} (ของเดิมแบบ before photo!)");

    for value in &mut mutable_numbers {  // วนแก้ไขทุกชิ้นแบบ transformation! 🔄
        *value *= 2;  // คูณ 2 ทุกชิ้นแบบ double trouble! ✖️2️⃣
    }
    println!("✏️ หลังแก้ไข (คูณ 2): {mutable_numbers:?} (ของใหม่แบบ after photo!)");

    // Vector methods ที่มีประโยชน์ - เครื่องมือจัดการกล่องแบบ Swiss Army knife! 🛠️
    println!("\n🛠️ === Vector Methods: เครื่องมือจัดการกล่องแบบ professional toolkit! === 🛠️");

    let mut demo_vec = vec![1, 2, 3, 4, 5];  // กล่องทดลองแบบ lab experiment! 🧪
    println!("📊 ความยาว: {} (นับของในกล่องแบบ census!)", demo_vec.len());
    println!("❓ ว่างหรือไม่: {} (ตรวจสอบกล่องแบบ health check!)", demo_vec.is_empty());

    // pop - เอาตัวสุดท้ายออก (เอาของออกจากกล่องแบบ last in, first out!) 📤
    if let Some(last) = demo_vec.pop() {
        println!("🗑️ เอาออก: {last}, เหลือ: {demo_vec:?} (เอาของชิ้นสุดท้ายออกแบบ magic trick!)");
    }

    // insert - แทรกที่ตำแหน่งที่กำหนด (แซงคิวใส่ของแบบ VIP!) 🚀
    demo_vec.insert(2, 99);
    println!("➕ แทรก 99 ที่ตำแหน่ง 2: {demo_vec:?} (แซงคิวสำเร็จแบบ fast pass!)");

    // remove - ลบที่ตำแหน่งที่กำหนด (เอาของออกจากตำแหน่งเฉพาะแบบ precision!) 🎯
    let removed = demo_vec.remove(2);
    println!("🗑️ ลบที่ตำแหน่ง 2 ({removed}): {demo_vec:?} (เอาของออกแล้วแบบ surgical precision!)");

    // contains - ตรวจสอบว่ามีข้อมูลหรือไม่ (หาของในกล่องแบบ detective!) 🔍
    println!("🔍 มี 3 หรือไม่: {} (ค้นหาในกล่องแบบ Sherlock Holmes!)", demo_vec.contains(&3));
    println!("🔍 มี 99 หรือไม่: {} (ค้นหาของที่หายไปแบบ missing person case!)", demo_vec.contains(&99));

    println!("\n🎉 จบการเรียนรู้ Vector! (เป็นนักเก็บของมืออาชีพแล้วแบบ Marie Kondo level! 🏆✨)");
}
