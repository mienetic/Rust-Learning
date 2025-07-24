//! # Code Generation Macros - เวทมนตร์สร้างโค้ดแบบ Code Summoning! 🏗️✨
//!
//! ตัวอย่างการใช้ macros สำหรับสร้าง code อัตโนมัติ
//! เหมือนการเรียกผีมาช่วยเขียนโค้ดให้เรา! 👻🪄
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ที่ช่วยสร้างโค้ดอัตโนมัติ!
//!
//! ⚠️ **หมายเหตุเกี่ยวกับ Warnings** (เรื่องปกติของเวทมนตร์!):
//! - Structs ที่สร้างจาก macros (Person, Product, Config) จะมี warning "fields never read"
//! - 🎭 **เหตุผล**: ฟิลด์เหล่านี้ถูกสร้างเพื่อสาธิตการทำงานของ macros เท่านั้น
//! - 🔮 ไม่มีการเข้าถึงฟิลด์โดยตรงในโค้ด เพียงแค่แสดงผลด้วย Debug trait
//! - 🪄 การลบฟิลด์เหล่านี้จะทำให้ตัวอย่าง macro ไม่สมบูรณ์
//! - ✨ Warnings เหล่านี้ไม่ส่งผลต่อการทำงานของโปรแกรม (เป็นเรื่องปกติของเวทมนตร์!)

// 🏗️ Code generation macros - ตัวอย่างการสร้างโค้ดด้วยเวทมนตร์! 🪄✨
// ไฟล์นี้แสดงตัวอย่างการใช้ macros สำหรับสร้างโค้ดอัตโนมัติ
// เหมือนมีผีช่วยเขียนโค้ดให้เรา! 👻📝

/// ตัวอย่าง macro สำหรับสร้าง struct - เวทมนตร์สร้างโครงสร้าง! 🏗️🔮
pub fn struct_generation_macros() {
    println!("\n🏗️✨ === ตัวอย่าง Struct Generation Macros - เวทมนตร์สร้างโครงสร้าง! === ✨🏗️");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยสร้าง struct อัตโนมัติ! 👻");

    // 📝 หมายเหตุ: ลบ macro create_struct ออกแล้วเนื่องจากต้องการ paste crate
    // ซึ่งไม่ได้ติดตั้งในโปรเจกต์นี้ และทำให้เกิด unused macro warning
    // 🎯 แทนที่จะใช้ paste เราจะใช้วิธีง่ายๆ แบบเวทมนตร์พื้นฐาน!
    // 🪄 เวทมนตร์สร้าง struct แบบง่ายๆ - Simple Struct Summoning Spell! 🏗️✨
    macro_rules! simple_struct {
        ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {
            #[derive(Debug)]  // 🔍 เวทมนตร์ Debug สำหรับแสดงผล!
            struct $struct_name {
                $($field_name: $field_type,)*  // 🎯 เวทมนตร์สร้างฟิลด์!
            }

            impl $struct_name {
                const fn new($($field_name: $field_type),*) -> Self {  // 🏗️ เวทมนตร์สร้างตัว!
                    Self {
                        $($field_name,)*  // 🪄 เวทมนตร์กำหนดค่า!
                    }
                }
            }
        };
    }

    // 👤 เวทมนตร์สร้างคน - Person Summoning Spell! 🧙‍♂️
    // ⚠️ Warning: fields `name`, `age` จะไม่ถูกใช้งานโดยตรง (เป็นเรื่องปกติของเวทมนตร์!)
    // 🎭 สาเหตุ: สร้างเพื่อสาธิตการทำงานของ macro เท่านั้น
    simple_struct!(Person {
        name: String,    // 📝 ชื่อของคนที่เรียกมา
        age: u32         // 🎂 อายุของคนที่เรียกมา
    });

    // 🛍️ เวทมนตร์สร้างสินค้า - Product Summoning Spell! 📦
    // ⚠️ Warning: fields `name`, `price`, `quantity` จะไม่ถูกใช้งานโดยตรง (เป็นเรื่องปกติของเวทมนตร์!)
    // 🎭 สาเหตุ: สร้างเพื่อสาธิตการทำงานของ macro เท่านั้น
    simple_struct!(Product {
        name: String,     // 📝 ชื่อสินค้าที่เรียกมา
        price: f64,       // 💰 ราคาของสินค้า
        quantity: u32     // 📊 จำนวนสินค้า
    });

    // 🪄 เรียกใช้เวทมนตร์สร้างตัวอย่าง! ✨
    let person = Person::new("สมชาย".to_string(), 25);           // 👤 เรียกคนมาจากเวทมนตร์!
    let product = Product::new("แล็ปท็อป".to_string(), 25000.0, 5); // 🛍️ เรียกสินค้ามาจากเวทมนตร์!

    println!("\n🎭 === ผลลัพธ์เวทมนตร์สร้างโครงสร้าง === 🎭");
    println!("👤✨ Person (คนที่เรียกมา): {person:?}");
    println!("   📝 ชื่อ: {}, 🎂 อายุ: {} ปี", person.name, person.age);
    
    println!("🛍️✨ Product (สินค้าที่เรียกมา): {product:?}");
    println!("   📝 ชื่อสินค้า: {}, 💰 ราคา: {} บาท, 📊 จำนวน: {} ชิ้น", product.name, product.price, product.quantity);
    
    println!("\n🎓 บทเรียนเวทมนตร์:");
    println!("📝 Macro สำหรับสร้าง struct พร้อม getter/setter methods");
    println!("   (ตัวอย่าง advanced macro ถูกลบออกเพราะต้องการ paste crate)");
    println!("🪄 เวทมนตร์นี้ช่วยลดการเขียนโค้ดซ้ำ ๆ อย่างมาก! ✨");
}

/// ตัวอย่าง macro สำหรับ configuration - เวทมนตร์ตั้งค่าแบบ Config Spells! ⚙️🔮
pub fn configuration_macros() {
    println!("\n⚙️✨ === ตัวอย่าง Configuration Macros - เวทมนตร์ตั้งค่า! === ✨⚙️");
    println!("🪄 เรียนรู้การสร้างเวทมนตร์ที่ช่วยจัดการ configuration! 📋");

    // ⚙️ Macro สำหรับสร้าง configuration struct - เวทมนตร์ตั้งค่าอัตโนมัติ! 📋✨
    macro_rules! config {
        {
            $(
                $key:ident: $value:expr  // 🔑 คีย์และค่าของการตั้งค่า
            ),* $(,)?
        } => {
            {
                #[derive(Debug)]  // 🔍 เวทมนตร์ Debug สำหรับแสดงผล!
                struct Config {
                    $($key: String,)*  // 🎯 เวทมนตร์สร้างฟิลด์ config!
                }

                Config {
                    $($key: $value.to_string(),)*  // 🪄 เวทมนตร์กำหนดค่า config!
                }
            }
        };
    }

    // 📱 เวทมนตร์สร้าง App Configuration - App Config Summoning! 🎯
    // ⚠️ Warning: fields `app_name`, `version`, `author`, `description` จะไม่ถูกใช้งานโดยตรง (เป็นเรื่องปกติของเวทมนตร์!)
    // 🎭 สาเหตุ: สร้างเพื่อสาธิตการทำงานของ config macro เท่านั้น
    // 🔮 ฟิลด์เหล่านี้ถูกแสดงผลผ่าน Debug trait แต่ไม่ได้เข้าถึงโดยตรง
    let app_config = config! {
        app_name: "Rust Learning App",     // 📱 ชื่อแอป
        version: "1.0.0",                 // 🔢 เวอร์ชัน
        author: "Rust Developer",         // 👨‍💻 ผู้พัฒนา
        description: "แอปเรียนรู้ Rust"    // 📝 คำอธิบาย
    };

    println!("\n📋✨ === ผลลัพธ์เวทมนตร์ตั้งค่า === ✨📋");
    println!("📱 App Configuration (การตั้งค่าแอป): {app_config:#?}");
    println!("   📱 แอป: {}, 🔢 เวอร์ชัน: {}", app_config.app_name, app_config.version);
    println!("   👨‍💻 ผู้พัฒนา: {}", app_config.author);
    println!("   📝 คำอธิบาย: {}", app_config.description);

    // 🌍 Macro สำหรับ environment variables - เวทมนตร์ตัวแปรสภาพแวดล้อม! 🔧✨
    macro_rules! env_or_default {
        ($env_var:expr, $default:expr) => {
            std::env::var($env_var).unwrap_or_else(|_| $default.to_string())  // 🪄 เวทมนตร์หาค่า env หรือใช้ค่าเริ่มต้น!
        };
    }

    let database_url = env_or_default!("DATABASE_URL", "sqlite://local.db");  // 🗄️ เวทมนตร์หา Database URL!
    let port = env_or_default!("PORT", "8080");                              // 🌐 เวทมนตร์หา Port!

    println!("\n🌍🔧 === ผลลัพธ์เวทมนตร์ตัวแปรสภาพแวดล้อม === 🔧🌍");
    println!("🗄️ Database URL: {database_url} (จากเวทมนตร์ env!)");
    println!("🌐 Port: {port} (จากเวทมนตร์ env!)");
    println!("\n🎉 เวทมนตร์ configuration สำเร็จ! 🪄✨");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_struct_macro() {
        macro_rules! simple_struct {
            ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {
                #[derive(Debug, PartialEq)]
                struct $struct_name {
                    $($field_name: $field_type,)*
                }

                impl $struct_name {
                    fn new($($field_name: $field_type),*) -> Self {
                        Self {
                            $($field_name,)*
                        }
                    }
                }
            };
        }

        simple_struct!(TestStruct {
            name: String,
            value: i32
        });

        let test_obj = TestStruct::new("test".to_string(), 42);
        assert_eq!(test_obj.name, "test");
        assert_eq!(test_obj.value, 42);
    }

    #[test]
    fn test_config_macro() {
        macro_rules! config {
            {
                $(
                    $key:ident: $value:expr
                ),* $(,)?
            } => {
                {
                    #[derive(Debug)]
                    struct Config {
                        $($key: String,)*
                    }

                    Config {
                        $($key: $value.to_string(),)*
                    }
                }
            };
        }

        let config = config! {
            name: "test",
            version: "1.0"
        };

        assert_eq!(config.name, "test");
        assert_eq!(config.version, "1.0");
    }

    #[test]
    fn test_code_generation_functions() {
        // ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic
        struct_generation_macros();
        configuration_macros();
    }
}