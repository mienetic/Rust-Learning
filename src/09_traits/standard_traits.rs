/// ฟังก์ชันสำหรับสอนเรื่อง Standard Library Traits
/// มาเรียนรู้ Traits ที่มาพร้อมกับ Rust กันเถอะ! เหมือนแอปที่ติดตั้งมาให้ในมือถือใหม่แบบ pre-installed apps! 📚
pub fn learn_standard_traits() {
    println!("\n📚 === Standard Library Traits: เครื่องมือพร้อมใช้แบบ ready-to-use toolkit! === 📚");
    println!("🛠️ เหมือนชุดเครื่องมือที่มาพร้อมกับบ้าน หรือซอฟต์แวร์ที่ติดตั้งมาให้แบบ built-in features! 🏠💻");

    // Clone trait - เครื่องถ่ายเอกสารสำหรับข้อมูลแบบ data photocopier!
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,  // ชื่อที่ไม่ซ้ำใครแบบ unique identity!
        age: u32,      // อายุที่เพิ่มขึ้นทุกปีแบบ annual increment!
    }

    let person1 = Person {
        name: String::from("สมชาย (คนต้นฉบับแบบ original person!)"),
        age: 25,  // วัยหนุ่มสาวที่เต็มไปด้วยความฝันแบบ dream-filled age!
    };

    let person2 = person1.clone();  // สร้างสำเนาแบบ perfect copy!

    println!("\n👥 === Clone Trait: เครื่องถ่ายเอกสารข้อมูลแบบ data duplicator! === 👥");
    println!("📄 เหมือนเครื่องถ่ายเอกสารที่ทำสำเนาได้เหมือนต้นฉบับ 100% แบบ perfect photocopy! 🖨️✨");
    println!("👤 คน 1: {person1:?} (ต้นฉบับแบบ original copy!)");
    println!("👤 คน 2 (โคลน): {person2:?} (สำเนาแบบ duplicate copy!)");
    println!("❓ เหมือนกันหรือไม่: {} (ตรวจสอบความเหมือนแบบ similarity check!)", person1 == person2);

    // PartialOrd และ Ord - ผู้พิพากษาการแข่งขันแบบ competition judge!
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Student {
        grade: u32,    // คะแนนที่บอกความเก่งแบบ intelligence score!
        name: String,  // ชื่อนักเรียนที่น่ารักแบบ adorable student name!
    }

    let mut students = vec![
        Student {
            grade: 85,  // คะแนนดีแบบ good score!
            name: String::from("อลิซ (นักเรียนขยันแบบ diligent student!)"),
        },
        Student {
            grade: 92,  // คะแนนเยี่ยมแบบ excellent score!
            name: String::from("บ็อบ (นักเรียนเก่งแบบ smart student!)"),
        },
        Student {
            grade: 78,  // คะแนนพอใช้แบบ decent score!
            name: String::from("ชาร์ลี (นักเรียนน่ารักแบบ cute student!)"),
        },
    ];

    println!("\n🎓 === PartialOrd/Ord Traits: ผู้พิพากษาการแข่งขันแบบ automatic ranking! === 🎓");
    println!("🏆 เหมือนระบบจัดอันดับอัตโนมัติ หรือผู้พิพากษาที่ไม่มีอคติแบบ fair judge! ⚖️🤖");
    println!("📊 ก่อนเรียง: {students:?} (ลำดับสุ่มแบบ random order!)");

    students.sort();  // เรียงลำดับแบบอัตโนมัติแบบ auto-sort magic!
    println!("📊 หลังเรียง: {students:?} (เรียงตามคะแนนแบบ score-based ranking!)");

    // Iterator trait - สายพานลำเลียงข้อมูลแบบ data conveyor belt!
    struct Counter {
        current: usize,  // ตำแหน่งปัจจุบันแบบ current position!
        max: usize,      // จุดหมายปลายทางแบบ destination point!
    }

    impl Counter {
        const fn new(max: usize) -> Self {
            Self { current: 0, max }  // เริ่มต้นจากศูนย์แบบ starting from zero!
        }
    }

    impl Iterator for Counter {
        type Item = usize;  // ประเภทของสินค้าที่ส่งแบบ product type!

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;  // เดินหน้าต่อไปแบบ move forward!
                Some(current)       // ส่งสินค้าแบบ deliver product!
            } else {
                None  // หมดสินค้าแล้วแบบ out of stock!
            }
        }
    }

    println!("\n🔢 === Iterator Trait: สายพานลำเลียงข้อมูลแบบ automated data delivery! === 🔢");
    println!("🏭 เหมือนโรงงานที่ผลิตข้อมูลออกมาทีละชิ้น หรือเครื่องจ่ายลูกอมแบบ candy dispenser! 🍬⚙️");

    let counter = Counter::new(5);  // สร้างเครื่องนับที่นับถึง 5 แบบ count-to-5 machine!
    let numbers: Vec<usize> = counter.collect();  // เก็บผลผลิทั้งหมดแบบ harvest all!
    println!("🔢 ตัวเลขจาก Counter: {numbers:?} (ผลผลิตจากโรงงานแบบ factory output!)");

    let counter2 = Counter::new(3);  // สร้างเครื่องนับใหม่แบบ new counting machine!
    let doubled: Vec<usize> = counter2.map(|x| x * 2).collect();  // แปลงข้อมูลแบบ data transformation!
    println!("🔢 ตัวเลขคูณ 2: {doubled:?} (ผลผลิตที่ผ่านการแปรรูปแบบ processed product!)");

    // Display trait - นักแสดงที่แสดงตัวเองได้สวยงามแบบ self-presentation artist!
    struct Temperature {
        celsius: f64,  // อุณหภูมิเซลเซียสที่อาจจะร้อนหรือเย็นแบบ hot-or-cold indicator!
    }

    impl std::fmt::Display for Temperature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // แสดงตัวแบบสวยงาม - เหมือนนางแบบโชว์เสื้อผ้าแบบ fashion model!
            write!(f, "{:.1}°C (อุณหภูมิที่พอดีแบบ perfect temperature!)", self.celsius)
        }
    }

    impl Temperature {
        const fn new(celsius: f64) -> Self {
            Self { celsius }  // สร้างเทอร์โมมิเตอร์ใหม่แบบ new thermometer!
        }

        fn to_fahrenheit(&self) -> f64 {
            // แปลงเป็นฟาเรนไฮต์ - เหมือนแปลภาษาแบบ temperature translation!
            self.celsius * 9.0 / 5.0 + 32.0
        }
    }

    println!("\n🌡️ === Display Trait: นักแสดงตัวเองแบบ self-presentation master! === 🌡️");
    println!("🎭 เหมือนนักแสดงที่รู้จักแสดงตัวเองให้สวยงาม หรือนางแบบที่โพสท่าได้เป็นแบบ natural poser! 📸✨");

    let temp = Temperature::new(25.0);  // สร้างอุณหภูมิที่สบายๆ แบบ comfortable temperature!
    println!("🌡️ อุณหภูมิ: {temp} (แสดงตัวแบบสวยงามแบบ beautiful display!)");
    println!("🌡️ ฟาเรนไฮต์: {:.1}°F (แปลภาษาอุณหภูมิแบบ temperature translation!)", temp.to_fahrenheit());

    println!("\n🎉 จบบทเรียน Standard Traits! ตอนนี้คุณรู้จักเครื่องมือพื้นฐานของ Rust แล้ว! 🎉");
    println!("🛠️ ยินดีด้วย! คุณเป็นช่างเทคนิคที่รู้จักใช้เครื่องมือมาตรฐานแล้ว! 🔧👨‍🔧");
}
