//! Collections Module - คลังสมบัติ Collections มหัศจรรย์แบบ Louvre Museum! 🏛️✨💎
//!
//! โมดูลนี้เป็นพิพิธภัณฑ์ Collections ที่ยิ่งใหญ่กว่า British Museum! 🏛️🌍 รวบรวมการเรียนรู้เกี่ยวกับ
//! Collections ต่างๆ ใน Rust แบ่งออกเป็นหมวดหมู่ตามประเภทอย่างเป็นระบบแบบ Marie Kondo! 📚✨

// Module declarations
mod hashmaps;
mod other_collections;
mod practice_collections;
mod vectors;

// Re-exports
pub use hashmaps::*;
pub use other_collections::*;
pub use practice_collections::*;
pub use vectors::*;

/// ฟังก์ชันสำหรับรันตัวอย่าง collections (เรียกจาก main.rs) - ทัวร์พิพิธภัณฑ์แบบ VIP! 🏛️🎫👑
pub fn run_collections_examples() {
    println!("   📊 Vectors (เวกเตอร์: กล่องเก็บของแบบยืดหยุ่นกว่ายางยืด!)");
    learn_vectors();

    println!("\n   🗂️ HashMaps (แฮชแมป: ตู้เก็บของแบบ key-value สไตล์ Netflix recommendation!)");
    learn_hashmaps();

    println!("\n   📦 Other Collections (คอลเลกชันอื่นๆ: ตู้เก็บของพิเศษแบบ limited edition!)");
    learn_other_collections();

    println!("\n   💪 แบบฝึกหัด Collections (ยิมฝึกจัดการคอลเลกชันแบบ CrossFit!)");
    practice_collections();
}

// Tests module
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);

        let popped = v.pop();
        assert_eq!(popped, Some(4));
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        assert_eq!(map.get("key1"), Some(&"value1"));
        assert_eq!(map.get("key3"), None);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_hashset_operations() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(1); // ซ้ำ (แต่ HashSet ไม่เก็บค่าซ้ำแบบ exclusive club! 🚫👑)

        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
        assert!(!set.contains(&3));
    }
}
