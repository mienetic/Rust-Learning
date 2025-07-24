/// ฟังก์ชันสำหรับสอนเรื่อง Lifetime Elision Rules - โรงเรียนสอนการหายตัวของอายุขัย! 🎩✨
/// เรียนรู้เวทมนตร์ที่ทำให้ lifetime หายไปโดยอัตโนมัติ! 🪄
pub fn learn_lifetime_elision() {
    println!("\n🎯 === Lifetime Elision Rules: โรงเรียนเวทมนตร์อายุขัย! === 🎯");

    println!("📋 กฎเวทมนตร์การ Elision (กฎของนักเวทย์อายุขัย!):");
    println!("1️⃣ แต่ละ reference parameter ได้ lifetime parameter ของตัวเอง (เหมือนแต่ละคนได้บัตรประชาชนเป็นของตัวเอง!)");
    println!("2️⃣ ถ้ามี input lifetime parameter เดียว ให้ใช้กับ output ทั้งหมด (เหมือนครูคนเดียวสอนนักเรียนทั้งห้อง!)");
    println!("3️⃣ ถ้ามี &self หรือ &mut self ให้ใช้ lifetime ของ self กับ output (เหมือนเจ้านายใช้อำนาจตัวเองกับลูกน้อง!)");

    // Rule 1: แต่ละ parameter ได้ lifetime ของตัวเอง - เหมือนแจกบัตรประชาชน! 🆔
    const fn rule1_example(s: &str) -> usize {
        // จริงๆ แล้วคือ: fn rule1_example<'a>(s: &'a str) -> usize
        // Rust เป็นนักเวทย์ที่เพิ่ม <'a> ให้เราโดยอัตโนมัติ! ✨
        s.len()
    }

    println!("\n1️⃣ === กฎที่ 1: การแจกบัตรประชาชนอายุขัย! === 1️⃣");
    let text = "Hello Rust";
    println!("📏 ความยาวของ '{}': {}", text, rule1_example(text));

    // Rule 2: input lifetime เดียว -> ใช้กับ output - เหมือนครูคนเดียวสอนทั้งห้อง! 👨‍🏫
    fn rule2_example(s: &str) -> &str {
        // จริงๆ แล้วคือ: fn rule2_example<'a>(s: &'a str) -> &'a str
        // Rust รู้ว่า output ต้องมีอายุขัยเดียวกับ input! 🧠
        &s[0..1]
    }

    println!("\n2️⃣ === กฎที่ 2: ครูคนเดียวสอนทั้งห้อง! === 2️⃣");
    let first_char = rule2_example(text);
    println!("🔤 ตัวอักษรแรกของ '{text}': '{first_char}'");

    // Rule 3: มี &self -> ใช้ lifetime ของ self - เหมือนเจ้านายใช้อำนาจ! 👑
    struct TextProcessor {
        prefix: String,  // คำนำหน้า (เหมือนยศของเจ้านาย!)
    }

    impl TextProcessor {
        fn new(prefix: &str) -> Self {
            Self {
                prefix: prefix.to_string(),
            }
        }

        // จริงๆ แล้วคือ: fn process<'a>(&'a self, input: &str) -> &'a str
        // เจ้านาย (self) ใช้อำนาจตัวเองกับ output! 👑
        fn process(&self, _input: &str) -> &str {
            &self.prefix
        }

        // ตัวอย่างที่ซับซ้อนกว่า
        fn combine(&self, suffix: &str) -> String {
            format!("{}-{}", self.prefix, suffix)
        }
    }

    println!("\n3️⃣ === กฎที่ 3: อำนาจของเจ้านาย! === 3️⃣");
    let processor = TextProcessor::new("RUST");
    let processed = processor.process("input");
    println!("⚙️ ประมวลผล: {processed}");

    let combined = processor.combine("LANG");
    println!("🔗 รวม: {combined}");

    // ตัวอย่างที่ต้องระบุ lifetime เอง (ไม่สามารถ elide ได้) - เวทมนตร์ล้มเหลว! 💥
    fn cannot_elide<'a>(x: &'a str, y: &str) -> &'a str {
        // ไม่สามารถ elide ได้เพราะมี 2 input lifetimes และไม่มี &self
        // Rust สับสน: "จะเลือก lifetime ไหนดี?" 🤔
        println!("🔍 เปรียบเทียบ: {x} vs {y}");
        x
    }

    println!("\n❌ === ไม่สามารถ Elide ได้: เวทมนตร์ล้มเหลว! === ❌");
    let str1 = "First";
    let str2 = "Second";
    let result = cannot_elide(str1, str2);
    println!("📤 ผลลัพธ์: {result}");

    // ตัวอย่างการใช้ lifetime bounds - กฎเหล็กของอายุขัย! ⛓️
    fn lifetime_bound_example<'a, T>(x: &'a T) -> &'a T
    where
        T: std::fmt::Display + 'a,  // T ต้องมีชีวิตอยู่นานพอ (เหมือนสัญญาจ้างงาน!)
    {
        println!("📄 แสดงค่า: {x}");
        x
    }

    println!("\n🔒 === Lifetime Bounds: กฎเหล็กของอายุขัย! === 🔒");
    let number = 42;
    let bounded_result = lifetime_bound_example(&number);
    println!("🔢 ผลลัพธ์ที่มี bound: {bounded_result}");
}
