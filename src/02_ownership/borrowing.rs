/// ฟังก์ชันสำหรับสอนเรื่อง Borrowing
/// มาเรียนรู้การยืมข้อมูลโดยไม่ต้องเป็นเจ้าของกันเถอะ! (เหมือนการยืมหนังสือจากห้องสมุด! 📚)
pub fn learn_borrowing() {
    println!("\n🤝 === Borrowing: ศิลปะการยืมแบบสุภาพ! === 🤝");

    // Immutable references (การยืมแบบอ่านอย่างเดียว! 👀)
    println!("📖 === Immutable References: ยืมมาดูแต่ห้ามแก้! === 📖");
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // ยืมโดยไม่เอา ownership (เหมือนการยืมหนังสือ! 📚)
    println!("✅ ความยาวของ '{s1}' คือ {len} (ดูแล้วคืนให้! 📏)"); // s1 ยังใช้ได้

    const fn calculate_length(s: &str) -> usize {
        s.len() // วัดความยาวแล้วคืนให้! 📐
    } // s ออกจาก scope แต่ไม่ได้เป็นเจ้าของ จึงไม่ถูกทำลาย (ยืมมาใช้แล้วคืน! 🔄)

    // Mutable references (การยืมแบบแก้ไขได้! ✏️)
    println!("\n✏️ === Mutable References: ยืมมาแก้ไขได้! === ✏️");
    let mut s = String::from("hello");
    change(&mut s); // ยืมไปแก้ไข (เหมือนยืมสมุดไปเขียนเพิ่ม! 📝)
    println!("✅ หลังเปลี่ยน: {s} (ได้รับการปรับปรุงแล้ว! ✨)");

    fn change(some_string: &mut String) {
        some_string.push_str(", world"); // เพิ่มคำใหม่! 🌍
    }

    // กฎของ references (กฎการยืมที่เข้มงวด! ⚖️)
    println!("\n📋 === กฎของ References: กฎการยืมที่ไม่มีข้อยกเว้น! === 📋");
    println!("1. ในเวลาหนึ่ง สามารถมี immutable reference หลายตัว หรือ mutable reference เพียงตัวเดียว (เหมือนห้องสมุด: อ่านได้หลายคน แต่เขียนได้คนเดียว! 📚✏️)");
    println!("2. References ต้องมีอายุการใช้งานที่ถูกต้องเสมอ (ไม่มีการยืมแล้วหายไป! 🕰️)");

    let mut s = String::from("hello");

    // หลาย immutable references - OK (ปาร์ตี้อ่านหนังสือ! 📖🎉)
    let r1 = &s;
    let r2 = &s;
    println!("✅ r1: {r1}, r2: {r2} (ทุกคนอ่านได้! 👥)");
    // r1 และ r2 ไม่ถูกใช้หลังจากนี้ (คืนหนังสือแล้ว! 📚)

    // mutable reference - OK (ได้สิทธิ์แก้ไขแล้ว! ✏️)
    let r3 = &mut s;
    println!("✅ r3: {r3} (เป็นคนเดียวที่แก้ไขได้! 👑)");

    // Reference scope (ขอบเขตการยืม! 🏠)
    println!("\n🔍 === Reference Scope: ขอบเขตการยืมที่ชัดเจน! === 🔍");
    let mut s = String::from("hello");

    {
        let r1 = &s; // OK (ยืมในห้องนี้! 🏠)
        println!("📖 r1: {r1} (ใช้งานในห้องนี้! 🚪)");
    } // r1 ออกจาก scope (ออกจากห้องแล้ว! 🚶‍♂️)

    let r2 = &mut s; // OK - r1 ไม่อยู่แล้ว (ห้องว่างแล้ว ยืมใหม่ได้! 🆕)
    println!("✏️ r2: {r2} (เจ้าของใหม่ของห้อง! 🔑)");

    // Slice references (การหั่นข้อมูลแบบเชฟ! 🔪)
    println!("\n🍰 === Slice References: การหั่นข้อมูลแบบมืออาชีพ! === 🍰");
    let s = String::from("hello world");
    let hello = &s[0..5]; // หั่นชิ้นแรก! 🥖
    let world = &s[6..11]; // หั่นชิ้นที่สอง! 🍞
    println!("✅ ส่วนแรก: '{hello}', ส่วนที่สอง: '{world}' (หั่นได้สวยงาม! 👨‍🍳)");

    // Array slices (หั่น array แบบโปร! 🔪)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // เอาแค่ชิ้นกลางๆ! 🎯
    println!("✅ slice ของ array: {slice:?} (เลือกแค่ที่ต้องการ! 🎪)");
}
