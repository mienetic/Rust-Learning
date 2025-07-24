/// ฟังก์ชันสำหรับฝึกฝน Lifetimes - ค่ายฝึกอบรมนักสู้อายุขัย! 🥋⏰
/// เตรียมตัวให้พร้อมสำหรับการต่อสู้กับ borrow checker! 🤺
#[allow(clippy::too_many_lines)]
pub fn practice_lifetimes() {
    println!("\n💪 === แบบฝึกหัด Lifetimes: ค่ายฝึกนักสู้อายุขัย! === 💪");

    // 1. String Analyzer - นักสืบสายลับสตริง! 🕵️‍♂️🔍
    // เก็บ reference ไว้เหมือนเก็บหลักฐานในคดี!
    struct StringAnalyzer<'a> {
        text: &'a str,           // ข้อความที่จะสืบสวน (มีอายุขัย 'a)
        case_sensitive: bool,    // สนใจตัวพิมพ์เล็ก-ใหญ่หรือไม่ (เหมือนนักสืบที่ละเอียดถี่ถ้วน!)
    }

    impl<'a> StringAnalyzer<'a> {
        const fn new(text: &'a str, case_sensitive: bool) -> Self {
            StringAnalyzer {
                text,
                case_sensitive,
            }
        }

        fn find_longest_word(&self) -> Option<&'a str> {
            self.text.split_whitespace().max_by_key(|word| word.len())
        }

        fn find_word(&self, target: &str) -> Option<&'a str> {
            self.text.split_whitespace().find(|&word| {
                if self.case_sensitive {
                    word == target
                } else {
                    word.to_lowercase() == target.to_lowercase()
                }
            })
        }

        fn get_words_starting_with(&self, prefix: &str) -> Vec<&'a str> {
            self.text
                .split_whitespace()
                .filter(|&word| {
                    if self.case_sensitive {
                        word.starts_with(prefix)
                    } else {
                        word.to_lowercase().starts_with(&prefix.to_lowercase())
                    }
                })
                .collect()
        }

        fn count_occurrences(&self, target: &str) -> usize {
            self.text
                .split_whitespace()
                .filter(|&word| {
                    if self.case_sensitive {
                        word == target
                    } else {
                        word.to_lowercase() == target.to_lowercase()
                    }
                })
                .count()
        }
    }

    println!("🔍 === String Analyzer: สำนักงานสืบสวนสตริง! === 🔍");

    let sample_text = "Rust is a systems programming language. Rust is fast and safe. Programming in Rust is fun!";
    let analyzer = StringAnalyzer::new(sample_text, false);

    println!("📝 ข้อความ: {sample_text}");

    if let Some(longest) = analyzer.find_longest_word() {
        println!("📏 คำที่ยาวที่สุด: '{}' ({} ตัวอักษร)", longest, longest.len());
    }

    if let Some(found) = analyzer.find_word("rust") {
        println!("🔍 พบคำ 'rust': '{found}'");
    }

    let words_with_p = analyzer.get_words_starting_with("p");
    println!("📋 คำที่ขึ้นต้นด้วย 'p': {words_with_p:?}");

    let rust_count = analyzer.count_occurrences("rust");
    println!("🔢 จำนวนคำ 'rust': {rust_count} ครั้ง");

    // 2. Reference Manager - ผู้จัดการอ้างอิงมืออาชีพ! 👔📋
    // จัดการ references หลายตัวเหมือนผู้จัดการที่เก่งมาก!
    struct ReferenceManager<'a, 'b> {
        primary: &'a str,     // ข้อมูลหลัก (VIP มีอายุขัย 'a)
        secondary: &'b str,   // ข้อมูลรอง (สำรอง มีอายุขัย 'b)
        priority: u8,         // ระดับความสำคัญ (0-10 เหมือนคะแนนสอบ!)
    }

    impl<'a, 'b> ReferenceManager<'a, 'b> {
        const fn new(primary: &'a str, secondary: &'b str, priority: u8) -> Self {
            ReferenceManager {
                primary,
                secondary,
                priority,
            }
        }

        const fn get_primary(&self) -> &'a str {
            self.primary
        }

        const fn get_secondary(&self) -> &'b str {
            self.secondary
        }

        const fn get_by_priority(&self) -> &str {
            if self.priority >= 5 {
                self.primary
            } else {
                self.secondary
            }
        }

        const fn compare_lengths(&self) -> &str {
            if self.primary.len() >= self.secondary.len() {
                self.primary
            } else {
                self.secondary
            }
        }

        fn create_summary(&self) -> String {
            format!(
                "Primary: '{}', Secondary: '{}', Priority: {}",
                self.primary, self.secondary, self.priority
            )
        }
    }

    println!("\n📊 === Reference Manager: สำนักงานจัดการอ้างอิง! === 📊");

    let primary_data = "ข้อมูลหลัก";
    let secondary_data = "ข้อมูลรอง";

    let manager = ReferenceManager::new(primary_data, secondary_data, 7);

    println!("📋 สรุป: {}", manager.create_summary());
    println!("🥇 หลัก: {}", manager.get_primary());
    println!("🥈 รอง: {}", manager.get_secondary());
    println!("⭐ ตาม priority: {}", manager.get_by_priority());
    println!("📏 ตามความยาว: {}", manager.compare_lengths());

    // 3. Cache System - ระบบแคชสุดเจ๋ง! 💾⚡
    // เก็บ references ไว้เหมือนตู้เซฟเก็บของมีค่า!
    struct Cache<'a> {
        data: Vec<&'a str>,   // คลังข้อมูล (เก็บ references ที่มีอายุขัย 'a)
        max_size: usize,      // ขนาดสูงสุด (เหมือนความจุของตู้เซฟ!)
    }

    impl<'a> Cache<'a> {
        const fn new(max_size: usize) -> Self {
            Cache {
                data: Vec::new(),
                max_size,
            }
        }

        fn add(&mut self, item: &'a str) {
            if self.data.len() >= self.max_size {
                self.data.remove(0); // Remove oldest
            }
            self.data.push(item);
        }

        // Method นี้ใช้เพื่อแสดงตัวอย่างการ return reference จาก cache
        // Warning: method ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        fn get(&self, index: usize) -> Option<&'a str> {
            self.data.get(index).copied()
        }

        fn find(&self, target: &str) -> Option<&'a str> {
            self.data.iter().find(|&&item| item == target).copied()
        }

        fn get_all(&self) -> &[&'a str] {
            &self.data
        }

        const fn size(&self) -> usize {
            self.data.len()
        }

        const fn is_full(&self) -> bool {
            self.data.len() >= self.max_size
        }

        fn clear(&mut self) {
            self.data.clear();
        }
    }

    println!("\n💾 === Cache System: ระบบแคชแห่งอนาคต! === 💾");

    let items = ["item1", "item2", "item3", "item4", "item5"];
    let mut cache = Cache::new(3);

    println!("📦 เพิ่มข้อมูลลง cache (ขนาดสูงสุด 3):");

    for (i, &item) in items.iter().enumerate() {
        cache.add(item);
        println!(
            "  {}. เพิ่ม '{}' - ขนาด: {}/{}, เต็มหรือไม่: {}",
            i + 1,
            item,
            cache.size(),
            cache.max_size,
            cache.is_full()
        );
        println!("     ข้อมูลใน cache: {:?}", cache.get_all());
    }

    println!("\n🔍 ค้นหาข้อมูล:");
    if let Some(found) = cache.find("item3") {
        println!("✅ พบ: {found}");
    } else {
        println!("❌ ไม่พบ item3");
    }

    if let Some(found) = cache.find("item1") {
        println!("✅ พบ: {found}");
    } else {
        println!("❌ ไม่พบ item1 (ถูกลบออกแล้ว)");
    }

    println!("\n🗑️ ล้าง cache:");
    cache.clear();
    println!("📊 ขนาดหลังล้าง: {}", cache.size());

    println!("\n🎉 จบแบบฝึกหัด Lifetimes! ยินดีด้วย คุณเป็นนักสู้อายุขัยแล้ว! 🏆⏰");
}
