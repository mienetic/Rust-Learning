//! Error Handling Module - ศูนย์จัดการข้อผิดพลาดมหาเทพ! 🚨✨
//!
//! โมดูลนี้เป็นโรงเรียนนักดับเพลิงข้อผิดพลาด! 🚒 สอนเกี่ยวกับการจัดการข้อผิดพลาดใน Rust
//! รวมถึง Result (ผลลัพธ์), Option (ตัวเลือก), Panic (ตื่นตระหนก), และ Error Propagation (การส่งต่อข้อผิดพลาด)! 🎯

mod panic_and_propagation;
mod practice_error_handling;
mod result_and_option;

pub use panic_and_propagation::learn_panic_and_error_propagation;
pub use practice_error_handling::practice_error_handling;
pub use result_and_option::learn_result_and_option;

/// ฟังก์ชันสำหรับรันตัวอย่าง error handling (เรียกจาก main.rs) - ฝึกเป็นนักดับเพลิง! 🚒🎓
pub fn run_error_handling_examples() {
    println!("   📋 Result และ Option Types (ประเภทผลลัพธ์และตัวเลือก: เครื่องมือนักสืบ!)");
    learn_result_and_option();

    println!("\n   💥 Panic และ Error Propagation (การตื่นตระหนกและส่งต่อข้อผิดพลาด: ระบบแจ้งเตือนภัย!)");
    learn_panic_and_error_propagation();

    println!("\n   💪 แบบฝึกหัด Error Handling (ยิมฝึกจัดการข้อผิดพลาด!)");
    practice_error_handling();
}

#[cfg(test)]
mod tests {
    // ลบ unused import super::* เนื่องจากไม่ได้ใช้งาน (ทำความสะอาดโค้ด! 🧹)

    #[test]
    fn test_option_some() {
        let text = "Hello Rust";
        let result = text.find("Rust");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_option_none() {
        let text = "Hello World";
        let result = text.find("Rust");
        assert!(result.is_none());
    }

    #[test]
    fn test_result_ok() {
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err(String::from("Division by zero"))  // หารด้วยศูนย์! 🚫
            } else {
                Ok(a / b)  // หารได้! ✅
            }
        }

        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_result_err() {
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err(String::from("Division by zero"))  // หารด้วยศูนย์! 🚫
            } else {
                Ok(a / b)  // หารได้! ✅
            }
        }

        let result = divide(10.0, 0.0);
        assert!(result.is_err());
    }
}
