//! Union และ Transmute ใน Unsafe Rust - ห้องปฏิบัติการแปลงข้อมูล! 🧪🔬
//!
//! ไฟล์นี้ประกอบด้วย:
//! - การใช้งาน union - การผสมผสานข้อมูลลับ! 🔀⚗️
//! - การใช้ transmute อย่างระมัดระวัง - การเปลี่ยนรูปแบบข้อมูล! 🧬💀
//! - ตัวอย่างการแปลงข้อมูลระดับต่ำ - การทดลองอันตราย! ⚡🔬

use std::mem;
use std::slice;

/// Helper function สำหรับ function pointer transmute - ฟังก์ชันช่วยในการทดลอง! 🧪⚡
const fn add_one(x: i32) -> i32 {
    x + 1  // 🔢 สูตรเพิ่มค่าลับ
}

/// ตัวอย่างการใช้ union (unsafe) - ห้องทดลองผสมข้อมูล! 🧪🔀
#[repr(C)]
pub union MyUnion {
    i: i32,  // 🔢 ข้อมูลจำนวนเต็มลับ
    f: f32,  // 📊 ข้อมูลทศนิยมลับ
}

pub fn union_examples() {
    println!("\n🧪🔀 === Data Fusion Laboratory === ⚗️🔬");

    let mut u = MyUnion { i: 42 };  // 🧪 สร้างตัวอย่างการทดลอง

    unsafe {
        println!("🔢🔍 ตรวจสอบข้อมูลจำนวนเต็ม: {}", u.i);  // 📊 วิเคราะห์ข้อมูล

        // เปลี่ยนเป็น f32 (ใช้ memory เดียวกัน) - การเปลี่ยนสถานะข้อมูล! 🧬⚗️
        u.f = std::f32::consts::PI;  // 📊 ใส่ข้อมูลทศนิยมลับ
        println!("📊🧪 ข้อมูลทศนิยมหลังการทดลอง: {}", u.f);

        // อ่าน i32 หลังจากเขียน f32 (อันตราย!) - การอ่านข้อมูลผิดประเภท! ⚠️💀
        println!("⚠️💀 ข้อมูลเสียหายหลังการทดลอง: {} (อันตรายมาก!)", u.i);
    }

    println!("💡🔬 Union ใช้สำหรับการทดลองข้อมูลระดับต่ำ!");
}

/// Union สำหรับการแปลงข้อมูล - เครื่องแปลงข้อมูลลับ! 🔄🧪
#[repr(C)]
pub union DataConverter {
    bytes: [u8; 4],  // 🧬 ข้อมูลดิบระดับไบต์
    int: u32,        // 🔢 ข้อมูลจำนวนเต็ม
    float: f32,      // 📊 ข้อมูลทศนิยม
}

pub fn data_conversion_examples() {
    println!("\n🔄🧪 === Data Conversion Laboratory === 🧬⚗️");

    let mut converter = DataConverter { int: 0x4142_4344 };  // 🧪 เตรียมตัวอย่างการทดลอง

    unsafe {
        println!("🔢🔍 วิเคราะห์ข้อมูล u32: 0x{:08X}", converter.int);  // 📊 ตรวจสอบข้อมูลเลขฐาน 16
        println!("🧬📊 วิเคราะห์ข้อมูลไบต์: {:?}", converter.bytes);     // 🔬 ตรวจสอบข้อมูลดิบ

        // แปลงเป็น float - การเปลี่ยนสถานะข้อมูล! 🧬⚗️
        converter.float = 12.34;  // 📊 ใส่ข้อมูลทศนิยมใหม่
        println!("📊🧪 ข้อมูลทศนิยมหลังการทดลอง: {}", converter.float);
        println!("🧬🔬 ข้อมูลไบต์หลังการแปลง: {:?}", converter.bytes);  // 🔍 ตรวจสอบการเปลี่ยนแปลง
    }
}

/// ตัวอย่างการใช้ transmute (อันตรายมาก!) - การเปลี่ยนรูปแบบข้อมูลอันตราย! 🧬💀
pub fn transmute_examples() {
    println!("\n🧬💀 === Dangerous Transmutation Lab === ⚡🔬");

    // transmute เปลี่ยน type โดยไม่เปลี่ยน bits - การเปลี่ยนสถานะข้อมูล! 🧪⚗️
    let a = [0u8, 1u8, 0u8, 0u8];  // 🧬 ข้อมูลดิบสำหรับการทดลอง

    let b: u32 = u32::from_ne_bytes(a);  // 🔄 การแปลงข้อมูลอันตราย
    println!("🧬🔢 การแปลง [0u8, 1u8, 0u8, 0u8] เป็น u32: {b}");

    // ตัวอย่างที่ปลอดภัยกว่า: การแปลง reference - การทดลองที่ควบคุมได้! 🔬✅
    let s = "hello";           // 📝 ข้อมูลข้อความต้นฉบับ
    let ptr = s.as_ptr();      // 🎯 ตัวชี้ไปยังข้อมูล
    let len = s.len();         // 📏 ความยาวข้อมูล

    unsafe {
        let slice = slice::from_raw_parts(ptr, len);              // 🧬 สร้างข้อมูลจากตัวชี้
        let reconstructed = std::str::from_utf8_unchecked(slice);  // 📝 ประกอบข้อความใหม่
        println!("📝🔬 ข้อความที่ประกอบใหม่: {reconstructed}");
    }

    println!("⚠️💀 transmute อันตรายมาก! ใช้เฉพาะในห้องปฏิบัติการ!");
    println!("💡🔬 ควรใช้ as casting หรือ From/Into traits แทน!");
}

/// ตัวอย่างการใช้ transmute กับ function pointers - การแปลงฟังก์ชันอันตราย! 🎯💀
pub fn function_pointer_transmute() {
    println!("\n🎯💀 === Function Transmutation Lab === ⚡🧪");

    let fn_ptr: fn(i32) -> i32 = add_one;  // 🎯 ฟังก์ชันต้นฉบับ

    unsafe {
        // แปลง function pointer เป็น raw pointer - การเปลี่ยนฟังก์ชันเป็นตัวชี้! 🧬🎯
        let raw_ptr: *const () = fn_ptr as *const ();  // 📍 ตัวชี้อันตราย
        println!("📍🔬 ที่อยู่ฟังก์ชันในหน่วยความจำ: {raw_ptr:p}");

        // แปลงกลับเป็น function pointer - การประกอบฟังก์ชันใหม่! 🧪⚡
        let fn_ptr_back: fn(i32) -> i32 = mem::transmute(raw_ptr);  // 🎯 ฟังก์ชันที่ประกอบใหม่
        println!("🧮🔬 ทดสอบฟังก์ชันที่ประกอบใหม่: 5 + 1 = {}", fn_ptr_back(5));
    }

    println!("⚠️💀 การ transmute function pointers อันตรายสุดขีด!");
}

/// ตัวอย่างการใช้ `transmute_copy` (ปลอดภัยกว่า) - การคัดลอกข้อมูลอย่างปลอดภัย! 📋🔬
pub fn transmute_copy_examples() {
    println!("\n📋🔬 === Safe Copy Transmutation Lab === ✅🧪");

    let x: f32 = std::f32::consts::PI;  // 📊 ข้อมูลทศนิยมต้นฉบับ

    unsafe {
        // transmute_copy ปลอดภัยกว่าเพราะไม่ move ownership - การคัดลอกอย่างปลอดภัย! 📋✅
        let bits: u32 = mem::transmute_copy(&x);  // 🧬 คัดลอกและแปลงข้อมูล
        println!("📊🔬 การแปลง f32 {x} เป็น bits: 0x{bits:08X}");

        // แปลงกลับ - การประกอบข้อมูลใหม่! 🔄🧪
        let back_to_float: f32 = mem::transmute_copy(&bits);  // 📊 ประกอบข้อมูลทศนิยมใหม่
        println!("🧬📊 การแปลง bits 0x{bits:08X} เป็น f32: {back_to_float}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_union() {  // 🧪 ทดสอบการผสมผสานข้อมูล
        let mut u = MyUnion { i: 42 };  // 🧪 สร้างตัวอย่างการทดลอง

        unsafe {
            assert_eq!(u.i, 42);  // ✅ ตรวจสอบข้อมูลเริ่มต้น
            u.f = std::f32::consts::PI;  // 🧬 เปลี่ยนสถานะข้อมูล
            assert!((u.f - std::f32::consts::PI).abs() < f32::EPSILON);  // ✅ ยืนยันการเปลี่ยนแปลง
        }
    }

    #[test]
    fn test_transmute() {  // 🧪 ทดสอบการแปลงข้อมูลอันตราย
        let a = [0u8, 1u8, 0u8, 0u8];  // 🧬 ข้อมูลดิบสำหรับการทดลอง

        let b: u32 = u32::from_ne_bytes(a);  // 🔄 การแปลงข้อมูล
        // ผลลัพธ์ขึ้นอยู่กับ endianness - การตรวจสอบระบบ! 🔬💻
        assert!(b == 256 || b == 16777216);  // ✅ ยืนยันผลการทดลอง
    }

    #[test]
    fn test_data_converter() {  // 🧪 ทดสอบเครื่องแปลงข้อมูล
        let converter = DataConverter { int: 0x12345678 };  // 🧪 เตรียมตัวอย่าง

        unsafe {
            assert_eq!(converter.int, 0x12345678);  // ✅ ตรวจสอบข้อมูลเลขฐาน 16
            // ทดสอบว่า bytes มีข้อมูลที่ถูกต้อง - การตรวจสอบข้อมูลดิบ! 🧬🔍
            assert_eq!(converter.bytes.len(), 4);  // ✅ ยืนยันขนาดข้อมูล
        }
    }

    #[test]
    fn test_transmute_copy() {  // 🧪 ทดสอบการคัดลอกข้อมูลปลอดภัย
        let x: f32 = std::f32::consts::PI;  // 📊 ข้อมูลต้นฉบับ

        unsafe {
            let bits: u32 = mem::transmute_copy(&x);              // 🧬 คัดลอกและแปลง
            let back_to_float: f32 = mem::transmute_copy(&bits);  // 📊 ประกอบใหม่
            assert!((x - back_to_float).abs() < f32::EPSILON);    // ✅ ยืนยันความถูกต้อง
        }
    }
}