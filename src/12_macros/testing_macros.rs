//! # Testing and Benchmarking Macros - เวทมนตร์ทดสอบแบบ Testing Spells! 🧪✨
//!
//! ตัวอย่างการใช้ macros สำหรับการทดสอบและวัดประสิทธิภาพ
//! เหมือนการสร้างเวทมนตร์ที่ช่วยตรวจสอบและวัดพลังของเวทมนตร์อื่นๆ! 🔬🪄
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ที่ช่วยทดสอบและวัดประสิทธิภาพ!

/// ตัวอย่าง macro สำหรับ testing - เวทมนตร์ทดสอบแบบ Testing Spells! 🧪🔮
pub fn testing_macros_examples() {
    println!("\n🧪✨ === ตัวอย่าง Testing Macros - เวทมนตร์ทดสอบ! === ✨🧪");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยตรวจสอบความถูกต้อง! 🔍");

    // 🎯 Macro สำหรับ assertion ที่ดีขึ้น - เวทมนตร์ตรวจสอบความใกล้เคียง! 📏✨
    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr, $tolerance:expr) => {
            {
                let left_val: f64 = $left as f64;      // 🔢 ค่าซ้าย
                let right_val: f64 = $right as f64;    // 🔢 ค่าขวา
                let tolerance_val: f64 = $tolerance as f64;  // 📏 ค่าความคลาดเคลื่อนที่ยอมรับได้
                let diff = (left_val - right_val).abs();     // 🧮 คำนวณความแตกต่าง
                if diff > tolerance_val {
                    panic!(
                        "💥 เวทมนตร์ assertion ล้มเหลว: `(left ≈ right)` \n  🔢 left: `{}`,\n  🔢 right: `{}`,\n  📊 diff: `{}`,\n  📏 tolerance: `{}`",
                        left_val, right_val, diff, tolerance_val
                    );
                }
            }
        };
    }

    // ⏱️ Macro สำหรับ benchmark ง่ายๆ - เวทมนตร์จับเวลาแบบ Speed Wizard! 🏃‍♂️✨
    macro_rules! simple_benchmark {
        ($name:expr, $code:block) => {{
            let start = std::time::Instant::now();  // ⏰ เริ่มจับเวลาเวทมนตร์!
            let result = $code;                     // 🪄 ทำเวทมนตร์!
            let duration = start.elapsed();        // ⏱️ หยุดจับเวลาเวทมนตร์!
            println!("⏱️✨ {}: {:?} 🪄", $name, duration);  // 📊 แสดงผลเวลาเวทมนตร์!
            result
        }};
    }

    println!("\n🎯 === ทดสอบเวทมนตร์ assertion === 🎯");
    // 🧪 ทดสอบ assertion macro - ทดสอบเวทมนตร์ตรวจสอบ!
    assert_approx_eq!(std::f64::consts::PI, 3.14160, 0.001);  // 🥧 ทดสอบค่า PI!
    println!("✅🎉 Approximate equality test ผ่าน - เวทมนตร์ตรวจสอบสำเร็จ! 🪄");

    println!("\n⏱️ === ทดสอบเวทมนตร์ benchmark === ⏱️");
    // 🏃‍♂️ ทดสอบ benchmark macro - ทดสอบเวทมนตร์จับเวลา!
    let result = simple_benchmark!("การคำนวณ fibonacci", {  // 🧮 เวทมนตร์คำนวณ Fibonacci!
        fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,  // 🎯 กรณีฐาน: F(0) = 0
                1 => 1,  // 🎯 กรณีฐาน: F(1) = 1
                _ => fibonacci(n - 1) + fibonacci(n - 2),  // 🔄 เรียกตัวเองแบบ recursive!
            }
        }
        fibonacci(20)  // 🎯 คำนวณ Fibonacci ลำดับที่ 20!
    });

    println!("🔢✨ Fibonacci(20) = {result} 🎉");
}

/// ตัวอย่าง macro สำหรับ performance testing - เวทมนตร์วัดประสิทธิภาพแบบ Performance Spells! ⚡🔮
pub fn performance_macros_examples() {
    println!("\n⚡✨ === ตัวอย่าง Performance Macros - เวทมนตร์วัดประสิทธิภาพ! === ✨⚡");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยวัดความเร็วและประสิทธิภาพ! 🏃‍♂️");

    // ⏰ Macro สำหรับวัดเวลาที่ละเอียดขึ้น - เวทมนตร์จับเวลาแบบ Precision Timer! 🎯✨
    #[allow(unused_macros)]  // 🤫 ปิดเสียง warning สำหรับเวทมนตร์นี้!
    macro_rules! time_it {
        ($name:expr, $code:block) => {{
            let start = std::time::Instant::now();  // ⏰ เริ่มจับเวลาเวทมนตร์!
            println!("🚀✨ เริ่ม: {} 🪄", $name);      // 🎬 ประกาศเริ่มเวทมนตร์!
            let result = $code;                     // 🪄 ทำเวทมนตร์!
            let duration = start.elapsed();        // ⏱️ หยุดจับเวลาเวทมนตร์!
            println!("🏁🎉 เสร็จ: {} ใช้เวลา {:?} ✨", $name, duration);  // 🎊 ประกาศเสร็จเวทมนตร์!
            result
        }};
    }

    // 🏆 Macro สำหรับเปรียบเทียบประสิทธิภาพ - เวทมนตร์แข่งขันความเร็ว! 🏃‍♂️⚡
    macro_rules! compare_performance {
        ($name1:expr, $code1:block, $name2:expr, $code2:block) => {{
            println!("🏆✨ เปรียบเทียบประสิทธิภาพเวทมนตร์: 🪄");

            let start1 = std::time::Instant::now();  // ⏰ เริ่มจับเวลาเวทมนตร์ที่ 1!
            let result1 = $code1;                    // 🪄 ทำเวทมนตร์ที่ 1!
            let duration1 = start1.elapsed();       // ⏱️ หยุดจับเวลาเวทมนตร์ที่ 1!

            let start2 = std::time::Instant::now();  // ⏰ เริ่มจับเวลาเวทมนตร์ที่ 2!
            let result2 = $code2;                    // 🪄 ทำเวทมนตร์ที่ 2!
            let duration2 = start2.elapsed();       // ⏱️ หยุดจับเวลาเวทมนตร์ที่ 2!

            println!("   🎭 {} ใช้เวลา: {:?}", $name1, duration1);  // 📊 ผลเวทมนตร์ที่ 1
            println!("   🎭 {} ใช้เวลา: {:?}", $name2, duration2);  // 📊 ผลเวทมนตร์ที่ 2

            if duration1 < duration2 {
                println!("   🥇✨ {} เร็วกว่า! 🏃‍♂️💨", $name1);  // 🏆 ผู้ชนะ!
            } else {
                println!("   🥇✨ {} เร็วกว่า! 🏃‍♂️💨", $name2);  // 🏆 ผู้ชนะ!
            }

            (result1, result2)
        }};
    }

    println!("\n🏆 === ทดสอบเวทมนตร์เปรียบเทียบประสิทธิภาพ === 🏆");
    // 🏃‍♂️ ทดสอบ performance comparison - แข่งขันเวทมนตร์!
    let (result1, result2) = compare_performance!(
        "Vector push",           // 🐌 เวทมนตร์ Vector แบบธรรมดา
        {
            let mut vec = Vec::new();  // 📦 สร้าง Vector ว่าง
            for i in 0..1000 {
                vec.push(i);           // ➕ เพิ่มทีละตัว
            }
            vec.len()                  // 📏 นับจำนวน
        },
        "Vector with_capacity",  // 🚀 เวทมนตร์ Vector แบบ optimized
        {
            let mut vec = Vec::with_capacity(1000);  // 📦 สร้าง Vector พร้อมขนาด
            for i in 0..1000 {
                vec.push(i);                         // ➕ เพิ่มทีละตัว (เร็วกว่า!)
            }
            vec.len()                                // 📏 นับจำนวน
        }
    );

    println!("\n📊🎉 ผลลัพธ์การแข่งขันเวทมนตร์: {result1} vs {result2} ✨");
    println!("\n🎉 เวทมนตร์ทดสอบสำเร็จ! 🧪✨");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_approx_eq_macro() {
        macro_rules! assert_approx_eq {
            ($left:expr, $right:expr, $tolerance:expr) => {{
                let diff = ($left as f64 - $right as f64).abs();
                assert!(diff <= $tolerance as f64);
            }};
        }

        assert_approx_eq!(std::f64::consts::PI, 3.14160, 0.001);
    }

    #[test]
    fn test_benchmark_macro() {
        macro_rules! simple_benchmark {
            ($name:expr, $code:block) => {{
                let start = std::time::Instant::now();
                let result = $code;
                let _duration = start.elapsed();
                result
            }};
        }

        let result = simple_benchmark!("test calculation", {
            let mut sum = 0;
            for i in 1..=100 {
                sum += i;
            }
            sum
        });

        assert_eq!(result, 5050); // 1+2+...+100 = 5050
    }

    #[test]
    fn test_testing_macros_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        testing_macros_examples();
        performance_macros_examples();
    }
}