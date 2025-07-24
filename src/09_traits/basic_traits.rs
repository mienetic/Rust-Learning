/// ฟังก์ชันสำหรับสอนเรื่อง Basic Traits
/// มาเรียนรู้การสร้างและใช้งาน Traits กันเถอะ! เหมือนสอนให้สัตว์ต่างชนิดทำท่าเดียวกันแบบ animal circus! 🎭
pub fn learn_basic_traits() {
    println!("🎭 === Traits ใน Rust: สัญญาพฤติกรรมที่ทุกคนต้องทำตามแบบ behavioral contract! === 🎭");

    // การสร้าง trait - เหมือนสัญญาที่บอกว่าต้องทำอะไรได้บ้างแบบ job description!
    trait Summary {
        fn summarize(&self) -> String;  // ฟังก์ชันบังคับ - ต้องมีแบบ mandatory skill!

        // Default implementation - ทักษะพื้นฐานที่ให้มาฟรีแบบ free starter pack!
        fn summarize_author(&self) -> String {
            String::from("(ไม่ระบุผู้เขียน - คนลึกลับแบบ anonymous hero!)")
        }

        // Default implementation ที่เรียกใช้ method อื่น - ทักษะผสมผสานแบบ combo skill!
        fn summarize_with_author(&self) -> String {
            format!("{}... โดย {} (แพ็คเกจคอมโบแบบ combo deal!)", self.summarize(), self.summarize_author())
        }
    }

    // Struct ที่ implement trait - นักข่าวที่ต้องเขียนข่าวให้อ่านง่ายแบบ news reporter!
    #[derive(Debug)]
    struct NewsArticle {
        headline: String,  // หัวข้อข่าวที่ต้องดึงดูดใจแบบ clickbait master!
        location: String,  // สถานที่เกิดเหตุแบบ crime scene!
        author: String,    // ผู้เขียนที่อาจจะดังหรือไม่ดังแบบ fame lottery!
        // Field นี้ใช้เพื่อแสดงตัวอย่างโครงสร้างข้อมูลข่าว
        // Warning: field ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        content: String,   // เนื้อหาข่าวที่ยาวเหยียดแบบ novel-length article!
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            // สรุปข่าวแบบกระชับ - เหมือนทำ TL;DR แบบ too long didn't read!
            format!("{}, {} ({}) - ข่าวสั้นๆ แต่ได้ใจความแบบ bite-sized news!", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            // ใส่ @ หน้าชื่อเหมือน social media แบบ instant fame!
            format!("@{} (นักข่าวมืออาชีพแบบ professional journalist!)", self.author)
        }
    }

    #[derive(Debug)]
    struct Tweet {
        username: String,  // ชื่อผู้ใช้ที่อาจจะเท่หรือแปลกแบบ username creativity!
        content: String,   // เนื้อหาทวีตที่จำกัด 280 ตัวอักษรแบบ character limit challenge!
        // Fields เหล่านี้ใช้เพื่อแสดงตัวอย่างโครงสร้างข้อมูลทวีต
        // Warning: fields ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        reply: bool,       // ตอบกลับหรือไม่ - เหมือนการสนทนาแบบ conversation starter!
        #[allow(dead_code)]
        retweet: bool,     // รีทวีตหรือไม่ - การแชร์ความคิดคนอื่นแบบ idea sharing!
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            // สรุปทวีตแบบสั้นๆ - เหมือนการพูดในลิฟต์แบบ elevator pitch!
            format!("{}: {} (ทวีตสั้นๆ แต่ได้ใจความแบบ micro-message!)", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            // ใส่ @ หน้าชื่อเหมือนเป็น influencer แบบ social media star!
            format!("@{} (นักทวีตมืออาชีพแบบ tweet master!)", self.username)
        }
    }

    println!("\n📰 === การใช้งาน Traits: เวลาโชว์ฝีมือแบบ talent show! === 📰");

    let article = NewsArticle {
        headline: String::from("Rust 1.70 เปิดตัวแล้ว! (ข่าวดีสำหรับนักเขียนโค้ดแบบ coder's paradise!)"),
        location: String::from("ออนไลน์ (โลกไซเบอร์แบบ cyberspace!)"),
        author: String::from("Rust Team (ทีมเทพแบบ legendary team!)"),
        content: String::from("Rust เวอร์ชันใหม่มาพร้อมฟีเจอร์ใหม่มากมาย... (เนื้อหายาวแบบ epic novel!)"),
    };

    let tweet = Tweet {
        username: String::from("rustlang (บัญชีทางการแบบ official account!)"),
        content: String::from("Rust is amazing! 🦀 (ความจริงที่ไม่ต้องโต้แย้งแบบ undeniable truth!)"),
        reply: false,  // ไม่ใช่การตอบกลับ - เป็นทวีตต้นฉบับแบบ original content!
        retweet: false, // ไม่ใช่รีทวีต - เป็นความคิดของตัวเองแบบ original thought!
    };

    println!("📄 บทความ: {} (ข่าวสารสำคัญแบบ breaking news!)", article.summarize());
    println!("🐦 ทวีต: {} (ข้อความสั้นๆ แต่ได้ใจความแบบ micro-blogging!)", tweet.summarize());
    println!("👤 ผู้เขียนบทความ: {} (เครดิตนักข่าวแบบ journalist credit!)", article.summarize_author());
    println!("👤 ผู้เขียนทวีต: {} (เครดิตนักทวีตแบบ tweeter credit!)", tweet.summarize_author());
    println!("📝 สรุปพร้อมผู้เขียน: {} (แพ็คเกจครบเซ็ตแบบ complete package!)", article.summarize_with_author());

    // Trait เป็น parameter - รับได้ทุกอย่างที่มี Summary แบบ universal acceptor!
    fn notify(item: &impl Summary) {
        println!("🔔 ข่าวด่วน! {} (แจ้งเตือนสำคัญแบบ urgent notification!)", item.summarize());
    }

    println!("\n🔔 === Trait เป็น Parameter: รับได้ทุกอย่างที่มี Summary แบบ flexible input! === 🔔");
    notify(&article);  // ส่งบทความเข้าไปแบบ article delivery!
    notify(&tweet);    // ส่งทวีตเข้าไปแบบ tweet delivery!

    // Trait bound syntax - วิธีเขียนแบบเป็นทางการมากขึ้นแบบ formal syntax!
    fn notify_verbose<T: Summary>(item: &T) {
        println!("📢 ประกาศ: {} (ประกาศอย่างเป็นทางการแบบ official announcement!)", item.summarize_with_author());
    }

    println!("\n📢 === Trait Bound Syntax: การเขียนแบบเป็นทางการแบบ formal declaration! === 📢");
    notify_verbose(&article);  // ประกาศบทความแบบเป็นทางการแบบ formal article announcement!
    notify_verbose(&tweet);    // ประกาศทวีตแบบเป็นทางการแบบ formal tweet announcement!
}
