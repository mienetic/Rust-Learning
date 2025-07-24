//! Lifetimes Module - เครื่องเวลาแห่งหน่วยความจำ! ⏰✨
//!
//! ที่นี่คือโลกแห่งการจัดการอายุขัย - เหมือนเป็นหมอดูดวงของตัวแปร! 🔮👻
//! Lifetimes บอกว่า "ใครจะอยู่ได้นานแค่ไหน" ป้องกันไม่ให้เกิด dangling pointer! 👻🔒
//! เหมือนเป็นนายอำเภอเวลาที่คอยดูแลให้ทุกอย่างอยู่ในกรอบเวลาที่ถูกต้อง! ⏰👮‍♂️
//!
//! จาก "เกิด-แก่-เจ็บ-ตาย" ของตัวแปร - Rust ดูแลทุกขั้นตอนเหมือนพยาบาลส่วนตัว! 🌱➡️🌳➡️🍂👩‍⚕️

mod basic_lifetimes;
mod lifetime_elision;
mod practice_lifetimes;
mod static_lifetime;
mod struct_lifetimes;

pub use basic_lifetimes::learn_basic_lifetimes;
pub use lifetime_elision::learn_lifetime_elision;
pub use practice_lifetimes::practice_lifetimes;
pub use static_lifetime::learn_static_lifetime;
pub use struct_lifetimes::learn_struct_lifetimes;

/// ฟังก์ชันสำหรับรันตัวอย่าง lifetimes (เรียกจาก main.rs) - ทัวร์เครื่องเวลา! ⏰🎫
pub fn run_lifetimes_examples() {
    println!("   ⏰ Basic Lifetimes (Lifetimes พื้นฐาน: เรียนรู้อายุการใช้งาน!)");
    learn_basic_lifetimes();

    println!("\n   🏗️ Struct Lifetimes (Lifetimes ในโครงสร้าง: อายุการใช้งานของโครงสร้าง!)");
    learn_struct_lifetimes();

    println!("\n   ♾️ Static Lifetime (Lifetime แบบคงที่: อายุการใช้งานตลอดกาล!)");
    learn_static_lifetime();

    println!("\n   🔍 Lifetime Elision (การละเว้น Lifetime: เวทมนตร์อัตโนมัติ!)");
    learn_lifetime_elision();

    println!("\n   💪 แบบฝึกหัด Lifetimes (ยิมฝึก Lifetimes!)");
    practice_lifetimes();
}

#[cfg(test)]
mod tests {
    // ลบ unused import super::* เนื่องจากไม่ได้ใช้งาน (ทำความสะอาดโค้ด! 🧹)

    #[test]
    fn test_longest_function() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ฟังก์ชันหาข้อความยาวสุด! 📏
            if x.len() > y.len() { x } else { y }  // เปรียบเทียบความยาว! ⚖️
        }

        assert_eq!(longest("hello", "world"), "world");
        assert_eq!(longest("rust", "programming"), "programming");
    }

    #[test]
    fn test_struct_with_lifetime() {
        struct TestStruct<'a> {  // โครงสร้างที่มี Lifetime! 🏗️⏰
            data: &'a str,
        }

        impl<'a> TestStruct<'a> {  // การใช้งานโครงสร้าง! ✨
            fn get_data(&self) -> &'a str {  // ดึงข้อมูล! 📤
                self.data  // คืนข้อมูล! 🎁
            }
        }

        let text = "test data";
        let test_struct = TestStruct { data: text };
        assert_eq!(test_struct.get_data(), "test data");
    }

    #[test]
    fn test_lifetime_elision() {
        fn first_word(s: &str) -> &str {  // ฟังก์ชันหาคำแรก! 🔍
            let bytes = s.as_bytes();  // แปลงเป็น bytes! 🔄
            for (i, &item) in bytes.iter().enumerate() {  // วนลูปหา! 🔍
                if item == b' ' {  // เจอช่องว่าง! 🎯
                    return &s[0..i];  // คืนคำแรก! 🎁
                }
            }
            s  // คืนทั้งหมด! 📦
        }

        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}
