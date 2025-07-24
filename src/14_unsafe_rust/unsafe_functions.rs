//! Unsafe Functions ใน Unsafe Rust - ปฏิบัติการนักสืบอันตราย! 🕵️‍♂️💀
//!
//! ไฟล์นี้ประกอบด้วย:
//! - การสร้างและเรียกใช้ unsafe functions - ภารกิจลับอันตราย! 🎯⚡
//! - การใช้ unsafe functions จาก standard library - เครื่องมือลับจากคลัง! 🛠️💀
//! - ตัวอย่างการใช้งานที่ถูกต้อง - การปฏิบัติการอย่างระมัดระวัง! 🛡️🔍

use std::slice;

// ฟังก์ชัน unsafe ที่เราสร้างเอง - ภารกิจลับสุดอันตราย! 💀⚡
unsafe fn dangerous_function() {
    println!("💀⚡ นี่คือภารกิจลับสุดอันตราย! 🕵️‍♂️💥");  // 🚨 การปฏิบัติการลับ
}

/// ตัวอย่างการใช้ unsafe functions - การฝึกปฏิบัติการอันตราย! 🎯💀
pub fn unsafe_functions_examples() {
    println!("\n🕵️‍♂️💀 === Dangerous Detective Operations Training === ⚡🎯");

    // การเรียกใช้ unsafe function ต้องอยู่ใน unsafe block - เขตปฏิบัติการอันตราย! ⚠️💀
    unsafe {
        dangerous_function();  // 🎯 เริ่มภารกิจลับ
    }

    // ตัวอย่างการใช้ slice::from_raw_parts - การสร้างหลักฐานปลอม! 📋⚡
    let mut v = vec![1, 2, 3, 4, 5];        // 📦 หลักฐานต้นฉบับ
    let ptr = v.as_mut_ptr();                // ⚡ เครื่องมือแก้ไข
    let len = v.len();                       // 📏 ขนาดหลักฐาน

    unsafe {
        // สร้าง slice จาก raw pointer - การปลอมแปลงหลักฐาน! 📋💀
        let slice = slice::from_raw_parts_mut(ptr, len);  // 🔧 สร้างหลักฐานปลอม
        slice[0] = 100;                      // ✏️ แก้ไขหลักฐาน
        println!("📋 หลักฐานปลอมที่สร้างขึ้น: {slice:?}");  // 📊 ผลการปลอมแปลง
    }

    println!("📋 หลักฐานต้นฉบับหลังถูกแก้ไข: {v:?}");  // 📋 ผลลัพธ์สุดท้าย
}

/// ฟังก์ชัน unsafe ตัวอย่าง - ภารกิจเพิ่มพลังหลักฐาน! ⚡🔢
#[allow(clippy::missing_safety_doc)]
pub unsafe fn multiply_by_two(ptr: *mut i32) {
    if !ptr.is_null() {                     // 🔍 ตรวจสอบเครื่องมือ
        unsafe {
            *ptr *= 2;                       // ⚡ เพิ่มพลังหลักฐานเป็น 2 เท่า
        }
    }
}

/// ฟังก์ชันที่ wrap unsafe operation - การปฏิบัติการอย่างปลอดภัย! 🛡️⚡
pub fn safe_multiply_by_two(value: &mut i32) {
    unsafe {
        multiply_by_two(std::ptr::from_mut::<i32>(value));  // 🛡️ ปฏิบัติการปลอดภัย
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;

    #[test]
    fn test_slice_from_raw_parts() {
        let v = [1, 2, 3, 4, 5];
        let ptr = v.as_ptr();
        let len = v.len();

        unsafe {
            let slice = slice::from_raw_parts(ptr, len);
            assert_eq!(slice, &[1, 2, 3, 4, 5]);
        }
    }

    #[test]
    fn test_safe_multiply_by_two() {
        let mut value = 21;
        safe_multiply_by_two(&mut value);
        assert_eq!(value, 42);
    }
}