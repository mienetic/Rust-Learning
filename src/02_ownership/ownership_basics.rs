/// ฟังก์ชันสำหรับสอนเรื่อง Ownership
/// มาเรียนรู้ระบบ Ownership ที่เป็นหัวใจของ Rust กันเถอะ! (เหมือนกฎหมายแต่สนุกกว่า! 🏛️)
pub fn learn_ownership() {
    println!("🔒 === Ownership ใน Rust: ระบบเจ้าของที่เข้มงวด! === 🔒");

    // กฎของ Ownership (กฎทองของ Rust! ✨)
    println!("📋 กฎของ Ownership (กฎที่ไม่มีข้อยกเว้น! ⚖️):");
    println!("1. แต่ละค่าใน Rust มี owner เพียงคนเดียว (เหมือนบ้านมีเจ้าของคนเดียว! 🏠)");
    println!("2. ในเวลาหนึ่งจะมี owner ได้เพียงคนเดียวเท่านั้น (ไม่มีการแชร์! 🚫)");
    println!("3. เมื่อ owner ออกจาก scope ค่านั้นจะถูกทำลาย (จบแล้วก็จบ! 💥)");

    // Move semantics (การย้ายบ้านแบบ Rust! 🚚)
    println!("\n📦 === Move Semantics: การย้ายความเป็นเจ้าของ! === 📦");
    let s1 = String::from("สวัสดี");
    let s2 = s1; // s1 ถูก move ไปยัง s2 (เหมือนการโอนกรรมสิทธิ์! 📋)
    // println!("{}", s1); // ❌ Error! s1 ไม่สามารถใช้ได้แล้ว (เจ้าของเก่าไม่มีสิทธิ์แล้ว! 🚫)
    println!("✅ s2: {s2} (เจ้าของใหม่ได้ครอบครองแล้ว! 👑)");

    // Clone - การคัดลอกข้อมูล (เหมือนการถ่ายเอกสาร! 📄)
    println!("\n📋 === Clone: การทำสำเนาแบบเป๊ะๆ! === 📋");
    let s3 = String::from("ยินดีต้อนรับ");
    let s4 = s3.clone(); // คัดลอกข้อมูล (ทำสำเนาเหมือนเครื่องถ่ายเอกสาร! 🖨️)
    println!("✅ s3: {s3} (ต้นฉบับยังอยู่! 📜)"); // ยังใช้ได้
    println!("✅ s4: {s4} (สำเนาใหม่เอี่ยม! ✨)");

    // Copy types - ชนิดข้อมูลที่ copy ได้ (ข้อมูลที่ใจดี! 😊)
    println!("\n📄 === Copy Types: ข้อมูลที่แชร์ได้ไม่เห็นแก! === 📄");
    let x = 5;
    let y = x; // copy ไม่ใช่ move (เหมือนการแจกขนม ทุกคนได้! 🍪)
    println!("✅ x: {x}, y: {y} (ทั้งคู่มีความสุข! 😄)"); // ทั้งคู่ใช้ได้

    // ฟังก์ชันและ ownership (การส่งมอบความรับผิดชอบ! 🤝)
    fn take_ownership(some_string: String) {
        println!("📥 รับ ownership: {some_string} (ได้รับมอบหมายแล้ว! 📋)");
    } // some_string ถูกทำลายที่นี่ (จบหน้าที่แล้ว! 💀)

    fn makes_copy(some_integer: i32) {
        println!("📄 copy ค่า: {some_integer} (ได้สำเนามาใช้! 📝)");
    }

    let s = String::from("hello");
    take_ownership(s); // s ถูก move เข้าไปในฟังก์ชัน (ส่งมอบหน้าที่! 📤)
    // println!("{}", s); // ❌ Error! s ไม่สามารถใช้ได้แล้ว (ไม่มีสิทธิ์แล้ว! 🚫)

    let x = 5;
    makes_copy(x); // x ถูก copy (แจกสำเนาให้! 📋)
    println!("✅ x ยังใช้ได้: {x} (ต้นฉบับยังอยู่! 😊)");

    // Return ownership (การคืนความเป็นเจ้าของ! 🔄)
    fn give_ownership() -> String {
        String::from("yours") // return และ move ownership ออกไป (ของขวัญจากฟังก์ชัน! 🎁)
    }

    const fn take_and_give_back(a_string: String) -> String {
        a_string // return ownership กลับไป (รับมาแล้วคืนไป! 🔄)
    }

    let s1 = give_ownership(); // ได้รับของขวัญ! 🎁
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2); // ส่งไปแล้วได้กลับมา! 🔄
    println!("✅ s1: {s1}, s3: {s3} (ทั้งคู่เป็นเจ้าของแล้ว! 👑)");
    // s2 ถูก move ไปแล้ว ไม่สามารถใช้ได้ (ไปอยู่ที่อื่นแล้ว! 🚚)
}
