//! Unsafe Traits ใน Unsafe Rust - ปฏิบัติการ Trait อันตราย! 🕵️‍♂️⚡
//!
//! ไฟล์นี้ประกอบด้วย:
//! - การสร้าง unsafe traits - การออกแบบภารกิจอันตราย! 🎯⚠️
//! - การ implement unsafe traits - การฝึกสายลับเฉพาะทาง! 🦀💀
//! - ตัวอย่างการใช้งาน Send และ Sync - การส่งสายลับข้ามมิติ! 🚀🌀

/// ตัวอย่าง unsafe trait - สัญญาอันตรายของสายลับ! 🎯⚠️
#[allow(clippy::missing_safety_doc)]
pub unsafe trait UnsafeTrait {
    fn unsafe_method(&self);  // 💀 เมธอดอันตรายที่ต้องระวัง
}

// การ implement unsafe trait ต้องใช้ unsafe - การฝึกสายลับเฉพาะทาง! 🦀💀
unsafe impl UnsafeTrait for i32 {
    fn unsafe_method(&self) {
        println!("🔢💀 สายลับตัวเลขปฏิบัติภารกิจอันตราย: {self}");  // ⚡ ภารกิจตัวเลข
    }
}

unsafe impl UnsafeTrait for String {
    fn unsafe_method(&self) {
        println!("📝💀 สายลับข้อความปฏิบัติภารกิจอันตราย: {self}");  // ⚡ ภารกิจข้อความ
    }
}

pub fn unsafe_traits_examples() {
    println!("\n🎯💀 === Dangerous Trait Operations === ⚡🕵️‍♂️");

    let number = 42i32;                              // 🔢 สายลับตัวเลข
    let text = String::from("Hello, Unsafe!");      // 📝 สายลับข้อความ

    // การเรียกใช้ method จาก unsafe trait - การสั่งการสายลับ! 📞💀
    number.unsafe_method();   // ⚡ สั่งการสายลับตัวเลข
    text.unsafe_method();     // ⚡ สั่งการสายลับข้อความ

    println!("💡⚠️ Unsafe traits ใช้เมื่อต้องการภารกิจที่ compiler ตรวจสอบไม่ได้!");
}

/// ตัวอย่างการสร้าง custom smart pointer - กล่องเก็บของลับของสายลับ! 📦🔐
pub struct MyBox<T> {
    ptr: *mut T,  // 🎯 ตัวชี้อันตรายไปยังข้อมูลลับ
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));  // 🔐 เข้ารหัสข้อมูลลับ
        Self { ptr }  // 📦 สร้างกล่องเก็บของลับ
    }

    #[must_use]
    pub fn get(&self) -> &T {
        unsafe { &*self.ptr }  // 🔍 เปิดดูข้อมูลลับ (อันตราย!)
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }  // ✏️ แก้ไขข้อมูลลับ (อันตรายมาก!)
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let _box = Box::from_raw(self.ptr);  // 🗑️ ทำลายหลักฐานลับ
            // Box จะ drop ค่าเมื่อออกจาก scope - การลบรอยเท้า! 👻
        }
    }
}

// ต้องใช้ unsafe เพื่อ implement Send และ Sync - การส่งสายลับข้ามมิติ! 🚀🌀
unsafe impl<T: Send> Send for MyBox<T> {}  // 📤 ส่งกล่องลับข้ามเธรด
unsafe impl<T: Sync> Sync for MyBox<T> {}  // 🔄 แชร์กล่องลับระหว่างเธรด

pub fn custom_smart_pointer_examples() {
    println!("\n📦🔐 === Secret Box Operations === 🕵️‍♂️💀");

    let mut my_box = MyBox::new(String::from("Hello, Unsafe Rust!"));  // 📦 สร้างกล่องเก็บความลับ

    println!("🔍📖 เปิดดูข้อมูลลับ: {}", my_box.get());  // 👀 แอบดูข้อมูล

    // เปลี่ยนค่า - การปลอมแปลงหลักฐาน! ✏️🔐
    my_box.get_mut().push_str(" - Modified");  // 📝 แก้ไขข้อมูลลับ
    println!("✏️🔐 ข้อมูลลับหลังปลอมแปลง: {}", my_box.get());

    // MyBox จะ drop อัตโนมัติเมื่อออกจาก scope - การทำลายหลักฐาน! 🗑️👻
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_box() {  // 🧪 ทดสอบกล่องเก็บความลับ
        let mut my_box = MyBox::new(42);  // 📦 สร้างกล่องลับ
        assert_eq!(*my_box.get(), 42);    // 🔍 ตรวจสอบข้อมูลลับ

        *my_box.get_mut() = 100;          // ✏️ ปลอมแปลงข้อมูล
        assert_eq!(*my_box.get(), 100);   // ✅ ยืนยันการปลอมแปลง
    }

    #[test]
    fn test_unsafe_trait() {  // 🧪 ทดสอบภารกิจอันตราย
        let number = 42i32;  // 🔢 สายลับตัวเลข

        // ทดสอบว่าสามารถเรียกใช้ unsafe method ได้ - การทดสอบภารกิจ! ⚡💀
        number.unsafe_method();  // 🎯 สั่งการสายลับ

        // ไม่มี assertion เพราะเป็นการทดสอบว่า compile ได้ - การยืนยันความสามารถ! ✅
    }
}