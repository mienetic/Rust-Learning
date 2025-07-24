//! Practice Modules - Library Management System: ห้องสมุดแห่งอนาคต! 📚🚀
//! ระบบจัดการห้องสมุดที่ใช้ Module System ของ Rust (เป็นบรรณารักษ์ดิจิทัล!)

// ใช้ #[allow(clippy::too_many_lines)] เพราะฟังก์ชันนี้เป็นตัวอย่างระบบจัดการห้องสมุด
// ที่ครอบคลุมหลายแนวคิดของ Module System ในไฟล์เดียว เพื่อความเข้าใจที่ต่อเนื่อง
// (เป็นห้องสมุดยักษ์ในไฟล์เดียว! 🏛️)
#[allow(clippy::too_many_lines)]
pub fn practice_modules() {
    // ระบบจัดการห้องสมุด - แสดงการใช้ Module System ในโปรเจคจริง (เป็นสถาปนิกห้องสมุด! 🏗️)
    mod library_system {  // ระบบห้องสมุด: เมืองแห่งหนังสือ! 🏙️📚
        pub mod books {  // โมดูลหนังสือ: คลังสมบัติแห่งความรู้! 💎📖
            #[derive(Debug, Clone)]  // ทำให้หนังสือพิมพ์และคัดลอกได้! 🖨️📄
            pub struct Book {  // โครงสร้างหนังสือ: DNA ของหนังสือ! 🧬
                pub id: u32,  // รหัสหนังสือ: บัตรประชาชนของหนังสือ! 🆔
                pub title: String,  // ชื่อหนังสือ: ป้ายชื่อเจ้าของ! 📛
                pub author: String,  // ผู้แต่ง: พ่อแม่ของหนังสือ! 👨‍💼
                // Field isbn ใช้เพื่อแสดงตัวอย่างการเก็บข้อมูลหนังสือ
                // ใช้ #[allow(dead_code)] เพราะ field นี้เป็นส่วนหนึ่งของโครงสร้างข้อมูลหนังสือ
                // แต่ไม่ได้ใช้ในตัวอย่างนี้ เก็บไว้เพื่อแสดงโครงสร้างข้อมูลที่สมบูรณ์
                // (รหัสลับของหนังสือ! 🔢)
                #[allow(dead_code)]
                pub isbn: String,  // ISBN: รหัสพันธุกรรมของหนังสือ! 🧬
                pub available: bool,  // สถานะว่าง: ไฟเขียว-แดง! 🚦
            }

            impl Book {  // การใช้งานหนังสือ: คู่มือการใช้งาน! 📋
                pub fn new(id: u32, title: &str, author: &str, isbn: &str) -> Self {  // สร้างหนังสือใหม่: โรงงานผลิตหนังสือ! 🏭
                    Self {
                        id,  // ใส่รหัส! 🏷️
                        title: title.to_string(),  // ใส่ชื่อ! 📝
                        author: author.to_string(),  // ใส่ผู้แต่ง! ✍️
                        isbn: isbn.to_string(),  // ใส่รหัส ISBN! 🔢
                        available: true,  // เริ่มต้นว่าง! ✅
                    }
                }

                pub fn borrow_book(&mut self) -> Result<(), String> {  // ยืมหนังสือ: เช่าหนังสือ! 🤝
                    if self.available {  // ถ้าว่าง! 🟢
                        self.available = false;  // เปลี่ยนเป็นไม่ว่าง! 🔴
                        Ok(())  // สำเร็จ! 🎉
                    } else {
                        Err(format!("หนังสือ '{}' ไม่ว่าง", self.title))  // ไม่ว่าง! ❌
                    }
                }

                pub const fn return_book(&mut self) {  // คืนหนังสือ: ส่งคืนบ้าน! 🏠
                    self.available = true;  // เปลี่ยนเป็นว่าง! 🟢
                }
            }

            pub fn search_by_title<'a>(books: &'a [Book], title: &str) -> Vec<&'a Book> {  // ค้นหาตามชื่อ: เป็นนักสืบหนังสือ! 🕵️‍♂️
                books
                    .iter()  // วนดูทุกเล่ม! 🔄
                    .filter(|book| book.title.to_lowercase().contains(&title.to_lowercase()))  // กรองตามชื่อ! 🔍
                    .collect()  // รวบรวมผลลัพธ์! 📦
            }

            pub fn search_by_author<'a>(books: &'a [Book], author: &str) -> Vec<&'a Book> {  // ค้นหาตามผู้แต่ง: หาผลงานนักเขียน! ✍️
                books
                    .iter()  // วนดูทุกเล่ม! 🔄
                    .filter(|book| book.author.to_lowercase().contains(&author.to_lowercase()))  // กรองตามผู้แต่ง! 👨‍💼
                    .collect()  // รวบรวมผลลัพธ์! 📦
            }
        }

        pub mod members {  // โมดูลสมาชิก: ชุมชนนักอ่าน! 👥📚
            #[derive(Debug, Clone)]  // ทำให้สมาชิกพิมพ์และคัดลอกได้! 🖨️👤
            pub struct Member {  // โครงสร้างสมาชิก: บัตรสมาชิกดิจิทัล! 💳
                pub id: u32,  // รหัสสมาชิก: หมายเลขประจำตัว! 🆔
                pub name: String,  // ชื่อ: ป้ายชื่อ! 📛
                pub email: String,  // อีเมล: ที่อยู่ดิจิทัล! 📧
                pub borrowed_books: Vec<u32>,  // หนังสือที่ยืม: กระเป๋าหนังสือ! 🎒 (Book IDs)
            }

            impl Member {  // การใช้งานสมาชิก: คู่มือสมาชิก! 📖
                pub fn new(id: u32, name: &str, email: &str) -> Self {  // สร้างสมาชิกใหม่: สมัครสมาชิก! 📝
                    Self {
                        id,  // ใส่รหัส! 🏷️
                        name: name.to_string(),  // ใส่ชื่อ! 📛
                        email: email.to_string(),  // ใส่อีเมล! 📧
                        borrowed_books: Vec::new(),  // กระเป๋าหนังสือเปล่า! 🎒
                    }
                }

                pub fn borrow_book(&mut self, book_id: u32) {  // ยืมหนังสือ: ใส่หนังสือในกระเป๋า! 📚➡️🎒
                    if !self.borrowed_books.contains(&book_id) {  // ถ้ายังไม่มี! ❌
                        self.borrowed_books.push(book_id);  // ใส่เข้าไป! ➕
                    }
                }

                pub fn return_book(&mut self, book_id: u32) {  // คืนหนังสือ: เอาหนังสือออกจากกระเป๋า! 🎒➡️📚
                    self.borrowed_books.retain(|&id| id != book_id);  // เก็บเฉพาะที่ไม่ใช่หนังสือนี้! 🗑️
                }

                // Method นี้ใช้เพื่อแสดงตัวอย่างการนับจำนวนหนังสือที่ยืม
                // ใช้ #[allow(dead_code)] เพราะ method นี้เป็นส่วนหนึ่งของ API ที่สมบูรณ์
                // แต่ไม่ได้ใช้ในตัวอย่างนี้ เก็บไว้เพื่อแสดงความสามารถของ struct
                // (เครื่องนับหนังสือในกระเป๋า! 🧮)
                #[allow(dead_code)]
                pub const fn get_borrowed_count(&self) -> usize {  // นับหนังสือ: เครื่องคิดเลข! 🧮
                    self.borrowed_books.len()  // ความยาวของกระเป๋า! 📏
                }
            }
        }

        pub mod transactions {  // โมดูลธุรกรรม: ธนาคารหนังสือ! 🏦📚
            use super::books::Book;  // นำเข้าหนังสือ! 📖
            use super::members::Member;  // นำเข้าสมาชิก! 👤

            pub fn borrow_book(member: &mut Member, book: &mut Book) -> Result<String, String> {  // ยืมหนังสือ: ธุรกรรมยืม! 🤝
                match book.borrow_book() {  // ลองยืม! 🎯
                    Ok(()) => {  // ถ้าสำเร็จ! ✅
                        member.borrow_book(book.id);  // เพิ่มในกระเป๋าสมาชิก! 🎒
                        Ok(format!("✅ {} ยืมหนังสือ '{}' สำเร็จ", member.name, book.title))  // ข้อความสำเร็จ! 🎉
                    }
                    Err(error) => Err(error),  // ถ้าล้มเหลว! ❌
                }
            }

            pub fn return_book(member: &mut Member, book: &mut Book) -> String {  // คืนหนังสือ: ธุรกรรมคืน! 🔄
                book.return_book();  // คืนหนังสือ! 📚
                member.return_book(book.id);  // เอาออกจากกระเป๋า! 🎒
                format!("📚 {} คืนหนังสือ '{}' แล้ว", member.name, book.title)  // ข้อความคืน! 🏠
            }

            pub fn get_member_books<'a>(member: &Member, books: &'a [Book]) -> Vec<&'a Book> {  // ดูหนังสือของสมาชิก: ตรวจกระเป๋า! 🎒👀
                books
                    .iter()  // วนดูทุกเล่ม! 🔄
                    .filter(|book| member.borrowed_books.contains(&book.id))  // กรองเฉพาะที่อยู่ในกระเป๋า! 🔍
                    .collect()  // รวบรวม! 📦
            }
        }

        // Re-exports สำหรับ convenience (ส่งออกใหม่เพื่อความสะดวก: ประตูลัด! 🚪)
        pub use books::{Book, search_by_author, search_by_title};  // ส่งออกหนังสือ! 📖
        pub use members::Member;  // ส่งออกสมาชิก! 👤
        pub use transactions::{borrow_book, get_member_books, return_book};  // ส่งออกธุรกรรม! 🏦
    }  // จบระบบห้องสมุด! 🏁

    println!("📚 === Library Management System: ห้องสมุดแห่งอนาคต! === 📚");

    // สร้างหนังสือ (เปิดโรงงานผลิตหนังสือ! 🏭)
    let mut books = vec![  // คลังหนังสือ! 📚
        library_system::Book::new(  // หนังสือเล่มที่ 1! 📖
            1,
            "The Rust Programming Language",
            "Steve Klabnik",
            "978-1593278281",
        ),
        library_system::Book::new(2, "Programming Rust", "Jim Blandy", "978-1491927281"),  // หนังสือเล่มที่ 2! 📗
        library_system::Book::new(3, "Rust in Action", "Tim McNamara", "978-1617294556"),  // หนังสือเล่มที่ 3! 📘
    ];

    // สร้างสมาชิก (เปิดรับสมัครสมาชิก! 📝)
    let mut members = vec![  // ชุมชนนักอ่าน! 👥
        library_system::Member::new(1, "อลิซ", "alice@example.com"),  // สมาชิกคนที่ 1! 👩
        library_system::Member::new(2, "บ็อบ", "bob@example.com"),  // สมาชิกคนที่ 2! 👨
    ];

    println!("\n📖 รายการหนังสือ: แคตตาล็อกหนังสือ! 📋");
    for book in &books {  // วนดูทุกเล่ม! 🔄
        println!(
            "  {}. '{}' โดย {} ({})",  // แสดงรายละเอียด! 📝
            book.id,
            book.title,
            book.author,
            if book.available {
                "ว่าง"  // ไฟเขียว! 🟢
            } else {
                "ไม่ว่าง"  // ไฟแดง! 🔴
            }
        );
    }

    println!("\n👥 รายการสมาชิก: ทะเบียนสมาชิก! 📋");
    for member in &members {  // วนดูทุกคน! 🔄
        println!("  {}. {} ({})", member.id, member.name, member.email);  // แสดงข้อมูลสมาชิก! 👤
    }

    // ทดสอบการยืม-คืน (เปิดธนาคารหนังสือ! 🏦)
    println!("\n📋 === การยืม-คืนหนังสือ: ธุรกรรมห้องสมุด! === 📋");

    match library_system::borrow_book(&mut members[0], &mut books[0]) {  // อลิซยืมหนังสือเล่มแรก! 👩📖
        Ok(message) => println!("📝 {message}"),  // สำเร็จ! ✅
        Err(error) => println!("❌ {error}"),  // ล้มเหลว! ❌
    }

    match library_system::borrow_book(&mut members[1], &mut books[1]) {  // บ็อบยืมหนังสือเล่มที่สอง! 👨📗
        Ok(message) => println!("📝 {message}"),  // สำเร็จ! ✅
        Err(error) => println!("❌ {error}"),  // ล้มเหลว! ❌
    }

    // ลองยืมหนังสือเล่มเดิมอีกครั้ง (ทดสอบระบบป้องกัน! 🛡️)
    match library_system::borrow_book(&mut members[1], &mut books[0]) {  // บ็อบลองยืมหนังสือของอลิซ! 👨➡️📖
        Ok(message) => println!("📝 {message}"),  // สำเร็จ! ✅
        Err(error) => println!("❌ {error}"),  // ล้มเหลว! ❌
    }

    // แสดงหนังสือที่สมาชิกยืม (ตรวจกระเป๋าหนังสือ! 🎒)
    println!("\n📚 === หนังสือที่ยืม: รายงานกระเป๋าหนังสือ! === 📚");
    for member in &members {  // วนดูทุกสมาชิก! 🔄
        let borrowed = library_system::get_member_books(member, &books);  // ดูหนังสือในกระเป๋า! 👀
        println!("👤 {} ยืม {} เล่ม:", member.name, borrowed.len());  // รายงานจำนวน! 📊
        for book in borrowed {  // วนดูทุกเล่มในกระเป๋า! 🔄
            println!("  - '{}'", book.title);  // แสดงชื่อหนังสือ! 📖
        }
    }

    // คืนหนังสือ (ธุรกรรมคืน! 🔄)
    println!("\n🔄 === การคืนหนังสือ: ส่งคืนบ้าน! === 🔄");
    let return_message = library_system::return_book(&mut members[0], &mut books[0]);  // อลิซคืนหนังสือ! 👩📚
    println!("📝 {return_message}");  // แสดงข้อความคืน! 📝

    // ค้นหาหนังสือ (เป็นนักสืบห้องสมุด! 🕵️‍♂️)
    println!("\n🔍 === การค้นหา: ภารกิจนักสืบ! === 🔍");
    let rust_books = library_system::search_by_title(&books, "rust");  // ค้นหาหนังสือ Rust! 🦀
    println!("📚 หนังสือที่มีคำว่า 'rust': {} เล่ม (พบสมบัติ!)", rust_books.len());
    for book in rust_books {  // แสดงผลการค้นหา! 📋
        println!("  - '{}' โดย {} (เจอแล้ว!)", book.title, book.author);
    }

    let steve_books = library_system::search_by_author(&books, "steve");  // ค้นหาผลงาน Steve! 👨‍💼
    println!("\n👨‍💼 หนังสือของ Steve: {} เล่ม (ผลงานมาสเตอร์!)", steve_books.len());
    for book in steve_books {  // แสดงผลงาน! 📚
        println!("  - '{}' (ผลงานเด็ด!)", book.title);
    }

    println!("\n🎉 จบแบบฝึกหัด Module System! (จบการผจญภัยห้องสมุด! 🏆)");
}

#[cfg(test)]  // การทดสอบ: ห้องแล็บทดลอง! 🧪
mod tests {  // โมดูลทดสอบ: ห้องทดสอบ! 🔬
    // ลบ unused import super::* เนื่องจากไม่ได้ใช้งาน (ทำความสะอาด! 🧹)

    #[test]  // ทดสอบที่ 1: ทดสอบการมองเห็น! 👀
    fn test_module_visibility() {  // ทดสอบการมองเห็นโมดูล! 🔍
        mod test_mod {  // โมดูลทดสอบ! 🧪
            pub fn public_fn() -> i32 {  // ฟังก์ชันสาธารณะ! 🔓
                private_fn() + 10  // เรียกฟังก์ชันส่วนตัว + 10! ➕
            }

            fn private_fn() -> i32 {  // ฟังก์ชันส่วนตัว! 🔒
                42  // ตัวเลขมหัศจรรย์! ✨
            }
        }

        assert_eq!(test_mod::public_fn(), 52);  // ตรวจสอบผลลัพธ์! ✅
    }

    #[test]  // ทดสอบที่ 2: ทดสอบ use statements! 📥
    fn test_use_statements() {  // ทดสอบการใช้ use! 🎯
        mod math {  // โมดูลคณิตศาสตร์! 🧮
            pub fn add(a: i32, b: i32) -> i32 {  // ฟังก์ชันบวก! ➕
                a + b  // บวกกัน! 🔢
            }

            pub fn multiply(a: i32, b: i32) -> i32 {  // ฟังก์ชันคูณ! ✖️
                a * b  // คูณกัน! 🔢
            }
        }

        use math::{add, multiply};  // นำเข้าฟังก์ชัน! 📥

        assert_eq!(add(2, 3), 5);  // ทดสอบการบวก! ✅
        assert_eq!(multiply(4, 5), 20);  // ทดสอบการคูณ! ✅
    }

    #[test]  // ทดสอบที่ 3: ทดสอบ re-export! 🔄
    fn test_reexporting() {  // ทดสอบการส่งออกใหม่! 📤
        mod library {  // โมดูลห้องสมุด! 📚
            mod internal {  // โมดูลภายใน! 🏠
                pub fn helper() -> String {  // ฟังก์ชันช่วย! 🤝
                    String::from("helper")  // คืนค่า "helper"! 📝
                }
            }

            pub use internal::helper;  // Re-export (ส่งออกใหม่! 🔄)
        }

        assert_eq!(library::helper(), "helper");  // ทดสอบการส่งออกใหม่! ✅
    }
}  // จบการทดสอบ! 🏁
