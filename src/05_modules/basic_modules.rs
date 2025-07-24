// 🦀 Module System: Basic Modules - ห้องเรียนโมดูลพื้นฐาน! 🦀

/// ฟังก์ชันสำหรับสอนเรื่อง Basic Modules - เป็นครูสอนโมดูลที่สอนดีกว่าครูในฝัน! 👨‍🏫✨
/// มาเรียนรู้ Module System ใน Rust กันเถอะ! เป็นสถาปนิกโค้ดที่สร้างปราสาทได้! 🏗️🏰
pub fn learn_basic_modules() {
    println!("📦 === Module System ใน Rust: โรงเรียนสอนจัดระเบียบที่ Marie Kondo ต้องอิจฉา! === 📦");

    println!("\n📖 === ความรู้พื้นฐาน: หนังสือเรียนโมดูลที่อ่านแล้วฉลาดขึ้น 200%! === 📖");
    println!("🏗️ Module: กลุ่มของ functions, structs, enums, constants และ modules อื่นๆ (ตู้เก็บของวิเศษที่ Doraemon ต้องอิจฉา!)");
    println!("📦 Package: โปรเจ็กต์ที่มี Cargo.toml และ crates อย่างน้อย 1 อัน (กล่องใหญ่ที่ Amazon ขายไม่ได้!)");
    println!("📚 Crate: หน่วยการคอมไพล์ (binary หรือ library) (หนังสือเล่มหนึ่งที่อ่านแล้วไม่ง่วง!)");
    println!("🌳 Module Tree: โครงสร้างแบบต้นไม้ของ modules (ต้นไผ่โมดูลที่โตเร็วกว่าไผ่จริง!)");

    // ตัวอย่าง module ภายใน function (ห้องเรียนในห้องเรียนแบบ Inception! 🏫🌀)
    mod inner_module {  // โมดูลภายใน: ห้องลับที่ลับกว่า Area 51! 🚪👽
        pub fn public_function() {  // ฟังก์ชันสาธารณะ: ประตูเปิดกว้างกว่าใจคนใจดี! 🔓💖
            println!("🔓 ฟังก์ชันสาธารณะใน inner module (ทุกคนเข้าได้แม้แต่หมาข้างบ้าน!)");
            private_function();  // เรียกฟังก์ชันส่วนตัวแบบเพื่อนซี้! 📞💕
        }

        fn private_function() {  // ฟังก์ชันส่วนตัว: ห้องลับที่ลับกว่าสูตรโค้ก! 🔒🥤
            println!("🔒 ฟังก์ชันส่วนตัวใน inner module (เฉพาะคนในครอบครัวและแมวเหมียว!)🐱");
        }

        pub mod nested {  // โมดูลซ้อน: ห้องในห้องแบบตุ๊กตารัสเซีย! 🪆🇷🇺
            pub fn nested_function() {  // ฟังก์ชันซ้อน: ลึกกว่าปรัชญาชีวิต! 🎯🤔
                println!("🪆 ฟังก์ชันใน nested module (ห้องลึกสุดที่ต้องใช้ GPS หา!)🗺️");
            }
        }

        pub struct PublicStruct {  // โครงสร้างสาธารณะ: บ้านที่มีห้องส่วนตัวแบบ VIP! 🏠✨
            pub public_field: String,  // ฟิลด์สาธารณะ: ห้องรับแขกที่หรูกว่าโรงแรม 5 ดาว! 🛋️👑
            private_field: i32,  // ฟิลด์ส่วนตัว: ห้องนอนที่ห้ามแม้แต่แม่บ้านเข้า! 🛏️🚫
        }

        impl PublicStruct {  // การใช้งานโครงสร้าง: คู่มือการใช้บ้านหรู! ⚙️🏘️
            pub const fn new(public_data: String, private_data: i32) -> Self {  // สร้างใหม่: สร้างบ้านแบบ DIY! ✨🔨
                Self {
                    public_field: public_data,  // ข้อมูลสาธารณะ: ป้ายหน้าบ้านที่ทุกคนอ่านได้! 📢🏠
                    private_field: private_data,  // ข้อมูลส่วนตัว: รหัสเซฟที่เก็บเงินออม! 🤫💰
                }
            }

            pub const fn get_private_field(&self) -> i32 {  // ดูข้อมูลส่วนตัว: แอบดูผ่านช่องกุญแจ! 👀🔑
                self.private_field  // คืนค่าส่วนตัว: ให้ของขวัญลับๆ! 🎁💝
            }
        }

        pub enum PublicEnum {  // อีนัมสาธารณะ: ตู้เสื้อผ้าหลากสีที่ Zara ต้องอิจฉา! 👗✨
            Variant1,  // แบบที่ 1: เสื้อเปล่าสไตล์มินิมอลที่ดูแพงโดยไม่ต้องแพง! 👕💫
            Variant2(String),  // แบบที่ 2: เสื้อมีข้อความที่เท่กว่าเสื้อ Supreme! 👔🔥
            Variant3 { x: i32, y: i32 },  // แบบที่ 3: เสื้อมีพิกัดที่แม่นกว่า GPS! 🗺️📍
        }
    }  // จบโมดูลภายใน! 🏁

    println!("\n🔧 === การใช้งาน Modules: เวลาปฏิบัติการแบบ Mission Impossible! === 🔧🎬");

    // เรียกใช้ฟังก์ชันจาก module (โทรหาเพื่อนแบบไม่เสียค่าโทร! 📞💸)
    inner_module::public_function();  // เรียกฟังก์ชันสาธารณะ: เปิดประตูแบบ VIP! 🔓👑
    inner_module::nested::nested_function();  // เรียกฟังก์ชันซ้อน: ดำดิ่งลึกแบบนักประดาน้ำ! 🪆🤿

    // สร้าง struct จาก module (สร้างบ้านใหม่แบบไม่ต้องกู้เงิน! 🏗️💰)
    let my_struct = inner_module::PublicStruct::new(String::from("ข้อมูลสาธารณะ"), 42);  // สร้างโครงสร้าง: ประกอบเฟอร์นิเจอร์ IKEA! ✨🛠️

    println!("📊 Public field: {} (ข้อมูลที่ทุกคนเห็นได้แม้แต่เพื่อนบ้านข้างบ้าน!)", my_struct.public_field);
    println!(
        "🔒 Private field (ผ่าน method): {} (ข้อมูลลับที่ต้องขออนุญาตแบบขอเข้าวัง!)",
        my_struct.get_private_field()
    );

    // ใช้ enum จาก module (เปิดตู้เสื้อผ้าแบบ walk-in closet! 👗🚶‍♀️)
    let enum_variants = [  // คอลเลกชันเสื้อผ้า: แฟชั่นโชว์ Paris Fashion Week! 👔✨
        inner_module::PublicEnum::Variant1,  // เสื้อเปล่า: สไตล์มินิมอลที่ดูแพงโดยไม่ต้องแพง! 👕💎
        inner_module::PublicEnum::Variant2(String::from("ข้อความ")),  // เสื้อมีข้อความ: เท่กว่าเสื้อ band ที่หาซื้อยาก! 👔🎸
        inner_module::PublicEnum::Variant3 { x: 10, y: 20 },  // เสื้อมีพิกัด: แฟชั่นไฮเทคที่ NASA ใช้! 🗺️🚀
    ];

    println!("\n🎭 === Enum Variants: แฟชั่นโชว์ที่เจ๋งกว่า Milan Fashion Week! === 🎭🌟");
    for (i, variant) in enum_variants.iter().enumerate() {  // เดินแฟชั่นโชว์: catwalk แบบ supermodel! 🚶‍♀️💃
        match variant {  // ตรวจสอบแฟชั่น: ตัดสินแบบ fashion critic! 👀👗
            inner_module::PublicEnum::Variant1 => {  // เสื้อเปล่า: ความเรียบง่ายที่ลึกซึ้ง! 👕🧘
                println!("{}. Variant1 (เสื้อเปล่าสไตล์มินิมอลที่ดูแพงโดยไม่ต้องแพง!)", i + 1);
            }
            inner_module::PublicEnum::Variant2(text) => {  // เสื้อมีข้อความ: statement piece! 👔💬
                println!("{}. Variant2: {} (เสื้อมีข้อความเท่ห์กว่าเสื้อ Supreme!)", i + 1, text);
            }
            inner_module::PublicEnum::Variant3 { x, y } => {  // เสื้อมีพิกัด: เทคโนโลยีสุดล้ำ! 🗺️🤖
                println!("{}. Variant3: x={}, y={} (เสื้อมีพิกัดสุดเจ๋งที่แม่นกว่า GPS!)", i + 1, x, y);
            }
        }
    }

    println!("\n✅ จบการเรียนรู้ Basic Modules! (จบคาบเรียนแล้วได้ปริญญาโมดูล! 🎓🏆)");
}
