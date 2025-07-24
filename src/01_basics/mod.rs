//! Basics Module - พื้นฐานของ Rust
//!
//! โมดูลนี้รวบรวมการเรียนรู้พื้นฐานของ Rust
//! แบ่งออกเป็นหมวดหมู่ตามประเภทของเนื้อหา

// Module declarations
mod comments;
mod compound_types;
mod constants_shadowing;
mod data_types;
mod operators;
mod practice_basics;
mod type_conversion;
mod variables;

// Re-exports
pub use comments::learn_comments_and_documentation;
pub use compound_types::learn_compound_types;
pub use constants_shadowing::learn_constants_and_shadowing;
pub use data_types::learn_data_types;
pub use operators::learn_operators;
pub use practice_basics::practice_basics;
pub use type_conversion::learn_type_conversion;
pub use variables::learn_variables;

// ฟังก์ชันหลักสำหรับเรียนรู้พื้นฐาน Rust (ถูกรวมเข้าใน run_basics_examples แล้ว)
// pub fn learn_basics() {
//     learn_variables();
//     learn_data_types();
//     learn_compound_types();
//     learn_constants_and_shadowing();
//     learn_type_conversion();
//     learn_operators();
//     learn_comments_and_documentation();
// }

/// ฟังก์ชันสำหรับรันตัวอย่างพื้นฐาน (เรียกจาก main.rs)
pub fn run_basics_examples() {
    println!("   📝 Variables และ Mutability");
    learn_variables();

    println!("\n   🔢 Data Types");
    learn_data_types();

    println!("\n   📦 Compound Types (Array, Tuple)");
    learn_compound_types();

    println!("\n   🔒 Constants และ Shadowing");
    learn_constants_and_shadowing();

    println!("\n   🔄 Type Conversion");
    learn_type_conversion();

    println!("\n   ⚡ Operators");
    learn_operators();

    println!("\n   💬 Comments และ Documentation");
    learn_comments_and_documentation();

    println!("\n   💪 แบบฝึกหัดพื้นฐาน");
    practice_basics();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables_work() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        learn_variables();
        learn_data_types();
        learn_compound_types();
        practice_basics();
    }

    #[test]
    fn test_data_types() {
        let age: u8 = 25;
        let temperature: i32 = -10;
        let pi: f64 = 3.14159;

        assert_eq!(age, 25);
        assert_eq!(temperature, -10);
        assert!((pi - 3.14159).abs() < f64::EPSILON);
    }

    #[test]
    fn test_compound_types() {
        let numbers = [1, 2, 3, 4, 5];
        let coordinates = (10.5, 20.3);

        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers[0], 1);
        assert_eq!(coordinates.0, 10.5);
        assert_eq!(coordinates.1, 20.3);
    }
}
