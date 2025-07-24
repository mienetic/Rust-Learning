//! # Logging and Utility Macros - เวทมนตร์บันทึกแบบ Logging Spells! 📝✨
//!
//! ตัวอย่างการใช้ macros สำหรับ logging และ utilities ต่างๆ
//! เหมือนการสร้างเวทมนตร์ที่ช่วยติดตามและบันทึกสิ่งที่เกิดขึ้น! 🔍🪄
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ที่ช่วยในการ debug และ logging!
//!
//! ⚠️ **หมายเหตุเกี่ยวกับ Warnings** (เรื่องปกติของเวทมนตร์!):
//! - macro `time_it` อาจแสดง warning "unused macro definition"
//! - 🎭 **เหตุผล**: macro นี้ถูกใช้งานภายในฟังก์ชัน `logging_macros()` เท่านั้น
//! - 🔮 Rust compiler อาจไม่สามารถตรวจสอบการใช้งาน macro ภายใน local scope ได้อย่างสมบูรณ์
//! - 🪄 การลบ macro นี้จะทำให้ตัวอย่างไม่สมบูรณ์ (เป็นเรื่องปกติของเวทมนตร์!)

/// ตัวอย่าง macro สำหรับ logging - เวทมนตร์บันทึกแบบ Logging Spells! 📝🔮
pub fn logging_macros() {
    println!("\n📝✨ === ตัวอย่าง Logging Macros - เวทมนตร์บันทึก! === ✨📝");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยบันทึกและติดตามการทำงาน! 🔍");

    // 📝 Macro สำหรับ logging ระดับต่างๆ - เวทมนตร์บันทึกหลายระดับ! 🎭✨
    macro_rules! log {
        (info, $($arg:tt)*) => {
            println!("ℹ️✨ [INFO] {} 🪄", format!($($arg)*));  // 💙 เวทมนตร์ข้อมูล!
        };
        (warn, $($arg:tt)*) => {
            println!("⚠️🔶 [WARN] {} 🪄", format!($($arg)*));  // 🧡 เวทมนตร์เตือน!
        };
        (error, $($arg:tt)*) => {
            println!("❌🔴 [ERROR] {} 🪄", format!($($arg)*)); // ❤️ เวทมนตร์ข้อผิดพลาด!
        };
        (debug, $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                println!("🐛🔍 [DEBUG] {} 🪄", format!($($arg)*)); // 💚 เวทมนตร์ debug!
            }
        };
    }

    println!("\n🎭 === เรียกใช้เวทมนตร์บันทึก === 🎭");
    log!(info, "แอปพลิเคชันเริ่มทำงาน");                    // 🚀 เวทมนตร์เริ่มต้น!
    log!(warn, "การตั้งค่า {} ไม่ถูกต้อง", "database");      // ⚠️ เวทมนตร์เตือนภัย!
    log!(error, "ไม่สามารถเชื่อมต่อกับ server ได้");         // 💥 เวทมนตร์แจ้งเหตุฉุกเฉิน!
    log!(debug, "ค่าตัวแปร x = {}", 42);                   // 🔍 เวทมนตร์ตรวจสอบ!

    // ⏰ Macro สำหรับ timing - เวทมนตร์จับเวลาแบบ Time Wizard! 🕐✨
    // ⚠️ Warning: macro `time_it` อาจแสดง warning "unused macro definition" (เป็นเรื่องปกติของเวทมนตร์!)
    // 🎭 เหตุผล: ใช้งานภายใน local scope ของฟังก์ชันนี้เท่านั้น
    macro_rules! time_it {
        ($name:expr, $code:block) => {{
            let start = std::time::Instant::now();  // ⏰ เริ่มจับเวลาเวทมนตร์!
            log!(info, "เริ่ม: {}", $name);          // 🚀 เวทมนตร์เริ่มงาน!
            let result = $code;                     // 🪄 ทำเวทมนตร์!
            let duration = start.elapsed();        // ⏱️ หยุดจับเวลาเวทมนตร์!
            log!(info, "เสร็จ: {} ใช้เวลา {:?}", $name, duration); // 🎉 เวทมนตร์เสร็จ!
            result
        }};
    }

    println!("\n⏰🧮 === เวทมนตร์จับเวลาการคำนวณ === 🧮⏰");
    let result = time_it!("การคำนวณซับซ้อน", {  // 🧮 เวทมนตร์คำนวณที่ซับซ้อน!
        let mut sum = 0;
        for i in 1..=1000 {
            sum += i * i;  // 🔢 เวทมนตร์ยกกำลังสอง!
        }
        sum
    });

    println!("\n🎉 ผลลัพธ์เวทมนตร์: {result} ✨");
}

/// ตัวอย่าง macro สำหรับ debugging - เวทมนตร์ตรวจสอบแบบ Debug Spells! 🐛🔮
pub fn debugging_macros() {
    println!("\n🐛✨ === ตัวอย่าง Debugging Macros - เวทมนตร์ตรวจสอบ! === ✨🐛");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยตรวจสอบและแก้ไขปัญหา! 🔍");

    // 🔍 Macro สำหรับ debug print ที่แสดงชื่อตัวแปรด้วย - เวทมนตร์ส่องตัวแปร! 🔮✨
    macro_rules! dbg_var {
        ($var:expr) => {
            println!("🔍✨ {} = {:?} 🪄", stringify!($var), $var);  // 🎯 เวทมนตร์แสดงค่าตัวแปร!
        };
    }

    // 📞 Macro สำหรับ trace function calls - เวทมนตร์ติดตามฟังก์ชัน! 🕵️‍♂️✨
    macro_rules! trace_fn {
        ($fn_name:expr, $($arg:expr),*) => {
            {
                println!("📞🪄 เรียกใช้ฟังก์ชันเวทมนตร์: {} ✨", $fn_name);  // 🎭 เวทมนตร์เรียกฟังก์ชัน!
                $(
                    println!("   📥🎯 อาร์กิวเมนต์เวทมนตร์: {:?}", $arg);  // 📦 เวทมนตร์แสดงพารามิเตอร์!
                )*
            }
        };
    }

    // 🧪 ทดสอบ debugging macros - ทดลองเวทมนตร์ตรวจสอบ! 🔬✨
    let x = 42;                           // 🔢 ตัวเลขเวทมนตร์
    let name = "Rust";                   // 📝 ชื่อเวทมนตร์
    let numbers = vec![1, 2, 3, 4, 5];   // 📊 รายการตัวเลขเวทมนตร์

    println!("\n🔍 === เวทมนตร์ส่องตัวแปร === 🔍");
    dbg_var!(x);        // 🔍 ส่องตัวเลข!
    dbg_var!(name);     // 🔍 ส่องชื่อ!
    dbg_var!(numbers);  // 🔍 ส่องรายการ!

    println!("\n📞 === เวทมนตร์ติดตามฟังก์ชัน === 📞");
    trace_fn!("calculate_sum", &numbers, "mode: fast");  // 🕵️‍♂️ ติดตามการเรียกฟังก์ชัน!
}

/// ตัวอย่าง macro สำหรับ conditional compilation - เวทมนตร์เงื่อนไขแบบ Conditional Spells! 🔀🔮
pub fn conditional_macros() {
    println!("\n🔀✨ === ตัวอย่าง Conditional Macros - เวทมนตร์เงื่อนไข! === ✨🔀");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ทำงานตามเงื่อนไข! 🎯");

    // 🏗️ Macro ที่ทำงานต่างกันตาม build configuration - เวทมนตร์โหมดการทำงาน! 🎭✨
    macro_rules! feature_flag {
        (development, $code:block) => {
            #[cfg(debug_assertions)]  // 🚧 เวทมนตร์โหมดพัฒนา!
            $code
        };
        (production, $code:block) => {
            #[cfg(not(debug_assertions))]  // 🚀 เวทมนตร์โหมดใช้งานจริง!
            $code
        };
    }

    println!("\n🏗️ === เวทมนตร์โหมดการทำงาน === 🏗️");
    feature_flag!(development, {  // 🚧 เวทมนตร์โหมดพัฒนา!
        println!("🚧✨ โหมด Development: แสดงข้อมูล debug 🔍");
        println!("   📊 Memory usage: 45MB");
        println!("   🔗 Active connections: 12");
        println!("   🪄 เวทมนตร์ debug กำลังทำงาน!");
    });

    feature_flag!(production, {  // 🚀 เวทมนตร์โหมดใช้งานจริง!
        println!("🚀✨ โหมด Production: ทำงานแบบ optimized 🎯");
        println!("   🪄 เวทมนตร์ประสิทธิภาพสูงกำลังทำงาน!");
    });

    // 💻 Macro สำหรับ platform-specific code - เวทมนตร์เฉพาะระบบปฏิบัติการ! 🖥️✨
    macro_rules! platform_specific {
        (windows, $code:block) => {
            #[cfg(target_os = "windows")]  // 🪟 เวทมนตร์ Windows!
            $code
        };
        (macos, $code:block) => {
            #[cfg(target_os = "macos")]    // 🍎 เวทมนตร์ macOS!
            $code
        };
        (linux, $code:block) => {
            #[cfg(target_os = "linux")]    // 🐧 เวทมนตร์ Linux!
            $code
        };
    }

    println!("\n💻 === เวทมนตร์เฉพาะระบบปฏิบัติการ === 💻");
    platform_specific!(windows, {  // 🪟 เวทมนตร์ Windows!
        println!("🪟✨ รันบน Windows - เวทมนตร์ Microsoft! 🎯");
    });

    platform_specific!(macos, {     // 🍎 เวทมนตร์ macOS!
        println!("🍎✨ รันบน macOS - เวทมนตร์ Apple! 🎯");
    });

    platform_specific!(linux, {    // 🐧 เวทมนตร์ Linux!
        println!("🐧✨ รันบน Linux - เวทมนตร์ Open Source! 🎯");
    });
    
    println!("\n🎉 เวทมนตร์ logging สำเร็จ! 🪄✨");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_macro() {
        macro_rules! log {
            (info, $($arg:tt)*) => {
                format!("[INFO] {}", format!($($arg)*))
            };
            (error, $($arg:tt)*) => {
                format!("[ERROR] {}", format!($($arg)*))
            };
        }

        let info_msg = log!(info, "Test message");
        let error_msg = log!(error, "Error: {}", "something went wrong");

        assert!(info_msg.contains("[INFO]"));
        assert!(error_msg.contains("[ERROR]"));
        assert!(error_msg.contains("something went wrong"));
    }

    #[test]
    fn test_dbg_var_macro() {
        macro_rules! dbg_var {
            ($var:expr) => {
                format!("{} = {:?}", stringify!($var), $var)
            };
        }

        let x = 42;
        let debug_output = dbg_var!(x);

        assert!(debug_output.contains("x = 42"));
    }

    #[test]
    fn test_logging_macros_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        logging_macros();
        debugging_macros();
        conditional_macros();
    }
}
