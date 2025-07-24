// 🦀 Module System: Use Statements และ Paths 🦀
// เหมือนการสร้างทางลัดในโลกของโค้ดที่เร็วกว่า Fast & Furious! 🛤️✨🏎️

/// ฟังก์ชันสำหรับสอนเรื่อง Use Statements และ Paths
/// เรียนรู้การใช้ทางลัดแบบมืออาชีพที่เทพกว่า Google Maps! 🎯🚀🗺️
pub fn learn_use_statements() {
    println!("\n🛤️ === Use Statements และ Paths: ทางลัดที่เจ๋งกว่า Waze! === 🛤️");

    // สร้าง module สำหรับตัวอย่าง - ร้านอาหารในฝันที่ Michelin ต้องให้ดาว! 🍽️✨⭐
    mod restaurant {
        // 🏪 ส่วนหน้าร้าน - ที่ลูกค้าเห็นและสัมผัสแบบ Disney World! 🎢
        pub mod front_of_house {
            // 👥 แผนกต้อนรับ - ยิ้มแย้มแจ่มใสกว่าพิธีกรรายการเช้า! 📺
            pub mod hosting {
                /// เพิ่มลูกค้าในรายการรอ - ไม่ให้ใครหิวแม้แต่นาทีเดียว! 😋⏰
                pub fn add_to_waitlist() {
                    println!("👥 เพิ่มลูกค้าในรายการรอ (VIP treatment!)");
                }

                /// จัดที่นั่งให้ลูกค้า - บริการระดับ 5 ดาวที่ Ritz Carlton ต้องอิจฉา! ⭐👑
                pub fn seat_at_table() {
                    println!("🪑 จัดที่นั่งให้ลูกค้า (โต๊ะพิเศษสุด!)");
                }
            }

            // 🍽️ แผนกเสิร์ฟ - ศิลปะแห่งการบริการที่ Gordon Ramsay ต้องยกนิ้วให้! 👨‍🍳👍
            pub mod serving {
                /// รับออเดอร์ - ฟังด้วยใจ จดด้วยมือแบบนักข่าว CNN! 📝📺
                pub fn take_order() {
                    println!("📝 รับออเดอร์ (จดละเอียดกว่าสมุดไดอารี่!)");
                }

                /// เสิร์ฟอาหาร - ส่งความอร่อยถึงโต๊ะเร็วกว่า Flash! 🚀⚡
                pub fn serve_order() {
                    println!("🍽️ เสิร์ฟอาหาร (ร้อนๆ เหมือนเพิ่งออกจากเตา!)");
                }

                /// รับชำระเงิน - จบด้วยรอยยิ้มที่หวานกว่าของหวาน! 😊🍰
                pub fn take_payment() {
                    println!("💰 รับชำระเงิน (ยิ้มแย้มจนลูกค้าอยากกลับมาอีก!)");
                }
            }
        }

        // 🍳 ส่วนหลังร้าน - ห้องครัวแห่งความมหัศจรรย์ที่ Harry Potter ต้องอิจฉา! 🪄✨
        pub mod back_of_house {
            /// โครงสร้างอาหารเช้า - เริ่มต้นวันใหม่ด้วยความอร่อยที่ดีกว่าการตื่นมาเจอเงินล้าน! 🌅💰
            pub struct Breakfast {
                pub toast: String, // ขนมปัง - เปลี่ยนได้ตามใจชอบแบบ Netflix playlist! 🍞📺
                seasonal_fruit: String, // ผลไม้ตามฤดู - เชฟเลือกให้แบบ personal shopper! 🍑🛍️
            }

            impl Breakfast {
                /// สร้างอาหารเช้าแบบฤดูร้อน - สดชื่นเหมือนลมเซาะที่เกาะมัลดีฟส์! ☀️🏝️
                pub fn summer(toast: &str) -> Self {
                    Self {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("ลูกพีช"), // หวานฉ่ำกว่าความรักแรกพบ! 🍑💕
                    }
                }

                /// ดูผลไม้ตามฤดู - ความลับของเชฟที่ลับกว่าสูตร KFC! 🤫🍗
                pub fn get_seasonal_fruit(&self) -> &str {
                    &self.seasonal_fruit
                }
            }

            /// อาหารเรียกน้ำย่อย - เปิดม่านความอร่อยแบบ Broadway show! 🎭🎬
            #[derive(Debug)]
            pub enum Appetizer {
                Soup,  // ซุป - อุ่นใจอุ่นท้องกว่าอ้อมกอดแม่! 🍲🤗
                Salad, // สลัด - สดใสเหมือนสายลมที่เกาะสันโตรินี! 🥗🇬🇷
            }

            /// แก้ไขออเดอร์ที่ผิด - เชฟไม่เคยยอมแพ้แบบ Rocky Balboa! 💪🥊
            pub fn fix_incorrect_order() {
                cook_order(); // ทำใหม่ด้วยใจที่ร้อนแรงกว่าเตาอบ! ❤️🔥
                super::front_of_house::serving::serve_order(); // ส่งออกไปด้วยความภาคภูมิแบบ Olympic champion! 🚀🏆
            }

            /// ทำอาหาร - ศิลปะแห่งการปรุงที่ Picasso ต้องอิจฉา! 🎨👨‍🎨
            fn cook_order() {
                println!("👨‍🍳 ทำอาหาร (masterpiece กำลังเกิดขึ้น!)");
            }
        }

        /// ฟังก์ชันหลักของร้านอาหาร - ประสบการณ์การรับประทานอาหารที่สมบูรณ์แบบกว่า fine dining! 🍽️✨👑
        pub fn eat_at_restaurant() {
            // Relative path - เส้นทางแบบญาติๆ (ใช้จากตำแหน่งปัจจุบันแบบ GPS ในบ้าน!) 🏠📍
            front_of_house::hosting::add_to_waitlist();

            // Absolute path - เส้นทางแบบสัมบูรณ์ (เริ่มจาก crate root แบบ Google Maps!) 🌍🗺️
            // Note: Cannot use absolute path for local module in function scope
            // self::restaurant::front_of_house::hosting::add_to_waitlist();

            // สั่งอาหารเช้า - เริ่มต้นวันใหม่ด้วยความอร่อยแบบ sunrise breakfast! 🌅🥞
            let mut meal = back_of_house::Breakfast::summer("ขนมปังโฮลวีต");
            meal.toast = String::from("ขนมปังไรย์"); // เปลี่ยนใจได้แบบ mood swing! 😊🎭
            println!("ฉันต้องการ {} กับ {} (combo ที่ perfect!)", meal.toast, meal.get_seasonal_fruit());

            // สั่งอาหารเรียกน้ำย่อย - เตรียมพร้อมสำหรับมื้อหลักแบบ warm-up ก่อนคอนเสิร์ต! 🥗🎵
            let order1 = back_of_house::Appetizer::Soup;
            let order2 = back_of_house::Appetizer::Salad;
            println!("อาหารเรียกน้ำย่อย: {order1:?} และ {order2:?} (double happiness!)");
        }
    }

    println!("\n🍽️ === ตัวอย่างร้านอาหาร === 🍽️");
    restaurant::eat_at_restaurant();

    // การใช้ use statements - ทางลัดสู่ความสะดวก! 🛤️✨
    {
        use restaurant::back_of_house::Appetizer; // นำ enum มาใช้โดยตรง! 🎯
        use restaurant::front_of_house::hosting; // นำ module มาใช้! 📦

        println!("\n📋 === ใช้ use statements === 📋");
        hosting::add_to_waitlist(); // เรียกใช้แบบสั้นๆ! 🚀
        hosting::seat_at_table(); // ไม่ต้องพิมพ์ path ยาวๆ! 😌

        let appetizer = Appetizer::Soup; // ใช้ enum โดยตรง! 🎁
        println!("🥣 สั่ง: {appetizer:?}");
    }

    // การใช้ use กับ as keyword - เปลี่ยนชื่อให้เท่ขึ้น! 🏷️✨
    {
        use restaurant::front_of_house::hosting as host; // เรียกสั้นๆ ว่า host! 🏨
        use restaurant::front_of_house::serving as serve; // เรียกสั้นๆ ว่า serve! 🍽️

        println!("\n🏷️ === ใช้ as keyword === 🏷️");
        host::add_to_waitlist(); // host ฟังดูเป็นมืออาชีพ! 💼
        serve::take_order(); // serve ฟังดูเรียบง่าย! ✨
        serve::serve_order(); // ส่งความอร่อยออกไป! 🚀
        serve::take_payment(); // รับเงินด้วยรอยยิ้ม! 😊
    }

    // การใช้ use กับ glob operator - เอาหลายๆ อย่างมาพร้อมกัน! 🌟📦
    {
        use restaurant::back_of_house::{Appetizer, Breakfast, fix_incorrect_order}; // เลือกมาหลายตัว! 🎯

        println!("\n🌟 === ใช้ glob operator (*) === 🌟");
        let breakfast = Breakfast::summer("ขนมปังฝรั่งเศส"); // อาหารเช้าแบบฝรั่งเศส! 🥐
        println!(
            "🥐 อาหารเช้า: {} กับ {}",
            breakfast.toast,
            breakfast.get_seasonal_fruit() // ผลไม้ลับของเชฟ! 🤫
        );

        let appetizers = vec![Appetizer::Soup, Appetizer::Salad]; // สั่งมาทั้งคู่! 🍲🥗
        println!("🥗 อาหารเรียกน้ำย่อย: {appetizers:?}");

        fix_incorrect_order(); // เชฟแก้ไขด้วยความใส่ใจ! 💪❤️
    }

    println!("\n🎯 สรุป Use Statements (บทสรุปแห่งความเข้าใจ!):");
    println!("   • use ช่วยให้เราเรียกใช้ฟังก์ชันได้สั้นลงแบบ shortcut ในชีวิต! ⚡");
    println!("   • Path มี 2 แบบ: Relative (แบบญาติๆ) และ Absolute (แบบ GPS เต็มที่!) 🗺️");
    println!("   • super ใช้เข้าถึง parent module (เหมือนเรียกพ่อแม่!) 👨‍👩‍👧‍👦");
    println!("   • crate ใช้เข้าถึงจาก root (เหมือนกลับบ้านเกิด!) 🏠");
    println!("   • pub ทำให้สามารถเข้าถึงจากภายนอกได้ (เปิดประตูต้อนรับ!) 🚪✨");
}
