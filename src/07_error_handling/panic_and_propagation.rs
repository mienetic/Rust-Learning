/// ฟังก์ชันสำหรับเรียนรู้ Panic และ Error Propagation - โรงเรียนนักดับเพลิงข้อผิดพลาด! 🚒🔥
///
/// # Panics
///
/// ฟังก์ชันนี้จะ panic เมื่อเรียกใช้ตัวอย่างที่สาธิต panic! macro (แต่เราจะไม่ให้มันระเบิดจริงๆ! 💣)
pub fn learn_panic_and_error_propagation() {
    println!("\n💥 === Panic และ Error Propagation: ระบบแจ้งเตือนภัยสุดเจ๋ง! === 💥");

    // Panic - เมื่อโปรแกรมไม่สามารถดำเนินการต่อได้ (เหมือนการกดปุ่มฉุกเฉิน! 🚨)
    println!("\n🚨 === Panic: ปุ่มฉุกเฉินของโปรแกรม! === 🚨");

    // ตัวอย่าง panic (แสดงความคิดเห็นเพื่อไม่ให้โปรแกรมระเบิด! 💥)
    // panic!("นี่คือ panic message! (ระเบิดแล้ว! 💣)");

    // การใช้ unwrap ที่อาจทำให้เกิด panic (เหมือนการเล่นไฟ! 🔥)
    let _some_value = Some(42);
    println!("ค่าที่ปลอดภัย: 42 (ไม่มีอันตราย! ✅)"); // แทนที่ some_value.unwrap()

    // let none_value: Option<i32> = None;
    // println!("unwrap ที่อันตราย: {}", none_value.unwrap()); // จะ panic! (ระเบิด! 💥)

    // การใช้ match แทน expect เพื่อ handle error อย่างปลอดภัย (ไม่มีระเบิด! 💌)
    let result = "42".parse::<i32>();
    match result {
        Ok(number) => println!("ตัวเลขที่แปลงได้: {number} (เย้! ได้แล้ว! 🎉)"),
        Err(e) => println!("❌ ไม่สามารถแปลงเป็นตัวเลขได้: {e} (อะไรกัน! 😱)"),
    }

    // Error Propagation ด้วย ? operator - เหมือนการส่งต่อข่าวร้าย! 📢
    println!("\n🔄 === Error Propagation: ระบบส่งต่อข้อผิดพลาดแบบมืออาชีพ! === 🔄");

    fn read_username_from_file() -> Result<String, std::io::Error> {
        // จำลองการอ่านไฟล์ (แกล้งทำเป็นอ่านไฟล์จริงๆ! 📁)
        // ในความเป็นจริงจะใช้ std::fs::read_to_string (แต่เราขี้เกียจ! 😅)
        Ok(String::from("john_doe"))  // ได้ username มาแล้ว! 🎉
    }

    fn get_user_id_from_username(username: String) -> Result<u32, std::num::ParseIntError> {
        // จำลองการแปลง username เป็น ID (ระบบฐานข้อมูลจำลอง! 🗃️)
        match username.as_str() {
            "john_doe" => Ok(12345),    // รู้จักคนนี้! 👋
            "jane_smith" => Ok(67890),  // รู้จักคนนี้ด้วย! 👋
            _ => "0".parse(), // ไม่รู้จัก! ส่ง error ไป! 🚫
        }
    }

    // ฟังก์ชันที่ใช้ ? operator สำหรับ error propagation (ระบบส่งต่อข้อผิดพลาดอัตโนมัติ! 🤖)
    fn get_user_id() -> Result<u32, Box<dyn std::error::Error>> {
        let username = read_username_from_file()?; // ถ้า error จะหนีทันที! 🏃‍♂️💨
        let user_id = get_user_id_from_username(username)?; // ถ้า error จะหนีอีกรอบ! 🏃‍♂️💨
        Ok(user_id)  // ทุกอย่างเรียบร้อย! ส่ง ID กลับไป! 🎁
    }

    match get_user_id() {
        Ok(id) => println!("✅ User ID: {id} (เจอแล้ว! 🎯)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (อุ๊ปส์! 🤦‍♂️)"),
    }

    // Custom Error Types - ออกแบบข้อผิดพลาดเอง! 🎨
    println!("\n🎨 === Custom Error Types: ร้านตัดเสื้อข้อผิดพลาด! === 🎨");

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,        // หารด้วยศูนย์ (ห้ามทำ! 🚫)
        NegativeSquareRoot,    // รากที่สองของจำนวนลบ (เป็นไปไม่ได้! 🤯)
    }

    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::DivisionByZero => write!(f, "ไม่สามารถหารด้วยศูนย์ได้ (คณิตศาสตร์ไม่อนุญาต! 🚫📐)"),
                Self::NegativeSquareRoot => write!(f, "ไม่สามารถหาค่ารากที่สองของจำนวนลบได้ (จินตนาการเกินไป! 🤯)"),
            }
        }
    }

    impl std::error::Error for MathError {}

    fn safe_divide_and_sqrt(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            return Err(MathError::DivisionByZero);  // หยุด! ห้ามหารด้วยศูนย์! 🛑
        }

        let result = a / b;
        if result < 0.0 {
            return Err(MathError::NegativeSquareRoot);  // หยุด! จำนวนลบไม่มีรากที่สอง! 🛑
        }

        Ok(result.sqrt())  // ปลอดภัย! คำนวณได้! ✅
    }

    println!("\n🧮 === ทดสอบระบบข้อผิดพลาดที่ออกแบบเอง! === 🧮");

    match safe_divide_and_sqrt(16.0, 4.0) {
        Ok(result) => println!("✅ √(16/4) = {result} (เยี่ยม! คำนวณได้! 🎯)"),
        Err(error) => println!("❌ {error} (เสียใจด้วย 😔)"),
    }

    match safe_divide_and_sqrt(16.0, 0.0) {
        Ok(result) => println!("✅ √(16/0) = {result} (เยี่ยม! คำนวณได้! 🎯)"),
        Err(error) => println!("❌ {error} (คาดไว้แล้ว! 🤦‍♂️)"),
    }

    match safe_divide_and_sqrt(-16.0, 4.0) {
        Ok(result) => println!("✅ √(-16/4) = {result} (เยี่ยม! คำนวณได้! 🎯)"),
        Err(error) => println!("❌ {error} (คาดไว้แล้วเช่นกัน! 🤷‍♂️)"),
    }
}
