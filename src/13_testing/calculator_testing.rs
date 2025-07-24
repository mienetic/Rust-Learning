//! # Calculator Testing Examples - เครื่องคิดเลขนักสืบ! 🧮🕵️‍♂️
//!
//! ตัวอย่างการทดสอบ struct และ methods
//! เหมือนการทดสอบเครื่องมือคำนวณของนักสืบโค้ด! 🔍✨
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การทดสอบ struct และ methods อย่างละเอียด!

/// เครื่องคิดเลขนักสืบ - เก็บหลักฐานตัวเลข! 🧮🔍
#[derive(Debug, PartialEq)]  // 🔬 สามารถ debug และเปรียบเทียบได้
pub struct Calculator {
    memory: f64,  // 💾 หน่วยความจำเก็บหลักฐาน
}

impl Default for Calculator {
    /// สร้างเครื่องคิดเลขแบบ default - เริ่มต้นการสืบสวน! 🆕🔍
    fn default() -> Self {
        Self::new()  // 🎯 เรียกใช้ constructor หลัก
    }
}

impl Calculator {
    /// สร้างเครื่องคิดเลขใหม่ - เริ่มการสืบสวนใหม่! 🆕🧮
    #[must_use]  // 🚨 ต้องใช้ผลลัพธ์!
    pub const fn new() -> Self {
        Self { memory: 0.0 }  // 💾 เริ่มต้นด้วยหลักฐานเป็นศูนย์
    }

    /// บวกค่า - เพิ่มหลักฐานใหม่! ➕🔍
    pub fn add(&mut self, value: f64) -> f64 {
        self.memory += value;  // ➕ เพิ่มหลักฐาน
        self.memory  // 📊 ส่งคืนผลรวมหลักฐาน
    }

    /// ลบค่า - ลดหลักฐาน! ➖🔍
    pub fn subtract(&mut self, value: f64) -> f64 {
        self.memory -= value;  // ➖ ลดหลักฐาน
        self.memory  // 📊 ส่งคืนผลต่างหลักฐาน
    }

    /// คูณค่า - ขยายหลักฐาน! ✖️🔍
    pub fn multiply(&mut self, value: f64) -> f64 {
        self.memory *= value;  // ✖️ ขยายหลักฐาน
        self.memory  // 📊 ส่งคืนผลคูณหลักฐาน
    }

    /// หารค่า - แบ่งหลักฐาน (ระวังการหารด้วยศูนย์!) ➗⚠️
    /// 
    /// # Errors
    /// 
    /// ส่งคืน error หากพยายามหารด้วยศูนย์ (หลักฐานผิดพลาด!)
    pub fn divide(&mut self, value: f64) -> Result<f64, String> {
        if value == 0.0 {
            Err("Cannot divide by zero".to_string())  // 🚨 หลักฐานผิดพลาด!
        } else {
            self.memory /= value;  // ➗ แบ่งหลักฐาน
            Ok(self.memory)  // ✅ ส่งคืนผลหารหลักฐาน
        }
    }

    /// ล้างหน่วยความจำ - เริ่มการสืบสวนใหม่! 🧹🔄
    pub const fn clear(&mut self) {
        self.memory = 0.0;  // 🧹 ล้างหลักฐานทั้งหมด
    }

    /// ดูหน่วยความจำ - ตรวจสอบหลักฐานปัจจุบัน! 👁️🔍
    #[must_use]  // 🚨 ต้องใช้ผลลัพธ์!
    pub const fn get_memory(&self) -> f64 {
        self.memory  // 📊 ส่งคืนหลักฐานปัจจุบัน
    }
}

/// ตัวอย่างการใช้งาน Calculator - ทดสอบเครื่องคิดเลขนักสืบ! 🧮🕵️‍♂️
pub fn calculator_testing_examples() {
    println!("🧮✨ === Calculator Testing Examples - เครื่องคิดเลขนักสืบ! === ✨🧮");
    println!("🕵️‍♂️ เริ่มทดสอบเครื่องมือคำนวณหลักฐาน! 🔍");

    // 🧮 ทดสอบ Calculator - เริ่มการสืบสวน!
    let mut calc = Calculator::new();
    println!("🆕🔍 Calculator ใหม่: memory = {} (เริ่มต้นการสืบสวน!)", calc.get_memory());

    calc.add(10.0);
    println!("➕🔍 เพิ่ม 10: memory = {} (เพิ่มหลักฐานแรก!)", calc.get_memory());

    calc.multiply(2.0);
    println!("✖️🔍 คูณ 2: memory = {} (ขยายหลักฐาน!)", calc.get_memory());

    calc.subtract(5.0);
    println!("➖🔍 ลบ 5: memory = {} (ลดหลักฐาน!)", calc.get_memory());

    // 🔍 ทดสอบการหาร - แบ่งหลักฐาน!
    match calc.divide(3.0) {
        Ok(result) => println!("➗✅ หาร 3: memory = {result} (แบ่งหลักฐานสำเร็จ!)"),
        Err(e) => println!("❌🚨 ข้อผิดพลาด: {e}"),
    }

    // 🚨 ทดสอบการหารด้วยศูนย์ - สถานการณ์อันตราย!
    match calc.divide(0.0) {
        Ok(result) => println!("➗✅ หาร 0: memory = {result} (ไม่น่าจะเกิดขึ้น!)"),
        Err(e) => println!("❌🔍 ข้อผิดพลาดที่คาดไว้: {e} (หลักฐานข้อผิดพลาด!)"),
    }

    calc.clear();
    println!("🧹✨ ล้างหน่วยความจำ: memory = {} (เริ่มการสืบสวนใหม่!)", calc.get_memory());
    
    println!("🎉 การทดสอบเครื่องคิดเลขนักสืบเสร็จสิ้น! ✨");
}

#[cfg(test)]  // 🧪 โหมดทดสอบ - เข้าสู่ห้องแล็บนักสืบ!
pub mod tests {
    use super::*;

    // 🧮 ทดสอบ Calculator - ตรวจสอบเครื่องคิดเลขนักสืบ!
    #[test]  // 🔬 การทดสอบที่ 1
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นค่าคงที่ (0.0) ซึ่งปลอดภัย
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_new() {
        let calc = Calculator::new();
        assert_eq!(calc.get_memory(), 0.0);  // ✅ ตรวจสอบการเริ่มต้น
    }

    #[test]  // 🔬 การทดสอบที่ 2
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(5.0), 5.0);    // ✅ หลักฐาน: +5=5
        assert_eq!(calc.add(3.0), 8.0);    // ✅ หลักฐาน: 5+3=8
        assert_eq!(calc.get_memory(), 8.0); // ✅ ตรวจสอบหน่วยความจำ
    }

    #[test]  // 🔬 การทดสอบที่ 3
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_subtract() {
        let mut calc = Calculator::new();
        calc.add(10.0);  // 🎯 เตรียมหลักฐาน
        assert_eq!(calc.subtract(3.0), 7.0);  // ✅ หลักฐาน: 10-3=7
        assert_eq!(calc.subtract(2.0), 5.0);  // ✅ หลักฐาน: 7-2=5
    }

    #[test]  // 🔬 การทดสอบที่ 4
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_multiply() {
        let mut calc = Calculator::new();
        calc.add(5.0);  // 🎯 เตรียมหลักฐาน
        assert_eq!(calc.multiply(3.0), 15.0);  // ✅ หลักฐาน: 5×3=15
        assert_eq!(calc.multiply(2.0), 30.0);  // ✅ หลักฐาน: 15×2=30
    }

    #[test]  // 🔬 การทดสอบที่ 5
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_divide() {
        let mut calc = Calculator::new();
        calc.add(20.0);  // 🎯 เตรียมหลักฐาน
        assert_eq!(calc.divide(4.0), Ok(5.0));   // ✅ หลักฐาน: 20÷4=5
        assert_eq!(calc.divide(2.0), Ok(2.5));   // ✅ หลักฐาน: 5÷2=2.5
    }

    #[test]  // 🔬 การทดสอบที่ 6
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_divide_by_zero() {
        let mut calc = Calculator::new();
        calc.add(10.0);  // 🎯 เตรียมหลักฐาน
        assert!(calc.divide(0.0).is_err());      // 🚨 ตรวจสอบ error
        assert_eq!(calc.get_memory(), 10.0);     // ✅ ค่าไม่เปลี่ยน (หลักฐานปลอดภัย!)
    }

    #[test]  // 🔬 การทดสอบที่ 7
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นค่าคงที่ (0.0) ซึ่งปลอดภัย
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_clear() {
        let mut calc = Calculator::new();
        calc.add(100.0);  // 🎯 เพิ่มหลักฐาน
        calc.clear();     // 🧹 ล้างหลักฐาน
        assert_eq!(calc.get_memory(), 0.0);  // ✅ ตรวจสอบการล้าง
    }

    // 🎯 ทดสอบ edge cases - สถานการณ์ซับซ้อน!
    #[test]  // 🔬 การทดสอบที่ 8
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะใช้ EPSILON ในการเปรียบเทียบ floating point
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_chain_operations() {
        let mut calc = Calculator::new();

        calc.add(10.0);       // ➕ เพิ่มหลักฐาน: 0+10=10
        calc.multiply(2.0);   // ✖️ ขยายหลักฐาน: 10×2=20
        calc.subtract(5.0);   // ➖ ลดหลักฐาน: 20-5=15
        calc.divide(3.0).unwrap();  // ➗ แบ่งหลักฐาน: 15÷3=5

        assert!((calc.get_memory() - 5.0).abs() < f64::EPSILON);  // ✅ ตรวจสอบความแม่นยำ
    }

    #[test]  // 🔬 การทดสอบที่ 9
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    // ใช้ #[allow(clippy::float_cmp)] เพราะเปรียบเทียบ f64 ที่เป็นผลลัพธ์ที่คาดหวัง ซึ่งแม่นยำ
    #[allow(clippy::float_cmp)]  // 🔢 อนุญาตการเปรียบเทียบ float
    pub fn test_calculator_negative_operations() {
        let mut calc = Calculator::new();

        calc.add(-5.0);  // ➕ เพิ่มหลักฐานลบ: 0+(-5)=-5
        assert_eq!(calc.get_memory(), -5.0);  // ✅ ตรวจสอบค่าลบ

        calc.multiply(-2.0);  // ✖️ คูณด้วยลบ: (-5)×(-2)=10
        assert_eq!(calc.get_memory(), 10.0);  // ✅ ลบคูณลบเป็นบวก

        calc.subtract(-3.0);  // ➖ ลบด้วยลบ: 10-(-3)=13
        assert_eq!(calc.get_memory(), 13.0);  // ✅ ลบลบเป็นบวก
    }

    #[test]  // 🔬 การทดสอบที่ 10
    // ใช้ #[allow(clippy::missing_panics_doc)] เพราะเป็น test function ที่ไม่ควร panic
    #[allow(clippy::missing_panics_doc)]  // 🤫 ปิดเสียง warning
    pub fn test_calculator_testing_functions() {
        // 🧪 ทดสอบว่าฟังก์ชันทำงานได้โดยไม่ panic - ตรวจสอบความปลอดภัย!
        calculator_testing_examples();  // 🔍 เรียกใช้การทดสอบเครื่องคิดเลข
    }
}