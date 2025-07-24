//! Practice Structs and Enums - แบบฝึกหัดการใช้งาน Structs และ Enums
//!
//! ไฟล์นี้มีตัวอย่างการใช้งาน Structs และ Enums ในสถานการณ์จริง
//! เช่น ระบบจัดการหนังสือ, เครื่องคิดเลข และระบบให้คะแนนนักเรียน
//! (สวนสนุกแห่งการเขียนโปรแกรม! 🎢)

/// ฟังก์ชันสำหรับฝึกฝน Structs และ Enums
/// มาทำแบบฝึกหัดกันเถอะ! (ยิมสำหรับสมองโปรแกรมเมอร์! 🧠💪)
#[allow(clippy::too_many_lines)]
pub fn practice_structs_and_enums() {
    println!("\n💪 === แบบฝึกหัด Structs และ Enums: สวนสนุกโปรแกรม! === 💪");

    // 1. Book Management System (ห้องสมุดแห่งอนาคต! 📚)
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Book {
        title: String,     // ชื่อเรื่องสุดเท่! 📖
        author: String,    // นักเขียนผู้ยิ่งใหญ่! ✍️
        pages: u32,        // จำนวนหน้าความรู้! 📄
        available: bool,   // สถานะพร้อมใช้! 🟢
    }

    impl Book {
        const fn new(title: String, author: String, pages: u32) -> Self {  // โรงงานผลิตหนังสือ! 🏭
            Self {
                title,
                author,
                pages,
                available: true,  // เกิดมาพร้อมใช้! ✨
            }
        }

        fn borrow_book(&mut self) -> Result<(), String> {  // เจ้าหน้าที่ให้ยืม! 👩‍💼
            if self.available {
                self.available = false;  // ติดป้าย "ไม่ว่าง"! 🚫
                Ok(())  // ยืมได้! 🎉
            } else {
                Err(String::from("หนังสือไม่ว่าง"))  // ขออภัย เต็มแล้ว! 😅
            }
        }

        const fn return_book(&mut self) {  // เจ้าหน้าที่รับคืน! 👨‍💼
            self.available = true;  // ติดป้าย "ว่าง"! ✅
        }
    }

    println!("📚 === ระบบจัดการหนังสือ: ห้องสมุดแห่งอนาคต! === 📚");
    let mut book1 = Book::new(
        String::from("The Rust Programming Language"),  // คัมภีร์แห่งการเขียนโปรแกรม! 📜
        String::from("Steve Klabnik"),                  // อาจารย์ใหญ่! 🧙‍♂️
        552,                                            // หน้าเพียบ! 📚
    );

    println!("📖 หนังสือ: {book1:?} (สมบัติล้ำค่า! 💎)");

    match book1.borrow_book() {  // ลูกค้าคนแรก! 👤
        Ok(()) => println!("✅ ยืมหนังสือสำเร็จ (โชคดี! 🍀)"),
        Err(e) => println!("❌ {e} (เสียใจด้วย! 😢)"),
    }

    match book1.borrow_book() {  // ลูกค้าคนที่สอง! 👥
        Ok(()) => println!("✅ ยืมหนังสือสำเร็จ (โชคดี! 🍀)"),
        Err(e) => println!("❌ {e} (มาช้าไป! ⏰)"),
    }

    book1.return_book();  // คืนหนังสือ! 📚
    println!("📚 คืนหนังสือแล้ว (ขอบคุณค่ะ! 🙏)");

    // 2. Calculator with Enums (เครื่องคิดเลขแห่งอนาคต! 🧮)
    #[derive(Debug)]
    enum Operation {
        Add(f64, f64),      // บวกสนุก! ➕
        Subtract(f64, f64), // ลบเก่ง! ➖
        Multiply(f64, f64), // คูณเท่! ✖️
        Divide(f64, f64),   // หารเจ๋ง! ➗
    }

    impl Operation {
        fn calculate(&self) -> Result<f64, String> {  // นักคณิตศาสตร์ตัวจิ๋ว! 🤓
            match self {
                Self::Add(a, b) => Ok(a + b),      // บวกง่ายๆ! 🎯
                Self::Subtract(a, b) => Ok(a - b), // ลบสบาย! 😌
                Self::Multiply(a, b) => Ok(a * b), // คูณคล่อง! 🚀
                Self::Divide(a, b) => {           // หารระวัง! ⚠️
                    if *b == 0.0 {
                        Err(String::from("ไม่สามารถหารด้วยศูนย์ได้"))  // อันตราย! 💥
                    } else {
                        Ok(a / b)  // หารได้! 🎉
                    }
                }
            }
        }
    }

    println!("\n🧮 === เครื่องคิดเลข: ศูนย์รวมคณิตศาสตร์! === 🧮");
    let operations = vec![
        Operation::Add(10.0, 5.0),      // บวกง่ายๆ! 🎯
        Operation::Subtract(10.0, 3.0), // ลบสบาย! 😌
        Operation::Multiply(4.0, 7.0),  // คูณคล่อง! 🚀
        Operation::Divide(15.0, 3.0),   // หารลื่น! ✨
        Operation::Divide(10.0, 0.0),   // หารอันตราย! 💣 (จะเกิดอะไรขึ้น?)
    ];

    for op in operations {
        match op.calculate() {  // ทดสอบความสามารถ! 🧪
            Ok(result) => println!("✅ {op:?} = {result} (เก่งมาก! 🌟)"),
            Err(error) => println!("❌ {op:?} -> {error} (โอ๊ะโอ! 😱)"),
        }
    }

    // 3. Student Grading System (ระบบให้คะแนนนักเรียน! 🎓)
    #[derive(Debug)]
    enum Grade {
        A,  // เทพ! 👑
        B,  // เก่ง! 🌟
        C,  // โอเค! 👍
        D,  // อุ๊ปส์! 😅
        F,  // อ๊ะ! 😱
    }

    impl Grade {
        const fn from_score(score: u8) -> Self {  // เครื่องแปลงคะแนน! 🔄
            match score {
                90..=100 => Self::A,  // เทพระดับ! 🏆
                80..=89 => Self::B,   // เก่งมาก! 🥈
                70..=79 => Self::C,   // ผ่านได้! 🥉
                60..=69 => Self::D,   // เฮ้อ... 😓
                _ => Self::F,         // อ๊ะ... 💀
            }
        }

        const fn to_gpa(&self) -> f32 {  // แปลงเป็น GPA! 📊
            match self {
                Self::A => 4.0,  // เต็ม! 💯
                Self::B => 3.0,  // ดี! 😊
                Self::C => 2.0,  // โอเค! 😐
                Self::D => 1.0,  // อุ๊ย! 😬
                Self::F => 0.0,  // โอ๊ะโอ! 😵
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Student {
        name: String,     // ชื่อนักเรียน! 👨‍🎓
        scores: Vec<u8>,  // คลังคะแนน! 📚
    }

    impl Student {
        const fn new(name: String) -> Self {  // โรงงานผลิตนักเรียน! 🏭
            Self {
                name,
                scores: Vec::new(),  // เริ่มต้นด้วยคะแนนเปล่า! 📝
            }
        }

        fn add_score(&mut self, score: u8) {  // เพิ่มคะแนน! ➕
            self.scores.push(score);  // ใส่คะแนนเข้าไป! 📊
        }

        fn calculate_gpa(&self) -> f32 {  // คำนวณ GPA! 🧮
            if self.scores.is_empty() {
                return 0.0;  // ไม่มีคะแนน = 0! 😅
            }

            let total_gpa: f32 = self
                .scores
                .iter()
                .map(|&score| Grade::from_score(score).to_gpa())  // รวมคะแนน GPA! 📈
                .sum();

            total_gpa / self.scores.len() as f32  // หาค่าเฉลี่ย! 📊
        }

        fn get_grades(&self) -> Vec<Grade> {  // แปลงคะแนนเป็นเกรด! 🔄
            self.scores
                .iter()
                .map(|&score| Grade::from_score(score))  // เวทมนตร์แปลงคะแนน! ✨
                .collect()
        }
    }

    println!("\n🎓 === ระบบให้คะแนนนักเรียน: โรงเรียนแห่งความสนุก! === 🎓");
    let mut student = Student::new(String::from("สมชาย"));  // นักเรียนตัวอย่าง! 👨‍🎓

    student.add_score(85);  // คะแนนดี! 😊
    student.add_score(92);  // เก่งมาก! 🌟
    student.add_score(78);  // โอเคนะ! 👍
    student.add_score(88);  // เยี่ยม! 🎉

    println!("👤 นักเรียน: {student:?} (นักเรียนดีเด่น! ⭐)");
    println!("📊 เกรด: {:?} (ผลงานสุดเจ๋ง! 🏆)", student.get_grades());
    println!("🎯 GPA: {:.2} (คะแนนความเก่ง! 📈)", student.calculate_gpa());

    println!("\n🎉 จบแบบฝึกหัด Structs และ Enums: ขอแสดงความยินดี! 🎊");
    println!("🏆 คุณได้เรียนรู้การใช้งาน Structs และ Enums แล้ว! (เก่งมาก! 👏)");
}
