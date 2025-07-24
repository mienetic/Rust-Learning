//! Generics Module - ห้องแล็บ Generics มหัศจรรย์! 🧬✨
//! โมดูลสำหรับเรียนรู้เรื่อง Generics ใน Rust - เวทมนตร์ประเภทข้อมูลที่ยืดหยุ่น! 🪄

mod generic_enums;
mod generic_functions;
mod generic_structs;
mod practice_generics;

pub use generic_enums::learn_generic_enums;
pub use generic_functions::learn_generic_functions;
pub use generic_structs::learn_generic_structs;
pub use practice_generics::practice_generics;

/// ฟังก์ชันสำหรับรันตัวอย่าง generics (เรียกจาก main.rs) - ทัวร์ห้องแล็บ! 🧬🎫
pub fn run_generics_examples() {
    println!("   🔧 Generic Functions (ฟังก์ชันทั่วไป: เวทมนตร์ฟังก์ชันยืดหยุ่น!)");
    learn_generic_functions();

    println!("\n   🏗️ Generic Structs (โครงสร้างทั่วไป: แบบพิมพ์ยืดหยุ่น!)");
    learn_generic_structs();

    println!("\n   📋 Generic Enums (อีนัมทั่วไป: ตัวเลือกยืดหยุ่น!)");
    learn_generic_enums();

    println!("\n   💪 แบบฝึกหัด Generics (ยิมฝึก Generics!)");
    practice_generics();
}

#[cfg(test)]
mod tests {
    // ลบ unused import super::* เนื่องจากไม่ได้ใช้งาน (ทำความสะอาดโค้ด! 🧹)

    #[test]
    fn test_generic_largest() {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {  // ฟังก์ชันหาค่าสูงสุด! 🏆
            let mut largest = list[0];  // เริ่มต้นด้วยตัวแรก! 🥇
            for &item in list {  // วนลูปตรวจสอบ! 🔍
                if item > largest {  // เจอตัวใหญ่กว่า! 📈
                    largest = item;  // อัปเดต! ✨
                }
            }
            largest  // คืนค่าสูงสุด! 🎁
        }

        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&numbers), 100);

        let chars = vec!['y', 'm', 'a', 'q'];
        assert_eq!(largest(&chars), 'y');
    }

    #[test]
    fn test_generic_point() {
        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer_point = Point { x: 5, y: 10 };
        let float_point = Point { x: 1.0, y: 4.0 };

        assert_eq!(integer_point.x, 5);
        // ใช้ epsilon สำหรับการเปรียบเทียบ floating point (ความแม่นยำทศนิยม! 🎯)
        const EPSILON: f64 = 1e-10;
        assert!((float_point.y - 4.0_f64).abs() < EPSILON);
    }

    #[test]
    fn test_generic_stack() {
        struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Stack<T> {  // การใช้งาน Stack! 📚
            fn new() -> Self {  // สร้าง Stack ใหม่! ✨
                Self { items: Vec::new() }
            }

            fn push(&mut self, item: T) {  // ใส่ของเข้า Stack! 📥
                self.items.push(item);
            }

            fn pop(&mut self) -> Option<T> {  // เอาของออกจาก Stack! 📤
                self.items.pop()
            }
        }

        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
