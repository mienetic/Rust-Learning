//! Raw Pointers ใน Unsafe Rust - ห้องทดลองนักสืบหน่วยความจำอันตราย! 🕵️‍♂️⚡
//!
//! ไฟล์นี้ประกอบด้วย:
//! - การสร้างและใช้งาน raw pointers - เครื่องมือสืบสวนอันตราย! 🔍⚠️
//! - การแปลงระหว่าง references และ raw pointers - การแปลงหลักฐาน! 🔄📍
//! - ตัวอย่างการใช้งานที่ปลอดภัย - การสืบสวนอย่างระมัดระวัง! 🛡️🔍

use std::ptr;

/// ตัวอย่างการใช้ raw pointers - การสืบสวนด้วยเครื่องมืออันตราย! 🔍⚡
pub fn raw_pointers_examples() {
    println!("🕵️‍♂️⚡ === Dangerous Memory Detective Lab === 🔍💀");

    let mut num = 5;  // 🎯 เป้าหมายการสืบสวน

    // สร้าง raw pointers - เครื่องมือสืบสวนอันตราย! ⚡🔍
    let r1 = &raw const num; // 👁️ เครื่องมือดูอย่างเดียว
    let r2 = &raw mut num;   // ✏️ เครื่องมือแก้ไขอันตราย

    println!("📍 ที่อยู่หลักฐาน: {r1:p}");  // 🗺️ แผนที่หน่วยความจำ
    println!("📊 ค่าหลักฐาน: {num}");        // 📋 ข้อมูลเริ่มต้น

    // การใช้ raw pointers ต้องอยู่ใน unsafe block - เขตอันตราย! ⚠️💀
    unsafe {
        println!("🔍 สืบสวนผ่านเครื่องมืออันตราย: {}", *r1);  // 👁️ การสอดแนม

        // เปลี่ยนค่าผ่าน mutable raw pointer - การปลอมแปลงหลักฐาน! ✏️⚡
        *r2 = 10;
        println!("✏️ ปลอมแปลงหลักฐานเป็น: {}", *r2);  // 🔧 การแก้ไขอันตราย
    }

    println!("📊 หลักฐานหลังปลอมแปลง: {num}");  // 📋 ผลลัพธ์การสืบสวน

    // ตัวอย่างการสร้าง raw pointer จากที่อยู่ - การปลอมแปลงพิกัด! 🗺️💀
    let address = 0x0001_2345_usize;  // 🎯 พิกัดปลอม
    #[allow(clippy::no_effect_underscore_binding)]
    let _r = address as *const i32;   // ⚡ เครื่องมือปลอมแปลง

    println!("⚠️💀 คำเตือน: การใช้เครื่องมือปลอมแปลงพิกัดอันตรายมาก!");  // 🚨 เตือนภัย
}

/// ตัวอย่างการจัดการ memory ด้วย unsafe - การจัดการคลังหลักฐานอันตราย! 🧠⚡
pub fn memory_management_examples() {
    println!("\n🧠💀 === Dangerous Evidence Vault Management === ⚡🗃️");

    // การ allocate memory ด้วย Box::into_raw - การสร้างคลังหลักฐานลับ! 📦⚡
    let x = Box::new(42);                    // 📦 หลักฐานในกล่อง
    let raw_ptr = Box::into_raw(x);          // ⚡ แปลงเป็นเครื่องมืออันตราย

    unsafe {
        println!("📦 หลักฐานในคลังลับ: {}", *raw_ptr);  // 🔍 สืบสวนคลังลับ

        // ต้อง deallocate memory ด้วยตัวเอง - ทำลายหลักฐาน! 💥🗑️
        let _x = Box::from_raw(raw_ptr);     // 🔄 กู้คืนเครื่องมือ
        // _x จะถูก drop เมื่อออกจาก scope - การทำลายอัตโนมัติ! 💥
    }

    // ตัวอย่างการใช้ std::ptr functions - เครื่องมือจัดการหลักฐานขั้นสูง! 🛠️⚡
    let mut data = [1, 2, 3, 4, 5];         // 📋 ชุดหลักฐาน
    let ptr = data.as_mut_ptr();             // ⚡ เครื่องมือแก้ไขหลักฐาน

    unsafe {
        // คัดลอกข้อมูล - การปลอมแปลงหลักฐาน! 📋🔄
        ptr::copy(ptr, ptr.add(1), 2);       // 🔄 คัดลอกหลักฐาน
        println!("📋 หลังปลอมแปลง: {data:?}");  // 📊 ผลการปลอมแปลง

        // เขียนค่าใหม่ - การแก้ไขหลักฐาน! ✏️💀
        ptr::write(ptr.add(4), 99);          // ✏️ แก้ไขหลักฐาน
        println!("📋 หลังแก้ไขหลักฐาน: {data:?}");  // 📊 หลักฐานที่ถูกแก้ไข
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_raw_pointers() {
        let mut num = 5;
        let r1 = &raw const num;
        let r2 = &raw mut num;

        unsafe {
            assert_eq!(*r1, 5);
            *r2 = 10;
            assert_eq!(*r2, 10);
        }

        assert_eq!(num, 10);
    }
}