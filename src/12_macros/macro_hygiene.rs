//! # Macro Hygiene and Advanced Features - เวทมนตร์ความสะอาดแบบ Clean Magic! 🧹✨
//!
//! ตัวอย่างการทำงานของ macro hygiene และ features ขั้นสูง
//! เหมือนการเรียนรู้ศิลปะการรักษาความสะอาดของเวทมนตร์! 🪄🧼
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ที่สะอาดและปลอดภัย!

/// ตัวอย่าง macro hygiene - เวทมนตร์ความสะอาดแบบ Hygiene Spells! 🧹🔮
pub fn macro_hygiene_examples() {
    println!("\n🧼✨ === ตัวอย่าง Macro Hygiene - เวทมนตร์ความสะอาด! === ✨🧼");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ไม่ทำให้ตัวแปรชนกัน! 🛡️");

    // 🧹 Macro ที่ใช้ตัวแปรภายใน - เวทมนตร์ป้องกันการชนกันของตัวแปร! 🛡️✨
    macro_rules! using_internal_var {
        ($x:expr) => {{
            let temp = $x;  // 🔒 ตัวแปรภายในเวทมนตร์ (ปลอดภัยจากการชน!)
            temp * 2        // 🧮 คำนวณด้วยเวทมนตร์!
        }};
    }

    let temp = 5;                        // 🏠 ตัวแปรภายนอกเวทมนตร์
    let result = using_internal_var!(10); // 🪄 เรียกใช้เวทมนตร์!

    println!("\n🔍 === ผลการทดสอบเวทมนตร์ความสะอาด === 🔍");
    println!("🏠 ตัวแปร temp ภายนอก: {temp}");      // ยังคงเป็น 5!
    println!("🪄 ผลลัพธ์จาก macro: {result}");      // เป็น 20!
    println!("✅🛡️ Macro hygiene ทำงานถูกต้อง - ตัวแปรไม่ชนกัน! เวทมนตร์ปลอดภัย! 🎉");

    // 🆕 Macro ที่สร้าง identifier ใหม่ - เวทมนตร์สร้างตัวแปรใหม่! 🎭✨
    macro_rules! create_var {
        ($var_name:ident, $value:expr) => {
            let $var_name = $value;  // 🎯 สร้างตัวแปรใหม่ด้วยเวทมนตร์!
        };
    }

    println!("\n🆕 === เวทมนตร์สร้างตัวแปรใหม่ === 🆕");
    create_var!(my_number, 42);           // 🔢 สร้างตัวเลขเวทมนตร์!
    create_var!(my_string, "Hello Rust"); // 📝 สร้างข้อความเวทมนตร์!

    println!("🎭 ตัวแปรที่สร้างจาก macro:");
    println!("   🔢 my_number = {my_number}");  // ตัวเลขที่เกิดจากเวทมนตร์!
    println!("   📝 my_string = {my_string}");  // ข้อความที่เกิดจากเวทมนตร์!
}

/// ตัวอย่าง macro ขั้นสูง - เวทมนตร์ขั้นสูงแบบ Advanced Wizardry! 🧙‍♂️🔮
pub fn advanced_macro_examples() {
    println!("\n🚀✨ === ตัวอย่าง Advanced Macros - เวทมนตร์ขั้นสูง! === ✨🚀");
    println!("🧙‍♂️ เรียนรู้เวทมนตร์ระดับมาสเตอร์! 🎓");

    // 🎯 Macro ที่ใช้ nested patterns - เวทมนตร์จับรูปแบบแบบ Pattern Wizard! 🔍✨
    macro_rules! nested_match {
        ($expr:expr; $($pattern:pat => $result:expr),* $(,)?) => {
            match $expr {
                $(
                    $pattern => {
                        println!("🎯✨ Pattern matched: {} 🪄", stringify!($pattern));  // 🎭 เวทมนตร์จับรูปแบบสำเร็จ!
                        $result
                    }
                )*
            }
        };
    }

    println!("\n🎯 === เวทมนตร์จับรูปแบบ === 🎯");
    let value = 42;  // 🔢 ค่าที่จะทดสอบเวทมนตร์!
    let result = nested_match! {
        value;
        0 => "zero",        // 🚫 ศูนย์!
        1..=10 => "small",   // 🤏 เล็ก!
        11..=50 => "medium", // 📏 กลาง!
        _ => "large"         // 🦣 ใหญ่!
    };

    println!("📊🎉 ผลลัพธ์เวทมนตร์: {result} ✨");

    // 🏗️ Macro ที่สร้าง multiple items - เวทมนตร์สร้างหลายสิ่งพร้อมกัน! 🎭✨
    macro_rules! create_enum_and_impl {
        ($enum_name:ident { $($variant:ident),* }) => {
            #[derive(Debug, PartialEq)]  // 🎯 เวทมนตร์ derive!
            enum $enum_name {
                $($variant,)*            // 🎨 สร้างตัวเลือกทั้งหมด!
            }

            impl $enum_name {            // 🛠️ เวทมนตร์ implementation!
                fn all_variants() -> Vec<Self> {
                    vec![
                        $(Self::$variant,)*  // 📦 รวบรวมตัวเลือกทั้งหมด!
                    ]
                }

                fn variant_count() -> usize {
                    Self::all_variants().len()  // 🔢 นับจำนวนตัวเลือก!
                }
            }
        };
    }

    println!("\n🎨 === เวทมนตร์สร้าง Enum และ Implementation === 🎨");
    create_enum_and_impl!(Color {  // 🎨 เวทมนตร์สร้างสี!
        Red,     // ❤️ แดง!
        Green,   // 💚 เขียว!
        Blue,    // 💙 น้ำเงิน!
        Yellow   // 💛 เหลือง!
    });

    println!("🌈 สีทั้งหมดจากเวทมนตร์: {:?}", Color::all_variants());
    println!("🔢✨ จำนวนสี: {} สี! 🎨", Color::variant_count());
}

/// ตัวอย่าง macro recursion - เวทมนตร์เรียกตัวเองแบบ Recursive Spells! 🔄🔮
pub fn recursive_macro_examples() {
    println!("\n🔄✨ === ตัวอย่าง Recursive Macros - เวทมนตร์เรียกตัวเอง! === ✨🔄");
    println!("🪄 เรียนรู้เวทมนตร์ที่เรียกตัวเองแบบ infinite magic! ♾️");

    // 🔗 Macro สำหรับสร้าง nested tuples - เวทมนตร์สร้างทูเปิลซ้อนกัน! 📦✨
    macro_rules! nested_tuple {
        ($single:expr) => {
            $single  // 🎯 กรณีฐาน - หยุดเวทมนตร์!
        };
        ($first:expr, $($rest:expr),+) => {
            ($first, nested_tuple!($($rest),+))  // 🔄 เรียกตัวเองแบบ recursive magic!
        };
    }

    println!("\n🔗 === เวทมนตร์สร้างทูเปิลซ้อนกัน === 🔗");
    let tuple = nested_tuple!(1, 2, 3, 4, 5);  // 🪄 เวทมนตร์สร้างทูเปิลซ้อน!
    println!("📦✨ Nested tuple จากเวทมนตร์: {tuple:?} 🎉");

    // 🧮 Macro สำหรับคำนวณ factorial ที่ compile time - เวทมนตร์คำนวณแฟกทอเรียล! 🔢✨
    // ⚠️ Warning: macro `factorial` จะแสดง warning "unused macro definition" (เป็นเรื่องปกติของเวทมนตร์!)
    // 🎭 เหตุผล: macro นี้สร้างเพื่อสาธิตแนวคิด recursive macro เท่านั้น
    // 🔮 ไม่ได้เรียกใช้จริงเพราะ Rust ไม่รองรับ recursive macro expansion ที่ runtime
    // 📚 เก็บไว้เพื่อแสดงตัวอย่างการเขียน recursive macro pattern

    #[allow(unused_macros)]  // 🤫 ปิดเสียง warning สำหรับเวทมนตร์นี้!
    macro_rules! factorial {
        (0) => {
            1  // 🎯 กรณีฐาน: 0! = 1
        };
        (1) => {
            1  // 🎯 กรณีฐาน: 1! = 1
        };
        ($n:expr) => {
            $n * factorial!($n - 1)  // 🔄 เรียกตัวเองแบบ recursive!
        };
    }

    println!("\n🧮 === เวทมนตร์คำนวณแฟกทอเรียล === 🧮");
    // 📝 Note: นี่เป็นตัวอย่างแนวคิด - ใน Rust จริงจะต้องใช้ const fn
    println!("🔢✨ Factorial concept (5! = 120) - เวทมนตร์คำนวณแนวคิด! 🎓");

    // ⛓️ Macro สำหรับสร้าง chain of function calls - เวทมนตร์เชื่อมต่อฟังก์ชัน! 🔗✨
    macro_rules! chain_calls {
        ($value:expr) => {
            $value  // 🎯 กรณีฐาน - หยุดเชื่อมต่อ!
        };
        ($value:expr, $method:ident $(, $($args:expr),*)?) => {
            $value.$method($($($args),*)?)  // 🪄 เรียกเมธอดเดียว!
        };
        ($value:expr, $method:ident $(, $($args:expr),*)?, $($rest:tt)*) => {
            chain_calls!($value.$method($($($args),*)?), $($rest)*)  // 🔄 เชื่อมต่อต่อไป!
        };
    }

    println!("\n⛓️ === เวทมนตร์เชื่อมต่อฟังก์ชัน === ⛓️");
    let text = "  hello world  ";  // 📝 ข้อความต้นฉบับ
    let result = text.trim().to_uppercase().replace("WORLD", "RUST");  // 🔗 เชื่อมต่อแบบปกติ
    println!("⛓️✨ Chain result: {result} 🪄");

    // 🔗 ตัวอย่างการใช้ chain_calls macro แบบง่าย - เวทมนตร์เชื่อมต่อง่ายๆ!
    let simple_result = chain_calls!(text, trim);  // 🪄 เวทมนตร์ตัดช่องว่าง!
    println!("🔗🎯 Simple chain: '{simple_result}' ✨");
    
    println!("\n🎉 เวทมนตร์ความสะอาดสำเร็จ! 🧹✨");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_hygiene() {
        macro_rules! using_internal_var {
            ($x:expr) => {{
                let temp = $x;
                temp * 2
            }};
        }

        let temp = 5;
        let result = using_internal_var!(10);

        // ตัวแปร temp ภายนอกไม่ถูกเปลี่ยนแปลง
        assert_eq!(temp, 5);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_create_var_macro() {
        macro_rules! create_var {
            ($var_name:ident, $value:expr) => {
                let $var_name = $value;
            };
        }

        create_var!(test_var, 42);
        assert_eq!(test_var, 42);
    }

    #[test]
    fn test_nested_tuple_macro() {
        macro_rules! nested_tuple {
            ($single:expr) => {
                $single
            };
            ($first:expr, $($rest:expr),+) => {
                ($first, nested_tuple!($($rest),+))
            };
        }

        let tuple = nested_tuple!(1, 2, 3);
        assert_eq!(tuple, (1, (2, 3)));
    }

    #[test]
    fn test_macro_hygiene_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        macro_hygiene_examples();
        advanced_macro_examples();
        recursive_macro_examples();
    }
}