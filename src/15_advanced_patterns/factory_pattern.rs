//! 🏭 Product Manufacturing Workshop - Factory Pattern Implementation
//!
//! ยินดีต้อนรับสู่ Product Manufacturing Workshop! 🎯
//! การใช้งาน Factory Pattern ใน Rust สำหรับการผลิตสินค้าเทคโนโลยี
//! ครอบคลุม Simple Factory, Factory Method, และ Abstract Factory
//! เหมือนการจัดการสายการผลิตในโรงงานสมัยใหม่! 🏗️

use std::collections::HashMap;
use std::fmt;

/// 📦 Product Blueprint - แบบแปลนสินค้าสำหรับโรงงานผลิต
pub trait Product: fmt::Debug {
    fn get_name(&self) -> &str;
    fn get_price(&self) -> f64;
    fn get_description(&self) -> &str;
}

/// 💻 Laptop Production Line - สายการผลิตแล็ปท็อป
#[derive(Debug, Clone)]
pub struct Laptop {
    name: String,
    price: f64,
    cpu: String,
    ram: u32,
    storage: String,
}

impl Laptop {
    /// 🔧 สร้างแล็ปท็อปใหม่จากสายการผลิต
    #[must_use] pub const fn new(name: String, price: f64, cpu: String, ram: u32, storage: String) -> Self {
        Self { name, price, cpu, ram, storage }
    }
    
    /// 📋 ดูข้อมูลสเปคของแล็ปท็อป
    #[must_use] pub fn get_specs(&self) -> String {
        format!("CPU: {}, RAM: {}GB, Storage: {}", self.cpu, self.ram, self.storage)
    }
}

impl Product for Laptop {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_price(&self) -> f64 {
        self.price
    }
    
    fn get_description(&self) -> &'static str {
        "🚀 High-performance laptop computer from our production line"
    }
}

/// 📱 Smartphone Production Line - สายการผลิตสมาร์ทโฟน
#[derive(Debug, Clone)]
pub struct Smartphone {
    name: String,
    price: f64,
    os: String,
    screen_size: f64,
    camera_mp: u32,
}

impl Smartphone {
    /// 🔧 สร้างสมาร์ทโฟนใหม่จากสายการผลิต
    #[must_use] pub const fn new(name: String, price: f64, os: String, screen_size: f64, camera_mp: u32) -> Self {
        Self { name, price, os, screen_size, camera_mp }
    }
    
    /// 📋 ดูข้อมูลสเปคของสมาร์ทโฟน
    #[must_use] pub fn get_specs(&self) -> String {
        format!("OS: {}, Screen: {}\" Camera: {}MP", self.os, self.screen_size, self.camera_mp)
    }
}

impl Product for Smartphone {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_price(&self) -> f64 {
        self.price
    }
    
    fn get_description(&self) -> &'static str {
        "📲 Advanced smartphone device from our mobile production line"
    }
}

/// 📟 Tablet Production Line - สายการผลิตแท็บเล็ต
#[derive(Debug, Clone)]
pub struct Tablet {
    name: String,
    price: f64,
    os: String,
    screen_size: f64,
    has_keyboard: bool,
}

impl Tablet {
    /// 🔧 สร้างแท็บเล็ตใหม่จากสายการผลิต
    #[must_use] pub const fn new(name: String, price: f64, os: String, screen_size: f64, has_keyboard: bool) -> Self {
        Self { name, price, os, screen_size, has_keyboard }
    }
    
    /// 📋 ดูข้อมูลสเปคของแท็บเล็ต
    #[must_use] pub fn get_specs(&self) -> String {
        format!("OS: {}, Screen: {}\" Keyboard: {}", 
               self.os, self.screen_size, 
               if self.has_keyboard { "Yes" } else { "No" })
    }
}

impl Product for Tablet {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_price(&self) -> f64 {
        self.price
    }
    
    fn get_description(&self) -> &'static str {
        "📱 Portable tablet computer from our mobile production line"
    }
}

/// 🏷️ Product Types - ประเภทสินค้าในโรงงาน
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductType {
    Laptop,     // 💻 แล็ปท็อป
    Smartphone, // 📱 สมาร์ทโฟน
    Tablet,     // 📟 แท็บเล็ต
}

/// 🏭 Simple Factory Pattern - โรงงานผลิตแบบง่าย
pub struct SimpleFactory;

impl SimpleFactory {
    /// 🏭 ผลิตสินค้าตามประเภทที่กำหนด - Mass production by type
    #[must_use] pub fn create_product(product_type: ProductType) -> Box<dyn Product> {
        match product_type {
            ProductType::Laptop => Box::new(Laptop::new(
                "🏭 Factory Standard Laptop".to_string(),
                999.99,
                "Intel i5".to_string(),
                8,
                "256GB SSD".to_string(),
            )),
            ProductType::Smartphone => Box::new(Smartphone::new(
                "🏭 Factory Standard Phone".to_string(),
                699.99,
                "Android".to_string(),
                6.1,
                48,
            )),
            ProductType::Tablet => Box::new(Tablet::new(
                "🏭 Factory Standard Tablet".to_string(),
                399.99,
                "iPadOS".to_string(),
                10.9,
                false,
            )),
        }
    }
    
    /// 🔧 ผลิตแล็ปท็อปแบบกำหนดเอง - Custom laptop production
    #[must_use] pub fn create_custom_laptop(name: String, cpu: String, ram: u32) -> Box<dyn Product> {
        let price = match ram {
            4 => 599.99,
            8 => 799.99,
            16 => 1199.99,
            32 => 1799.99,
            _ => 999.99,
        };
        
        Box::new(Laptop::new(
            format!("🔧 Custom {name}"),
            price,
            cpu,
            ram,
            "512GB SSD".to_string(),
        ))
    }
}

/// 🏭 Factory Method Pattern - รูปแบบโรงงานผลิตเฉพาะแบรนด์
pub trait ProductFactory {
    fn create_product(&self) -> Box<dyn Product>;
    fn get_brand(&self) -> &str;
}

/// 🍎 Apple Manufacturing Plant - โรงงานผลิต Apple
pub struct AppleFactory;

impl ProductFactory for AppleFactory {
    fn create_product(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "🍎 MacBook Pro".to_string(),
            2499.99,
            "Apple M2 Pro".to_string(),
            16,
            "1TB SSD".to_string(),
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "🍎 Apple"
    }
}

impl AppleFactory {
    /// 📱 ผลิต iPhone จากสายการผลิต Apple
    #[must_use] pub fn create_iphone(&self) -> Box<dyn Product> {
        Box::new(Smartphone::new(
            "🍎 iPhone 15 Pro".to_string(),
            1199.99,
            "iOS 17".to_string(),
            6.1,
            48,
        ))
    }
    
    /// 📟 ผลิต iPad จากสายการผลิต Apple
    #[must_use] pub fn create_ipad(&self) -> Box<dyn Product> {
        Box::new(Tablet::new(
            "🍎 iPad Pro".to_string(),
            1099.99,
            "iPadOS 17".to_string(),
            12.9,
            true,
        ))
    }
}

/// 🌟 Samsung Manufacturing Plant - โรงงานผลิต Samsung
pub struct SamsungFactory;

impl ProductFactory for SamsungFactory {
    fn create_product(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "🌟 Galaxy Book Pro".to_string(),
            1299.99,
            "Intel i7".to_string(),
            16,
            "512GB SSD".to_string(),
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "🌟 Samsung"
    }
}

impl SamsungFactory {
    /// 📱 ผลิต Galaxy Phone จากสายการผลิต Samsung
    #[must_use] pub fn create_galaxy_phone(&self) -> Box<dyn Product> {
        Box::new(Smartphone::new(
            "🌟 Galaxy S24 Ultra".to_string(),
            1299.99,
            "Android 14".to_string(),
            6.8,
            200,
        ))
    }
    
    /// 📟 ผลิต Galaxy Tab จากสายการผลิต Samsung
    #[must_use] pub fn create_galaxy_tab(&self) -> Box<dyn Product> {
        Box::new(Tablet::new(
            "🌟 Galaxy Tab S9".to_string(),
            899.99,
            "Android 14".to_string(),
            11.0,
            true,
        ))
    }
}

/// 🖥️ Dell Manufacturing Plant - โรงงานผลิต Dell
pub struct DellFactory;

impl ProductFactory for DellFactory {
    fn create_product(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "🖥️ XPS 13".to_string(),
            1199.99,
            "Intel i7".to_string(),
            16,
            "512GB SSD".to_string(),
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "🖥️ Dell"
    }
}

/// 🏗️ Abstract Factory Pattern - รูปแบบโรงงานผลิตครบวงจร
pub trait AbstractFactory {
    fn create_laptop(&self) -> Box<dyn Product>;
    fn create_smartphone(&self) -> Box<dyn Product>;
    fn create_tablet(&self) -> Box<dyn Product>;
    fn get_brand(&self) -> &str;
}

/// 💎 Premium Product Manufacturing Line - สายการผลิตสินค้าพรีเมียม
pub struct PremiumFactory;

impl AbstractFactory for PremiumFactory {
    fn create_laptop(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "💎 Premium Laptop Pro".to_string(),
            2999.99,
            "Intel i9".to_string(),
            32,
            "2TB SSD".to_string(),
        ))
    }
    
    fn create_smartphone(&self) -> Box<dyn Product> {
        Box::new(Smartphone::new(
            "💎 Premium Phone Pro Max".to_string(),
            1599.99,
            "Premium OS".to_string(),
            6.7,
            108,
        ))
    }
    
    fn create_tablet(&self) -> Box<dyn Product> {
        Box::new(Tablet::new(
            "💎 Premium Tablet Pro".to_string(),
            1299.99,
            "Premium OS".to_string(),
            12.9,
            true,
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "💎 Premium"
    }
}

/// 💰 Budget Product Manufacturing Line - สายการผลิตสินค้าประหยัด
pub struct BudgetFactory;

impl AbstractFactory for BudgetFactory {
    fn create_laptop(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "💰 Budget Laptop".to_string(),
            499.99,
            "AMD Ryzen 3".to_string(),
            4,
            "128GB SSD".to_string(),
        ))
    }
    
    fn create_smartphone(&self) -> Box<dyn Product> {
        Box::new(Smartphone::new(
            "💰 Budget Phone".to_string(),
            199.99,
            "Android Go".to_string(),
            5.5,
            13,
        ))
    }
    
    fn create_tablet(&self) -> Box<dyn Product> {
        Box::new(Tablet::new(
            "💰 Budget Tablet".to_string(),
            149.99,
            "Android".to_string(),
            8.0,
            false,
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "💰 Budget"
    }
}

/// 🎮 Gaming Product Manufacturing Line - สายการผลิตสินค้าเกมมิ่ง
pub struct GamingFactory;

impl AbstractFactory for GamingFactory {
    fn create_laptop(&self) -> Box<dyn Product> {
        Box::new(Laptop::new(
            "🎮 Gaming Laptop Beast".to_string(),
            2499.99,
            "Intel i9 + RTX 4080".to_string(),
            32,
            "1TB NVMe SSD".to_string(),
        ))
    }
    
    fn create_smartphone(&self) -> Box<dyn Product> {
        Box::new(Smartphone::new(
            "🎮 Gaming Phone Pro".to_string(),
            899.99,
            "Gaming Android".to_string(),
            6.8,
            64,
        ))
    }
    
    fn create_tablet(&self) -> Box<dyn Product> {
        Box::new(Tablet::new(
            "🎮 Gaming Tablet".to_string(),
            699.99,
            "Gaming OS".to_string(),
            11.0,
            true,
        ))
    }
    
    fn get_brand(&self) -> &'static str {
        "🎮 Gaming"
    }
}

/// 📋 Factory Registry - ทะเบียนโรงงานผลิตแบบไดนามิก
pub struct FactoryRegistry {
    factories: HashMap<String, Box<dyn AbstractFactory>>,
}

impl Default for FactoryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FactoryRegistry {
    /// 🏭 สร้างทะเบียนโรงงานผลิตใหม่
    #[must_use] pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };
        
        registry.register_factory("premium".to_string(), Box::new(PremiumFactory));
        registry.register_factory("budget".to_string(), Box::new(BudgetFactory));
        registry.register_factory("gaming".to_string(), Box::new(GamingFactory));
        
        registry
    }
    
    /// 📝 ลงทะเบียนโรงงานผลิตใหม่
    pub fn register_factory(&mut self, name: String, factory: Box<dyn AbstractFactory>) {
        self.factories.insert(name, factory);
    }
    
    /// 🔍 ค้นหาโรงงานผลิตตามชื่อ
    #[must_use] pub fn get_factory(&self, name: &str) -> Option<&dyn AbstractFactory> {
        self.factories.get(name).map(std::convert::AsRef::as_ref)
    }
    
    /// 📋 แสดงรายชื่อโรงงานผลิตทั้งหมด
    #[must_use] pub fn list_factories(&self) -> Vec<&String> {
        self.factories.keys().collect()
    }
    
    /// 🎁 สร้างชุดสินค้าจากโรงงานที่กำหนด
    #[must_use] pub fn create_product_set(&self, factory_name: &str) -> Option<(Box<dyn Product>, Box<dyn Product>, Box<dyn Product>)> {
        if let Some(factory) = self.get_factory(factory_name) {
            Some((
                factory.create_laptop(),
                factory.create_smartphone(),
                factory.create_tablet(),
            ))
        } else {
            None
        }
    }
}

/// 🔧 Product Builder - เครื่องมือสร้างสินค้าแบบกำหนดเอง (Builder Pattern with Factory)
pub struct ProductBuilder {
    product_type: Option<ProductType>,
    name: Option<String>,
    price: Option<f64>,
    specs: HashMap<String, String>,
}

impl Default for ProductBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductBuilder {
    /// 🆕 สร้าง Product Builder ใหม่
    #[must_use] pub fn new() -> Self {
        Self {
            product_type: None,
            name: None,
            price: None,
            specs: HashMap::new(),
        }
    }
    
    /// 📱 กำหนดประเภทสินค้า
    #[must_use] pub const fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }
    
    /// 🏷️ กำหนดชื่อสินค้า
    #[must_use] pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    /// 💰 กำหนดราคาสินค้า
    #[must_use] pub const fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
    
    /// ⚙️ เพิ่มข้อมูลสเปค
    #[must_use] pub fn spec(mut self, key: String, value: String) -> Self {
        self.specs.insert(key, value);
        self
    }
    
    /// 🔨 สร้างสินค้าตามข้อมูลที่กำหนด
    pub fn build(self) -> Result<Box<dyn Product>, String> {
        let product_type = self.product_type.ok_or("Product type not specified")?;
        let name = self.name.unwrap_or_else(|| "🔧 Custom Product".to_string());
        let price = self.price.unwrap_or(0.0);
        
        match product_type {
            ProductType::Laptop => {
                let cpu = self.specs.get("cpu").cloned().unwrap_or_else(|| "Generic CPU".to_string());
                let ram = self.specs.get("ram")
                    .and_then(|r| r.parse().ok())
                    .unwrap_or(8);
                let storage = self.specs.get("storage").cloned().unwrap_or_else(|| "256GB SSD".to_string());
                
                Ok(Box::new(Laptop::new(name, price, cpu, ram, storage)))
            }
            ProductType::Smartphone => {
                let os = self.specs.get("os").cloned().unwrap_or_else(|| "Android".to_string());
                let screen_size = self.specs.get("screen_size")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(6.0);
                let camera_mp = self.specs.get("camera_mp")
                    .and_then(|c| c.parse().ok())
                    .unwrap_or(12);
                
                Ok(Box::new(Smartphone::new(name, price, os, screen_size, camera_mp)))
            }
            ProductType::Tablet => {
                let os = self.specs.get("os").cloned().unwrap_or_else(|| "Android".to_string());
                let screen_size = self.specs.get("screen_size")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10.0);
                let has_keyboard = self.specs.get("has_keyboard")
                    .is_some_and(|k| k == "true");
                
                Ok(Box::new(Tablet::new(name, price, os, screen_size, has_keyboard)))
            }
        }
    }
}

/// 🎭 สาธิตการใช้งาน Factory Pattern - การแสดงการทำงานของโรงงานผลิต
pub fn demonstrate_factory_pattern() {
    println!("🏭 === Factory Pattern Manufacturing Workshop === 🏭");
    
    // Simple Factory
    println!("\n🔧 1. Simple Factory Pattern - โรงงานผลิตแบบง่าย:");
    println!("{:-<50}", "");
    
    let laptop = SimpleFactory::create_product(ProductType::Laptop);
    let phone = SimpleFactory::create_product(ProductType::Smartphone);
    let tablet = SimpleFactory::create_product(ProductType::Tablet);
    
    println!("✅ สินค้าที่ผลิตได้:");
    println!("- {}: ${:.2} - {}", laptop.get_name(), laptop.get_price(), laptop.get_description());
    println!("- {}: ${:.2} - {}", phone.get_name(), phone.get_price(), phone.get_description());
    println!("- {}: ${:.2} - {}", tablet.get_name(), tablet.get_price(), tablet.get_description());
    
    let custom_laptop = SimpleFactory::create_custom_laptop(
        "🎮 Custom Gaming Laptop".to_string(),
        "Intel i9".to_string(),
        32,
    );
    println!("🎨 สินค้าแบบกำหนดเอง: {}: ${:.2} - {}", custom_laptop.get_name(), custom_laptop.get_price(), custom_laptop.get_description());
    
    // Factory Method
    println!("\n🏭 2. Factory Method Pattern - วิธีการผลิตแบบเฉพาะ:");
    println!("{:-<50}", "");
    
    let apple_factory = AppleFactory;
    let samsung_factory = SamsungFactory;
    let dell_factory = DellFactory;
    
    let factories: Vec<&dyn ProductFactory> = vec![&apple_factory, &samsung_factory, &dell_factory];
    
    for factory in factories {
        let product = factory.create_product();
        println!("{} สินค้า: {} - ${:.2}", 
                factory.get_brand(), product.get_name(), product.get_price());
    }
    
    // Brand-specific products
    println!("\n🏷️ สินค้าเฉพาะแบรนด์:");
    let iphone = apple_factory.create_iphone();
    let ipad = apple_factory.create_ipad();
    let galaxy_phone = samsung_factory.create_galaxy_phone();
    let galaxy_tab = samsung_factory.create_galaxy_tab();
    
    println!("🍎 Apple: {} (${:.2}), {} (${:.2})", 
            iphone.get_name(), iphone.get_price(),
            ipad.get_name(), ipad.get_price());
    println!("📱 Samsung: {} (${:.2}), {} (${:.2})", 
            galaxy_phone.get_name(), galaxy_phone.get_price(),
            galaxy_tab.get_name(), galaxy_tab.get_price());
    
    // Abstract Factory
    println!("\n🏢 3. Abstract Factory Pattern - โรงงานผลิตแบบนามธรรม:");
    println!("{:-<50}", "");
    
    let premium_factory = PremiumFactory;
    let budget_factory = BudgetFactory;
    let gaming_factory = GamingFactory;
    
    let abstract_factories: Vec<&dyn AbstractFactory> = vec![
        &premium_factory, &budget_factory, &gaming_factory
    ];
    
    for factory in abstract_factories {
        println!("\n{} ครอบครัวสินค้า:", factory.get_brand());
        
        let laptop = factory.create_laptop();
        let phone = factory.create_smartphone();
        let tablet = factory.create_tablet();
        
        println!("  💻 แล็ปท็อป: {} - ${:.2}", laptop.get_name(), laptop.get_price());
        println!("  📱 โทรศัพท์: {} - ${:.2}", phone.get_name(), phone.get_price());
        println!("  📟 แท็บเล็ต: {} - ${:.2}", tablet.get_name(), tablet.get_price());
    }
    
    // Factory Registry
    println!("\n📋 4. Factory Registry Pattern - ทะเบียนโรงงานผลิต:");
    println!("{:-<50}", "");
    
    let registry = FactoryRegistry::new();
    
    println!("🏭 โรงงานที่มีอยู่: {:?}", registry.list_factories());
    
    for factory_name in registry.list_factories() {
        if let Some((laptop, phone, tablet)) = registry.create_product_set(factory_name) {
            println!("\n{factory_name} ชุดสินค้า:");
            println!("  {} (${:.2})", laptop.get_name(), laptop.get_price());
            println!("  {} (${:.2})", phone.get_name(), phone.get_price());
            println!("  {} (${:.2})", tablet.get_name(), tablet.get_price());
        }
    }
    
    // Product Builder
    println!("\n🔧 5. Product Builder with Factory - เครื่องมือสร้างสินค้าแบบกำหนดเอง:");
    println!("{:-<50}", "");
    
    let custom_laptop = ProductBuilder::new()
        .product_type(ProductType::Laptop)
        .name("🔧 Custom Workstation".to_string())
        .price(3499.99)
        .spec("cpu".to_string(), "Intel Xeon".to_string())
        .spec("ram".to_string(), "64".to_string())
        .spec("storage".to_string(), "4TB NVMe SSD".to_string())
        .build();
    
    match custom_laptop {
        Ok(laptop) => {
            println!("✅ สร้างแล็ปท็อปแบบกำหนดเอง: {} - ${:.2}", laptop.get_name(), laptop.get_price());
        }
        Err(e) => println!("❌ ไม่สามารถสร้างแล็ปท็อป: {e}"),
    }
    
    let custom_phone = ProductBuilder::new()
        .product_type(ProductType::Smartphone)
        .name("🎮 Custom Gaming Phone".to_string())
        .price(1299.99)
        .spec("os".to_string(), "Gaming Android".to_string())
        .spec("screen_size".to_string(), "6.8".to_string())
        .spec("camera_mp".to_string(), "200".to_string())
        .build();
    
    match custom_phone {
        Ok(phone) => {
            println!("✅ สร้างโทรศัพท์แบบกำหนดเอง: {} - ${:.2}", phone.get_name(), phone.get_price());
        }
        Err(e) => println!("❌ ไม่สามารถสร้างโทรศัพท์: {e}"),
    }
    
    println!("\n🎯 === ประโยชน์ของ Factory Pattern === 🎯");
    println!("✅ การห่อหุ้ม: ซ่อนตรรกะการสร้างออบเจ็กต์");
    println!("✅ ความยืดหยุ่น: เพิ่มประเภทสินค้าใหม่ได้ง่าย");
    println!("✅ ความสม่ำเสมอ: กระบวนการสร้างที่มาตรฐาน");
    println!("✅ การทดสอบ: สร้าง mock factories ได้ง่าย");
    println!("✅ การบำรุงรักษา: การเปลี่ยนแปลงแยกออกในคลาส factory");
    println!("✅ พอลิมอร์ฟิซึม: ทำงานกับสินค้าผ่าน interface ร่วม");
    
    println!("\n🏭 Factory patterns demonstration completed! 🏭");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪 ทดสอบ Simple Factory Pattern - การผลิตสินค้าแบบง่าย
    #[test]
    fn test_simple_factory() {
        let laptop = SimpleFactory::create_product(ProductType::Laptop);
        assert_eq!(laptop.get_name(), "🏭 Factory Standard Laptop");
        assert_eq!(laptop.get_price(), 999.99);
        
        let phone = SimpleFactory::create_product(ProductType::Smartphone);
        assert_eq!(phone.get_name(), "🏭 Factory Standard Phone");
        assert_eq!(phone.get_price(), 699.99);
    }

    /// 🧪 ทดสอบ Factory Method Pattern - วิธีการผลิตแบบเฉพาะแบรนด์
    #[test]
    fn test_factory_method() {
        let apple_factory = AppleFactory;
        let product = apple_factory.create_product();
        
        assert_eq!(apple_factory.get_brand(), "🍎 Apple");
        assert_eq!(product.get_name(), "🍎 MacBook Pro");
        
        let samsung_factory = SamsungFactory;
        let samsung_product = samsung_factory.create_product();
        assert_eq!(samsung_factory.get_brand(), "🌟 Samsung");
        assert_eq!(samsung_product.get_name(), "🌟 Galaxy Book Pro");
    }

    /// 🧪 ทดสอบ Abstract Factory Pattern - โรงงานผลิตครบวงจร
    #[test]
    fn test_abstract_factory() {
        let premium_factory = PremiumFactory;
        
        let laptop = premium_factory.create_laptop();
        let phone = premium_factory.create_smartphone();
        let tablet = premium_factory.create_tablet();
        
        assert_eq!(premium_factory.get_brand(), "💎 Premium");
        assert!(laptop.get_price() > 2000.0);
        assert!(phone.get_price() > 1000.0);
        assert!(tablet.get_price() > 1000.0);
    }

    /// 🧪 ทดสอบ Factory Registry Pattern - ทะเบียนโรงงานผลิต
    #[test]
    fn test_factory_registry() {
        let registry = FactoryRegistry::new();
        
        assert!(registry.get_factory("premium").is_some());
        assert!(registry.get_factory("budget").is_some());
        assert!(registry.get_factory("gaming").is_some());
        assert!(registry.get_factory("nonexistent").is_none());
        
        // ทดสอบการสร้างชุดสินค้า
        let product_set = registry.create_product_set("premium");
        assert!(product_set.is_some());
        
        let factories = registry.list_factories();
        assert_eq!(factories.len(), 3);
    }

    /// 🧪 ทดสอบ Product Builder Pattern - เครื่องมือสร้างสินค้าแบบกำหนดเอง
    #[test]
    fn test_product_builder() {
        let laptop = ProductBuilder::new()
            .product_type(ProductType::Laptop)
            .name("🧪 Test Manufacturing Laptop".to_string())
            .price(1500.0)
            .spec("cpu".to_string(), "Intel i7 Test CPU".to_string())
            .spec("ram".to_string(), "16".to_string())
            .spec("storage".to_string(), "512GB SSD".to_string())
            .build();
        
        assert!(laptop.is_ok());
        let laptop = laptop.unwrap();
        assert_eq!(laptop.get_name(), "🧪 Test Manufacturing Laptop");
        assert_eq!(laptop.get_price(), 1500.0);
    }

    /// 🧪 ทดสอบ Product Builder ที่ขาดข้อมูลประเภทสินค้า
    #[test]
    fn test_product_builder_missing_type() {
        let result = ProductBuilder::new()
            .name("🧪 Test Product".to_string())
            .price(100.0)
            .build();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Product type not specified");
    }
    
    /// 🧪 ทดสอบการสร้างสินค้าแบบกำหนดเองจาก Simple Factory
    #[test]
    fn test_custom_laptop_creation() {
        let custom_laptop = SimpleFactory::create_custom_laptop(
            "🧪 Test Gaming Laptop".to_string(),
            "Intel i9".to_string(),
            32,
        );
        
        assert_eq!(custom_laptop.get_name(), "🔧 Custom 🧪 Test Gaming Laptop");
        assert_eq!(custom_laptop.get_price(), 1799.99); // 32GB RAM price
    }
    
    /// 🧪 ทดสอบการสร้างสินค้าเฉพาะแบรนด์
    #[test]
    fn test_brand_specific_products() {
        let apple_factory = AppleFactory;
        let iphone = apple_factory.create_iphone();
        let ipad = apple_factory.create_ipad();
        
        assert_eq!(iphone.get_name(), "🍎 iPhone 15 Pro");
        assert_eq!(ipad.get_name(), "🍎 iPad Pro");
        
        let samsung_factory = SamsungFactory;
        let galaxy_phone = samsung_factory.create_galaxy_phone();
        let galaxy_tab = samsung_factory.create_galaxy_tab();
        
        assert_eq!(galaxy_phone.get_name(), "🌟 Galaxy S24 Ultra");
        assert_eq!(galaxy_tab.get_name(), "🌟 Galaxy Tab S9");
    }
}