//! Traits Module - โรงละครแห่งพฤติกรรม! 🎭✨
//!
//! ที่นี่คือโรงละครที่ทุกตัวละครสามารถแสดงได้หลายบท!
//! Traits คือสัญญาพฤติกรรม - เหมือนสคริปต์ที่ทุกคนต้องทำตาม! 🎬🪄
//! 
//! จาก "ฉันเป็นใคร" เป็น "ฉันทำอะไรได้บ้าง" - นี่คือพลังของ Traits! 🌟
//! เหมือนโรงเรียนสอนมารยาทที่สอนให้ทุกคนมีพฤติกรรมดี! 🎓📚
//! หรือเหมือนใบประกาศนียบัตรที่รับรองว่าคุณทำอะไรได้บ้าง! 📜🏆

mod basic_traits;
mod practice_traits;
mod standard_traits;
mod trait_bounds;
mod trait_objects;

pub use basic_traits::learn_basic_traits;
pub use practice_traits::practice_traits;
pub use standard_traits::learn_standard_traits;
pub use trait_bounds::learn_trait_bounds;
pub use trait_objects::learn_trait_objects;

/// ฟังก์ชันสำหรับรันตัวอย่าง traits (เรียกจาก main.rs) - ทัวร์โรงละคร Traits! 🎭🎫
/// เหมือนการดูโชว์ที่มีหลายรอบ แต่ละรอบมีเสน่ห์ต่างกัน! 🎪🌟
pub fn run_traits_examples() {
    println!("   🎯 Basic Traits (Traits พื้นฐาน: เรียนรู้พฤติกรรมร่วม!)");
    println!("   🎭 เหมือนเรียนมารยาทพื้นฐาน หรือเรียนกิริยาที่ดี! 🙏✨");
    learn_basic_traits();

    println!("\n   🔗 Trait Bounds (ขอบเขต Traits: กำหนดเงื่อนไข!)");
    println!("   🚧 เหมือนป้ายบอกทาง หรือกฎระเบียบที่ต้องปฏิบัติตาม! 📋⚖️");
    learn_trait_bounds();

    println!("\n   📚 Standard Traits (Traits มาตรฐาน: เครื่องมือพร้อมใช้!)");
    println!("   🛠️ เหมือนชุดเครื่องมือที่มาพร้อมกับบ้าน หรือแอปที่ติดตั้งมาให้! 📱🏠");
    learn_standard_traits();

    println!("\n   🎭 Trait Objects (วัตถุ Traits: การแสดงแบบไดนามิก!)");
    println!("   🎪 เหมือนนักแสดงที่เปลี่ยนบทได้ตลอดเวลา หรือหุ่นยนต์ที่ทำงานได้หลายอย่าง! 🤖🎨");
    learn_trait_objects();

    println!("\n   💪 แบบฝึกหัด Traits (ยิมฝึก Traits!)");
    println!("   🏋️‍♂️ เหมือนเข้าฟิตเนสเพื่อฝึกกล้ามเนื้อ หรือเรียนพิเศษเพื่อเก่งขึ้น! 🎯📈");
    practice_traits();
}

#[cfg(test)]
mod tests {
    // ลบ unused import super::* เนื่องจากไม่ได้ใช้งาน (ทำความสะอาดโค้ด! 🧹)

    #[test]
    fn test_trait_implementation() {
        trait Testable {  // Trait สำหรับทดสอบ! 🧪
            fn test_value(&self) -> i32;
        }

        struct TestStruct {  // โครงสร้างทดสอบ! 🏗️
            value: i32,
        }

        impl Testable for TestStruct {  // การใช้งาน Trait! ✨
            fn test_value(&self) -> i32 {
                self.value  // คืนค่า! 🎁
            }
        }

        let test_obj = TestStruct { value: 42 };
        assert_eq!(test_obj.test_value(), 42);
    }

    #[test]
    fn test_trait_bounds() {
        #[allow(clippy::needless_pass_by_value)]
        fn compare_and_display<T: PartialOrd + std::fmt::Display>(a: T, b: T) -> String {  // ฟังก์ชันเปรียบเทียบ! ⚖️
            if a > b {  // มากกว่า! 📈
                format!("{a} > {b}")
            } else if a < b {  // น้อยกว่า! 📉
                format!("{a} < {b}")
            } else {  // เท่ากัน! ⚖️
                format!("{a} = {b}")
            }
        }

        assert_eq!(compare_and_display(5, 3), "5 > 3");
        assert_eq!(compare_and_display(2, 7), "2 < 7");
        assert_eq!(compare_and_display(4, 4), "4 = 4");
    }

    #[test]
    fn test_trait_objects() {
        trait TestShape {
            fn area(&self) -> f64;
        }

        struct TestCircle {
            radius: f64,
        }
        struct TestSquare {
            side: f64,
        }

        impl TestShape for TestCircle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
        }

        impl TestShape for TestSquare {
            fn area(&self) -> f64 {
                self.side * self.side
            }
        }

        let shapes: Vec<Box<dyn TestShape>> = vec![
            Box::new(TestCircle { radius: 1.0 }),
            Box::new(TestSquare { side: 2.0 }),
        ];

        let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
        assert!((total_area - 7.14159).abs() < 0.001);
    }
}
