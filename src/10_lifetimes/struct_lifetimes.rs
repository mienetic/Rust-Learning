/// ฟังก์ชันสำหรับสอนเรื่อง Lifetimes ใน Structs - โรงงานผลิต Struct อมตะ! 🏭⚰️
/// เรียนรู้การสร้าง Struct ที่เก็บ references ไว้เหมือนพิพิธภัณฑ์! 🏛️
///
/// # Panics
///
/// ฟังก์ชันนี้อาจ panic เมื่อเข้าถึง array index ที่ไม่ถูกต้อง
pub fn learn_struct_lifetimes() {
    println!("\n🏗️ === Lifetimes ใน Structs: โรงงานผลิต Struct อมตะ! === 🏗️");

    // Struct ที่เก็บ reference - ตู้เก็บข้อความสำคัญ! 📚🔐
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,     // ข้อความสำคัญ (มีอายุขัย 'a เหมือนหนังสือโบราณ!)
        // Fields เหล่านี้ใช้เพื่อแสดงตัวอย่างข้อมูลเพิ่มเติมใน struct
        // Warning: fields ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        page: u32,         // หน้าที่ (เหมือนเลขที่บ้าน!)
        #[allow(dead_code)]
        chapter: u32,      // บทที่ (เหมือนเลขที่ห้อง!)
    }

    impl<'a> ImportantExcerpt<'a> {
        // Method ที่ไม่ต้องระบุ lifetime (elision rules) - เวทมนตร์อัตโนมัติ! ✨
        const fn level(&self) -> i32 {
            3  // ระดับความสำคัญ (เหมือนดาวมิชลิน!)
        }

        // Method ที่ return reference - โฆษกประจำตู้เก็บ! 📢
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("📣 ความสำคัญ (ประกาศจากโฆษก): {announcement}");
            self.part  // ส่งคืนข้อความสำคัญ (เหมือนโฆษกอ่านข่าว!)
        }

        // Method ที่มี multiple lifetime parameters - นักสืบเปรียบเทียบ! 🕵️‍♂️
        fn compare_with<'b>(&self, other: &'b str) -> &'a str {
            println!("🔍 เปรียบเทียบ '{}' กับ '{}' (เหมือนนักสืบเปรียบเทียบหลักฐาน!)", self.part, other);
            self.part  // เลือกข้อความของตัวเอง (เหมือนคนที่มั่นใจในตัวเอง!)
        }
    }

    let novel = String::from("เรียกฉันว่าอิชมาเอล. เมื่อหลายปีก่อน ไม่ต้องสนใจว่าเมื่อไหร่แน่ๆ...");

    let first_sentence = novel.split('.').next().expect("ไม่พบประโยค");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
        page: 1,
        chapter: 1,
    };

    println!("\n📚 === ข้อความสำคัญ: ตู้เก็บข้อความโบราณ! === 📚");
    println!("📖 Excerpt: {excerpt:?}");
    println!("⭐ ระดับความสำคัญ: {}", excerpt.level());

    let announcement = "นี่คือข้อความที่สำคัญมาก!";
    let returned_part = excerpt.announce_and_return_part(announcement);
    println!("📝 ส่วนที่ส่งคืน: {returned_part}");

    let other_text = "ข้อความอื่น";
    let compared = excerpt.compare_with(other_text);
    println!("🔍 ผลการเปรียบเทียบ: {compared}");

    // Struct ที่มี multiple references - สำนักงานรีวิวหนังสือ! 📝⭐
    #[derive(Debug)]
    struct BookReview<'a, 'b> {
        title: &'a str,    // ชื่อหนังสือ (มีอายุขัย 'a)
        content: &'b str,  // เนื้อหารีวิว (มีอายุขัย 'b)
        rating: u8,        // คะแนน (0-5 ดาว เหมือนรีวิวร้านอาหาร!)
    }

    impl<'a, 'b> BookReview<'a, 'b> {
        const fn new(title: &'a str, content: &'b str, rating: u8) -> Self {
            BookReview {
                title,
                content,
                rating,
            }
        }

        fn summary(&self) -> String {
            format!("📚 '{}' - คะแนน: {}/5 ⭐", self.title, self.rating)
        }

        fn full_review(&self) -> String {
            format!(
                "📚 หนังสือ: {}\n📝 รีวิว: {}\n⭐ คะแนน: {}/5",
                self.title, self.content, self.rating
            )
        }
    }

    println!("\n📚 === รีวิวหนังสือ: สำนักงานรีวิวมืออาชีพ! === 📚");

    let book_title = "The Rust Programming Language";
    let review_content = "หนังสือที่ยอดเยี่ยมสำหรับการเรียนรู้ Rust ตั้งแต่พื้นฐานจนถึงขั้นสูง";

    let review = BookReview::new(book_title, review_content, 5);

    println!("📋 สรุป: {}", review.summary());
    println!("\n📄 รีวิวเต็ม:\n{}", review.full_review());

    // Nested structs with lifetimes - ระบบจัดเก็บข้อมูลนักเขียน! 👨‍💼📚
    #[derive(Debug)]
    struct Author<'a> {
        // Fields ใช้เพื่อแสดงตัวอย่างการใช้ lifetime ใน nested structs
        // Warning: fields ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        name: &'a str,  // ชื่อนักเขียน (เหมือนป้ายชื่อ!)
        #[allow(dead_code)]
        bio: &'a str,   // ประวัตินักเขียน (เหมือน CV!)
    }

    #[derive(Debug)]
    struct Book<'a, 'b> {
        // Fields ใช้เพื่อแสดงตัวอย่างการใช้ multiple lifetimes
        // Warning: fields ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        title: &'a str,      // ชื่อหนังสือ (มีอายุขัย 'a)
        #[allow(dead_code)]
        author: Author<'b>,  // ข้อมูลนักเขียน (มีอายุขัย 'b)
        #[allow(dead_code)]
        isbn: &'a str,       // รหัส ISBN (มีอายุขัย 'a เหมือนชื่อหนังสือ!)
    }

    println!("\n📖 === ข้อมูลหนังสือ: ห้องสมุดดิจิทัล! === 📖");

    let author_name = "Steve Klabnik และ Carol Nichols";
    let author_bio = "ผู้เขียนหนังสือ Rust ที่มีชื่อเสียง";

    let author = Author {
        name: author_name,
        bio: author_bio,
    };

    let book_title2 = "The Rust Programming Language";
    let book_isbn = "978-1593278281";

    let book = Book {
        title: book_title2,
        author,
        isbn: book_isbn,
    };

    println!("📚 หนังสือ: {book:?}");
}
