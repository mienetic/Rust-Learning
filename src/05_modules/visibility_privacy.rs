// 🦀 Module System: Visibility และ Privacy 🦀
// เหมือนการตั้งกฎการเข้าถึงในบ้าน - ใครเข้าได้ ใครเข้าไม่ได้! 🏠🔐

/// ฟังก์ชันสำหรับสอนเรื่อง Visibility และ Privacy
/// เรียนรู้การควบคุมการเข้าถึงแบบมืออาชีพ! 👮‍♂️✨
pub fn learn_visibility_privacy() {
    println!("\n👁️ === Visibility และ Privacy === 👁️");

    // 🏠 ตัวอย่างการควบคุมความเป็นส่วนตัว - เหมือนกฎในบ้าน!
    mod privacy_example {
        // Private function (default) - ห้องส่วนตัว ห้ามเข้า! 🚫
        fn private_function() {
            println!("🔒 ฟังก์ชันส่วนตัว");
        }

        // Public function - ห้องรับแขก ยินดีต้อนรับ! 🎉
        pub fn public_function() {
            println!("🔓 ฟังก์ชันสาธารณะ");
            private_function(); // สามารถเรียกใช้ได้ภายใน module เดียวกัน - เหมือนคนในบ้าน! 🏡
            nested::nested_function(); // เรียกลูกได้! 👶
        }

        // Public function ที่เรียกใช้ private function - เหมือนพ่อบ้านเปิดห้องส่วนตัวให้แขก! 🗝️
        pub fn indirect_access() {
            println!("🔄 เข้าถึงทางอ้อม:");
            private_function(); // เข้าถึงของส่วนตัวผ่านคนในบ้าน! 🤝
        }

        // 🪆 Module ซ้อน - เหมือนห้องในห้อง!
        pub mod nested {
            /// ฟังก์ชันในห้องซ้อน - ลูกในบ้าน! 👶
            pub fn nested_function() {
                println!("🪆 ฟังก์ชันใน nested module");
                // super::private_function(); // ลูกเข้าห้องส่วนตัวของพ่อไม่ได้! 🚫
            }
        }

        // 🏗️ Struct กับ field visibility - เหมือนบ้านที่มีห้องสาธารณะและส่วนตัว!
        pub struct MixedVisibility {
            pub public_field: String, // ห้องรับแขก - ใครก็เข้าได้! 🚪
            private_field: i32, // ห้องส่วนตัว - เจ้าของเท่านั้น! 🔐
        }

        impl MixedVisibility {
            /// สร้างบ้านใหม่ - มีทั้งส่วนสาธารณะและส่วนตัว! 🏠
            pub const fn new(public_data: String, private_data: i32) -> Self {
                Self {
                    public_field: public_data,
                    private_field: private_data,
                }
            }

            /// ดูของในห้องส่วนตัว - ผ่านเจ้าของบ้าน! 👀
            pub const fn get_private(&self) -> i32 {
                self.private_field
            }

            /// เปลี่ยนของในห้องส่วนตัว - เจ้าของเท่านั้น! 🔧
            pub const fn set_private(&mut self, value: i32) {
                self.private_field = value;
            }
        }

        // 🎭 Enum กับ variant visibility - เหมือนเมนูอาหาร!
        pub enum PublicEnum {
            PublicVariant1, // เมนูสาธารณะ - ใครก็สั่งได้! 📋
            PublicVariant2(String), // เมนูพิเศษ - มีข้อความแนบ! 🌟
        }

        // 🤫 Enum ส่วนตัว - เมนูลับของเชฟ!
        enum PrivateEnum {
            _PrivateVariant1, // เมนูลับ 1 - เชฟเท่านั้น! 👨‍🍳
            _PrivateVariant2(i32), // เมนูลับ 2 - มีตัวเลขลึกลับ! 🔢
        }

        /// ใช้เมนูลับ - เฉพาะคนในครัว! 🍳
        pub fn use_private_enum() -> String {
            let _private = PrivateEnum::_PrivateVariant1; // เชฟใช้เมนูลับ! 🤐
            String::from("ใช้ private enum ภายใน module")
        }
    }

    println!("\n🔧 === การทดสอบ Visibility === 🔧");

    // เรียกใช้ public function
    privacy_example::public_function();
    privacy_example::indirect_access();

    // สร้าง struct และเข้าถึง fields
    let mut mixed = privacy_example::MixedVisibility::new(String::from("ข้อมูลสาธารณะ"), 100);

    println!("📊 Public field: {}", mixed.public_field);
    println!("🔒 Private field: {}", mixed.get_private());

    // แก้ไข fields
    mixed.public_field = String::from("ข้อมูลใหม่");
    mixed.set_private(200);

    println!("📊 Updated public field: {}", mixed.public_field);
    println!("🔒 Updated private field: {}", mixed.get_private());

    // ใช้ public enum
    let enum_values = [
        privacy_example::PublicEnum::PublicVariant1,
        privacy_example::PublicEnum::PublicVariant2(String::from("ข้อความ")),
    ];

    println!("\n🎭 === Public Enum === 🎭");
    for (i, value) in enum_values.iter().enumerate() {
        match value {
            privacy_example::PublicEnum::PublicVariant1 => {
                println!("{}. Variant1", i + 1);
            }
            privacy_example::PublicEnum::PublicVariant2(text) => {
                println!("{}. Variant2: {}", i + 1, text);
            }
        }
    }

    // ใช้ฟังก์ชันที่เข้าถึง private enum
    println!("\n🔒 === Private Enum === 🔒");
    println!("📝 {}", privacy_example::use_private_enum());

    // 🎯 Advanced visibility modifiers - ระบบควบคุมการเข้าถึงแบบโปร!
    mod advanced_visibility {
        // 🏢 pub(crate) - เห็นได้ทั้งบริษัท (crate) แต่ไม่ใช่คู่แข่ง!
        pub fn crate_visible() {
            println!("📦 มองเห็นได้ทั้ง crate - เหมือนข้อมูลภายในบริษัท! 🏢");
        }

        // 👨‍👩‍👧‍👦 pub(super) - เห็นได้แค่พ่อแม่ (parent module)!
        pub(super) fn super_visible() {
            println!("⬆️ มองเห็นได้จาก parent module - เหมือนความลับครอบครัว! 👨‍👩‍👧‍👦");
        }

        // 🗺️ pub(in path) - เห็นได้แค่ในเส้นทางที่กำหนด!
        pub(in crate::modules) fn path_visible() {
            println!("🛤️ มองเห็นได้ใน modules path - เหมือนทางลัดเฉพาะคน! 🗺️");
        }

        pub fn public_caller() {
            crate_visible();
            super_visible();
            path_visible();
        }
    }

    println!("\n🎯 === Advanced Visibility === 🎯");
    advanced_visibility::public_caller();
    
    // 🚀 ใช้งาน advanced visibility - ทดสอบระบบรักษาความปลอดภัย!
    advanced_visibility::crate_visible(); // เรียกฟังก์ชันระดับบริษัท! 🏢
    advanced_visibility::super_visible(); // เรียกฟังก์ชันครอบครัว! 👨‍👩‍👧‍👦
    advanced_visibility::path_visible(); // เรียกฟังก์ชันทางลัด! 🗺️

    println!("\n🔒 Privacy และ Visibility ใน Rust - ระบบรักษาความปลอดภัยแบบโปร!");
    println!("📚 ใช้ pub, pub(crate), pub(super), pub(in path) เหมือนการออกแบบระบบรักษาความปลอดภัย! 🛡️");
    println!("🎯 เลือกระดับการเข้าถึงให้เหมาะสม - ไม่เปิดมากเกินไป ไม่ปิดจนใช้ไม่ได้!");
}
