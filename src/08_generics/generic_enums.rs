//! Generic Enums - การแจงนับที่ไม่เลือกปฏิบัติ! 🎯✨
//!
//! เหมือนกล่องลึกลับที่เปิดแล้วได้อะไรก็ได้ หรือเครื่องสุ่มที่ออกได้ทุกอย่าง!
//! Enum เดียว รองรับได้หลายประเภท - นี่คือความยืดหยุ่นของ Generics! 🎲🎪
//! เหมือนตู้เสื้อผ้าที่ใส่ได้ทุกไซส์ หรือร้านขายของที่มีทุกอย่าง! 👕🏪

/// ฟังก์ชันสำหรับสอนเรื่อง Generic Enums
/// มาเรียนรู้การสร้าง Enum ที่ทำงานกับหลายประเภทข้อมูลกันเถอะ! เหมือนกล่องลึกลับที่เปิดแล้วได้อะไรก็ได้! 🎯
/// 
/// ใช้ #[`allow(clippy::too_many_lines)`] เพราะฟังก์ชันนี้เป็นตัวอย่างการเรียนรู้
/// ที่ครอบคลุมหลายแนวคิดของ Generic Enums ในไฟล์เดียว เพื่อความเข้าใจที่ต่อเนื่อง
#[allow(clippy::too_many_lines)]
pub fn learn_generic_enums() {
    println!("🎯 === Generic Enums ใน Rust: Enum ที่ไม่เลือกปฏิบัติ! === 🎯");
    println!("🎪 เหมือนโชว์วิเศษที่เปลี่ยนได้ทุกรูปแบบ หรือนักแสดงที่เล่นได้ทุกบท! 🎭✨");

    println!("\n📦 === Option<T>: กล่องที่อาจมีของหรือไม่มี! === 📦");
    println!("🎁 เหมือนกล่องของขวัญที่ไม่รู้ว่าข้างในมีอะไร หรือลอตเตอรี่ที่อาจถูกหรือไม่! 🎟️🤞");

    // Option<T> - built-in generic enum (Enum ที่มาพร้อม Rust!)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("🔢 Some number: {some_number:?} (มีเลข 5 อยู่ข้างใน!)");
    println!("📝 Some string: {some_string:?} (มีข้อความซ่อนอยู่!)");
    println!("❌ Absent number: {absent_number:?} (กล่องว่างเปล่า ไม่มีอะไร!)");

    // Pattern matching with Option - เปิดกล่องดูว่ามีอะไรข้างใน!
    match some_number {
        Some(value) => println!("✅ Found value: {value} (เจอของแล้ว!)"),
        None => println!("❌ No value found (กล่องว่างเปล่า!)"),
    }

    println!("\n⚡ === Result<T, E>: กล่องที่บอกว่าสำเร็จหรือล้มเหลว! === ⚡");
    println!("🎯 เหมือนการสอบที่ได้ผ่านหรือตก หรือการเล่นเกมที่ชนะหรือแพ้! 📝🎮");

    // Result<T, E> - built-in generic enum (Enum สำหรับจัดการความผิดพลาด!)
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("Something went wrong");

    println!("✅ Good result: {good_result:?} (ทุกอย่างเรียบร้อย!)");
    println!("❌ Bad result: {bad_result:?} (มีปัญหาเกิดขึ้น!)");

    // Pattern matching with Result - ตรวจสอบว่าสำเร็จหรือล้มเหลว!
    match good_result {
        Ok(value) => println!("✅ Success: {value} (ได้ผลลัพธ์ที่ต้องการ!)"),
        Err(error) => println!("❌ Error: {error} (เกิดข้อผิดพลาด!)"),
    }

    match bad_result {
        Ok(value) => println!("✅ Success: {value} (ไม่น่าจะมาถึงบรรทัดนี้!)"),
        Err(error) => println!("❌ Error: {error} (คาดไว้แล้วว่าจะผิดพลาด!)"),
    }

    println!("\n🔀 === Custom Generic Enum: Either - เลือกซ้ายหรือขวา! === 🔀");
    println!("🛤️ เหมือนทางแยกที่ต้องเลือกทิศทาง หรือเมนูที่มีให้เลือก 2 อย่าง! 🍕🍔");

    // Custom generic enum - Enum ที่เราสร้างเอง!
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),   // ทางซ้าย
        Right(R),  // ทางขวา
    }

    impl<L, R> Either<L, R> {
        const fn is_left(&self) -> bool {
            matches!(self, Self::Left(_))
        }

        const fn is_right(&self) -> bool {
            matches!(self, Self::Right(_))
        }

        fn left(self) -> Option<L> {
            match self {
                Self::Left(value) => Some(value),
                Self::Right(_) => None,
            }
        }

        fn right(self) -> Option<R> {
            match self {
                Self::Left(_) => None,
                Self::Right(value) => Some(value),
            }
        }
    }

    let left_value: Either<i32, String> = Either::Left(42);
    let right_value: Either<i32, String> = Either::Right(String::from("Hello"));

    println!("⬅️ Left value: {left_value:?} (เลือกทางซ้าย ได้ตัวเลข!)");
    println!("➡️ Right value: {right_value:?} (เลือกทางขวา ได้ข้อความ!)");

    println!("⬅️ Is left? {} (ตรวจสอบว่าเป็นทางซ้ายไหม?)", left_value.is_left());
    println!("➡️ Is right? {} (ตรวจสอบว่าเป็นทางขวาไหม?)", right_value.is_right());

    // Extract values - ดึงค่าออกมาจากกล่อง!
    if let Some(value) = left_value.left() {
        println!("⬅️ Extracted left value: {value} (ดึงค่าจากทางซ้ายได้!)");
    }

    if let Some(value) = right_value.right() {
        println!("➡️ Extracted right value: {value} (ดึงค่าจากทางขวาได้!)");
    }

    println!("\n🌳 === Generic Binary Tree: ต้นไม้ที่แตกสาขาได้! === 🌳");
    println!("🌿 เหมือนต้นไผ่ที่แตกหน่อใหม่ หรือแผนผังครอบครัวที่มีสายเลือด! 🎋👨‍👩‍👧‍👦");

    // More complex generic enum - Binary Tree (โครงสร้างข้อมูลที่ซับซ้อน!)
    #[derive(Debug, Clone)]
    enum BinaryTree<T> {
        Empty,  // ต้นไม้ว่างเปล่า
        Node {
            value: T,                        // ค่าในโหนด
            left: Box<BinaryTree<T>>,       // กิ่งซ้าย
            right: Box<BinaryTree<T>>,      // กิ่งขวา
        },
    }

    impl<T> BinaryTree<T> {
        const fn new() -> Self {
            Self::Empty
        }

        fn leaf(value: T) -> Self {
            Self::Node {
                value,
                left: Box::new(Self::Empty),
                right: Box::new(Self::Empty),
            }
        }

        fn node(value: T, left: Self, right: Self) -> Self {
            Self::Node {
                value,
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        const fn is_empty(&self) -> bool {
            matches!(self, Self::Empty)
        }
    }

    impl<T: std::fmt::Display> BinaryTree<T> {
        fn print_inorder(&self) {
            // เดินทางผ่านต้นไม้แบบ in-order (ซ้าย-กลาง-ขวา)
            match self {
                Self::Empty => {}, // ไม่มีอะไรให้พิมพ์
                Self::Node { value, left, right } => {
                    left.print_inorder();   // เดินทางกิ่งซ้ายก่อน
                    print!("{value} ");      // พิมพ์ค่าตรงกลาง
                    right.print_inorder();  // เดินทางกิ่งขวาสุดท้าย
                }
            }
        }
    }

    // Create a binary tree - สร้างต้นไม้ที่มีโครงสร้าง!
    let tree = BinaryTree::node(
        1,  // รากของต้นไม้
        BinaryTree::node(2, BinaryTree::leaf(4), BinaryTree::leaf(5)), // กิ่งซ้าย
        BinaryTree::node(3, BinaryTree::leaf(6), BinaryTree::leaf(7)), // กิ่งขวา
    );

    println!("🌳 Binary Tree: {tree:?} (ต้นไม้ที่มีโครงสร้างสวยงาม!)");
    print!("🔄 In-order traversal: (เดินทางผ่านต้นไม้แบบเรียงลำดับ!) ");
    tree.print_inorder();
    println!();

    let empty_tree: BinaryTree<i32> = BinaryTree::new();
    println!("🌳 Empty tree is empty: {} (ต้นไม้ว่างเปล่าจริงๆ!)", empty_tree.is_empty());
    println!("🌳 Main tree is empty: {} (ต้นไม้หลักไม่ว่างเปล่า!)", tree.is_empty());
}
