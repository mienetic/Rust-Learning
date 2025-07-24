//! # Declarative Macros Examples - เวทมนตร์ประกาศแบบ Spell Casting! 🔮✨
//!
//! ตัวอย่างการใช้งาน `macro_rules`! สำหรับสร้าง declarative macros
//! เหมือนการเขียนคาถาเวทมนตร์ที่สามารถเรียกใช้ซ้ำได้! 🪄📜
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ประกาศที่ช่วยลดการเขียนโค้ดซ้ำ!

/// ตัวอย่าง declarative macros พื้นฐาน - เวทมนตร์ประกาศแบบ Basic Spells! 🔮
pub fn declarative_macros_examples() {
    println!("🔮✨ === ตัวอย่าง Declarative Macros - เวทมนตร์ประกาศ! === ✨🔮");
    println!("🪄 เรียนรู้การสร้างคาถาเวทมนตร์ด้วย macro_rules! 📜");

    // 🎭 Macro สำหรับสร้าง function ง่ายๆ - เวทมนตร์เรียกผี Function! 👻
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("🎪 คุณเรียกใช้ฟังก์ชันเวทมนตร์: {} 🪄", stringify!($func_name));
            }
        };
    }

    // 🪄 สร้างฟังก์ชันด้วย macro - เหมือนเรียกผีมาช่วยงาน! 👻✨
    create_function!(hello);    // 👋 เวทมนตร์ทักทาย
    create_function!(goodbye);  // 👋 เวทมนตร์ลาก่อน

    println!("\n🎭 === เรียกใช้เวทมนตร์ที่สร้างขึ้น! === 🎭");
    hello();    // 🎪 เรียกเวทมนตร์ทักทาย!
    goodbye();  // 🎪 เรียกเวทมนตร์ลาก่อน!

    // 🧮 Macro สำหรับคำนวณ - เวทมนตร์นักคณิตศาสตร์! 🔢✨
    macro_rules! calculate {
        ($x:expr, +, $y:expr) => {
            $x + $y  // ➕ เวทมนตร์บวก!
        };
        ($x:expr, -, $y:expr) => {
            $x - $y  // ➖ เวทมนตร์ลบ!
        };
        ($x:expr, *, $y:expr) => {
            $x * $y  // ✖️ เวทมนตร์คูณ!
        };
        ($x:expr, /, $y:expr) => {
            $x / $y  // ➗ เวทมนตร์หาร!
        };
    }

    println!("\n🧮🔢 === การคำนวณด้วยเวทมนตร์ === 🔢🧮");
    println!("   ➕ 5 + 3 = {} (เวทมนตร์บวก!)", calculate!(5, +, 3));
    println!("   ➖ 10 - 4 = {} (เวทมนตร์ลบ!)", calculate!(10, -, 4));
    println!("   ✖️ 6 * 7 = {} (เวทมนตร์คูณ!)", calculate!(6, *, 7));
    println!("   ➗ 15 / 3 = {} (เวทมนตร์หาร!)", calculate!(15, /, 3));
}

/// ตัวอย่าง macro ที่รับ arguments หลายแบบ - เวทมนตร์พารามิเตอร์ไม่จำกัด! 🌟🔮
pub fn variadic_macros_examples() {
    println!("\n🌟📝 === ตัวอย่าง Variadic Macros - เวทมนตร์พารามิเตอร์ไม่จำกัด! === 📝🌟");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่รับข้อมูลได้ไม่จำกัด! ✨");

    // 📦 Macro สำหรับสร้าง vector - เวทมนตร์รวบรวมสิ่งของ! 🎒✨
    macro_rules! vec_of {
        ($($x:expr),*) => {
            {
                vec![$($x),*]  // 🪄 เวทมนตร์รวบรวมทุกอย่างใส่กล่อง!
            }
        };
    }

    let numbers = vec_of![1, 2, 3, 4, 5];                    // 🔢 เวทมนตร์รวบรวมตัวเลข!
    let words = vec_of!["hello", "world", "rust"];          // 📝 เวทมนตร์รวบรวมคำ!

    println!("\n📦🔢 === ผลลัพธ์เวทมนตร์รวบรวม === 🔢📦");
    println!("📊 Vector ของตัวเลขเวทมนตร์: {numbers:?} 🎯");
    println!("📊 Vector ของคำเวทมนตร์: {words:?} 🎯");

    // 📜 Macro สำหรับ print หลายบรรทัด - เวทมนตร์พิมพ์หนังสือ! 📚✨
    macro_rules! print_lines {
        ($($line:expr),*) => {
            $(
                println!("📄✨ {}", $line);  // 🪄 เวทมนตร์พิมพ์แต่ละบรรทัด!
            )*
        };
    }

    println!("\n📜📚 === เวทมนตร์พิมพ์หนังสือ === 📚📜");
    print_lines!("📖 บรรทัดเวทมนตร์ที่ 1", "📖 บรรทัดเวทมนตร์ที่ 2", "📖 บรรทัดเวทมนตร์ที่ 3");
    println!("🎉 เวทมนตร์พิมพ์สำเร็จ! 🪄");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macro() {
        macro_rules! calculate {
            ($x:expr, +, $y:expr) => {
                $x + $y
            };
            ($x:expr, *, $y:expr) => {
                $x * $y
            };
        }

        assert_eq!(calculate!(5, +, 3), 8);
        assert_eq!(calculate!(4, *, 6), 24);
    }

    #[test]
    fn test_vec_macro() {
        macro_rules! vec_of {
            ($($x:expr),*) => {
                {
                    vec![$($x),*]
                }
            };
        }

        let numbers = vec_of![1, 2, 3];
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_declarative_macros() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        declarative_macros_examples();
        variadic_macros_examples();
    }
}