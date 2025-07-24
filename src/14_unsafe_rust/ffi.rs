//! FFI (Foreign Function Interface) ใน Unsafe Rust - ปฏิบัติการนักสืบข้ามชาติ! 🕵️‍♂️🌍
//!
//! ไฟล์นี้ประกอบด้วย:
//! - การเรียกใช้ C functions - การติดต่อกับสายลับต่างชาติ! 📞🇨
//! - การส่งออก Rust functions ให้ C - การส่งสายลับไปต่างประเทศ! 🚀🦀
//! - ตัวอย่างการทำงานกับ C libraries - การร่วมมือกับองค์กรต่างชาติ! 🤝🌐

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// การเรียกใช้ C standard library functions - การติดต่อกับหน่วยงานต่างชาติ! 📞🇨
unsafe extern "C" {
    fn abs(input: i32) -> i32;    // 🔢 สายลับคำนวณค่าสัมบูรณ์
    fn sqrt(input: f64) -> f64;   // 📐 สายลับคำนวณรากที่สอง
}

/// ตัวอย่าง FFI พื้นฐาน - การฝึกปฏิบัติการข้ามชาติ! 🌍🕵️‍♂️
pub fn ffi_examples() {
    println!("\n🕵️‍♂️🌍 === International Detective Operations === 📞🤝");

    unsafe {
        println!("🔢 การติดต่อสายลับ C: abs(-42) = {}", abs(-42));      // 📞 เรียกสายลับต่างชาติ
        println!("📐 การติดต่อสายลับ C: sqrt(16.0) = {}", sqrt(16.0));  // 📞 เรียกสายลับคำนวณ
    }

    // ตัวอย่างการส่งออกฟังก์ชันให้ C - การส่งสายลับ Rust ไปต่างประเทศ! 🚀🦀
    println!(
        "🚀 สายลับ Rust ที่ส่งไปต่างประเทศ: rust_function(5) = {}",
        rust_function(5)  // 🦀 สายลับ Rust ปฏิบัติการ
    );

    println!("💡🌍 FFI ใช้สำหรับสร้างเครือข่ายสายลับข้ามชาติ!");  // 🤝 ความร่วมมือระหว่างประเทศ
}

/// ฟังก์ชัน Rust ที่ส่งออกให้ C - สายลับ Rust ปฏิบัติการต่างประเทศ! 🚀🦀
#[unsafe(no_mangle)]
pub const extern "C" fn rust_function(x: i32) -> i32 {
    x * 2  // 🔢 ภารกิจเพิ่มพลังเป็น 2 เท่า
}

/// ฟังก์ชันสำหรับการคำนวณที่ส่งออกให้ C - สายลับคำนวณ! 🧮🦀
#[unsafe(no_mangle)]
pub const extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b  // ➕ ภารกิจรวมข้อมูล
}

/// ฟังก์ชันสำหรับการทำงานกับ string ที่ส่งออกให้ C - สายลับถอดรหัสข้อความ! 📝🔍
#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub const extern "C" fn rust_string_length(s: *const std::os::raw::c_char) -> usize {
    if s.is_null() {
        return 0;  // 📭 ไม่มีข้อความลับ
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);  // 🔍 ถอดรหัสข้อความ
        c_str.to_bytes().len()                    // 📏 นับความยาวข้อความลับ
    }
}

/// ตัวอย่างการทำงานกับ C strings - การส่งข้อความลับข้ามชาติ! 📝🌍
/// 
/// # Panics
/// 
/// Panics if `CString::new` fails
#[allow(clippy::missing_panics_doc)]
pub fn c_string_examples() {
    println!("\n📝🕵️‍♂️ === Secret Message Exchange === 🌍📞");

    // สร้าง C string จาก Rust string - การเข้ารหัสข้อความลับ! 🔐📝
    let rust_string = "Hello from Rust!";  // 🦀 ข้อความลับจากสายลับ Rust
    let c_string = CString::new(rust_string).expect("CString::new failed");  // 🔐 เข้ารหัสข้อความ

    // ใช้งาน C string - การส่งข้อความลับไปยังสายลับ C! 📞🌍
    let c_ptr: *const c_char = c_string.as_ptr();  // 📡 ช่องทางสื่อสารลับ
    let length = rust_string_length(c_ptr);  // 📏 วัดความยาวข้อความลับ

    println!("📏🔍 ความยาวข้อความลับ: {length}");

    // แปลงกลับเป็น Rust string - การถอดรหัสข้อความลับ! 🔓📝
    unsafe {
        let c_string_ref = CStr::from_ptr(c_ptr);  // 🔍 ดักฟังข้อความ
        let back_to_rust = c_string_ref.to_str().expect("Invalid UTF-8");  // 🔓 ถอดรหัส
        println!("🔓🦀 ถอดรหัสข้อความลับสำเร็จ: {back_to_rust}");
    }
    
    println!("💡🔐 C strings ใช้ null terminator (\\0) เป็นสัญญาณจบข้อความลับ!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_functions() {  // 🧪 ทดสอบการติดต่อสายลับต่างชาติ
        unsafe extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            assert_eq!(abs(-42), 42);  // 🔢 ทดสอบสายลับคำนวณค่าสัมบูรณ์
            assert_eq!(abs(42), 42);   // 🔢 ทดสอบสายลับคำนวณค่าสัมบูรณ์
        }
    }

    #[test]
    fn test_rust_exported_functions() {  // 🧪 ทดสอบสายลับ Rust ที่ส่งออก
        assert_eq!(rust_function(5), 10);  // 🚀 ทดสอบภารกิจเพิ่มพลัง
        assert_eq!(rust_add(3, 4), 7);     // ➕ ทดสอบภารกิจรวมข้อมูล
    }

    #[test]
    fn test_c_string_length() {  // 🧪 ทดสอบการถอดรหัสข้อความลับ
        let test_string = CString::new("Hello").unwrap();  // 📝 สร้างข้อความลับ
        let length = rust_string_length(test_string.as_ptr());  // 📏 วัดความยาว
        assert_eq!(length, 5);  // ✅ ตรวจสอบความยาวข้อความลับ

        // ทดสอบกับ null pointer - การตรวจสอบข้อความว่าง! 📭🔍
        let null_length = rust_string_length(std::ptr::null());
        assert_eq!(null_length, 0);  // ✅ ไม่มีข้อความลับ
    }
}