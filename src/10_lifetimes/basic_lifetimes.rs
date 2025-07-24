/// ฟังก์ชันสำหรับสอนเรื่อง Basic Lifetimes - โรงเรียนสอนอายุขัย! 🎓⏰
/// มาเรียนรู้ Lifetimes ใน Rust กันเถอะ! เหมือนเรียนวิชาประวัติศาสตร์ของตัวแปร! 📚👻
pub fn learn_basic_lifetimes() {
    println!("⏰ === Lifetimes ใน Rust: โรงเรียนสอนอายุขัย! === ⏰");

    println!("\n📖 === ทำไมต้องมี Lifetimes: เหตุผลของนายอำเภอเวลา! === 📖");
    println!("🔍 Lifetimes ช่วยให้ Rust รู้ว่า references มีอายุการใช้งานนานแค่ไหน (เหมือนบัตรประชาชนของตัวแปร!)");
    println!("🛡️ ป้องกัน dangling references และ memory safety issues (เหมือนเจ้าหน้าที่รักษาความปลอดภัย!)");
    println!("⚡ ทำให้ Rust ไม่ต้องใช้ garbage collector (ประหยัดพลังงานเหมือนรีไซเคิลขยะ!)");

    // ตัวอย่างง่ายๆ ของ lifetime
    let string1 = String::from("Hello");
    let string2 = String::from("World");

    // ฟังก์ชันที่ไม่ต้องระบุ lifetime (lifetime elision)
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        s
    }

    println!("\n📝 === Lifetime Elision === 📝");
    let sentence = "Hello Rust World";
    let first = first_word(sentence);
    println!("📄 ประโยค: {sentence}");
    println!("🔤 คำแรก: {first}");

    // ฟังก์ชันที่ต้องระบุ lifetime
    const fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    println!("\n🏆 === ฟังก์ชันหาสตริงที่ยาวที่สุด === 🏆");
    let result = longest(&string1, &string2);
    println!("📏 สตริง 1: {} (ความยาว: {})", string1, string1.len());
    println!("📏 สตริง 2: {} (ความยาว: {})", string2, string2.len());
    println!("🥇 สตริงที่ยาวกว่า: {result}");

    // ตัวอย่างการใช้งานใน scope ต่างๆ
    {
        let string3 = String::from("Short");
        let string4 = String::from("Very long string");
        let result2 = longest(&string3, &string4);
        println!("\n🔍 === ใน Scope ย่อย === 🔍");
        println!("📏 สตริง 3: {} (ความยาว: {})", string3, string3.len());
        println!("📏 สตริง 4: {} (ความยาว: {})", string4, string4.len());
        println!("🥇 สตริงที่ยาวกว่า: {result2}");
    }

    // Multiple lifetime parameters
    fn announce_and_return_first<'b>(announcement: &str, x: &'b str, y: &'b str) -> &'b str {
        println!("📢 ประกาศ: {announcement}");
        if x.len() > y.len() { x } else { y }
    }

    println!("\n📢 === Multiple Lifetime Parameters === 📢");
    let announcement = "เปรียบเทียบสตริง!";
    let str1 = "Rust";
    let str2 = "Programming";
    let result3 = announce_and_return_first(announcement, str1, str2);
    println!("🎯 ผลลัพธ์: {result3}");
}
