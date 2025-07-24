//! Operators Module - การเรียนรู้เกี่ยวกับ Operators ใน Rust (เครื่องมือคำนวณของโปรแกรมเมอร์! 🧮)

/// ฟังก์ชันสำหรับเรียนรู้ Operators (มาเรียนรู้การทำคณิตศาสตร์แบบโปรแกรมเมอร์! 🤓)
pub fn learn_operators() {
    println!("\n🧮 === Operators: เครื่องมือวิเศษของโปรแกรมเมอร์! === 🧮");

    // Arithmetic operators (เครื่องคิดเลขในโลกโค้ด! 🧮)
    println!("➕ === Arithmetic Operators: นักคณิตศาสตร์ดิจิทัล! === ➕");

    let first_num = 10;
    let second_num = 3;

    println!("🔢 first_num = {first_num}, second_num = {second_num} (คู่หูคำนวณ! 👫)");
    println!("➕ first_num + second_num = {} (การบวกที่ไม่มีวันผิด! ✅)", first_num + second_num);
    println!("➖ first_num - second_num = {} (การลบที่ไม่ใช่การลบเพื่อน! 😄)", first_num - second_num);
    println!("✖️ first_num * second_num = {} (การคูณที่ทำให้ตัวเลขอ้วน! 🍔)", first_num * second_num);
    println!("➗ first_num / second_num = {} (การหารที่ไม่ใช่การแบ่งปัน! 🍕)", first_num / second_num);
    println!("📐 first_num % second_num = {} (เศษที่เหลือ เหมือนข้าวที่เหลือในจาน! 🍚)", first_num % second_num);

    // Floating point arithmetic (คณิตศาสตร์ทศนิยมที่ลอยๆ! 🎈)
    let float_first = 10.5;
    let float_second = 3.2;

    println!("\n🔢 float_first = {float_first}, float_second = {float_second} (ตัวเลขที่มีจุดทศนิยม! 🎯)");
    println!("➕ float_first + float_second = {} (บวกแบบมีจุด! 📍)", float_first + float_second);
    println!("➖ float_first - float_second = {} (ลบแบบละเอียด! 🔍)", float_first - float_second);
    println!("✖️ float_first * float_second = {} (คูณแบบทศนิยม! 💫)", float_first * float_second);
    println!("➗ float_first / float_second = {} (หารแบบแม่นยำ! 🎯)", float_first / float_second);
    println!("📐 float_first % float_second = {} (เศษทศนิยมที่เหลือ! 🍰)", float_first % float_second);

    // Comparison operators (ผู้พิพากษาในโลกตัวเลข! ⚖️)
    println!("\n🔍 === Comparison Operators: ผู้พิพากษาดิจิทัล! === 🔍");

    let num1 = 5;
    let num2 = 10;

    println!("🔢 num1 = {num1}, num2 = {num2} (คู่แข่งในสนามเปรียบเทียบ! 🥊)");
    println!("❓ num1 == num2: {} (เท่ากันมั้ย? เหมือนฝาแฝด! 👯)", num1 == num2);
    println!("❓ num1 != num2: {} (ต่างกันมั้ย? เหมือนคนละดาว! 🌟)", num1 != num2);
    println!("❓ num1 < num2: {} (น้อยกว่ามั้ย? เหมือนเด็กกับผู้ใหญ่! 👶👨)", num1 < num2);
    println!("❓ num1 > num2: {} (มากกว่ามั้ย? เหมือนยักษ์กับคน! 👹👤)", num1 > num2);
    println!("❓ num1 <= num2: {} (น้อยกว่าหรือเท่ากับ! 📉)", num1 <= num2);
    println!("❓ num1 >= num2: {} (มากกว่าหรือเท่ากับ! 📈)", num1 >= num2);

    // Logical operators (นักตรรกะในโลกดิจิทัล! 🧠)
    println!("\n🧠 === Logical Operators: นักปรัชญาโค้ด! === 🧠");

    let is_true = true;
    let is_false = false;

    println!("✅ is_true = {is_true}, is_false = {is_false} (ความจริงกับความเท็จ! ⚡)");
    println!("🔗 is_true && is_false: {} (AND: ต้องจริงทั้งคู่! เหมือนต้องมีกุญแจ 2 ดอก! 🔐🔐)", is_true && is_false);
    println!("🔗 is_true || is_false: {} (OR: จริงอันใดอันหนึ่งก็พอ! เหมือนมีประตู 2 บาน! 🚪🚪)", is_true || is_false);
    println!("🔄 !is_true: {} (NOT: กลับด้าน! เหมือนกระจกเวทย์! 🪞)", !is_true);
    println!("🔄 !is_false: {} (NOT: เปลี่ยนเท็จเป็นจริง! ✨)", !is_false);

    // Bitwise operators (นักเล่นกับบิตในโลกดิจิทัล! 🔢)
    println!("\n🔢 === Bitwise Operators: นักเวทย์บิต! === 🔢");

    let bits1 = 0b1100; // 12 in decimal (เลขฐานสองที่สวยงาม! 💎)
    let bits2 = 0b1010; // 10 in decimal (อีกหนึ่งเลขฐานสองที่น่ารัก! 🌟)

    println!("🔢 bits1 = {bits1} (binary: {bits1:04b}) (เลขฐานสองที่หน้าตาดี! 😍)");
    println!("🔢 bits2 = {bits2} (binary: {bits2:04b}) (คู่หูของ bits1! 👫)");
    println!(
        "🔗 bits1 & bits2 = {} (binary: {:04b}) (AND: เอาแต่ที่เหมือนกัน! 🤝)",
        bits1 & bits2,
        bits1 & bits2
    );
    println!(
        "🔗 bits1 | bits2 = {} (binary: {:04b}) (OR: รวมทุกอย่าง! 🤗)",
        bits1 | bits2,
        bits1 | bits2
    );
    println!(
        "🔗 bits1 ^ bits2 = {} (binary: {:04b}) (XOR: เอาแต่ที่ต่างกัน! 🎭)",
        bits1 ^ bits2,
        bits1 ^ bits2
    );
    println!("🔄 !bits1 = {} (binary: {:04b}) (NOT: กลับหัวกลับหาง! 🙃)", !bits1, !bits1);
    println!(
        "⬅️ bits1 << 1 = {} (binary: {:04b}) (เลื่อนซ้าย: ทำให้ใหญ่ขึ้น! 📈)",
        bits1 << 1,
        bits1 << 1
    );
    println!(
        "➡️ bits1 >> 1 = {} (binary: {:04b}) (เลื่อนขวา: ทำให้เล็กลง! 📉)",
        bits1 >> 1,
        bits1 >> 1
    );

    // Assignment operators (นักเปลี่ยนแปลงค่า! 📝)
    println!("\n📝 === Assignment Operators: นักแก้ไขตัวแปร! === 📝");

    let mut value = 10;
    println!("🔢 เริ่มต้น value = {value} (จุดเริ่มต้นของการผจญภัย! 🚀)");

    value += 5;
    println!("➕ หลัง value += 5: {value} (เพิ่มพลังให้ตัวแปร! 💪)");

    value -= 3;
    println!("➖ หลัง value -= 3: {value} (ลดน้ำหนักตัวแปร! 🏃‍♂️)");

    value *= 2;
    println!("✖️ หลัง value *= 2: {value} (ทำให้ตัวแปรโตเป็น 2 เท่า! 🎈)");

    value /= 4;
    println!("➗ หลัง value /= 4: {value} (แบ่งตัวแปรให้เล็กลง! 🍕)");

    value %= 3;
    println!("📐 หลัง value %= 3: {value} (เหลือแค่เศษ! 🍰)");

    println!("\n🎉 จบการเรียนรู้ Operators! (ตอนนี้คุณเป็นนักคณิตศาสตร์โค้ดแล้ว! 🧮✨)");
}
