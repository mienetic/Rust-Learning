// 🦀 Module System: Re-exporting และ Module Organization 🦀
// เหมือนการจัดระเบียบบ้าน แต่สำหรับโค้ด! 🏠✨

/// ฟังก์ชันสำหรับสอนเรื่อง Re-exporting และ Module Organization
/// เหมือนเป็นนักจัดระเบียบมืออาชีพ แต่สำหรับโมดูล! 📦🎯
#[allow(clippy::too_many_lines)]
pub fn learn_reexporting() {
    println!("\n🔄 === Re-exporting และ Module Organization === 🔄");

    // 📚 ห้องสมุดแห่งความรู้ที่จัดระเบียบอย่างเป็นระบบ
    mod library {
        // 🧮 โมดูลคณิตศาสตร์ - เพื่อนรักของนักคิดเลข
        pub mod math {
            /// ฟังก์ชันบวก - ง่ายๆ แต่ใช้ได้ทุกที่! ➕
            pub const fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            /// ฟังก์ชันคูณ - ทำให้ตัวเลขโตขึ้นเร็วๆ! ✖️
            pub const fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }

            /// ฟังก์ชันยกกำลัง - พลังแห่งการคูณซ้ำๆ! 🚀
            pub const fn power(base: i32, exp: u32) -> i32 {
                base.pow(exp)
            }
        }

        // 📝 โมดูลจัดการข้อความ - ช่วยให้ข้อความสวยงามขึ้น
        pub mod string_utils {
            /// กลับข้อความ - เหมือนอ่านในกระจก! 🪞
            pub fn reverse(s: &str) -> String {
                s.chars().rev().collect()
            }

            /// นับจำนวนคำ - เหมือนมีเลขานุการส่วนตัว! 📊
            pub fn count_words(s: &str) -> usize {
                s.split_whitespace().count()
            }

            /// แปลงเป็น Title Case - ทำให้ดูเป็นทางการขึ้น! 🎩
            pub fn to_title_case(s: &str) -> String {
                s.split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                // ตัวแรกใหญ่ ที่เหลือเล็ก - กฎทองของ Title Case!
                                first.to_uppercase().collect::<String>()
                                    + &chars.as_str().to_lowercase()
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }

        // 📊 โมดูลจัดการคอลเลกชัน - นักสถิติตัวจิ๋ว
        pub mod collections {
            /// หาค่าสูงสุด - เหมือนหาดาวเด่นในกลุ่ม! ⭐
            pub fn find_max(numbers: &[i32]) -> Option<i32> {
                numbers.iter().max().copied()
            }

            /// หาค่าต่ำสุด - เหมือนหาคนเก็บตัวในปาร์ตี้! 🙈
            pub fn find_min(numbers: &[i32]) -> Option<i32> {
                numbers.iter().min().copied()
            }

            /// รวมทุกอย่าง - เหมือนรวมเงินในกระปุก! 🏦
            pub fn sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }

            /// คำนวณค่าเฉลี่ย - ความยุติธรรมในรูปแบบตัวเลข! ⚖️
            pub fn average(numbers: &[i32]) -> Option<f64> {
                if numbers.is_empty() {
                    None // ไม่มีข้อมูล = ไม่มีค่าเฉลี่ย (เหมือนหารศูนย์!)
                } else {
                    Some(f64::from(numbers.iter().sum::<i32>()) / numbers.len() as f64)
                }
            }
        }

        // 🔄 Re-export สำหรับ API ที่สะดวกใช้ - เหมือนทางลัดในห้างสรรพสินค้า!
        pub use collections as stats; // เปลี่ยนชื่อให้ฟังดูเท่ขึ้น! 📈
        pub use math::{add, multiply}; // Re-export เฉพาะฟังก์ชันที่ใช้บ่อย - คัดสรรแล้ว! ⭐
        pub use string_utils::*; // Re-export ทั้งหมด - เอาหมดเลย! 🎁
    }

    println!("\n🧮 === การใช้ Re-exported Functions === 🧮");

    // ใช้ฟังก์ชันที่ re-export แล้ว - สะดวกเหมือนมีรีโมทคอนโทรล! 🎮
    let sum_result = library::add(10, 20);
    let product_result = library::multiply(5, 6);

    println!("➕ 10 + 20 = {sum_result}");
    println!("✖️ 5 × 6 = {product_result}");

    // ใช้ฟังก์ชันที่ไม่ได้ re-export (ต้องใช้ path เต็ม) - เหมือนเดินทางไกลหน่อย! 🚶‍♂️
    let power_result = library::math::power(2, 8);
    println!("🔢 2^8 = {power_result}");

    // ใช้ string utilities ที่ re-export ด้วย * - เหมือนได้ของแถมทั้งชุด! 🎁
    let text = "hello rust world";
    let reversed = library::reverse(text); // กลับข้อความเหมือนเทปย้อนกลับ! 📼
    let word_count = library::count_words(text); // นับคำเหมือนนักข่าวมืออาชีพ! 📰
    let title_case = library::to_title_case(text); // แต่งตัวอักษรให้เป็นทางการ! 👔

    println!("\n📝 === String Utilities === 📝");
    println!("📄 ข้อความต้นฉบับ: {text}");
    println!("🔄 กลับด้าน: {reversed}");
    println!("🔢 จำนวนคำ: {word_count}");
    println!("🎩 Title Case: {title_case}");

    // ใช้ collections ที่ re-export เป็น stats - เปลี่ยนชื่อให้ฟังดูเป็นมืออาชีพ! 📊
    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];

    println!("\n📊 === Statistics === 📊");
    println!("🔢 ตัวเลข: {numbers:?}");

    // หาค่าสูงสุด - ใครเป็นแชมป์ในกลุ่มนี้? 🏆
    if let Some(max) = library::stats::find_max(&numbers) {
        println!("📈 ค่าสูงสุด: {max}");
    }

    // หาค่าต่ำสุด - ใครเป็นคนเก็บตัวที่สุด? 🙈
    if let Some(min) = library::stats::find_min(&numbers) {
        println!("📉 ค่าต่ำสุด: {min}");
    }

    // รวมทุกอย่าง - เหมือนรวมเงินในกระปุก! 💰
    let total = library::stats::sum(&numbers);
    println!("➕ ผลรวม: {total}");

    // คำนวณค่าเฉลี่ย - ความยุติธรรมในรูปแบบตัวเลข! ⚖️
    if let Some(avg) = library::stats::average(&numbers) {
        println!("📊 ค่าเฉลี่ย: {avg:.2}");
    }

    // ตัวอย่างการจัดระเบียบ module แบบ hierarchical - เหมือนแผนผังองค์กร! 🏢
    mod app {
        // 🎨 โมดูล UI - ศูนย์รวมความสวยงาม
        pub mod ui {
            // 🧩 คอมโพเนนต์ต่างๆ - ชิ้นส่วนสำคัญของ UI
            pub mod components {
                /// ปุ่มกด - คลิกแล้วเกิดเวทมนตร์! ✨
                pub fn button() -> String {
                    String::from("🔘 Button Component")
                }

                /// ช่องใส่ข้อมูล - ประตูสู่การสื่อสาร! 📬
                pub fn input() -> String {
                    String::from("📝 Input Component")
                }
            }

            // 🏗️ เลย์เอาต์ต่างๆ - โครงสร้างหลักของหน้าเว็บ
            pub mod layouts {
                /// ส่วนหัว - หน้าตาแรกที่เห็น! 👑
                pub fn header() -> String {
                    String::from("📋 Header Layout")
                }

                /// ส่วนท้าย - จบด้วยดี! 🎬
                pub fn footer() -> String {
                    String::from("📄 Footer Layout")
                }
            }

            // 🔄 Re-export สำหรับ convenience - ทางลัดสู่ความสะดวก!
            pub use components::*; // เอาคอมโพเนนต์มาทั้งหมด! 🎁
            pub use layouts::{footer, header}; // เลือกเฉพาะที่ต้องการ! 🎯
        }

        // 🌐 โมดูล API - ศูนย์กลางการสื่อสาร
        pub mod api {
            // 🔐 ระบบยืนยันตัวตน - ผู้พิทักษ์ประตู
            pub mod auth {
                /// เข้าสู่ระบบ - ยินดีต้อนรับ! 🎉
                pub fn login() -> String {
                    String::from("🔐 Login API")
                }

                /// ออกจากระบบ - ไปด้วยดี! 👋
                pub fn logout() -> String {
                    String::from("🚪 Logout API")
                }
            }

            // 📊 จัดการข้อมูล - คลังข้อมูลแห่งความรู้
            pub mod data {
                /// ดึงข้อมูลผู้ใช้ - เรียกทัพมาประชุม! 👥
                pub fn fetch_users() -> String {
                    String::from("👥 Fetch Users API")
                }

                /// บันทึกผู้ใช้ - เก็บไว้ในหีบสมบัติ! 💎
                pub fn save_user() -> String {
                    String::from("💾 Save User API")
                }
            }

            // 🔄 Re-export ทั้งหมด - เปิดประตูให้หมด!
            pub use auth::*; // ระบบรักษาความปลอดภัยทั้งชุด! 🛡️
            pub use data::*; // ข้อมูลทั้งหมด! 📚
        }

        // 🔄 Re-export หลัก - ประตูใหญ่สู่ทุกฟีเจอร์!
        pub use api::*; // API ทั้งหมด! 🌐
        pub use ui::*; // UI ทั้งหมด! 🎨
    }

    println!("\n🏗️ === App Module Structure === 🏗️");

    // ใช้ components ผ่าน re-export - สะดวกเหมือนมีรีโมทคอนโทรล! 🎮
    println!("🎨 UI Components:");
    println!("  {}", app::button()); // ปุ่มวิเศษ! ✨
    println!("  {}", app::input()); // ช่องใส่ข้อมูล! 📝
    println!("  {}", app::header()); // ส่วนหัวสวยงาม! 👑
    println!("  {}", app::footer()); // ส่วนท้ายเรียบร้อย! 🎬

    println!("\n🌐 API Functions:");
    println!("  {}", app::login()); // เข้าสู่ระบบ! 🔐
    println!("  {}", app::logout()); // ออกจากระบบ! 🚪
    println!("  {}", app::fetch_users()); // ดึงข้อมูลผู้ใช้! 👥
    println!("  {}", app::save_user()); // บันทึกผู้ใช้! 💾

    // ยังสามารถเข้าถึงผ่าน path เต็มได้ - เดินทางแบบละเอียด! 🗺️
    println!("\n🛤️ === Full Path Access === 🛤️");
    println!("📱 {}", app::ui::components::button()); // เส้นทางเต็มสู่ปุ่ม!
    println!("🔐 {}", app::api::auth::login()); // เส้นทางเต็มสู่การล็อกอิน!
}
