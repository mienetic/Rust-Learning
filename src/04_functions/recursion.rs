//! Recursion - การเรียกตัวเองซ้ำใน Rust
//!
//! ไฟล์นี้สอนเรื่อง recursive functions, factorial, fibonacci, GCD,
//! binary search และ tree traversal
//! (ห้องกระจก: ฟังก์ชันที่เห็นตัวเองไม่จบ! 🪞🌀)

use std::cmp::Ordering;

/// ฟังก์ชันสำหรับเรียนรู้ Recursion
/// ยินดีต้อนรับสู่ห้องกระจกแห่งการเรียกตัวเอง! 🪞
pub fn learn_recursion() {
    println!("\n🌀 === Recursion: ห้องกระจกไม่มีที่สิ้นสุด! === 🌀");

    // Factorial (นักคูณผู้เรียกตัวเอง! 🔢)
    fn factorial(n: u64) -> u64 {  // ฟังก์ชันนักคูณ! ✖️
        if n <= 1 { 1 } else { n * factorial(n - 1) }  // เรียกตัวเองจนเบื่อ! 🔄
    }

    println!("🔢 === Factorial: โรงงานคูณเลข! === 🔢");
    for i in 0..=5 {  // ทดสอบจาก 0 ถึง 5! 🧪
        println!("{}! = {} (คูณไปเรื่อยๆ! 🎯)", i, factorial(i));
    }

    // Fibonacci (recursive) (กระต่ายนักเพิ่มพันธุ์! 🐰)
    fn fibonacci_recursive(n: u32) -> u64 {  // ฟังก์ชันกระต่าย! 🐇
        match n {  // ตรวจสอบรุ่น! 👶
            0 => 0,  // รุ่นที่ 0: ไม่มีกระต่าย! 🚫
            1 => 1,  // รุ่นที่ 1: กระต่าย 1 ตัว! 🐰
            _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),  // รวมกระต่าย 2 รุ่นก่อน! 👨‍👩‍👧‍👦
        }
    }

    println!("\n🌀 === Fibonacci: ฟาร์มกระต่าย! === 🌀");
    for i in 0..10 {  // ดูกระต่าย 10 รุ่น! 📊
        println!("F({}) = {} (กระต่ายรุ่นที่ {}! 🐰)", i, fibonacci_recursive(i), i);
    }

    // Greatest Common Divisor (GCD) (นักหาตัวหารร่วมมาก! 🔍)
    fn gcd(a: u64, b: u64) -> u64 {  // ฟังก์ชันนักสืบตัวหาร! 🕵️‍♂️
        if b == 0 { a } else { gcd(b, a % b) }  // หารจนกว่าจะเหลือ 0! 🎯
    }

    println!("\n🔢 === Greatest Common Divisor: นักสืบตัวหาร! === 🔢");
    println!("GCD(48, 18) = {} (หาเจอแล้ว! 🎯)", gcd(48, 18));
    println!("GCD(100, 25) = {} (ง่ายมาก! 😎)", gcd(100, 25));

    // Binary search (recursive) (นักสืบแบ่งครึ่ง! 🕵️‍♀️)
    fn binary_search(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {  // ฟังก์ชันนักสืบ! 🔍
        if left > right {  // หาไม่เจอแล้ว! 😅
            return None;
        }

        let mid = left + (right - left) / 2;  // หาจุดกึ่งกลาง! ⚖️

        match arr[mid].cmp(&target) {  // เปรียบเทียบ! 🤔
            Ordering::Equal => Some(mid),  // เจอแล้ว! 🎯
            Ordering::Greater => {  // ใหญ่เกินไป! 📈
                if mid == 0 {
                    None  // หาไม่เจอ! 😢
                } else {
                    binary_search(arr, target, left, mid - 1)  // หาทางซ้าย! ⬅️
                }
            }
            Ordering::Less => binary_search(arr, target, mid + 1, right),  // หาทางขวา! ➡️
        }
    }

    println!("\n🔍 === Binary Search: นักสืบแบ่งครึ่ง! === 🔍");
    let sorted_array = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];  // อาร์เรย์เรียงแล้ว! 📊
    let target = 7;  // เป้าหมาย! 🎯

    match binary_search(&sorted_array, target, 0, sorted_array.len() - 1) {  // เริ่มสืบ! 🔍
        Some(index) => println!("🎯 พบ {target} ที่ตำแหน่ง {index} (เก่งมาก! 🏆)"),
        None => println!("❌ ไม่พบ {target} (หาไม่เจอ! 😅)"),
    }

    // Tree traversal example (สวนต้นไม้ดิจิทัล! 🌳)
    #[derive(Debug)]  // ต้นไม้ที่ debug ได้! 🐛
    struct TreeNode {  // โครงสร้างต้นไม้! 🌲
        value: i32,  // ค่าในโหนด! 💎
        left: Option<Box<TreeNode>>,  // กิ่งซ้าย! ⬅️
        right: Option<Box<TreeNode>>,  // กิ่งขวา! ➡️
    }

    impl TreeNode {  // วิธีการของต้นไม้! 🛠️
        const fn new(value: i32) -> Self {  // สร้างต้นไม้ใหม่! 🌱
            Self {
                value,
                left: None,  // ยังไม่มีกิ่งซ้าย! 🚫
                right: None,  // ยังไม่มีกิ่งขวา! 🚫
            }
        }

        fn insert(&mut self, value: i32) {  // ปลูกใบใหม่! 🍃
            if value < self.value {  // ถ้าเล็กกว่า! 📉
                match &mut self.left {  // ดูกิ่งซ้าย! 👀
                    Some(left) => left.insert(value),  // มีแล้ว ปลูกต่อ! 🌿
                    None => self.left = Some(Box::new(Self::new(value))),  // ไม่มี สร้างใหม่! 🌱
                }
            } else {  // ถ้าใหญ่กว่าหรือเท่า! 📈
                match &mut self.right {  // ดูกิ่งขวา! 👀
                    Some(right) => right.insert(value),  // มีแล้ว ปลูกต่อ! 🌿
                    None => self.right = Some(Box::new(Self::new(value))),  // ไม่มี สร้างใหม่! 🌱
                }
            }
        }

        fn inorder_traversal(&self) {  // เดินชมสวนแบบเรียงลำดับ! 🚶‍♂️
            if let Some(left) = &self.left {  // ถ้ามีกิ่งซ้าย! ⬅️
                left.inorder_traversal();  // เดินซ้ายก่อน! 👈
            }
            print!("{} ", self.value);  // แวะดูโหนดปัจจุบัน! 👁️
            if let Some(right) = &self.right {  // ถ้ามีกิ่งขวา! ➡️
                right.inorder_traversal();  // เดินขวาทีหลัง! 👉
            }
        }
    }

    println!("\n🌳 === Binary Tree Traversal: สวนต้นไม้ดิจิทัล! === 🌳");
    let mut root = TreeNode::new(5);  // ปลูกต้นไม้หลัก! 🌲
    root.insert(3);  // ปลูกใบที่ 3! 🍃
    root.insert(7);  // ปลูกใบที่ 7! 🍃
    root.insert(1);  // ปลูกใบที่ 1! 🍃
    root.insert(9);  // ปลูกใบที่ 9! 🍃

    print!("🔄 Inorder traversal: ");  // เริ่มเดินชม! 🚶‍♂️
    root.inorder_traversal();  // เดินชมสวน! 🌿
    println!(" (เดินชมเสร็จแล้ว! 🎊)");

    println!("\n🎉 จบการเรียนรู้ Recursion! (ออกจากห้องกระจกแล้ว! 🪞✨)");
}
