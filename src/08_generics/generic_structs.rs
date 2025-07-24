//! Generic Structs - โครงสร้างข้อมูลที่ไม่เลือกปฏิบัติ! 🏗️✨
//!
//! เหมือนกล่องวิเศษที่ใส่อะไรก็ได้ หรือบ้านที่อยู่ได้ทุกสายพันธุ์!
//! Struct เดียว รองรับได้หลายประเภท - นี่คือความยืดหยุ่นของ Generics! 📦🎭
//! เหมือนโรงแรม 5 ดาวที่รับแขกทุกเชื้อชาติ หรือร้านอาหารที่ทำได้ทุกเมนู! 🏨🍽️

/// ฟังก์ชันสำหรับสอนเรื่อง Generic Structs
/// มาเรียนรู้การสร้าง Struct ที่ทำงานกับหลายประเภทข้อมูลกันเถอะ!
pub fn learn_generic_structs() {
    println!("🏗️ === Generic Structs ใน Rust: โครงสร้างที่ไม่เลือกปฏิบัติ! === 🏗️");
    println!("🎪 เหมือนเต็นท์ยักษ์ที่รองรับได้ทุกการแสดง หรือสนามกีฬาที่เล่นได้ทุกกีฬา! ⚽🏀🎾");

    // Point struct with single type parameter - จุดที่รับทุกประเภท!
    // เหมือนจุดนัดพบที่ทุกคนมาได้ หรือสถานีรถไฟที่รถทุกขบวนแวะ! 🚂🚉
    #[derive(Debug, Clone)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        const fn x(&self) -> &T {
            &self.x
        }

        const fn y(&self) -> &T {
            &self.y
        }
    }

    // Methods for specific types - เมธอดพิเศษสำหรับ f32 เท่านั้น!
    // เหมือนบริการพิเศษสำหรับสมาชิก VIP หรือเมนูลับที่มีแค่บางคน! 💎🎫
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            self.x.hypot(self.y)
        }
    }

    println!("\n📍 === Point Struct: จุดที่ไม่จำกัดมิติ! === 📍");
    println!("🌟 เหมือนดาวที่ส่องแสงได้ทุกสี หรือเข็มทิศที่ชี้ได้ทุกทิศ! ⭐🧭");

    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);

    println!("🔢 Integer Point: {integer_point:?} (จุดที่ชอบเลขเต็ม!)");
    println!("🔢 x: {}, y: {} (พิกัดที่ไม่มีทศนิยม!)", integer_point.x(), integer_point.y());

    println!("🔢 Float Point: {float_point:?} (จุดที่ชอบทศนิยม!)");
    println!(
        "📏 Distance from origin: {:.2} (ระยะทางจากจุดกำเนิด - ใช้ทฤษฎีพีทาโกรัส!)",
        float_point.distance_from_origin()
    );

    // Point with multiple type parameters - จุดที่ชอบความหลากหลาย!
    // เหมือนงานแต่งงานข้ามวัฒนธรรม หรือร้านอาหารฟิวชั่นที่ผสมทุกอย่าง! 💒🍜
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> MixedPoint<T, U> {
        const fn new(x: T, y: U) -> Self {
            Self { x, y }
        }

        fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
            // เอา x จากตัวเอง y จากคนอื่น - เหมือนการผสมพันธุ์!
            MixedPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    println!("\n🎭 === Mixed Point: จุดที่ชอบความแปลกใหม่! === 🎭");

    let p1 = MixedPoint::new(5, 10.4);
    let p2 = MixedPoint::new("Hello", 'c');

    println!("📍 Point 1: {p1:?} (ตัวเลขกับทศนิยม - คู่หวาน!)");
    println!("📍 Point 2: {p2:?} (ข้อความกับตัวอักษร - คู่แปลก!)");

    let p3 = p1.mixup(p2);
    println!("🎭 Mixed Point: {p3:?} (ผลลัพธ์จากการผสมพันธุ์!)");

    // Container struct - กล่องวิเศษที่ใส่อะไรก็ได้!
    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
    }

    impl<T> Container<T> {
        const fn new() -> Self {
            Self { items: Vec::new() }
        }

        fn add(&mut self, item: T) {
            self.items.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }

        const fn len(&self) -> usize {
            self.items.len()
        }
    }

    impl<T: std::fmt::Display> Container<T> {
        fn display_all(&self) {
            // แสดงทุกอย่างในกล่อง - เหมือนเปิดกล่องของขวัญ!
            for (i, item) in self.items.iter().enumerate() {
                println!("  [{i}]: {item}");
            }
        }
    }

    println!("\n📦 === Container: กล่องที่ไม่เลือกของ! === 📦");
    println!("🎁 เหมือนกล่องโดเรมอนที่ใส่ของวิเศษได้ทุกอย่าง หรือกระเป๋าแฮร์รี่ พอตเตอร์! 🪄✨");

    let mut number_container = Container::new();
    number_container.add(1);
    number_container.add(2);
    number_container.add(3);

    println!("🔢 Number Container: {number_container:?} (กล่องเลขที่เรียงตัวดี!)");
    println!("📊 Length: {} (มีของ {} ชิ้น!)", number_container.len(), number_container.len());
    println!("🔍 Item at index 1: {:?} (ของชิ้นที่ 2 คือ...)", number_container.get(1));

    println!("📋 All items: (เปิดกล่องดูของทุกชิ้น!)");
    number_container.display_all();

    let mut string_container = Container::new();
    string_container.add(String::from("Hello"));
    string_container.add(String::from("World"));

    println!("\n📝 String Container: {string_container:?} (กล่องข้อความที่น่ารัก!)");
    println!("📋 All strings: (ข้อความทั้งหมดในกล่อง!)");
    string_container.display_all();
}
