//! 🎨 Mobile UI Components
//! 
//! สาธิตการสร้าง UI components สำหรับแอปมือถือด้วย Rust
//! รวมถึง responsive design, theming, และ accessibility
//! เหมือนการเป็นนักออกแบบ UI/UX แต่เขียนโค้ด! 🎭✨

use std::collections::HashMap;
use std::fmt;

/// 📱 Screen Size Categories
/// เหมือนการแบ่งขนาดเสื้อผ้า แต่สำหรับหน้าจอ! 👕📏
#[derive(Debug, Clone, PartialEq)]
pub enum ScreenSize {
    Phone,      // < 600dp - จิ๋วแต่แจ๋ว! 📱
    Tablet,     // 600dp - 840dp - ขนาดกำลังดี! 📋
    Desktop,    // > 840dp - ใหญ่โตมาก! 🖥️
}

/// 🎨 Theme Mode
/// เหมือนการเปลี่ยนโหมดแสงในห้อง! 💡🌙
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeMode {
    Light,      // สว่างสดใส เหมือนตอนเช้า! ☀️
    Dark,       // มืดมิด เหมือนตอนกลางคืน! 🌙
    System,     // ตามระบบ เหมือนคนไม่มีความเห็น! 🤖
}

/// 🌈 Color Palette
/// ชุดสีสวยๆ เหมือนกล่องสีของจิตรกร! 🎨🖌️
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub accent: String,
    pub error: String,
    pub warning: String,
    pub success: String,
}

impl ColorPalette {
    /// สร้างธีมสว่าง - เหมือนแสงแดดยามเช้า! ☀️✨
    pub fn light_theme() -> Self {
        Self {
            primary: "#007AFF".to_string(),
            secondary: "#5856D6".to_string(),
            background: "#FFFFFF".to_string(),
            surface: "#F2F2F7".to_string(),
            text_primary: "#000000".to_string(),
            text_secondary: "#8E8E93".to_string(),
            accent: "#FF9500".to_string(),
            error: "#FF3B30".to_string(),
            warning: "#FFCC02".to_string(),
            success: "#34C759".to_string(),
        }
    }
    
    /// สร้างธีมมืด - เหมือนท้องฟ้ายามค่ำคืน! 🌙⭐
    pub fn dark_theme() -> Self {
        Self {
            primary: "#0A84FF".to_string(),
            secondary: "#5E5CE6".to_string(),
            background: "#000000".to_string(),
            surface: "#1C1C1E".to_string(),
            text_primary: "#FFFFFF".to_string(),
            text_secondary: "#8E8E93".to_string(),
            accent: "#FF9F0A".to_string(),
            error: "#FF453A".to_string(),
            warning: "#FFD60A".to_string(),
            success: "#30D158".to_string(),
        }
    }
}

/// 📏 Spacing System
/// ระบบการจัดระยะห่าง เหมือนการจัดเฟอร์นิเจอร์ในบ้าน! 🏠📐
#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: f32,    // 4dp - เล็กจิ๋ว เหมือนเม็ดข้าว! 🌾
    pub sm: f32,    // 8dp - เล็ก เหมือนเหรียญ! 🪙
    pub md: f32,    // 16dp - กลาง เหมือนนิ้วโป้ง! 👍
    pub lg: f32,    // 24dp - ใหญ่ เหมือนฝ่ามือ! ✋
    pub xl: f32,    // 32dp - ใหญ่มาก เหมือนหนังสือ! 📖
    pub xxl: f32,   // 48dp - ยักษ์ เหมือนแท็บเล็ต! 📱
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

/// 🔤 Typography System
/// ระบบตัวอักษร เหมือนการเลือกฟอนต์ในเวิร์ด! 📝✍️
#[derive(Debug, Clone)]
pub struct Typography {
    pub h1: TextStyle,
    pub h2: TextStyle,
    pub h3: TextStyle,
    pub body1: TextStyle,
    pub body2: TextStyle,
    pub caption: TextStyle,
    pub button: TextStyle,
}

/// สไตล์ตัวอักษร - แต่งหน้าให้ข้อความ! 💄📝
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,        // ขนาดตัวอักษร
    pub font_weight: FontWeight, // น้ำหนักตัวอักษร
    pub line_height: f32,      // ความสูงบรรทัด
    pub letter_spacing: f32,   // ระยะห่างตัวอักษร
}

/// น้ำหนักตัวอักษร - จากผอมจนถึงอ้วน! 🏋️‍♂️
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Light,      // เบา เหมือนขนนก! 🪶
    Regular,    // ปกติ เหมือนคนทั่วไป! 😊
    Medium,     // กลาง เหมือนนักกีฬา! 🏃‍♂️
    SemiBold,   // ค่อนข้างหนา เหมือนนักมวย! 🥊
    Bold,       // หนา เหมือนนักเพาะกาย! 💪
}

impl Typography {
    /// สร้างระบบตัวอักษรเริ่มต้น - ชุดฟอนต์สำเร็จรูป! 📦✨
    pub fn default() -> Self {
        Self {
            h1: TextStyle {
                font_size: 32.0,
                font_weight: FontWeight::Bold,
                line_height: 40.0,
                letter_spacing: -0.5,
            },
            h2: TextStyle {
                font_size: 24.0,
                font_weight: FontWeight::SemiBold,
                line_height: 32.0,
                letter_spacing: -0.25,
            },
            h3: TextStyle {
                font_size: 20.0,
                font_weight: FontWeight::Medium,
                line_height: 28.0,
                letter_spacing: 0.0,
            },
            body1: TextStyle {
                font_size: 16.0,
                font_weight: FontWeight::Regular,
                line_height: 24.0,
                letter_spacing: 0.15,
            },
            body2: TextStyle {
                font_size: 14.0,
                font_weight: FontWeight::Regular,
                line_height: 20.0,
                letter_spacing: 0.25,
            },
            caption: TextStyle {
                font_size: 12.0,
                font_weight: FontWeight::Regular,
                line_height: 16.0,
                letter_spacing: 0.4,
            },
            button: TextStyle {
                font_size: 16.0,
                font_weight: FontWeight::Medium,
                line_height: 24.0,
                letter_spacing: 0.5,
            },
        }
    }
}

/// 🎨 Design System
/// ระบบออกแบบครบครัน เหมือนกล่องเครื่องมือของนักออกแบบ! 🧰✨
#[derive(Debug, Clone)]
pub struct DesignSystem {
    pub colors: ColorPalette,
    pub spacing: Spacing,
    pub typography: Typography,
    pub theme_mode: ThemeMode,
    pub screen_size: ScreenSize,
}

impl DesignSystem {
    /// สร้างระบบออกแบบใหม่ - เหมือนการตั้งค่าสตูดิโอ! 🎨🏗️
    pub fn new(theme_mode: ThemeMode, screen_size: ScreenSize) -> Self {
        let colors = match theme_mode {
            ThemeMode::Light => ColorPalette::light_theme(),
            ThemeMode::Dark => ColorPalette::dark_theme(),
            ThemeMode::System => ColorPalette::light_theme(), // Default to light - เลือกสว่างเป็นค่าเริ่มต้น! ☀️
        };
        
        Self {
            colors,
            spacing: Spacing::default(),
            typography: Typography::default(),
            theme_mode,
            screen_size,
        }
    }
    
    /// คำนวณ padding ตามขนาดหน้าจอ - เหมือนการปรับขนาดเสื้อผ้า! 👕📏
    pub fn get_responsive_padding(&self) -> f32 {
        match self.screen_size {
            ScreenSize::Phone => self.spacing.md,      // มือถือ - พื้นที่จำกัด! 📱
            ScreenSize::Tablet => self.spacing.lg,     // แท็บเล็ต - กำลังดี! 📋
            ScreenSize::Desktop => self.spacing.xl,    // เดสก์ท็อป - เยอะเลย! 🖥️
        }
    }
    
    /// คำนวณขนาดฟอนต์ตามหน้าจอ - เหมือนการซูมข้อความ! 🔍📝
    pub fn get_responsive_font_scale(&self) -> f32 {
        match self.screen_size {
            ScreenSize::Phone => 1.0,       // มือถือ - ขนาดปกติ! 📱
            ScreenSize::Tablet => 1.1,      // แท็บเล็ต - ใหญ่ขึ้นนิดหน่อย! 📋
            ScreenSize::Desktop => 1.2,     // เดสก์ท็อป - ใหญ่สบายตา! 🖥️
        }
    }
}

/// 🧩 UI Component Trait
/// สัญญาที่ทุก UI Component ต้องทำตาม เหมือนกฎของเกม! 📋✅
pub trait UIComponent {
    fn render(&self, design_system: &DesignSystem) -> String;
    fn get_accessibility_label(&self) -> Option<String>;
    fn handle_interaction(&mut self, interaction: Interaction);
    fn get_component_type(&self) -> ComponentType;
}

/// 🎯 Component Types
/// ประเภทของ Component ต่างๆ เหมือนเมนูในร้านอาหาร! 🍽️📋
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Button,         // ปุ่มกด - คลิกเพื่อทำอะไรสักอย่าง! 🔘
    TextField,      // ช่องใส่ข้อความ - พิมพ์ได้เลย! ⌨️
    Label,          // ป้ายข้อความ - อ่านอย่างเดียว! 🏷️
    Image,          // รูปภาพ - สวยงาม! 🖼️
    Card,           // การ์ด - ใส่ข้อมูลเป็นกลุ่ม! 🃏
    List,           // รายการ - เรียงกันเป็นแถว! 📝
    Navigation,     // การนำทาง - ไปไหนมาไหน! 🧭
    Modal,          // หน้าต่างป๊อปอัป - โผล่มาแปป! 🪟
    Tab,            // แท็บ - เปลี่ยนหน้าได้! 📑
    Switch,         // สวิตช์ - เปิด/ปิด! 🔄
}

/// 👆 User Interactions
/// การโต้ตอบของผู้ใช้ เหมือนภาษากายดิจิทัล! 🤏📱
#[derive(Debug, Clone)]
pub enum Interaction {
    Tap { x: f32, y: f32 },                                    // แตะ - ทัชเบาๆ! 👆
    LongPress { x: f32, y: f32 },                              // กดค้าง - อดทนหน่อย! ⏰
    Swipe { direction: SwipeDirection, velocity: f32 },        // ปัด - เหมือนเปิดหนังสือ! 📖
    Pinch { scale: f32 },                                      // หยิก - ซูมเข้าออก! 🤏
    TextInput { text: String },                                // พิมพ์ข้อความ - แป้นพิมพ์ทำงาน! ⌨️
    Focus,                                                     // โฟกัส - จับจ้อง! 👀
    Blur,                                                      // เบลอ - มองไม่ชัด! 😵‍💫
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwipeDirection {
    Up,         // ขึ้น - เหมือนบินขึ้นฟ้า! ⬆️
    Down,       // ลง - เหมือนตกลงมา! ⬇️
    Left,       // ซ้าย - ไปทางซ้าย! ⬅️
    Right,      // ขวา - ไปทางขวา! ➡️
}

/// 🔘 Button Component
/// ปุ่มกดที่ทำงานได้หลากหลาย เหมือนรีโมทคอนโทรล! 📺🎮
#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,                           // ข้อความบนปุ่ม
    pub variant: ButtonVariant,                 // รูปแบบปุ่ม
    pub size: ButtonSize,                       // ขนาดปุ่ม
    pub enabled: bool,                          // เปิด/ปิดการใช้งาน
    pub loading: bool,                          // สถานะกำลังโหลด
    pub icon: Option<String>,                   // ไอคอนประกอบ
    pub accessibility_label: Option<String>,    // ป้ายสำหรับคนพิการ
    pub on_tap: Option<String>,                 // Action identifier - ทำอะไรเมื่อกด!
}

/// รูปแบบปุ่มต่างๆ - เหมือนชุดเดรสโค้ด! 👔👗
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,        // หลัก - ดูสำคัญที่สุด! ⭐
    Secondary,      // รอง - สำคัญรองลงมา! 🥈
    Outlined,       // มีขอบ - เรียบร้อยดี! 📦
    Text,           // ข้อความอย่างเดียว - เรียบง่าย! 📝
    Destructive,    // ทำลาย - ระวังก่อนกด! ⚠️
}

/// ขนาดปุ่ม - เหมือนขนาดเสื้อผ้า! 👕
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,      // เล็ก - จิ๋วแต่แจ๋ว! 🤏
    Medium,     // กลาง - ขนาดมาตรฐาน! 👌
    Large,      // ใหญ่ - เห็นชัดเจน! 👍
}

impl Button {
    /// สร้างปุ่มใหม่ - เหมือนการสั่งทำปุ่มที่ร้านช่าง! 🔨✨
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            enabled: true,
            loading: false,
            icon: None,
            accessibility_label: None,
            on_tap: None,
        }
    }
    
    /// เปลี่ยนรูปแบบปุ่ม - เหมือนเปลี่ยนชุด! 👔
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    /// เปลี่ยนขนาดปุ่ม - ใหญ่หรือเล็กตามใจ! 📏
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    /// เพิ่มไอคอน - ตกแต่งให้สวยงาม! 🎨
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
    
    /// ปิดการใช้งาน - ห้ามกด! 🚫
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    
    /// ตั้งสถานะโหลด - รอสักครู่นะ! ⏳
    pub fn loading(mut self) -> Self {
        self.loading = true;
        self
    }
    
    /// เลือกสีปุ่มตามรูปแบบ - เหมือนการเลือกสีเสื้อผ้า! 🎨👔
    fn get_button_colors(&self, design_system: &DesignSystem) -> (String, String) {
        if !self.enabled {
            return ("#CCCCCC".to_string(), "#666666".to_string()); // ปิดใช้งาน - สีเทาๆ! 😴
        }
        
        match self.variant {
            ButtonVariant::Primary => (
                design_system.colors.primary.clone(),
                design_system.colors.background.clone(),
            ),
            ButtonVariant::Secondary => (
                design_system.colors.secondary.clone(),
                design_system.colors.background.clone(),
            ),
            ButtonVariant::Outlined => (
                "transparent".to_string(),
                design_system.colors.primary.clone(),
            ),
            ButtonVariant::Text => (
                "transparent".to_string(),
                design_system.colors.primary.clone(),
            ),
            ButtonVariant::Destructive => (
                design_system.colors.error.clone(),
                design_system.colors.background.clone(),
            ),
        }
    }
    
    /// คำนวณ padding ของปุ่ม - เหมือนการใส่เบาะรองนั่ง! 🪑💺
    fn get_button_padding(&self, design_system: &DesignSystem) -> (f32, f32) {
        let base_padding = design_system.get_responsive_padding();
        
        match self.size {
            ButtonSize::Small => (base_padding * 0.5, base_padding * 0.75),    // เล็ก - พื้นที่น้อย! 🤏
            ButtonSize::Medium => (base_padding * 0.75, base_padding),         // กลาง - พอดี! 👌
            ButtonSize::Large => (base_padding, base_padding * 1.25),          // ใหญ่ - สบาย! 👍
        }
    }
}

impl UIComponent for Button {
    fn render(&self, design_system: &DesignSystem) -> String {
        let (bg_color, text_color) = self.get_button_colors(design_system);
        let (h_padding, v_padding) = self.get_button_padding(design_system);
        let font_scale = design_system.get_responsive_font_scale();
        let font_size = design_system.typography.button.font_size * font_scale;
        
        let content = if self.loading {
            "Loading...".to_string()
        } else if let Some(ref icon) = self.icon {
            format!("{} {}", icon, self.text)
        } else {
            self.text.clone()
        };
        
        format!(
            "Button {{
  text: \"{}\",
  background: \"{}\",
  color: \"{}\",
  padding: {}px {}px,
  font-size: {}px,
  enabled: {},
  variant: {:?},
  size: {:?}
}}",
            content, bg_color, text_color, v_padding, h_padding, font_size, self.enabled, self.variant, self.size
        )
    }
    
    fn get_accessibility_label(&self) -> Option<String> {
        self.accessibility_label.clone().or_else(|| {
            if self.loading {
                Some(format!("{}, loading", self.text))
            } else if !self.enabled {
                Some(format!("{}, disabled", self.text))
            } else {
                Some(format!("{}, button", self.text))
            }
        })
    }
    
    fn handle_interaction(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::Tap { .. } => {
                if self.enabled && !self.loading {
                    println!("Button '{}' tapped!", self.text);
                    if let Some(ref action) = self.on_tap {
                        println!("Executing action: {}", action);
                    }
                }
            }
            _ => {}
        }
    }
    
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Button
    }
}

/// 📝 TextField Component
/// ช่องใส่ข้อความที่ฉลาดและใช้งานง่าย เหมือนกล่องรับจดหมาย! 📮✍️
#[derive(Debug, Clone)]
pub struct TextField {
    pub placeholder: String,                    // ข้อความแนะนำ
    pub value: String,                          // ค่าที่ใส่ไว้
    pub label: Option<String>,                  // ป้ายชื่อ
    pub error: Option<String>,                  // ข้อความแสดงข้อผิดพลาด
    pub input_type: InputType,                  // ประเภทการใส่ข้อมูล
    pub enabled: bool,                          // เปิด/ปิดการใช้งาน
    pub required: bool,                         // บังคับใส่หรือไม่
    pub max_length: Option<usize>,              // ความยาวสูงสุด
    pub accessibility_label: Option<String>,    // ป้ายสำหรับคนพิการ
}

/// ประเภทการใส่ข้อมูล - เหมือนการเลือกประเภทกุญแจ! 🗝️🔐
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text,           // ข้อความทั่วไป - พิมพ์อะไรก็ได้! 📝
    Email,          // อีเมล - ต้องมี @ นะ! 📧
    Password,       // รหัสผ่าน - ความลับ! 🔒
    Number,         // ตัวเลข - เฉพาะเลขเท่านั้น! 🔢
    Phone,          // เบอร์โทร - โทรหาได้! 📞
    Url,            // ลิงก์ - ไปเว็บไซต์! 🌐
    Multiline,      // หลายบรรทัด - เขียนยาวๆ ได้! 📄
}

impl TextField {
    /// สร้าง TextField ใหม่ - เหมือนการเตรียมกระดาษเปล่า! 📄✏️
    pub fn new(placeholder: &str) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            value: String::new(),
            label: None,
            error: None,
            input_type: InputType::Text,
            enabled: true,
            required: false,
            max_length: None,
            accessibility_label: None,
        }
    }
    
    /// เพิ่มป้ายชื่อ - ติดป้ายบอกว่าใส่อะไร! 🏷️
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    
    /// กำหนดประเภทการใส่ข้อมูล - เลือกโหมดพิเศษ! 🎯
    pub fn with_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }
    
    /// ทำให้เป็นช่องบังคับ - ต้องใส่นะ! ⚠️
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// กำหนดความยาวสูงสุด - อย่าพิมพ์ยาวเกินไป! 📏
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
    
    /// ตั้งข้อความแสดงข้อผิดพลาด - บอกว่าผิดตรงไหน! ❌
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
    
    /// ตรวจสอบความถูกต้อง - เหมือนครูตรวจการบ้าน! 👩‍🏫✅
    pub fn validate(&self) -> Result<(), String> {
        if self.required && self.value.trim().is_empty() {
            return Err("This field is required".to_string());
        }
        
        if let Some(max_len) = self.max_length {
            if self.value.len() > max_len {
                return Err(format!("Maximum {} characters allowed", max_len));
            }
        }
        
        match self.input_type {
            InputType::Email => {
                if !self.value.contains('@') || !self.value.contains('.') {
                    return Err("Please enter a valid email address".to_string());
                }
            }
            InputType::Phone => {
                if self.value.chars().filter(|c| c.is_ascii_digit()).count() < 10 {
                    return Err("Please enter a valid phone number".to_string());
                }
            }
            InputType::Url => {
                if !self.value.starts_with("http://") && !self.value.starts_with("https://") {
                    return Err("Please enter a valid URL".to_string());
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

impl UIComponent for TextField {
    fn render(&self, design_system: &DesignSystem) -> String {
        let border_color = if self.error.is_some() {
            &design_system.colors.error
        } else {
            &design_system.colors.text_secondary
        };
        
        let padding = design_system.get_responsive_padding();
        let font_scale = design_system.get_responsive_font_scale();
        let font_size = design_system.typography.body1.font_size * font_scale;
        
        let mut output = String::new();
        
        if let Some(ref label) = self.label {
            output.push_str(&format!(
                "Label {{
  text: \"{}{}\",
  color: \"{}\",
  font-size: {}px
}}\n",
                label,
                if self.required { " *" } else { "" },
                design_system.colors.text_primary,
                font_size * 0.875
            ));
        }
        
        output.push_str(&format!(
            "TextField {{
  placeholder: \"{}\",
  value: \"{}\",
  type: {:?},
  border-color: \"{}\",
  background: \"{}\",
  color: \"{}\",
  padding: {}px,
  font-size: {}px,
  enabled: {}
}}",
            self.placeholder,
            self.value,
            self.input_type,
            border_color,
            design_system.colors.surface,
            design_system.colors.text_primary,
            padding,
            font_size,
            self.enabled
        ));
        
        if let Some(ref error) = self.error {
            output.push_str(&format!(
                "\nErrorText {{
  text: \"{}\",
  color: \"{}\",
  font-size: {}px
}}",
                error,
                design_system.colors.error,
                font_size * 0.75
            ));
        }
        
        output
    }
    
    fn get_accessibility_label(&self) -> Option<String> {
        self.accessibility_label.clone().or_else(|| {
            let mut label = self.label.clone().unwrap_or_else(|| self.placeholder.clone());
            if self.required {
                label.push_str(", required");
            }
            if let Some(ref error) = self.error {
                label.push_str(&format!(", error: {}", error));
            }
            Some(label)
        })
    }
    
    fn handle_interaction(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::TextInput { text } => {
                if self.enabled {
                    if let Some(max_len) = self.max_length {
                        if text.len() <= max_len {
                            self.value = text;
                        }
                    } else {
                        self.value = text;
                    }
                    
                    // Clear error on input
                    if self.error.is_some() {
                        self.error = None;
                    }
                }
            }
            Interaction::Blur => {
                if let Err(error) = self.validate() {
                    self.error = Some(error);
                }
            }
            _ => {}
        }
    }
    
    fn get_component_type(&self) -> ComponentType {
        ComponentType::TextField
    }
}

/// 🏷️ Label Component
/// ป้ายข้อความที่สวยงาม เหมือนป้ายบอกทาง! 🪧✨
#[derive(Debug, Clone)]
pub struct Label {
    pub text: String,                       // ข้อความที่จะแสดง
    pub style: LabelStyle,                  // รูปแบบตัวอักษร
    pub color: Option<String>,              // สีของข้อความ
    pub alignment: TextAlignment,           // การจัดตำแหน่ง
    pub max_lines: Option<usize>,           // จำนวนบรรทัดสูงสุด
}

/// รูปแบบตัวอักษร - เลือกขนาดให้เหมาะสม! 📏✍️
#[derive(Debug, Clone, PartialEq)]
pub enum LabelStyle {
    H1,         // หัวข้อใหญ่ - โดดเด่นสุด! 🎯
    H2,         // หัวข้อกลาง - สำคัญรอง! 📢
    H3,         // หัวข้อเล็ก - ย่อยๆ! 📝
    Body1,      // เนื้อหาหลัก - อ่านง่าย! 📖
    Body2,      // เนื้อหารอง - เล็กกว่า! 📄
    Caption,    // คำอธิบาย - เล็กสุด! 🔍
}

/// การจัดตำแหน่งข้อความ - เรียงให้สวย! 📐
#[derive(Debug, Clone, PartialEq)]
pub enum TextAlignment {
    Left,       // ชิดซ้าย - แบบปกติ! ⬅️
    Center,     // กึ่งกลาง - สมดุล! ⚖️
    Right,      // ชิดขวา - ย้อนกลับ! ➡️
    Justify,    // เต็มบรรทัด - เรียบร้อย! 📏
}

impl Label {
    /// สร้าง Label ใหม่ - เหมือนการเขียนป้าย! 🏷️✏️
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: LabelStyle::Body1,
            color: None,
            alignment: TextAlignment::Left,
            max_lines: None,
        }
    }
    
    /// เปลี่ยนรูปแบบตัวอักษร - แต่งให้สวย! 💅
    pub fn with_style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }
    
    /// เปลี่ยนสีข้อความ - ทาสีให้สดใส! 🎨
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }
    
    /// จัดตำแหน่งข้อความ - เรียงให้เป็นระเบียบ! 📐
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }
    
    /// กำหนดจำนวนบรรทัดสูงสุด - อย่าให้ยาวเกินไป! 📏
    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = Some(max_lines);
        self
    }
    
    /// เลือกรูปแบบตัวอักษรจาก Design System - หยิบมาใช้! 🎯
    fn get_text_style<'a>(&self, design_system: &'a DesignSystem) -> &'a TextStyle {
        match self.style {
            LabelStyle::H1 => &design_system.typography.h1,
            LabelStyle::H2 => &design_system.typography.h2,
            LabelStyle::H3 => &design_system.typography.h3,
            LabelStyle::Body1 => &design_system.typography.body1,
            LabelStyle::Body2 => &design_system.typography.body2,
            LabelStyle::Caption => &design_system.typography.caption,
        }
    }
}

impl UIComponent for Label {
    fn render(&self, design_system: &DesignSystem) -> String {
        let text_style = self.get_text_style(design_system);
        let font_scale = design_system.get_responsive_font_scale();
        let font_size = text_style.font_size * font_scale;
        
        let color = self.color.as_ref().unwrap_or(&design_system.colors.text_primary);
        
        format!(
            "Label {{
  text: \"{}\",
  style: {:?},
  color: \"{}\",
  font-size: {}px,
  font-weight: {:?},
  line-height: {}px,
  alignment: {:?},
  max-lines: {:?}
}}",
            self.text,
            self.style,
            color,
            font_size,
            text_style.font_weight,
            text_style.line_height * font_scale,
            self.alignment,
            self.max_lines
        )
    }
    
    fn get_accessibility_label(&self) -> Option<String> {
        Some(self.text.clone())
    }
    
    fn handle_interaction(&mut self, _interaction: Interaction) {
        // Labels typically don't handle interactions
    }
    
    fn get_component_type(&self) -> ComponentType {
        ComponentType::Label
    }
}

/// 🃏 Card Component
/// การ์ดสวยๆ ที่ใส่อะไรก็ได้ เหมือนกล่องของขวัญ! 🎁📦
pub struct Card {
    pub children: Vec<Box<dyn UIComponent>>,    // ลูกๆ ข้างใน
    pub elevation: f32,                        // ความสูงเงา
    pub padding: Option<f32>,                  // ระยะห่างด้านใน
    pub margin: Option<f32>,                   // ระยะห่างด้านนอก
    pub background_color: Option<String>,      // สีพื้นหลัง
    pub border_radius: f32,                    // ความโค้งมุม
    pub clickable: bool,                       // กดได้หรือไม่
}

impl Card {
    /// สร้างการ์ดใหม่ - เหมือนการพับกล่อง! 📦✨
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            elevation: 2.0,
            padding: None,
            margin: None,
            background_color: None,
            border_radius: 8.0,
            clickable: false,
        }
    }
    
    /// เพิ่มเงา - ทำให้ลอยขึ้น! 🌤️
    pub fn with_elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }
    
    /// เพิ่มระยะห่างด้านใน - ให้มีพื้นที่หายใจ! 🫁
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }
    
    /// ทำให้กดได้ - เปิดโหมดโต้ตอบ! 👆
    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }
}

// Note: Card implementation would need special handling for children
// This is a simplified version for demonstration

/// 📱 Mobile Screen
/// หน้าจอมือถือที่รวมทุกอย่างเข้าด้วยกัน เหมือนผืนผ้าใบของศิลปิน! 🎨📱
pub struct MobileScreen {
    pub title: String,                             // ชื่อหน้าจอ
    pub components: Vec<Box<dyn UIComponent>>,     // รายการ components
    pub design_system: DesignSystem,               // ระบบดีไซน์
    pub safe_area_insets: SafeAreaInsets,          // พื้นที่ปลอดภัย
}

/// พื้นที่ปลอดภัยของหน้าจอ - หลีกเลี่ยงรอยบาก! 📱🛡️
#[derive(Debug, Clone)]
pub struct SafeAreaInsets {
    pub top: f32,       // ด้านบน - หลีกเลี่ยงรอยบาก
    pub bottom: f32,    // ด้านล่าง - หลีกเลี่ยงปุ่มโฮม
    pub left: f32,      // ด้านซ้าย
    pub right: f32,     // ด้านขวา
}

impl SafeAreaInsets {
    /// ค่าเริ่มต้นสำหรับ iPhone - ปลอดภัยแน่นอน! 🛡️
    pub fn default() -> Self {
        Self {
            top: 44.0,    // Status bar + notch
            bottom: 34.0, // Home indicator
            left: 0.0,
            right: 0.0,
        }
    }
}

impl MobileScreen {
    /// สร้างหน้าจอใหม่ - เหมือนการเตรียมผืนผ้าใบ! 🎨
    pub fn new(title: &str, design_system: DesignSystem) -> Self {
        Self {
            title: title.to_string(),
            components: Vec::new(),
            design_system,
            safe_area_insets: SafeAreaInsets::default(),
        }
    }
    
    /// เพิ่ม component - วางชิ้นส่วนลงบนหน้าจอ! 🧩
    pub fn add_component(&mut self, component: Box<dyn UIComponent>) {
        self.components.push(component);
    }
    
    /// เรนเดอร์หน้าจอทั้งหมด - วาดภาพออกมา! 🖼️
    pub fn render(&self) -> String {
        let mut output = format!(
            "=== {} ===\nTheme: {:?} | Screen: {:?}\n\n",
            self.title, self.design_system.theme_mode, self.design_system.screen_size
        );
        
        for (i, component) in self.components.iter().enumerate() {
            output.push_str(&format!("Component {}:\n{}", i + 1, component.render(&self.design_system)));
            output.push_str("\n\n");
        }
        
        output
    }
    
    /// จัดการการโต้ตอบ - รับมือกับการกระทำของผู้ใช้! 👆
    pub fn handle_interaction(&mut self, component_index: usize, interaction: Interaction) {
        if let Some(component) = self.components.get_mut(component_index) {
            component.handle_interaction(interaction);
        }
    }
}

/// 🎨 สาธิตการใช้งาน Mobile UI Components
/// ฟังก์ชันสาธิตที่แสดงความสามารถทั้งหมด เหมือนการแสดงแฟชั่นโชว์! 👗✨
pub fn demonstrate_mobile_ui_components() {
    println!("🎨 === Mobile UI Components Demo ===");
    
    // สร้าง Design Systems สำหรับ themes และ screen sizes ต่างๆ
    let light_phone = DesignSystem::new(ThemeMode::Light, ScreenSize::Phone);
    let dark_tablet = DesignSystem::new(ThemeMode::Dark, ScreenSize::Tablet);
    
    println!("\n📱 Creating Mobile Screens:");
    
    // สร้าง Login Screen
    let mut login_screen = MobileScreen::new("Login Screen", light_phone);
    
    // เพิ่ม components
    let title = Label::new("Welcome Back")
        .with_style(LabelStyle::H1)
        .with_alignment(TextAlignment::Center);
    
    let email_field = TextField::new("Enter your email")
        .with_label("Email")
        .with_type(InputType::Email)
        .required();
    
    let password_field = TextField::new("Enter your password")
        .with_label("Password")
        .with_type(InputType::Password)
        .required();
    
    let login_button = Button::new("Sign In")
        .with_variant(ButtonVariant::Primary)
        .with_size(ButtonSize::Large);
    
    let forgot_button = Button::new("Forgot Password?")
        .with_variant(ButtonVariant::Text)
        .with_size(ButtonSize::Medium);
    
    login_screen.add_component(Box::new(title));
    login_screen.add_component(Box::new(email_field));
    login_screen.add_component(Box::new(password_field));
    login_screen.add_component(Box::new(login_button));
    login_screen.add_component(Box::new(forgot_button));
    
    println!("\n{}", login_screen.render());
    
    // สร้าง Settings Screen สำหรับ Dark Theme
    let mut settings_screen = MobileScreen::new("Settings Screen", dark_tablet);
    
    let settings_title = Label::new("Settings")
        .with_style(LabelStyle::H2);
    
    let profile_button = Button::new("Edit Profile")
        .with_variant(ButtonVariant::Outlined)
        .with_icon("👤");
    
    let notifications_button = Button::new("Notifications")
        .with_variant(ButtonVariant::Outlined)
        .with_icon("🔔");
    
    let privacy_button = Button::new("Privacy & Security")
        .with_variant(ButtonVariant::Outlined)
        .with_icon("🔒");
    
    let logout_button = Button::new("Sign Out")
        .with_variant(ButtonVariant::Destructive)
        .with_icon("🚪");
    
    settings_screen.add_component(Box::new(settings_title));
    settings_screen.add_component(Box::new(profile_button));
    settings_screen.add_component(Box::new(notifications_button));
    settings_screen.add_component(Box::new(privacy_button));
    settings_screen.add_component(Box::new(logout_button));
    
    println!("{}", settings_screen.render());
    
    // สาธิต Component Interactions
    println!("\n👆 Testing Component Interactions:");
    
    let mut interactive_button = Button::new("Interactive Button")
        .with_variant(ButtonVariant::Primary);
    
    let mut interactive_field = TextField::new("Type something...")
        .with_label("Interactive Field")
        .required();
    
    // Test button interaction
    interactive_button.handle_interaction(Interaction::Tap { x: 100.0, y: 50.0 });
    
    // Test text field interactions
    interactive_field.handle_interaction(Interaction::TextInput { 
        text: "Hello World".to_string() 
    });
    interactive_field.handle_interaction(Interaction::Blur);
    
    // Test validation
    interactive_field.handle_interaction(Interaction::TextInput { 
        text: "".to_string() 
    });
    interactive_field.handle_interaction(Interaction::Blur);
    
    println!("   Button after tap: {}", interactive_button.get_accessibility_label().unwrap_or_default());
    println!("   TextField after validation: {}", interactive_field.get_accessibility_label().unwrap_or_default());
    
    // สาธิต Responsive Design
    println!("\n📐 Responsive Design Comparison:");
    show_responsive_comparison();
    
    // สาธิต Accessibility Features
    println!("\n♿ Accessibility Features:");
    show_accessibility_features();
    
    // สาธิต Theme Switching
    println!("\n🌓 Theme Switching:");
    show_theme_switching();
    
    // Best Practices
    println!("\n💡 Mobile UI Best Practices:");
    show_mobile_ui_best_practices();
}

/// 📐 Responsive Design Comparison
/// เปรียบเทียบการแสดงผลในหน้าจอขนาดต่างๆ - ดูให้เข้าใจ! 📱📟🖥️
fn show_responsive_comparison() {
    let phone = DesignSystem::new(ThemeMode::Light, ScreenSize::Phone);
    let tablet = DesignSystem::new(ThemeMode::Light, ScreenSize::Tablet);
    let desktop = DesignSystem::new(ThemeMode::Light, ScreenSize::Desktop);
    
    let button = Button::new("Sample Button").with_size(ButtonSize::Medium);
    
    println!("   📱 Phone:");
    println!("      Padding: {}px", phone.get_responsive_padding());
    println!("      Font Scale: {}x", phone.get_responsive_font_scale());
    println!("      {}", button.render(&phone));
    
    println!("\n   📱 Tablet:");
    println!("      Padding: {}px", tablet.get_responsive_padding());
    println!("      Font Scale: {}x", tablet.get_responsive_font_scale());
    
    println!("\n   🖥️ Desktop:");
    println!("      Padding: {}px", desktop.get_responsive_padding());
    println!("      Font Scale: {}x", desktop.get_responsive_font_scale());
}

/// ♿ Accessibility Features
/// คุณสมบัติเพื่อการเข้าถึง - ใครๆ ก็ใช้ได้! 🤝💙
fn show_accessibility_features() {
    let features = vec![
        "🔤 Semantic labels for screen readers",
        "🎯 Minimum touch target size (44x44 points)",
        "🌈 High contrast color ratios (4.5:1 minimum)",
        "📝 Alternative text for images",
        "⌨️ Keyboard navigation support",
        "🔊 Voice control compatibility",
        "📏 Dynamic type size support",
        "🎨 Reduced motion preferences",
        "🔍 Focus indicators for interactive elements",
        "📱 Platform-specific accessibility APIs",
    ];
    
    for feature in features {
        println!("   {}", feature);
    }
    
    println!("\n   📋 Accessibility Testing:");
    println!("      • Test with screen readers (VoiceOver, TalkBack)");
    println!("      • Verify color contrast ratios");
    println!("      • Test keyboard navigation");
    println!("      • Check touch target sizes");
    println!("      • Validate semantic markup");
}

/// 🌓 Theme Switching
/// การเปลี่ยนธีม - สลับโหมดกลางวันกลางคืน! ☀️🌙
fn show_theme_switching() {
    let light_theme = ColorPalette::light_theme();
    let dark_theme = ColorPalette::dark_theme();
    
    println!("   ☀️ Light Theme:");
    println!("      Primary: {}", light_theme.primary);
    println!("      Background: {}", light_theme.background);
    println!("      Text: {}", light_theme.text_primary);
    
    println!("\n   🌙 Dark Theme:");
    println!("      Primary: {}", dark_theme.primary);
    println!("      Background: {}", dark_theme.background);
    println!("      Text: {}", dark_theme.text_primary);
    
    println!("\n   🔄 Theme Switching Implementation:");
    println!("      • Detect system theme preference");
    println!("      • Provide manual theme toggle");
    println!("      • Persist user preference");
    println!("      • Animate theme transitions");
    println!("      • Update status bar appearance");
}

/// 💡 Mobile UI Best Practices
/// แนวทางปฏิบัติที่ดีที่สุด - เคล็ดลับจากผู้เชี่ยวชาญ! 🎯📚
fn show_mobile_ui_best_practices() {
    let practices = vec![
        "👆 Design for touch - minimum 44pt touch targets",
        "📱 Follow platform design guidelines (HIG, Material)",
        "🎨 Use consistent visual hierarchy",
        "⚡ Optimize for performance - 60fps animations",
        "🔋 Consider battery impact of UI choices",
        "📐 Design responsive layouts for multiple screen sizes",
        "♿ Ensure accessibility from the start",
        "🌐 Support internationalization and RTL languages",
        "🎯 Provide clear visual feedback for interactions",
        "📊 Use progressive disclosure to reduce cognitive load",
        "🔄 Implement proper loading and error states",
        "💾 Handle offline scenarios gracefully",
        "🎨 Maintain visual consistency across the app",
        "📱 Test on real devices with various screen sizes",
        "🔍 Optimize for one-handed usage",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   🛠️ Recommended Tools:");
    println!("      • Figma/Sketch for design");
    println!("      • Zeplin for design handoff");
    println!("      • Accessibility Inspector");
    println!("      • Color contrast analyzers");
    println!("      • Device simulators and real devices");
    println!("      • Performance profiling tools");
}

/// 🧪 Tests - ทดสอบให้แน่ใจว่าทุกอย่างทำงานได้! ✅🔬
#[cfg(test)]
mod tests {
    use super::*;
    
    /// ทดสอบ Design System - ตรวจสอบระบบดีไซน์
    #[test]
    fn test_design_system() {
        let design_system = DesignSystem::new(ThemeMode::Light, ScreenSize::Phone);
        assert_eq!(design_system.theme_mode, ThemeMode::Light);
        assert_eq!(design_system.screen_size, ScreenSize::Phone);
        assert_eq!(design_system.get_responsive_padding(), 16.0);
        assert_eq!(design_system.get_responsive_font_scale(), 1.0);
    }
    
    /// ทดสอบ Button Component - ตรวจสอบปุ่ม
    #[test]
    fn test_button_component() {
        let design_system = DesignSystem::new(ThemeMode::Light, ScreenSize::Phone);
        let button = Button::new("Test Button")
            .with_variant(ButtonVariant::Primary)
            .with_size(ButtonSize::Large);
        
        assert_eq!(button.text, "Test Button");
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert_eq!(button.size, ButtonSize::Large);
        assert!(button.enabled);
        
        let rendered = button.render(&design_system);
        assert!(rendered.contains("Test Button"));
        assert!(rendered.contains("Primary"));
    }
    
    /// ทดสอบการตรวจสอบ TextField - ตรวจสอบช่องใส่ข้อมูล
    #[test]
    fn test_text_field_validation() {
        let mut field = TextField::new("Email")
            .with_type(InputType::Email)
            .required();
        
        // Test empty required field
        assert!(field.validate().is_err());
        
        // Test invalid email
        field.value = "invalid-email".to_string();
        assert!(field.validate().is_err());
        
        // Test valid email
        field.value = "test@example.com".to_string();
        assert!(field.validate().is_ok());
    }
    
    /// ทดสอบ Label Component - ตรวจสอบป้ายข้อความ
    #[test]
    fn test_label_component() {
        let design_system = DesignSystem::new(ThemeMode::Dark, ScreenSize::Tablet);
        let label = Label::new("Test Label")
            .with_style(LabelStyle::H1)
            .with_alignment(TextAlignment::Center);
        
        assert_eq!(label.text, "Test Label");
        assert_eq!(label.style, LabelStyle::H1);
        assert_eq!(label.alignment, TextAlignment::Center);
        
        let rendered = label.render(&design_system);
        assert!(rendered.contains("Test Label"));
        assert!(rendered.contains("H1"));
        assert!(rendered.contains("Center"));
    }
    
    /// ทดสอบ Mobile Screen - ตรวจสอบหน้าจอมือถือ
    #[test]
    fn test_mobile_screen() {
        let design_system = DesignSystem::new(ThemeMode::Light, ScreenSize::Phone);
        let mut screen = MobileScreen::new("Test Screen", design_system);
        
        let button = Button::new("Test");
        screen.add_component(Box::new(button));
        
        assert_eq!(screen.title, "Test Screen");
        assert_eq!(screen.components.len(), 1);
        
        let rendered = screen.render();
        assert!(rendered.contains("Test Screen"));
        assert!(rendered.contains("Component 1"));
    }
    
    #[test]
    fn test_color_palettes() {
        let light = ColorPalette::light_theme();
        let dark = ColorPalette::dark_theme();
        
        assert_ne!(light.background, dark.background);
        assert_ne!(light.text_primary, dark.text_primary);
        assert_eq!(light.primary, "#007AFF");
        assert_eq!(dark.background, "#000000");
    }
}