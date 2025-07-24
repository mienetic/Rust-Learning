//! Generic Functions - ฟังก์ชันที่ทำงานได้ทุกประเภท! 🪄✨
//!
//! เหมือนมีดสวิสที่ตัดได้ทุกอย่าง หรือนักมายากลที่เปลี่ยนอะไรก็ได้!
//! ฟังก์ชันเดียว ทำงานได้หลายประเภท - นี่คือพลังของ Generics! 🔧🎭
//! เหมือนพ่อครัวที่ทำอาหารได้ทุกชาติ หรือนักแปลที่พูดได้ทุกภาษา! 👨‍🍳🌍

/// ฟังก์ชันสำหรับสอนเรื่อง Generic Functions
/// มาเรียนรู้การสร้างฟังก์ชันที่ทำงานกับหลายประเภทข้อมูลกันเถอะ!
pub fn learn_generic_functions() {
    println!("🔧 === Generic Functions ใน Rust: ฟังก์ชันที่ทำงานได้ทุกประเภทข้อมูล! === 🔧");
    println!("🎪 เหมือนนักแสดงที่เล่นได้ทุกบท หรือเสื้อผ้าที่ใส่ได้ทุกคน! 🎭👕");

    // ฟังก์ชันหาค่าที่ใหญ่ที่สุด - แบบไม่ใช้ generics (แบบโบราณ)
    // หรือเหมือนมีช้อนแยกสำหรับแต่ละอาหาร - ยุ่งยากมาก! 🥄🍜🍝
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("\n📊 === ฟังก์ชันแบบเฉพาะเจาะจง: เหมือนช้อนที่กินได้แค่ข้าว! === 📊");
    println!("🔧 เหมือนมีเครื่องมือแยกสำหรับแต่ละงาน - ยุ่งยากและเปลืองพื้นที่! 🧰📦");

    let number_list = [34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("🔢 ตัวเลขที่ใหญ่ที่สุด: {result} (เหมือนหาคนที่สูงที่สุดในห้องเรียน!)");

    let char_list = ['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("🔤 ตัวอักษรที่ใหญ่ที่สุด: {result} (เรียงตาม ASCII นะ ไม่ใช่ขนาดของตัวอักษร!)");

    // ฟังก์ชันแบบ Generic - เหมือนมีดสวิสที่ตัดได้ทุกอย่าง!
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    println!("\n✨ === ฟังก์ชันแบบ Generic: ฟังก์ชันที่ไม่จำกัดประเภทข้อมูล! === ✨");
    println!("🎯 เหมือนมีดสวิสที่ทำได้ทุกอย่าง หรือซุปเปอร์ฮีโร่ที่มีพลังพิเศษหลากหลาย! 🦸‍♂️⚡");

    let number_list = [34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("🔢 ตัวเลขที่ใหญ่ที่สุด (Generic): {result} (ฟังก์ชันเดียวกันใช้ได้กับหลายประเภทข้อมูล!)");

    let char_list = ['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("🔤 ตัวอักษรที่ใหญ่ที่สุด (Generic): {result} (ฟังก์ชันเดียวกันแต่รองรับข้อมูลหลายประเภท!)");

    // ตัวอย่าง Generic functions เพิ่มเติม - เหมือนเครื่องมือที่ใช้ได้กับทุกประเภทข้อมูล!
    const fn swap<T>(a: T, b: T) -> (T, T) {
        (b, a)
    }

    let (x, y) = swap(1, 2);
    println!("🔄 สลับตัวเลข: ({x}, {y}) (ฟังก์ชันสลับค่าแบบ Generic!)");

    let (s1, s2) = swap("hello", "world");
    println!("🔄 สลับข้อความ: ({s1}, {s2}) (ฟังก์ชันเดียวกันใช้ได้กับข้อความด้วย!)");

    // Multiple type parameters - รองรับหลายประเภทข้อมูลในฟังก์ชันเดียว!
    const fn make_pair<T, U>(first: T, second: U) -> (T, U) {
        (first, second)
    }

    let pair1 = make_pair(42, "answer");
    println!("👫 คู่ผสม: {pair1:?} (ตัวเลขกับข้อความ รวมกันได้!)");

    // ใช้ค่าคงที่ PI จาก standard library แทนค่าประมาณ
    let pair2 = make_pair(true, std::f64::consts::PI);
    println!("👫 คู่ผสม: {pair2:?} (Boolean กับ PI รวมกันได้ด้วย!)");
}
