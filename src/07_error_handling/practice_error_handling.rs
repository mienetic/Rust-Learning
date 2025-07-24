/// ฟังก์ชันสำหรับฝึกฝน Error Handling - ยิมฝึกจัดการข้อผิดพลาด! 💪🏋️‍♂️
/// มาเรียนรู้การจัดการข้อผิดพลาดใน Rust กันเถอะ! (เตรียมตัวให้พร้อม! 🚀)
/// 
/// ใช้ #[`allow(clippy::too_many_lines)`] เพราะฟังก์ชันนี้เป็นตัวอย่างการเรียนรู้
/// ที่ครอบคลุมหลายแนวคิดของ Error Handling ในไฟล์เดียว เพื่อความเข้าใจที่ต่อเนื่อง (ยาวแต่คุ้มค่า! 📚)
#[allow(clippy::too_many_lines)]
pub fn practice_error_handling() {
    println!("\n💪 === แบบฝึกหัด Error Handling: ยิมฝึกจัดการข้อผิดพลาด! === 💪");

    // 1. File Reader Simulator - จำลองการอ่านไฟล์แบบมีอารมณ์! 📁😄
    #[derive(Debug)]
    enum FileError {
        NotFound,           // หาไม่เจอ (หายไปไหนนะ? 🤔)
        PermissionDenied,   // ไม่ได้รับอนุญาต (ห้ามเข้า! 🚫)
        InvalidFormat,      // รูปแบบผิด (อะไรนี่? 🤨)
    }

    impl std::fmt::Display for FileError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::NotFound => write!(f, "ไม่พบไฟล์ (หายไปไหนแล้ว? 👻)"),
                Self::PermissionDenied => write!(f, "ไม่มีสิทธิ์เข้าถึงไฟล์ (ห้ามเข้า! 🚫🔒)"),
                Self::InvalidFormat => write!(f, "รูปแบบไฟล์ไม่ถูกต้อง (อะไรนี่เนี่ย? 🤨📄)"),
            }
        }
    }

    fn read_file_simulator(filename: &str) -> Result<String, FileError> {
        match filename {
            "config.txt" => Ok(String::from("port=8080\nhost=localhost")),  // ไฟล์ config ปกติ! ✅
            "secret.txt" => Err(FileError::PermissionDenied),  // ลับสุดยอด! ห้ามดู! 🔐
            "data.json" => Ok(String::from("{\"name\": \"Rust\", \"version\": \"1.0\"}")),  // JSON สวยๆ! 📊
            "corrupted.bin" => Err(FileError::InvalidFormat),  // ไฟล์เสีย! 💥
            _ => Err(FileError::NotFound),  // ไม่รู้จักไฟล์นี้! 🤷‍♂️
        }
    }

    println!("📁 === File Reader Simulator: เครื่องจำลองการอ่านไฟล์! === 📁");

    let files = vec![
        "config.txt",
        "secret.txt",
        "data.json",
        "corrupted.bin",
        "missing.txt",
    ];

    for filename in files {
        match read_file_simulator(filename) {
            Ok(content) => println!("✅ อ่านไฟล์ '{filename}' สำเร็จ: {content} (เยี่ยม! 🎉)"),
            Err(error) => println!("❌ ไม่สามารถอ่านไฟล์ '{filename}': {error} (อุ๊ปส์! 😅)"),
        }
    }

    // 2. Calculator with Error Handling - เครื่องคิดเลขที่ปลอดภัย! 🧮🛡️
    #[derive(Debug)]
    enum CalculatorError {
        DivisionByZero,      // หารด้วยศูนย์ (ห้ามทำ! 🚫)
        InvalidOperation,    // การดำเนินการผิด (ไม่รู้จักสัญลักษณ์นี้! ❓)
        Overflow,           // ตัวเลขใหญ่เกินไป (ระเบิด! 💥)
    }

    impl std::fmt::Display for CalculatorError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::DivisionByZero => write!(f, "ไม่สามารถหารด้วยศูนย์ได้ (คณิตศาสตร์ไม่อนุญาต! 🚫📐)"),
                Self::InvalidOperation => write!(f, "การดำเนินการไม่ถูกต้อง (ไม่รู้จักสัญลักษณ์นี้! ❓🤔)"),
                Self::Overflow => write!(f, "ผลลัพธ์เกินขีดจำกัด (ตัวเลขใหญ่เกินไป! 💥🔢)"),
            }
        }
    }

    fn safe_calculator(a: i32, b: i32, operation: &str) -> Result<i32, CalculatorError> {
        match operation {
            "+" => a.checked_add(b).ok_or(CalculatorError::Overflow),  // บวกอย่างปลอดภัย! ➕
            "-" => a.checked_sub(b).ok_or(CalculatorError::Overflow),  // ลบอย่างปลอดภัย! ➖
            "*" => a.checked_mul(b).ok_or(CalculatorError::Overflow),  // คูณอย่างปลอดภัย! ✖️
            "/" => {
                if b == 0 {
                    Err(CalculatorError::DivisionByZero)  // หยุด! ห้ามหารด้วยศูนย์! 🛑
                } else {
                    Ok(a / b)  // หารได้! เย้! ➗
                }
            }
            _ => Err(CalculatorError::InvalidOperation),  // ไม่รู้จักสัญลักษณ์นี้! 🤷‍♂️
        }
    }

    println!("\n🧮 === Safe Calculator: เครื่องคิดเลขปลอดภัย! === 🧮");

    let calculations = vec![
        (10, 5, "+"),
        (10, 3, "-"),
        (6, 7, "*"),
        (15, 3, "/"),
        (10, 0, "/"),
        (i32::MAX, 1, "+"),
        (10, 5, "%"),
    ];

    for (a, b, op) in calculations {
        match safe_calculator(a, b, op) {
            Ok(result) => println!("✅ {a} {op} {b} = {result} (คำนวณได้! 🎯)"),
            Err(error) => println!("❌ {a} {op} {b}: {error} (อุ๊ปส์! 😅)"),
        }
    }

    // 3. Chain of Operations with Error Propagation - สายพานการประมวลผล! 🏭
    fn parse_and_double(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let number = input.trim().parse::<i32>()?;  // แปลงข้อความเป็นตัวเลข 🔢
        let doubled = number.checked_mul(2).ok_or("Overflow occurred")?;  // คูณด้วย 2 อย่างปลอดภัย! ✖️2
        Ok(doubled)  // ส่งผลลัพธ์กลับ! 🎁
    }

    fn process_numbers(inputs: Vec<&str>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        for input in inputs {
            let result = parse_and_double(input)?;  // ประมวลผลแต่ละตัว (ถ้าผิดพลาดจะหยุดทันที! 🛑)
            results.push(result);  // เก็บผลลัพธ์ 📦
        }
        Ok(results)  // ส่งผลลัพธ์ทั้งหมดกลับ! 🎉
    }

    println!("\n🔗 === Chain of Operations: สายพานการประมวลผล! === 🔗");

    let good_inputs = vec!["5", "10", "15", "20"];  // ข้อมูลดีๆ ทั้งหมด! ✨
    match process_numbers(good_inputs) {
        Ok(results) => println!("✅ ผลลัพธ์: {results:?} (สายพานทำงานเรียบร้อย! 🏭✅)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (เสียใจด้วย 😔)"),
    }

    let bad_inputs = vec!["5", "abc", "15"];  // มีข้อมูลเสียปนอยู่! 💥
    match process_numbers(bad_inputs) {
        Ok(results) => println!("✅ ผลลัพธ์: {results:?} (สายพานทำงานเรียบร้อย! 🏭✅)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (สายพานหยุดทำงาน! 🏭🛑)"),
    }

    println!("\n🎉 จบแบบฝึกหัด Error Handling! (เก่งมาก! ผ่านการฝึกแล้ว! 💪🏆)");
}
