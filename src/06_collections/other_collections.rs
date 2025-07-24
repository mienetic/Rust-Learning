//! Other Collections - คลังสมบัติพิเศษสุดเจ๋งกว่า Amazon warehouse! 🎁✨📦
//!
//! ไฟล์นี้สอนเรื่องการใช้งาน `HashSet`, `BTreeMap` และ `VecDeque`
//! รวมถึงการเลือกใช้ Collection ที่เหมาะสมกับงาน
//! เหมือนมีกล่องเครื่องมือพิเศษสำหรับทุกสถานการณ์แบบ Swiss Army knife! 🧰🔧⚡

use std::collections::{BTreeMap, HashSet, VecDeque};

/// ฟังก์ชันสำหรับสอนเรื่อง Collections อื่นๆ
/// มาเรียนรู้คลังสมบัติพิเศษกันเถอะ! เป็น treasure hunter! 🧰🎁🗺️
pub fn learn_other_collections() {
    println!("🧰 === Collections อื่นๆ: คลังสมบัติพิเศษแบบ treasure vault! === 🧰");

    // HashSet - เก็บค่าที่ไม่ซ้ำแบบ anti-duplicate security system! 🔗
    println!("\n🔗 === HashSet: ตู้เซฟป้องกันของซ้ำแบบ Fort Knox! === 🔗");

    let mut books = HashSet::new();  // เปิดตู้เซฟใหม่แบบ high-tech vault! 🔐

    // เพิ่มหนังสือ - ใส่หนังสือลงตู้เซฟแบบ library management! 📚
    books.insert("The Rust Programming Language");  // หนังสือเล่มแรกแบบ bestseller! 📖
    books.insert("Programming Rust");               // หนังสือเล่มสองแบบ sequel! 📗
    books.insert("Rust in Action");                 // หนังสือเล่มสามแบบ action movie! 📘
    books.insert("The Rust Programming Language");  // ซ้ำ - ตู้เซฟปฏิเสธแบบ access denied! 🚫

    println!("📚 หนังสือในชั้น: {books:?} (ไม่มีซ้ำแบบ unique collection!)");
    println!("📊 จำนวนหนังสือ: {} (นับไม่ซ้ำแบบ distinct count!)", books.len());

    // ตรวจสอบว่ามีหนังสือหรือไม่ - ค้นหาในตู้เซฟแบบ search engine! 🔍
    let book_to_find = "Rust in Action";  // หาหนังสือเล่มนี้แบบ treasure hunt! 🎯
    if books.contains(book_to_find) {
        println!("✅ พบหนังสือ: {book_to_find} (เจอในตู้เซฟแบบ jackpot!)");
    } else {
        println!("❌ ไม่พบหนังสือ: {book_to_find} (ไม่มีในตู้เซฟแบบ 404 not found!)");
    }

    // การดำเนินการกับ Set - เปิดตู้เซฟอีกใบแบบ expansion pack! 🔄
    let mut programming_books = HashSet::new();  // ตู้เซฟใบที่สองแบบ backup vault! 🔐
    programming_books.insert("Clean Code");                    // หนังสือโปรแกรมมิ่งแบบ coding bible! 💻
    programming_books.insert("The Rust Programming Language"); // หนังสือซ้ำกับตู้แรกแบบ duplicate entry! 🔄
    programming_books.insert("Design Patterns");              // หนังสือ Pattern แบบ architect's guide! 🎨

    println!("\n🔄 === Set Operations: การผสมผสานตู้เซฟแบบ data fusion! === 🔄");

    // Union - รวมกันแบบ merge operation! 🤝
    let union: HashSet<_> = books.union(&programming_books).collect();
    println!("🤝 Union (รวม): {union:?} (ผสมทุกอย่างเข้าด้วยกันแบบ all-in-one!)");

    // Intersection - ส่วนที่เหมือนกันแบบ common ground! 🎯
    let intersection: HashSet<_> = books.intersection(&programming_books).collect();
    println!("🎯 Intersection (เหมือนกัน): {intersection:?} (ของที่มีซ้ำกันแบบ overlap!)");

    // Difference - ส่วนต่างแบบ exclusive content! ➖
    let difference: HashSet<_> = books.difference(&programming_books).collect();
    println!("➖ Difference (ต่าง): {difference:?} (ของเฉพาะตู้แรกแบบ unique items!)");

    // BTreeMap - Map ที่เรียงลำดับแบบ auto-sort dictionary! 🌳
    println!("\n🌳 === BTreeMap: พจนานุกรมที่เรียบร้อยแบบ Marie Kondo! === 🌳");

    let mut btree_map = BTreeMap::new();  // สร้างพจนานุกรมเรียงลำดับแบบ organized library! 📖
    btree_map.insert(3, "three");  // ใส่แบบสุ่มแบบ random chaos! 🎲
    btree_map.insert(1, "one");    // ใส่แบบสุ่มแบบ messy input! 🎲
    btree_map.insert(2, "two");    // ใส่แบบสุ่มแบบ disorder! 🎲
    btree_map.insert(5, "five");   // ใส่แบบสุ่มแบบ random order! 🎲
    btree_map.insert(4, "four");   // ใส่แบบสุ่มแบบ shuffled! 🎲

    println!("🗂️ BTreeMap (เรียงลำดับ): {btree_map:?} (เรียงให้อัตโนมัติแบบ magic sorting!)");

    // VecDeque - Double-ended queue แบบ flexible snake! ↔️
    println!("\n↔️ === VecDeque: คิวสองหัวสุดเจ๋งแบบ two-headed dragon! === ↔️");

    let mut deque = VecDeque::new();  // สร้างคิวสองหัวแบบ magical creature! 🎭

    // เพิ่มข้างหน้า - ใส่ของด้านหน้าแบบ front-loading! ⬅️
    deque.push_front(1);  // ใส่หน้า: 1 แบบ first place! 🥇
    deque.push_front(2);  // ใส่หน้า: 2 แบบ VIP entry! 🥈

    // เพิ่มข้างหลัง - ใส่ของด้านหลังแบบ back-loading! ➡️
    deque.push_back(3);   // ใส่หลัง: 3 แบบ regular entry! 🥉
    deque.push_back(4);   // ใส่หลัง: 4 แบบ last but not least! 🏅

    println!("📦 VecDeque: {deque:?} (คิวสองหัวพร้อมแล้วแบบ ready to serve!)");

    // เอาออกจากข้างหน้า - เอาของจากหน้าแบบ VIP checkout! ⬅️
    if let Some(front) = deque.pop_front() {
        println!("⬅️ เอาจากหน้า: {front}, เหลือ: {deque:?} (เอาหัวคิวแบบ first served!)");
    }

    // เอาออกจากข้างหลัง - เอาของจากหลังแบบ regular checkout! ➡️
    if let Some(back) = deque.pop_back() {
        println!("➡️ เอาจากหลัง: {back}, เหลือ: {deque:?} (เอาท้ายคิวแบบ last out!)");
    }

    println!("\n🎉 จบการเรียนรู้ Collections อื่นๆ! (เป็นนักสะสมคลังสมบัติมืออาชีพแล้วแบบ master collector! 🏆💎🎯)");
}
