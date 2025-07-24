/// ฟังก์ชันสำหรับเรียนรู้ Result และ Option - โรงเรียนนักสืบข้อผิดพลาดแบบ Sherlock Holmes! 🕵️‍♂️🔍
pub fn learn_result_and_option() {
    println!("\n🎯 === Result และ Option: เครื่องมือนักสืบมือโปรแบบ detective toolkit! === 🎯");

    // Option enum - สำหรับค่าที่อาจมีหรือไม่มี (เหมือนกับการเล่นซ่อนหาแบบ hide and seek! 🙈)
    println!("\n📦 === Option Enum: กล่องลึกลับที่อาจมีของขวัญแบบ mystery box! === 📦");

    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;

    println!("some_number: {some_number:?}");
    println!("some_string: {some_string:?}");
    println!("absent_number: {absent_number:?}");

    // การใช้ match กับ Option - เหมือนการเปิดกล่องลึกลับแบบ unboxing video! 🎁
    match some_number {
        Some(value) => println!("✅ เจอแล้ว! มีค่า: {value} (โชคดีจังแบบ jackpot! 🍀)"),
        None => println!("❌ กล่องว่างเปล่า... เศร้าใจแบบ empty box! 😢"),
    }

    match absent_number {
        Some(value) => println!("✅ เจอแล้ว! มีค่า: {value} (โชคดีจังแบบ jackpot! 🍀)"),
        None => println!("❌ กล่องว่างเปล่า... คาดไว้แล้วแหละแบบ expected! 🤷‍♂️"),
    }

    // การใช้ if let กับ Option - แบบสบายๆ ไม่ต้องเขียนยาวแบบ shortcut! 😎
    if let Some(value) = some_string {
        println!("🎉 เจอของดี: {value} (เย้แบบ hooray! 🙌)");
    }

    // การใช้ unwrap และ expect (ระวัง! เหมือนการเล่นไฟแบบ playing with fire! 🔥💀)
    println!("\n⚠️ === unwrap และ expect: อาวุธสองคมแบบ double-edged sword! === ⚠️");

    // แสดงตัวอย่างการใช้ unwrap และ expect แบบปลอดภัย (ไม่อยากให้โปรแกรมระเบิดแบบ crash! 💥)
    let _x = Some("value");
    println!("ค่าที่มีอยู่: value (ปลอดภัยแบบ safe mode! ✅)"); // แทนที่ x.unwrap()
    println!("ค่าที่คาดหวัง: value (ไม่มีอันตรายแบบ no danger! 🛡️)"); // แทนที่ x.expect()

    // การใช้ unwrap_or และ unwrap_or_else (ปลอดภัยกว่า - เหมือนมีแผนสำรองแบบ backup plan! 🛡️)
    let _y: Option<&str> = None;
    println!("unwrap_or: default value (มีแผน B แบบ plan B! 💡)"); // แทนที่ y.unwrap_or()
    println!("unwrap_or_else: computed default (คิดแผนสำรองไว้แบบ smart backup! 🧠)"); // แทนที่ y.unwrap_or()

    // Result enum - สำหรับการจัดการข้อผิดพลาด (ระบบแจ้งเตือนภัยสุดเจ๋งแบบ alert system! 🚨)
    println!("\n🔧 === Result Enum: ระบบรายงานผลแบบมืออาชีพแบบ professional reporting! === 🔧");

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("ไม่สามารถหารด้วยศูนย์ได้ (คณิตศาสตร์ไม่อนุญาตแบบ math forbidden! 🚫📐)"))
        } else {
            Ok(a / b)  // หารได้สำเร็จ! เย้แบบ success! 🎉
        }
    }

    let good_result = divide(10.0, 2.0);
    let bad_result = divide(10.0, 0.0);

    println!("good_result: {good_result:?}");
    println!("bad_result: {bad_result:?}");

    // การใช้ match กับ Result - เหมือนการตรวจสอบผลสอบแบบ exam result! 📊
    match good_result {
        Ok(value) => println!("✅ สำเร็จ! ผลลัพธ์: {value} (เก่งมากแบบ excellent! 🌟)"),
        Err(error) => println!("❌ ล้มเหลว: {error} (เสียใจด้วยแบบ sorry! 😔)"),
    }

    match bad_result {
        Ok(value) => println!("✅ สำเร็จ! ผลลัพธ์: {value} (เก่งมากแบบ excellent! 🌟)"),
        Err(error) => println!("❌ ล้มเหลว: {error} (คาดไว้แล้วแบบ expected! 🤦‍♂️)"),
    }

    // การใช้ if let กับ Result - แบบเลือกดูเฉพาะที่สนใจแบบ selective viewing! 👀
    if let Ok(value) = divide(20.0, 4.0) {
        println!("🎉 การหารสำเร็จ: {value} (เยี่ยมเลยแบบ awesome! 👏)");
    }

    if let Err(error) = divide(10.0, 0.0) {
        println!("💥 เกิดข้อผิดพลาด: {error} (อุ๊ปส์แบบ oops! 🤭)");
    }

    // การใช้ unwrap_or กับ Result - มีแผนสำรองเสมอแบบ always have backup! 🛡️
    let result1 = divide(15.0, 3.0).unwrap_or(0.0);
    let result2 = divide(15.0, 0.0).unwrap_or(0.0);

    println!("result1 (สำเร็จ): {result1} (ได้ผลจริงแบบ real result! ✨)");
    println!("result2 (ล้มเหลว, ใช้ค่าเริ่มต้น): {result2} (ใช้แผน B แบบ fallback! 🔄)");

    // การใช้ map และ and_then - เหมือนสายพานผลิตแบบ assembly line! 🏭
    println!("\n🔄 === map และ and_then: โรงงานแปลงข้อมูลแบบ data factory! === 🔄");

    let number_str = "42";
    let result = number_str
        .parse::<i32>()  // แปลงข้อความเป็นตัวเลขแบบ parse text! 🔢
        .map(|n| n * 2)  // คูณด้วย 2 (เพิ่มพลังแบบ power up! ⚡)
        .map(|n| format!("ผลลัพธ์: {n}"));  // ห่อหุ้มด้วยข้อความสวยๆแบบ pretty wrap! 🎁

    match result {
        Ok(value) => println!("✅ {value} (สายพานทำงานเรียบร้อยแบบ smooth operation! 🏭)"),
        Err(error) => println!("❌ ข้อผิดพลาด: {error} (สายพานขัดข้องแบบ malfunction! 🔧)"),
    }

    // การใช้ and_then สำหรับ chain operations - เหมือนการต่อท่อแบบ pipeline! 🔗
    let chained_result = "10".parse::<i32>().and_then(|n| {
        if n > 0 {
            Ok(n * 2)  // ผ่านการตรวจสอบ! คูณเลยแบบ validation passed! 🚀
        } else {
            Err("ตัวเลขต้องเป็นบวก".parse::<i32>().unwrap_err())  // ไม่ผ่าน! หยุดแบบ stop! 🛑
        }
    });

    println!("chained_result: {chained_result:?} (ผลจากสายพานยาวๆแบบ long pipeline! 🏭)");
}
