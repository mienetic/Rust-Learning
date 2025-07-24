//! # Basic Testing Examples - ห้องทดลองพื้นฐานนักสืบ! 🔬🕵️‍♂️
//!
//! ตัวอย่างการเขียน unit tests พื้นฐานใน Rust
//! เหมือนการเรียนรู้เครื่องมือพื้นฐานของนักสืบโค้ด! 🧪✨
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การใช้เครื่องมือสืบสวนพื้นฐาน!

/// ตัวอย่างฟังก์ชันที่จะใช้ทดสอบ - เครื่องคิดเลขนักสืบ! 🧮🔍
#[must_use]  // 🚨 ต้องใช้ผลลัพธ์! (ไม่ให้ทิ้งหลักฐาน!)
pub const fn add(a: i32, b: i32) -> i32 {
    a + b  // ➕ การบวกแบบง่ายๆ - หลักฐานชิ้นแรก!
}

/// ฟังก์ชันสำหรับหารเลข - เครื่องมือสืบสวนที่อันตราย! ⚠️🔍
///
/// # Errors
///
/// ฟังก์ชันนี้จะ return error เมื่อตัวหารเป็น 0 (เหมือนการหารด้วยศูนย์ในคดีสืบสวน!)
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())  // 🚨 หลักฐานผิดพลาด!
    } else {
        Ok(a / b)  // ✅ หลักฐานถูกต้อง!
    }
}

/// ตัวอย่างการใช้งาน basic testing - เริ่มการสืบสวนพื้นฐาน! 🔬🕵️‍♂️
pub fn basic_testing_examples() {
    println!("🔬✨ === Basic Testing Examples - ห้องทดลองพื้นฐาน! === ✨🔬");
    println!("🕵️‍♂️ เริ่มการสืบสวนด้วยเครื่องมือพื้นฐาน! 🧪");

    // 🧮 ทดสอบฟังก์ชันพื้นฐาน - ตรวจสอบหลักฐานแรก!
    println!("➕🔍 การบวก: 5 + 3 = {} (หลักฐานการคำนวณ!)", add(5, 3));

    // 🔍 ทดสอบการหาร - สืบสวนกรณีปกติ!
    match divide(10.0, 2.0) {
        Ok(result) => println!("➗✅ การหาร: 10 ÷ 2 = {result} (หลักฐานถูกต้อง!)"),
        Err(e) => println!("❌🚨 ข้อผิดพลาด: {e}"),
    }

    // 🚨 ทดสอบการหารด้วยศูนย์ - สืบสวนกรณีอันตราย!
    match divide(10.0, 0.0) {
        Ok(result) => println!("➗✅ การหาร: 10 ÷ 0 = {result} (ไม่น่าจะเกิดขึ้น!)"),
        Err(e) => println!("❌🔍 ข้อผิดพลาดที่คาดไว้: {e} (หลักฐานข้อผิดพลาด!)"),
    }
    
    println!("🎉 การสืบสวนพื้นฐานเสร็จสิ้น! ✨");
}

#[cfg(test)]  // 🧪 โหมดทดสอบ - เข้าสู่ห้องแล็บนักสืบ!
pub mod tests {
    use super::*;

    // 🧮 ทดสอบฟังก์ชันพื้นฐาน - ตรวจสอบหลักฐานการบวก!
    #[test]  // 🔬 การทดสอบที่ 1
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);        // ✅ หลักฐาน: 2+3=5
        assert_eq!(add(0, 0), 0);        // ✅ หลักฐาน: 0+0=0
        assert_eq!(add(100, 200), 300);  // ✅ หลักฐาน: 100+200=300
    }

    #[test]  // 🔬 การทดสอบที่ 2
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);  // ✅ หลักฐาน: (-2)+(-3)=-5
        assert_eq!(add(-5, 5), 0);    // ✅ หลักฐาน: (-5)+5=0
        assert_eq!(add(10, -3), 7);   // ✅ หลักฐาน: 10+(-3)=7
    }

    #[test]  // 🔬 การทดสอบที่ 3
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));  // ✅ หลักฐาน: 10÷2=5
        assert_eq!(divide(7.0, 2.0), Ok(3.5));   // ✅ หลักฐาน: 7÷2=3.5
        assert_eq!(divide(0.0, 5.0), Ok(0.0));   // ✅ หลักฐาน: 0÷5=0
    }

    #[test]  // 🔬 การทดสอบที่ 4
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());  // 🚨 ตรวจสอบว่าเกิด error
        assert_eq!(divide(5.0, 0.0), Err("Cannot divide by zero".to_string()));  // 🚨 ตรวจสอบข้อความ error
    }

    #[test]  // 🔬 การทดสอบที่ 5
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_result_with_question_mark() {
        // 🎭 ฟังก์ชันจำลองสถานการณ์ - เครื่องมือนักสืบ!
        fn operation_that_might_fail(should_fail: bool) -> Result<i32, String> {
            if should_fail {
                Err("Operation failed".to_string())  // 🚨 จำลองความล้มเหลว
            } else {
                Ok(42)  // ✅ จำลองความสำเร็จ
            }
        }

        assert_eq!(operation_that_might_fail(false), Ok(42));  // ✅ ทดสอบกรณีสำเร็จ
        assert!(operation_that_might_fail(true).is_err());     // 🚨 ทดสอบกรณีล้มเหลว
    }

    #[test]  // 🔬 การทดสอบที่ 6
    #[should_panic(expected = "This should panic")]  // 💥 คาดหวังให้ panic!
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_panic_example() {
        panic!("This should panic");  // 💥 ทดสอบการ panic - สถานการณ์วิกฤต!
    }

    #[test]  // 🔬 การทดสอบที่ 7
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_basic_testing_functions() {
        // 🧪 ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic - ตรวจสอบความปลอดภัย!
        basic_testing_examples();  // 🔍 เรียกใช้การสืบสวนพื้นฐาน
    }
}