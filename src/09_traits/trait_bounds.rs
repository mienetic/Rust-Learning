/// ฟังก์ชันสำหรับสอนเรื่อง Multiple Traits และ Trait Bounds
/// มาเรียนรู้การกำหนดเงื่อนไขให้ Traits กันเถอะ! เหมือนการสอบใบขับขี่ที่ต้องผ่านหลายวิชาแบบ multi-skill requirement! 🔗
pub fn learn_trait_bounds() {
    println!("\n🔗 === Trait Bounds และ Multiple Traits: การกำหนดเงื่อนไขแบบ skill requirements! === 🔗");

    // Multiple traits - หลายทักษะที่ต้องมีแบบ multi-talented requirement!
    trait Display {
        fn display(&self) -> String;  // ทักษะการแสดงตัวแบบ presentation skill!
    }

    trait Debug {
        fn debug_info(&self) -> String;  // ทักษะการดีบักแบบ debugging expertise!
    }

    #[derive(Clone)]  // ให้สามารถคัดลอกได้แบบ copy machine!
    struct Product {
        name: String,     // ชื่อสินค้าที่ต้องจำแบบ memorable name!
        price: f64,       // ราคาที่อาจจะแพงหรือถูกแบบ price lottery!
        category: String, // หมวดหมู่ที่จัดระเบียบแบบ organized category!
    }

    impl Display for Product {
        fn display(&self) -> String {
            // แสดงสินค้าแบบสวยงาม - เหมือนป้ายราคาในห้างแบบ price tag display!
            format!("{} - {:.2}฿ (ราคาดีที่สุดแบบ best price!)", self.name, self.price)
        }
    }

    impl Debug for Product {
        fn debug_info(&self) -> String {
            // แสดงข้อมูลแบบละเอียด - เหมือนใบเสร็จที่มีรายละเอียดครบแบบ detailed receipt!
            format!(
                "Product {{ name: {}, price: {}, category: {} }} (ข้อมูลครบถ้วนแบบ complete info!)",
                self.name, self.price, self.category
            )
        }
    }

    // ฟังก์ชันที่ต้องการ multiple trait bounds - เหมือนสอบใบขับขี่ที่ต้องผ่านหลายวิชาแบบ comprehensive exam!
    fn print_product_info<T>(item: &T)
    where
        T: Display + Debug + Clone,  // ต้องมีทักษะครบ 3 อย่าง - เหมือนนักกีฬาไตรกีฬาแบบ triathlete!
    {
        println!("📦 แสดง: {} (โชว์ฝีมือการแสดงแบบ display talent!)", item.display());
        println!("🔍 ดีบัก: {} (โชว์ฝีมือการดีบักแบบ debugging skill!)", item.debug_info());
    }

    // Alternative syntax - วิธีเขียนแบบสั้นๆ แต่ได้ผลเหมือนกันแบบ shorthand notation!
    fn print_product_simple(item: &(impl Display + Debug + Clone)) {
        println!("📦 แสดงง่าย: {} (วิธีง่ายๆ แต่ได้ผลเหมือนกันแบบ simple but effective!)", item.display());
        println!("🔍 ดีบักง่าย: {} (ดีบักแบบง่ายๆ แต่ครบถ้วนแบบ simple debugging!)", item.debug_info());
    }

    let product = Product {
        name: String::from("MacBook Pro (เครื่องในฝันของนักเขียนโค้ดแบบ coder's dream!)"),
        price: 65000.0,  // ราคาที่ทำให้กระเป๋าร้องไห้แบบ wallet crying price!
        category: String::from("Laptop (คอมพิวเตอร์พกพาแบบ portable powerhouse!)"),
    };

    println!("\n🛍️ === การใช้ Multiple Trait Bounds: การทดสอบทักษะหลายอย่างแบบ multi-skill test! === 🛍️");
    print_product_info(&product);    // ทดสอบด้วยวิธีแบบเต็มรูปแบบแบบ full formal test!
    print_product_simple(&product);  // ทดสอบด้วยวิธีแบบง่ายๆ แบบ simplified test!

    // Conditional trait implementation - การใช้งานแบบมีเงื่อนไขแบบ conditional magic!
    struct Wrapper<T>(T);  // กล่องห่อหุ้มที่ใส่อะไรก็ได้แบบ universal wrapper!

    impl<T: Display> Display for Wrapper<T> {  // ถ้า T มี Display เราก็จะมี Display ด้วยแบบ inherited talent!
        fn display(&self) -> String {
            // ห่อหุ้มการแสดงผลด้วยคำว่า Wrapper - เหมือนของขวัญที่ห่อสวยแบบ gift wrapping!
            format!("Wrapper({}) (ห่อหุ้มด้วยความรักแบบ wrapped with love!)", self.0.display())
        }
    }

    let wrapped_product = Wrapper(product);  // ห่อสินค้าด้วยกล่องพิเศษแบบ special packaging!
    println!("\n📦 === Conditional Implementation: การใช้งานแบบมีเงื่อนไขแบบ smart inheritance! === 📦");
    println!("🎁 Wrapped: {} (ของขวัญที่ห่อสวยแบบ beautifully wrapped gift!)", wrapped_product.display());

    // Returning traits - คืนค่าแบบลึกลับที่รู้แค่ว่ามี Display แบบ mystery return!
    fn create_summary() -> impl Display {  // สร้างอะไรบางอย่างที่แสดงได้ แต่ไม่บอกว่าเป็นอะไรแบบ mystery creation!
        Product {
            name: String::from("iPhone (โทรศัพท์ในตำนานแบบ legendary phone!)"),
            price: 35000.0,  // ราคาที่ทำให้ต้องกินมาม่าเป็นเดือนแบบ instant noodle budget!
            category: String::from("Phone (เครื่องสื่อสารยุคใหม่แบบ modern communication!)"),
        }
    }

    let summary_item = create_summary();  // รับของลึกลับที่สร้างขึ้นมาแบบ mystery item!
    println!("\n🏭 === Returning Traits: การคืนค่าแบบลึกลับแบบ mystery return! === 🏭");
    println!("📱 สร้างสินค้า: {} (ผลิตภัณฑ์ใหม่จากโรงงานแบบ factory fresh!)", summary_item.display());

    println!("\n🎉 จบบทเรียน Trait Bounds! ตอนนี้คุณเป็นนักกำหนดเงื่อนไขมืออาชีพแล้ว! 🎉");
}
