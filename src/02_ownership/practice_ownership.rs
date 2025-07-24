/// ฟังก์ชันสำหรับฝึกฝน Ownership และ Borrowing
/// มาทำแบบฝึกหัดกันเถอะ! (ยิมสำหรับสมองโปรแกรมเมอร์! 🧠💪)
pub fn practice_ownership_and_borrowing() {
    println!("\n💪 === แบบฝึกหัด Ownership และ Borrowing: ยิมแห่งการยืม! === 💪");

    // 1. String manipulation (นักสืบหาคำแรก! 🕵️‍♂️)
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes(); // แปลงเป็นไบต์เพื่อตรวจสอบ! 🔍

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { // เจอช่องว่างแล้ว! 🎯
                return &s[0..i]; // คืนคำแรกที่เจอ! 🏆
            }
        }

        &s[..] // ถ้าไม่มีช่องว่าง คืนทั้งหมด! 📝
    }

    println!("🔤 === การจัดการ String: นักสืบหาคำ! === 🔤");
    let sentence = String::from("hello world rust");
    let word = first_word(&sentence); // ยืมมาหาคำแรก! 🔍
    println!("✅ คำแรก: '{word}' (เจอแล้ว! 🎯)");
    println!("✅ ประโยคเดิม: '{sentence}' (ยังคงสมบูรณ์! 📜)"); // ยังใช้ได้

    // 2. Vector operations (นักล่าตัวเลขที่ใหญ่ที่สุด! 🏹)
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0]; // เริ่มต้นด้วยตัวแรก! 🥇

        for item in list {
            if item > largest { // เจอตัวที่ใหญ่กว่า! 📈
                largest = item; // อัปเดตแชมป์ใหม่! 👑
            }
        }

        largest // คืนแชมป์สุดท้าย! 🏆
    }

    println!("\n📊 === การทำงานกับ Vector: การแข่งขันหาแชมป์! === 📊");
    let numbers = vec![34, 50, 25, 100, 65]; // ผู้เข้าแข่งขัน! 🏃‍♂️
    let result = largest(&numbers); // ยืมมาหาแชมป์! 🏆
    println!("✅ ตัวเลขที่ใหญ่ที่สุด: {result} (แชมป์แห่งปี! 👑)");
    println!("✅ vector เดิม: {numbers:?} (ผู้เข้าแข่งขันยังอยู่ครบ! 👥)"); // ยังใช้ได้

    // 3. String slicing (เชฟหั่นข้อความ! 👨‍🍳)
    fn get_slice(s: &str, start: usize, end: usize) -> &str {
        &s[start..end] // หั่นแบบแม่นยำ! 🔪
    }

    println!("\n🍰 === การตัด String: เชฟแห่งข้อความ! === 🍰");
    let text = "สวัสดีครับ";
    let slice = get_slice(text, 0, 6); // หั่นชิ้นแรก! 🥖
    println!("✅ ส่วนที่ตัด: '{slice}' (หั่นได้สวยงาม! ✨)");

    // 4. Ownership transfer (บริษัทขนส่งข้อมูล! 🚛)
    fn process_and_return(mut s: String) -> String {
        s.push_str(" - processed"); // ประทับตราว่าผ่านการประมวลผล! 📋
        s // ส่งคืนให้ลูกค้า! 🎁
    }

    println!("\n🚚 === การถ่ายโอน Ownership: บริษัทขนส่งแห่งข้อมูล! === 🚚");
    let original = String::from("data");
    let processed = process_and_return(original); // ส่งพัสดุไป! 📦
    // original ไม่สามารถใช้ได้แล้ว (ของหายไปแล้ว! 👻)
    println!("✅ ข้อมูลที่ประมวลผล: '{processed}' (พัสดุกลับมาแล้ว! 🎉)");

    // 5. Reference validation (นักตรวจสอบมืออาชีพ! 🕵️‍♀️)
    fn validate_references() {
        let mut data = vec![1, 2, 3, 4, 5]; // ทีมนักสู้! 🥊

        // อ่านข้อมูล (แอบดูไม่ให้ใครรู้! 👀)
        let first = &data[0]; // จับมือหัวหน้าทีม! 🤝
        let last = &data[data.len() - 1]; // จับมือคนสุดท้าย! 🤝
        println!("📖 ตัวแรก: {first}, ตัวสุดท้าย: {last} (ทักทายเสร็จแล้ว! 👋)");

        // แก้ไขข้อมูล (หลังจาก immutable references ไม่ถูกใช้แล้ว)
        data.push(6); // เพิ่มสมาชิกใหม่! 🆕
        println!("✏️ หลังเพิ่มข้อมูล: {data:?} (ทีมใหญ่ขึ้นแล้ว! 📈)");
    }

    println!("\n🕵️ === การตรวจสอบ References: สำนักงานสืบสวน! === 🕵️");
    validate_references();

    println!("\n🎉 จบแบบฝึกหัด Ownership และ Borrowing: ยิมปิดแล้ว! 🏋️‍♂️");
}
