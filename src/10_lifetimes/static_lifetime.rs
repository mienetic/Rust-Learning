/// ฟังก์ชันสำหรับสอนเรื่อง Static Lifetime - ดินแดนแห่งความเป็นอมตะ! ⚰️👻
/// เรียนรู้เรื่องตัวแปรที่มีชีวิตนิรันดร์! 🧛‍♂️
pub fn learn_static_lifetime() {
    println!("\n♾️ === Static Lifetime: ดินแดนแห่งความเป็นอมตะ! === ♾️");

    // Static string literal - สตริงอมตะ! 🧛‍♂️📜
    let s: &'static str = "ฉันมีอายุการใช้งานตลอดโปรแกรม (เหมือนแวมไพร์ที่ไม่ตาย!)";
    println!("📝 Static string (สตริงอมตะ): {s}");

    // Static variable - ตัวแปรโลกาภิวัตน์! 🌍👑
    static GLOBAL_MESSAGE: &str = "นี่คือข้อความ global (เหมือนประกาศของกษัตริย์ที่ทั่วโลกได้ยิน!)";
    println!("🌍 Global message (ข้อความโลกาภิวัตน์): {GLOBAL_MESSAGE}");

    // ฟังก์ชันที่ return static reference - โรงงานผลิตความเป็นอมตะ! 🏭⚰️
    const fn get_static_str() -> &'static str {
        "สตริงที่มี static lifetime (ผลิตภัณฑ์จากโรงงานอมตะ!)"
    }

    println!("🔄 จากฟังก์ชัน: {}", get_static_str());

    // Struct กับ static lifetime - พิพิธภัณฑ์ข้อมูลอมตะ! 🏛️📚
    #[derive(Debug)]
    struct Config {
        name: &'static str,        // ชื่อที่จารึกในหิน!
        version: &'static str,     // เวอร์ชันที่ไม่เปลี่ยนแปลง!
        description: &'static str, // คำอธิบายนิรันดร์!
    }

    const APP_CONFIG: Config = Config {
        name: "RustApp",
        version: "1.0.0",
        description: "แอปพลิเคชัน Rust ที่ยอดเยี่ยม",
    };

    println!("\n⚙️ === การตั้งค่าแอป: พิพิธภัณฑ์ข้อมูลอมตะ! === ⚙️");
    println!("📱 ชื่อแอป: {}", APP_CONFIG.name);
    println!("🔢 เวอร์ชัน: {}", APP_CONFIG.version);
    println!("📄 คำอธิบาย: {}", APP_CONFIG.description);

    // ฟังก์ชันที่ต้องการ static lifetime - ตู้เซฟแห่งนิรันดร์! 🔐💎
    fn store_important_message(msg: &'static str) -> &'static str {
        // ในความเป็นจริง เราอาจเก็บไว้ใน static variable
        // เหมือนเก็บสมบัติในตู้เซฟที่ไม่มีวันเปิด!
        println!("💾 เก็บข้อความสำคัญในตู้เซฟนิรันดร์: {msg}");
        msg
    }

    let important = store_important_message("ข้อความที่สำคัญมาก");
    println!("📋 ข้อความที่เก็บไว้: {important}");

    // การใช้ Box::leak เพื่อสร้าง static lifetime (ใช้อย่างระวัง!) - เวทมนตร์ดำ! 🧙‍♂️💀
    let dynamic_string = String::from("สตริงที่สร้างขึ้นแบบ dynamic");
    let leaked_string: &'static str = Box::leak(dynamic_string.into_boxed_str());
    println!("\n⚠️ === Box::leak (เวทมนตร์ดำ - ใช้อย่างระวัง!) === ⚠️");
    println!("🔓 Leaked string (สตริงที่หลุดออกมา): {leaked_string}");
    println!("⚠️ หมายเหตุ: Box::leak จะทำให้ memory ไม่ถูก deallocate! (เหมือนสาปให้เป็นซอมบี้!)");

    // Lazy static pattern (จำลอง) - คนขี้เกียจที่เป็นอมตะ! 😴♾️
    struct LazyStatic {
        data: &'static str,  // ข้อมูลที่ขี้เกียจโหลด (แต่เป็นอมตะ!)
    }

    impl LazyStatic {
        const fn new() -> Self {
            Self {
                data: "ข้อมูลที่โหลดแบบ lazy (เหมือนคนขี้เกียจที่ตื่นสาย!)",
            }
        }

        const fn get_data(&self) -> &'static str {
            self.data
        }
    }

    static LAZY_DATA: LazyStatic = LazyStatic::new();

    println!("\n🔄 === Lazy Static Pattern: คนขี้เกียจอมตะ! === 🔄");
    println!("📊 ข้อมูล lazy (ข้อมูลขี้เกียจ): {}", LAZY_DATA.get_data());
}
