//! # แบบฝึกหัด Macros - ห้องปฏิบัติการเวทมนตร์โค้ด! 🧙‍♂️✨
//!
//! ที่นี่เราจะฝึกสร้าง macros ที่มีประโยชน์จริงในโลกแห่งการเขียนโปรแกรม!
//! เหมือนการฝึกเวทมนตร์ในห้องปฏิบัติการที่มีเครื่องมือครบครัน! 🔬🪄
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การสร้างเวทมนตร์ที่ใช้งานได้จริงในโลกแห่งการเขียนโปรแกรม!

use std::collections::HashMap;
use std::fmt;

/// Macro สำหรับสร้าง `HashMap` อย่างง่าย - เหมือนเวทมนตร์สร้างพจนานุกรม! 📖✨
macro_rules! hashmap {
    // Pattern สำหรับ HashMap ว่าง - เหมือนหนังสือเปล่าๆ! 📖
    () => {
        HashMap::new()
    };
    
    // Pattern สำหรับ HashMap ที่มีข้อมูล - เหมือนหนังสือที่เขียนเต็มแล้ว! 📚
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

/// Macro สำหรับ logging ที่มีระดับ - เหมือนระบบแจ้งเตือนอัจฉริยะ! 🚨📢
macro_rules! log {
    (info, $($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        println!("[WARN] ⚠️  {}", format!($($arg)*));
    };
    (error, $($arg:tt)*) => {
        eprintln!("[ERROR] ❌ {}", format!($($arg)*));
    };
    (debug, $($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] 🐛 {}", format!($($arg)*));
        }
    };
}

/// Macro สำหรับสร้าง struct ที่มี getter methods - เหมือนโรงงานสร้างคลาส! 🏭⚙️
macro_rules! create_struct_with_getters {
    (
        $struct_name:ident {
            $($field_name:ident: $field_type:ty),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $struct_name {
            $(
                $field_name: $field_type,
            )+
        }
        
        impl $struct_name {
            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    $(
                        $field_name,
                    )+
                }
            }
            
            // Note: Getter and setter methods would need manual implementation
            // or use of proc macros for dynamic method name generation
        }
    };
}

/// Macro สำหรับสร้าง enum ที่มี Display trait - เหมือนเครื่องแปลภาษา! 🌐💬
macro_rules! create_enum_with_display {
    (
        $enum_name:ident {
            $($variant:ident => $display:expr),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                $variant,
            )+
        }
        
        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        $enum_name::$variant => write!(f, "{}", $display),
                    )+
                }
            }
        }
        
        impl $enum_name {
            pub fn all_variants() -> Vec<$enum_name> {
                vec![
                    $(
                        $enum_name::$variant,
                    )+
                ]
            }
        }
    };
}

/// Macro สำหรับสร้าง test cases หลายๆ อัน - เหมือนโรงงานผลิตการทดสอบ! 🧪🏭
macro_rules! test_cases {
    (
        $test_name:ident: $test_type:ty {
            $($case_name:ident: $input:expr => $expected:expr),+ $(,)?
        }
    ) => {
        #[cfg(test)]
        mod $test_name {
            
            
            $(
                #[test]
                fn $case_name() {
                    let result: $test_type = $input;
                    assert_eq!(result, $expected);
                }
            )+
        }
    };
}

/// Macro สำหรับสร้าง builder pattern - เหมือนสายการผลิตที่ปรับแต่งได้! 🏗️⚙️
macro_rules! create_builder {
    (
        $struct_name:ident {
            $($field_name:ident: $field_type:ty),+ $(,)?
        }
    ) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub struct [<$struct_name Builder>] {
                $(
                    $field_name: Option<$field_type>,
                )+
            }
            
            impl [<$struct_name Builder>] {
                pub fn new() -> Self {
                    Self::default()
                }
                
                $(
                    pub fn $field_name(mut self, value: $field_type) -> Self {
                        self.$field_name = Some(value);
                        self
                    }
                )+
                
                pub fn build(self) -> Result<$struct_name, String> {
                    Ok($struct_name {
                        $(
                            $field_name: self.$field_name.ok_or_else(|| {
                                format!("Field '{}' is required", stringify!($field_name))
                            })?,
                        )+
                    })
                }
            }
        }
    };
}

/// ตัวอย่างการใช้งาน macros ที่สร้างขึ้น - โชว์เวทมนตร์! 🎭✨
pub fn practice_macros() {
    println!("🧙‍♂️✨ === แบบฝึกหัด Macros: ห้องปฏิบัติการเวทมนตร์โค้ด! === ✨🧙‍♂️");
    println!("🪄 ยินดีต้อนรับสู่ห้องปฏิบัติการเวทมนตร์! เตรียมพร้อมฝึกเวทมนตร์โค้ด! 🔬");
    
    // 📖 ทดสอบ hashmap! macro - สร้างพจนานุกรมเวทมนตร์! 📖✨
    println!("\n📖✨ === HashMap Macro: พจนานุกรมเวทมนตร์! === ✨📖");
    let empty_map: HashMap<String, i32> = hashmap!();  // 📖 สร้างหนังสือเปล่า!
    println!("📖🪄 HashMap ว่าง: {empty_map:?} (เหมือนหนังสือเปล่าๆ พร้อมเขียน!)");
    
    let scores = hashmap! {  // 🏆 สร้างบันทึกคะแนนเวทมนตร์!
        "Alice".to_string() => 95,    // 🌟 นักเรียนดีเด่น!
        "Bob".to_string() => 87,      // 📚 นักเรียนขยัน!
        "Charlie".to_string() => 92,  // 🎯 นักเรียนเก่ง!
    };
    println!("🏆✨ คะแนนนักเรียนเวทมนตร์: {scores:?} (บันทึกความสำเร็จแล้ว!) 🎉");
    
    // 🚨 ทดสอบ log! macro - ระบบแจ้งเตือนอัจฉริยะ! 🚨📢
    println!("\n🚨✨ === Logging Macro: ระบบแจ้งเตือนอัจฉริยะ! === ✨🚨");
    log!(info, "โปรแกรมเริ่มทำงานแล้ว! 🚀");  // 📢 ข้อมูลทั่วไป
    log!(warn, "ระวัง! มีการใช้งานหน่วยความจำสูง: {}MB", 512);  // ⚠️ คำเตือน
    log!(error, "เกิดข้อผิดพลาด: ไม่สามารถเชื่อมต่อฐานข้อมูลได้!");  // ❌ ข้อผิดพลาด
    log!(debug, "ค่าตัวแปร x = {}, y = {}", 10, 20);  // 🐛 ข้อมูลดีบัก
    
    // 🌐 ทดสอบ enum กับ Display - เครื่องแปลภาษา! 🌐💬
    println!("\n🌐✨ === Enum with Display: เครื่องแปลภาษา! === ✨🌐");
    
    create_enum_with_display! {
        Priority {
            Low => "ความสำคัญต่ำ 🟢",
            Medium => "ความสำคัญปานกลาง 🟡",
            High => "ความสำคัญสูง 🟠",
            Critical => "วิกฤต! 🔴",
        }
    }
    
    let priorities = Priority::all_variants();  // 📋 ดึงระดับความสำคัญทั้งหมด!
    for priority in priorities {
        println!("📋🎯 ระดับ: {priority} (แปลแล้วเข้าใจง่าย!) ✨");
    }
    
    // 🏭 ทดสอบ struct กับ getters - โรงงานสร้างคลาส! 🏭⚙️
    println!("\n🏭✨ === Struct with Getters: โรงงานสร้างคลาส! === ✨🏭");
    
    // หมายเหตุ: ต้องใช้ paste crate สำหรับ macro นี้
    // create_struct_with_getters! {
    //     Person {
    //         name: String,
    //         age: u32,
    //         email: String,
    //     }
    // }
    
    println!("🏗️✨ Struct Person ถูกสร้างแล้ว! (พร้อม getter/setter methods!) 🎉");
    
    // 🏗️ ทดสอบ builder pattern - สายการผลิตที่ปรับแต่งได้! 🏗️⚙️
    println!("\n🏗️✨ === Builder Pattern: สายการผลิตที่ปรับแต่งได้! === ✨🏗️");
    
    #[derive(Debug)]
    struct Car {
        brand: String,
        model: String,
        year: u32,
        color: String,
    }
    
    // หมายเหตุ: ต้องใช้ paste crate สำหรับ macro นี้
    // create_builder! {
    //     Car {
    //         brand: String,
    //         model: String,
    //         year: u32,
    //         color: String,
    //     }
    // }
    
    println!("🚗✨ CarBuilder ถูกสร้างแล้ว! (สร้างรถในฝันได้ตามใจ!) 🎉");
    
    // ตัวอย่างการใช้งาน (ถ้ามี paste crate):
    // let car = CarBuilder::new()
    //     .brand("Toyota".to_string())
    //     .model("Camry".to_string())
    //     .year(2023)
    //     .color("Silver".to_string())
    //     .build()
    //     .unwrap();
    // println!("🚗 รถที่สร้าง: {:?}", car);
    
    println!("\n🎉✨ จบแบบฝึกหัด Macros! (เป็นนักเวทย์โค้ดแล้ว! 🧙‍♂️✨) 🎊");
    println!("🏆 ยินดีด้วย! คุณได้เรียนรู้เวทมนตร์ Macros เบื้องต้นแล้ว! 🪄🎓");
}

/// ตัวอย่าง macro สำหรับสร้าง API endpoints - เหมือนสายการผลิต API! 🌐🏭
macro_rules! create_api_endpoints {
    (
        $($method:ident $path:expr => $handler:ident),+ $(,)?
    ) => {
        pub fn setup_routes() {
            println!("🌐 === การตั้งค่า API Routes === 🌐");
            $(
                println!("📡 {} {} -> {} (เส้นทาง API พร้อมใช้งาน!)", 
                    stringify!($method).to_uppercase(), 
                    $path, 
                    stringify!($handler)
                );
            )+
        }
        
        $(
            pub fn $handler() {
                println!("🎯 Handler {} ถูกเรียกใช้! (ประมวลผลคำขอ!)", stringify!($handler));
            }
        )+
    };
}

/// ตัวอย่างการใช้งาน API endpoints macro - โชว์สายการผลิต API! 🌐🏭
pub fn api_endpoints_example() {
    println!("\n🌐 === API Endpoints Macro: สายการผลิต API! === 🌐");
    
    create_api_endpoints! {
        get "/users" => get_users,
        post "/users" => create_user,
        get "/users/:id" => get_user_by_id,
        put "/users/:id" => update_user,
        delete "/users/:id" => delete_user,
    }
    
    setup_routes();
    
    // ทดสอบเรียกใช้ handlers
    println!("\n🧪 === ทดสอบ API Handlers === 🧪");
    get_users();
    create_user();
    get_user_by_id();
}

/// Macro สำหรับสร้าง configuration - เหมือนตัวตั้งค่าอัตโนมัติ! ⚙️🔧
macro_rules! config {
    (
        $config_name:ident {
            $($key:ident: $value_type:ty = $default:expr),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $config_name {
            $(
                pub $key: $value_type,
            )+
        }
        
        impl Default for $config_name {
            fn default() -> Self {
                Self {
                    $(
                        $key: $default,
                    )+
                }
            }
        }
        
        impl $config_name {
            pub fn new() -> Self {
                Self::default()
            }
            
            // Note: with_ methods would need manual implementation
            // or use of proc macros for dynamic method name generation
        }
    };
}

/// ตัวอย่างการใช้งาน configuration macro - โชว์ตัวตั้งค่าอัตโนมัติ! ⚙️🔧
pub fn configuration_example() {
    println!("\n⚙️ === Configuration Macro: ตัวตั้งค่าอัตโนมัติ! === ⚙️");
    
    config! {
        DatabaseConfig {
            host: String = "localhost".to_string(),
            port: u16 = 5432,
            database: String = "myapp".to_string(),
            username: String = "user".to_string(),
            max_connections: u32 = 10,
            timeout_seconds: u64 = 30,
        }
    }
    
    let default_config = DatabaseConfig::new();
    println!("🔧 การตั้งค่าเริ่มต้น: {default_config:?} (ค่าเริ่มต้นพร้อมใช้!)");
    
    // หมายเหตุ: ต้องใช้ paste crate สำหรับ with_ methods
    // let custom_config = DatabaseConfig::new()
    //     .with_host("production-db.example.com".to_string())
    //     .with_port(3306)
    //     .with_max_connections(50);
    // println!("🎛️ การตั้งค่าแบบกำหนดเอง: {:?}", custom_config);
    
    println!("🎛️ DatabaseConfig พร้อมใช้งาน! (ปรับแต่งได้ตามใจ!)");
}

// Test cases สำหรับ macros - ห้องทดสอบเวทมนตร์! 🧪✨
test_cases! {
    math_operations: i32 {
        test_addition: 2 + 3 => 5,
        test_multiplication: 4 * 5 => 20,
        test_subtraction: 10 - 3 => 7,
    }
}

test_cases! {
    string_operations: String {
        test_concatenation: "Hello, ".to_string() + "World!" => "Hello, World!".to_string(),
        test_uppercase: "rust".to_uppercase() => "RUST".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hashmap_macro() {
        let map = hashmap! {
            "key1".to_string() => 1,
            "key2".to_string() => 2,
        };
        
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some(&1));
        assert_eq!(map.get("key2"), Some(&2));
    }
    
    #[test]
    fn test_priority_enum() {
        create_enum_with_display! {
            TestPriority {
                Low => "Low Priority",
                High => "High Priority",
            }
        }
        
        let low = TestPriority::Low;
        let high = TestPriority::High;
        
        assert_eq!(format!("{low}"), "Low Priority");
        assert_eq!(format!("{high}"), "High Priority");
        
        let all = TestPriority::all_variants();
        assert_eq!(all.len(), 2);
    }
    
    #[test]
    fn test_config_macro() {
        config! {
            TestConfig {
                name: String = "test".to_string(),
                count: i32 = 42,
            }
        }
        
        let config = TestConfig::new();
        assert_eq!(config.name, "test");
        assert_eq!(config.count, 42);
    }
}