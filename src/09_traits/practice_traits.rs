/// ฟังก์ชันสำหรับฝึกฝน Traits
/// มาฝึกฝนการใช้ Traits กันเถอะ! เหมือนไปยิมฝึกกล้ามเนื้อแต่สำหรับสมองแบบ brain gym! 🧠💪
#[allow(clippy::too_many_lines)]
pub fn practice_traits() {
    println!("\n💪 === แบบฝึกหัด Traits: ยิมสำหรับโปรแกรมเมอร์แบบ programmer's gym! === 💪");
    println!("🏋️ เตรียมตัวฝึกฝนทักษะ Traits กันเถอะ! เหมือนนักกีฬาที่ฝึกซ้อมแบบ skill training! 🎯🔥");

    // 1. Animal Trait System - ระบบสัตว์โลกแบบ animal kingdom system!
    trait Animal {
        fn name(&self) -> &str;        // ชื่อที่เรียกขานแบบ calling name!
        fn make_sound(&self) -> String; // เสียงที่ส่งออกแบบ sound output!
        fn age(&self) -> u32;           // อายุที่นับได้แบบ countable age!

        fn introduce(&self) -> String {
            // แนะนำตัวเองแบบสุภาพ - เหมือนการทักทายในงานปาร์ตี้แบบ party introduction!
            format!("สวัสดี! ฉันชื่อ {} อายุ {} ปี (ยินดีที่ได้รู้จักแบบ nice to meet you!)", self.name(), self.age())
        }

        fn speak(&self) -> String {
            // พูดคุยแบบมีเสียง - เหมือนนักพากย์ที่ใส่เสียงให้ตัวละครแบบ voice actor!
            format!("{} พูดว่า: {} (เสียงที่ไพเราะแบบ melodious voice!)", self.name(), self.make_sound())
        }
    }

    trait Mammal: Animal {
        fn fur_color(&self) -> &str;    // สีขนที่สวยงามแบบ beautiful fur color!

        fn describe_fur(&self) -> String {
            // อธิบายขนแบบนักแฟชั่น - เหมือนนักออกแบบที่บรรยายเสื้อผ้าแบบ fashion designer!
            format!("{} มีขนสี{} (สวยงามแบบ gorgeous fur!)", self.name(), self.fur_color())
        }
    }

    struct Dog {
        name: String,      // ชื่อสุนัขที่น่ารักแบบ adorable dog name!
        age: u32,          // อายุที่นับเป็นปีสุนัขแบบ dog years!
        // Field นี้ใช้เพื่อแสดงตัวอย่างข้อมูลสายพันธุ์สุนัข - เหมือนใบประกาศนียบัตรแบบ pedigree certificate!
        // Warning: field ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        breed: String,     // สายพันธุ์ที่มีเกียรติแบบ noble breed!
        fur_color: String, // สีขนที่เป็นเอกลักษณ์แบบ signature fur color!
    }

    impl Animal for Dog {
        fn name(&self) -> &str {
            // ส่งคืนชื่อแบบภูมิใจ - เหมือนการแนะนำตัวเองแบบ proud self-introduction!
            &self.name
        }

        fn make_sound(&self) -> String {
            // เสียงเห่าแบบคลาสสิก - เหมือนนักร้องที่มีเพลงประจำตัวแบบ signature song!
            String::from("โฮ่ง โฮ่ง! (เสียงที่ดังใสแบบ clear bark!)")
        }

        fn age(&self) -> u32 {
            // ส่งคืนอายุแบบตรงไปตรงมา - เหมือนการบอกอายุจริงแบบ honest age!
            self.age
        }
    }

    impl Mammal for Dog {
        fn fur_color(&self) -> &str {
            // ส่งคืนสีขนแบบภาคภูมิใจ - เหมือนการโชว์สีผมใหม่แบบ hair color reveal!
            &self.fur_color
        }
    }

    struct Cat {
        name: String,      // ชื่อแมวที่หรูหราแบบ elegant cat name!
        age: u32,          // อายุที่นับเป็นปีแมวแบบ cat years!
        // Field นี้ใช้เพื่อแสดงตัวอย่างข้อมูลแมวในบ้าน/นอกบ้าน - เหมือนสถานะที่อยู่อาศัยแบบ residence status!
        // Warning: field ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        indoor: bool,      // สถานะอยู่ในบ้านแบบ indoor lifestyle!
        fur_color: String, // สีขนที่เป็นเอกลักษณ์แบบ signature fur color!
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            // ส่งคืนชื่อแบบสง่างาม - เหมือนการแนะนำตัวเองแบบ graceful introduction!
            &self.name
        }

        fn make_sound(&self) -> String {
            // เสียงเหมียวแบบน่ารัก - เหมือนนักร้องโอเปร่าแมวแบบ cat opera singer!
            String::from("เหมียว เหมียว! (เสียงที่นุ่มนวลแบบ soft meow!)")
        }

        fn age(&self) -> u32 {
            // ส่งคืนอายุแบบลึกลับ - เหมือนสุภาพสตรีที่ไม่บอกอายุแบบ mysterious age!
            self.age
        }
    }

    impl Mammal for Cat {
        fn fur_color(&self) -> &str {
            // ส่งคืนสีขนแบบหรูหรา - เหมือนการโชว์เสื้อผ้าแบรนด์เนมแบบ luxury fashion show!
            &self.fur_color
        }
    }

    println!("\n🐾 === ระบบสัตว์: สวนสัตว์ดิจิทัลแบบ digital zoo! === 🐾");
    println!("🦁 เหมือนสวนสัตว์ที่สัตว์ทุกตัวมีบุคลิกเฉพาะตัวแบบ unique personality zoo! 🎪🌟");

    let dog = Dog {
        name: String::from("บัดดี้"),                        // ชื่อที่เป็นมิตรแบบ friendly name!
        age: 3,                                            // อายุวัยรุ่นแบบ teenage years!
        breed: String::from("โกลเด้น รีทรีฟเวอร์"),          // สายพันธุ์ที่ดีแบบ premium breed!
        fur_color: String::from("ทอง"),                    // สีทองแบบ golden color!
    };

    let cat = Cat {
        name: String::from("วิสกี้"),                       // ชื่อที่หรูหราแบบ classy name!
        age: 2,                                            // อายุวัยเด็กแบบ young age!
        indoor: true,                                      // แมวบ้านแบบ house cat!
        fur_color: String::from("ส้ม"),                    // สีส้มแบบ orange color!
    };

    println!("🐕 {}", dog.introduce());     // สุนัขแนะนำตัวแบบ dog introduction!
    println!("🐕 {}", dog.speak());         // สุนัขพูดแบบ dog speech!
    println!("🐕 {}", dog.describe_fur());  // สุนัขอธิบายขนแบบ fur description!

    println!("\n🐱 {}", cat.introduce());     // แมวแนะนำตัวแบบ cat introduction!
    println!("🐱 {}", cat.speak());         // แมวพูดแบบ cat speech!
    println!("🐱 {}", cat.describe_fur());  // แมวอธิบายขนแบบ fur description!

    // 2. Drawable Trait with different implementations - ระบบวาดรูปแบบ digital art system!
    trait Drawable {
        fn draw(&self) -> String;       // วาดรูปแบบ drawing method!
        fn color(&self) -> &str;        // สีที่ใช้แบบ color palette!

        fn draw_with_color(&self) -> String {
            // วาดพร้อมสีแบบศิลปิน - เหมือนจิตรกรที่ใส่สีให้ภาพแบบ colorful artist!
            format!("{} (สี{} แบบ artistic color!)", self.draw(), self.color())
        }
    }

    struct Square {
        size: f64,     // ขนาดที่เท่าทุกด้านแบบ equal-sided size!
        color: String, // สีที่เลือกมาแบบ chosen color!
    }

    impl Drawable for Square {
        fn draw(&self) -> String {
            // วาดสี่เหลี่ยมแบบสมมาตร - เหมือนสถาปนิกที่วาดแบบบ้านแบบ architect's blueprint!
            format!("สี่เหลี่ยมจัตุรัส {}x{} (รูปทรงที่สมบูรณ์แบบแบบ perfect square!)", self.size, self.size)
        }

        fn color(&self) -> &str {
            // ส่งคืนสีแบบภาคภูมิใจ - เหมือนศิลปินที่โชว์สีโปรดแบบ favorite color reveal!
            &self.color
        }
    }

    struct Line {
        length: f64,   // ความยาวที่วัดได้แบบ measurable length!
        color: String, // สีที่เลือกมาแบบ selected color!
    }

    impl Drawable for Line {
        fn draw(&self) -> String {
            // วาดเส้นตรงแบบเรียบง่าย - เหมือนนักเขียนที่ขีดเส้นใต้แบบ underline master!
            format!("เส้นตรงยาว {:.1} (เส้นที่ตรงแบบ perfectly straight line!)", self.length)
        }

        fn color(&self) -> &str {
            // ส่งคืนสีแบบมั่นใจ - เหมือนดีไซเนอร์ที่เลือกสีแบบ confident color choice!
            &self.color
        }
    }

    println!("\n🎨 === ระบบวาดรูป: สตูดิโอศิลปะดิจิทัลแบบ digital art studio! === 🎨");
    println!("🖌️ เหมือนสตูดิโอศิลปะที่มีเครื่องมือหลากหลายแบบ diverse art tools studio! 🎭🌈");

    let drawings: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square {
            size: 5.0,                          // ขนาดใหญ่แบบ large size!
            color: String::from("แดง"),          // สีแดงแบบ red color!
        }),
        Box::new(Line {
            length: 10.0,                       // ความยาวมากแบบ long length!
            color: String::from("น้ำเงิน"),      // สีน้ำเงินแบบ blue color!
        }),
        Box::new(Square {
            size: 3.0,                          // ขนาดเล็กแบบ small size!
            color: String::from("เขียว"),        // สีเขียวแบบ green color!
        }),
    ];

    for (i, drawing) in drawings.iter().enumerate() {
        // แสดงผลงานศิลปะแบบ art exhibition!
        println!("{}. {} (ผลงานที่สวยงามแบบ beautiful artwork!)", i + 1, drawing.draw_with_color());
    }

    // 3. Serializable trait - ระบบแปลงข้อมูลแบบ data transformation system!
    trait Serializable {
        fn serialize(&self) -> String;      // แปลงเป็นข้อความแบบ text conversion!
        fn deserialize(data: &str) -> Result<Self, String>  // แปลงกลับแบบ reverse conversion!
        where
            Self: Sized;
    }

    #[derive(Debug, Clone)]
    struct User {
        id: u32,           // รหัสประจำตัวแบบ unique ID!
        username: String,  // ชื่อผู้ใช้แบบ user identity!
        email: String,     // อีเมลแบบ contact info!
    }

    impl Serializable for User {
        fn serialize(&self) -> String {
            // แปลงข้อมูลเป็นข้อความแบบ data packing - เหมือนการห่อของขวัญแบบ gift wrapping!
            format!("{}|{}|{}", self.id, self.username, self.email)
        }

        fn deserialize(data: &str) -> Result<Self, String> {
            // แปลงข้อความกลับเป็นข้อมูลแบบ data unpacking - เหมือนการแกะของขวัญแบบ gift unwrapping!
            let parts: Vec<&str> = data.split('|').collect();
            if parts.len() != 3 {
                return Err(String::from("Invalid format (รูปแบบไม่ถูกต้องแบบ wrong format!)"));
            }

            let id = parts[0]
                .parse::<u32>()
                .map_err(|_| String::from("Invalid ID (รหัสไม่ถูกต้องแบบ invalid ID!)"))?;

            Ok(Self {
                id,                                    // รหัสที่แปลงแล้วแบบ converted ID!
                username: parts[1].to_string(),        // ชื่อผู้ใช้ที่กู้คืนแบบ restored username!
                email: parts[2].to_string(),           // อีเมลที่กู้คืนแบบ restored email!
            })
        }
    }

    println!("\n💾 === ระบบ Serialization: โรงงานแปลงข้อมูลแบบ data transformation factory! === 💾");
    println!("📦 เหมือนโรงงานที่แปลงข้อมูลไปมาได้แบบ bidirectional data converter! 🔄✨");

    let user = User {
        id: 1,                                        // รหัสหนึ่งแบบ number one!
        username: String::from("john_doe"),           // ชื่อผู้ใช้แบบคลาสสิกแบบ classic username!
        email: String::from("john@example.com"),      // อีเมลตัวอย่างแบบ sample email!
    };

    let serialized = user.serialize();               // แปลงข้อมูลแบบ data conversion!
    println!("📤 Serialized: {serialized} (ข้อมูลที่ห่อแล้วแบบ packaged data!)");

    match User::deserialize(&serialized) {
        Ok(deserialized_user) => {
            println!("📥 Deserialized: {deserialized_user:?} (ข้อมูลที่แกะแล้วแบบ unpacked data!)");
        }
        Err(error) => {
            println!("❌ Error: {error} (เกิดข้อผิดพลาดแบบ unexpected error!)");
        }
    }

    // ทดสอบกับข้อมูลผิด - การทดสอบความแข็งแกร่งแบบ stress testing!
    match User::deserialize("invalid|data") {
        Ok(user) => println!("📥 Unexpected success: {user:?} (สำเร็จแบบไม่คาดคิดแบบ surprising success!)"),
        Err(error) => println!("❌ Expected error: {error} (ข้อผิดพลาดที่คาดหวังแบบ expected failure!)"),
    }

    println!("\n🎉 จบแบบฝึกหัด Traits! ตอนนี้คุณเป็นนักกีฬา Traits ที่แข็งแกร่งแล้ว! 🎉");
    println!("🏆 ยินดีด้วย! คุณผ่านการฝึกซ้อมแบบครบถ้วนและพร้อมใช้ Traits ในโลกจริงแล้ว! 💪🌟");
}
