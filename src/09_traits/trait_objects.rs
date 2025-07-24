/// ฟังก์ชันสำหรับสอนเรื่อง Trait Objects
/// มาเรียนรู้การใช้ Trait Objects กันเถอะ! เหมือนนักแสดงที่เปลี่ยนบทได้ตลอดเวลาแบบ shape-shifting performer! 🎭
pub fn learn_trait_objects() {
    println!("\n🎯 === Trait Objects: นักแสดงแปลงร่างแบบ dynamic performer! === 🎯");
    println!("🎪 เหมือนนักแสดงที่เปลี่ยนบทได้ตลอดเวลา หรือหุ่นยนต์ที่ทำงานได้หลายอย่างแบบ multi-function robot! 🤖🎨");

    // Trait สำหรับ shapes - สัญญาของรูปทรงทุกชนิดแบบ universal shape contract!
    trait Shape {
        fn area(&self) -> f64;        // พื้นที่ที่ครอบครองแบบ territory size!
        fn perimeter(&self) -> f64;   // เส้นรอบรูปที่เดินรอบได้แบบ walking distance!
        fn describe(&self) -> String; // แนะนำตัวเองแบบ self-introduction!
    }

    struct Circle {
        radius: f64,  // รัศมีที่กำหนดขนาดแบบ size controller!
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            // คำนวณพื้นที่แบบสูตรคณิตศาสตร์ - เหมือนนักคิดเลขแบบ math wizard!
            std::f64::consts::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            // คำนวณเส้นรอบวงแบบ perfect circle - เหมือนวิ่งรอบสนามแบบ track runner!
            2.0 * std::f64::consts::PI * self.radius
        }

        fn describe(&self) -> String {
            format!("วงกลมรัศมี {:.1} (รูปทรงที่สมบูรณ์แบบแบบ perfect shape!)", self.radius)
        }
    }

    struct Rectangle {
        width: f64,   // ความกว้างที่วัดได้แบบ measurable width!
        height: f64,  // ความสูงที่มองเห็นแบบ visible height!
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            // คำนวณพื้นที่แบบง่ายๆ - เหมือนนับกระเบื้องแบบ tile counting!
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            // คำนวณเส้นรอบรูปแบบเดินรอบบ้าน - เหมือนเดินรอบสวนแบบ garden walk!
            2.0 * (self.width + self.height)
        }

        fn describe(&self) -> String {
            format!("สี่เหลี่ยม {}x{} (รูปทรงที่เป็นระเบียบแบบ organized shape!)", self.width, self.height)
        }
    }

    struct Triangle {
        base: f64,    // ฐานที่เป็นรากฐานแบบ foundation base!
        height: f64,  // ความสูงที่ชี้ฟ้าแบบ sky-pointing height!
        side1: f64,   // ด้านที่หนึ่งแบบ first side!
        side2: f64,   // ด้านที่สองแบบ second side!
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            // คำนวณพื้นที่แบบครึ่งหนึ่ง - เหมือนแบ่งพิซซ่าแบบ pizza sharing!
            0.5 * self.base * self.height
        }

        fn perimeter(&self) -> f64 {
            // คำนวณเส้นรอบรูปแบบรวมทุกด้าน - เหมือนวัดรั้วบ้านแบบ fence measurement!
            self.base + self.side1 + self.side2
        }

        fn describe(&self) -> String {
            format!("สามเหลี่ยมฐาน {:.1} สูง {:.1} (รูปทรงที่มีมุมแหลมแบบ pointy shape!)", self.base, self.height)
        }
    }

    // ใช้ trait objects - สร้างคณะนักแสดงที่หลากหลายแบบ diverse cast!
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),  // วงกลมนักแสดงหลักแบบ main character!
        Box::new(Rectangle {
            width: 4.0,   // กว้างพอดีแบบ just right width!
            height: 6.0,  // สูงงามแบบ elegant height!
        }),
        Box::new(Triangle {
            base: 3.0,    // ฐานมั่นคงแบบ stable base!
            height: 4.0,  // สูงปานกลางแบบ moderate height!
            side1: 3.0,   // ด้านแรกแบบ first side!
            side2: 5.0,   // ด้านสองแบบ second side!
        }),
    ];

    println!("\n📐 === รายการรูปทรง: คณะนักแสดงหลากหลายแบบ diverse performers! === 📐");
    println!("🎭 เหมือนการแสดงที่มีนักแสดงหลายคน แต่ละคนมีความสามารถเฉพาะตัวแบบ unique talents! 🌟");

    let mut total_area = 0.0;      // พื้นที่รวมแบบ total territory!
    let mut total_perimeter = 0.0; // เส้นรอบรูปรวมแบบ total boundary!

    for (i, shape) in shapes.iter().enumerate() {
        let area = shape.area();           // คำนวณพื้นที่แบบ calculate territory!
        let perimeter = shape.perimeter(); // คำนวณเส้นรอบรูปแบบ calculate boundary!

        println!(
            "{}. {} - พื้นที่: {:.2} (ครอบครองพื้นที่แบบ territory control!), เส้นรอบรูป: {:.2} (เขตแดนแบบ border line!)",
            i + 1,
            shape.describe(),
            area,
            perimeter
        );

        total_area += area;           // สะสมพื้นที่แบบ accumulate territory!
        total_perimeter += perimeter; // สะสมเส้นรอบรูปแบบ accumulate boundary!
    }

    println!("\n📊 สรุป: รายงานผลการแสดงแบบ performance report!");
    println!("📏 พื้นที่รวม: {total_area:.2} (อาณาจักรทั้งหมดแบบ total kingdom!)");
    println!("📏 เส้นรอบรูปรวม: {total_perimeter:.2} (เขตแดนทั้งหมดแบบ total border!)");

    // ฟังก์ชันที่รับ trait object - ผู้สัมภาษณ์ที่พูดได้กับทุกคนแบบ universal interviewer!
    fn print_shape_info(shape: &dyn Shape) {
        println!(
            "🔍 {}: พื้นที่ {:.2} (ขนาดอาณาเขตแบบ territory size!), เส้นรอบรูป {:.2} (ความยาวเขตแดนแบบ border length!)",
            shape.describe(),
            shape.area(),
            shape.perimeter()
        );
    }

    println!("\n🔍 === การใช้ฟังก์ชันกับ Trait Objects: ผู้สัมภาษณ์มืออาชีพแบบ professional interviewer! === 🔍");
    println!("🎤 เหมือนผู้สัมภาษณ์ที่พูดได้กับทุกคน ไม่ว่าจะเป็นใครก็ตามแบบ universal communicator! 📺🌟");

    let circle = Circle { radius: 3.0 };  // วงกลมขนาดกลางแบบ medium circle!
    let rectangle = Rectangle {
        width: 5.0,   // กว้างมากแบบ wide width!
        height: 2.0,  // เตี้ยหน่อยแบบ short height!
    };

    print_shape_info(&circle);    // สัมภาษณ์วงกลมแบบ circle interview!
    print_shape_info(&rectangle); // สัมภาษณ์สี่เหลี่ยมแบบ rectangle interview!

    println!("\n🎉 จบบทเรียน Trait Objects! ตอนนี้คุณเป็นผู้กำกับที่ควบคุมนักแสดงได้หลายประเภทแล้ว! 🎉");
    println!("🎬 ยินดีด้วย! คุณเป็นผู้กำกับมืออาชีพที่ทำงานกับนักแสดงหลากหลายได้แล้ว! 🎭🏆");
}
