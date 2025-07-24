//! บทที่ 14: Unsafe Rust
//!
//! บทนี้จะสอนเกี่ยวกับ:
//! - การใช้ unsafe blocks
//! - Raw pointers
//! - การเรียกใช้ unsafe functions
//! - การสร้าง unsafe traits
//! - FFI (Foreign Function Interface)
//! - Memory management ขั้นสูง

// นำเข้าโมดูลย่อย
pub mod ffi;
pub mod inline_assembly;
pub mod raw_pointers;
pub mod union_transmute;
pub mod unsafe_functions;
pub mod unsafe_traits;

// Re-export สำหรับความสะดวก
pub use ffi::*;
pub use inline_assembly::*;
pub use raw_pointers::*;
pub use union_transmute::*;
pub use unsafe_functions::*;
pub use unsafe_traits::*;

// เนื้อหาต่างๆ ถูกย้ายไปยังไฟล์ย่อยแล้ว:
// - raw_pointers.rs: การใช้งาน raw pointers และ memory management
// - unsafe_functions.rs: การสร้างและใช้งาน unsafe functions
// - unsafe_traits.rs: unsafe traits และ custom smart pointers
// - ffi.rs: Foreign Function Interface
// - union_transmute.rs: Union และ transmute operations
// - inline_assembly.rs: Inline assembly และ performance optimization

// ฟังก์ชันเหล่านี้ถูกย้ายไปยังไฟล์ย่อยแล้ว:
// - ffi_examples() -> ffi.rs
// - inline_assembly_examples() -> inline_assembly.rs
// - transmute_examples() -> union_transmute.rs

/// ฟังก์ชันหลักสำหรับรันตัวอย่างทั้งหมด
pub fn run_unsafe_examples() {
    println!("\n🦀 === บทที่ 14: Unsafe Rust ===");
    println!("📚 เรียนรู้การใช้ unsafe Rust อย่างปลอดภัย\n");

    println!("⚠️ คำเตือน: Unsafe Rust ต้องใช้ด้วยความระมัดระวัง!");
    println!("💡 ใช้เฉพาะเมื่อจำเป็นและเข้าใจความเสี่ยง\n");

    // เรียกใช้ฟังก์ชันจากไฟล์ย่อยต่างๆ
    raw_pointers_examples();
    memory_management_examples();
    unsafe_functions_examples();
    unsafe_traits_examples();
    custom_smart_pointer_examples();
    union_examples();
    data_conversion_examples();
    ffi_examples();
    c_string_examples();
    transmute_examples();
    transmute_copy_examples();
    inline_assembly_examples();
    assembly_syntax_examples();

    println!("\n✅ จบบทที่ 14: Unsafe Rust!");
    println!("💡 คุณได้เรียนรู้:");
    println!("   - การใช้ raw pointers และ memory management");
    println!("   - การสร้างและเรียกใช้ unsafe functions");
    println!("   - การสร้าง unsafe traits และ custom smart pointers");
    println!("   - การใช้ FFI เพื่อเชื่อมต่อกับ C libraries");
    println!("   - การใช้ union และ transmute operations");
    println!("   - การเข้าใจ inline assembly และ performance optimization");
    println!("\n🛡️ จำไว้: \"With great power comes great responsibility\"");
    println!("📁 เนื้อหาถูกจัดระเบียบในไฟล์ย่อยเพื่อความชัดเจน");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_module_integration() {
        // ทดสอบว่าสามารถเรียกใช้ฟังก์ชันจากไฟล์ย่อยต่างๆ ได้

        // ทดสอบ raw pointers
        raw_pointers_examples();
        memory_management_examples();

        // ทดสอบ unsafe functions
        unsafe_functions_examples();

        // ทดสอบ unsafe traits
        unsafe_traits_examples();
        custom_smart_pointer_examples();

        // ทดสอบ FFI
        ffi_examples();
        c_string_examples();

        // ทดสอบ union และ transmute
        union_examples();
        data_conversion_examples();
        transmute_examples();
        transmute_copy_examples();

        // ทดสอบ inline assembly
        inline_assembly_examples();
        assembly_syntax_examples();
    }

    #[test]
    fn test_run_all_examples() {
        // ทดสอบว่าฟังก์ชันหลักทำงานได้
        run_unsafe_examples();
    }

    // หมายเหตุ: Tests เฉพาะของแต่ละโมดูลอยู่ในไฟล์ย่อยแล้ว:
    // - raw_pointers.rs มี test_raw_pointers
    // - unsafe_functions.rs มี test_slice_from_raw_parts และ test_safe_multiply_by_two
    // - unsafe_traits.rs มี test_my_box และ test_unsafe_trait
    // - ffi.rs มี test_ffi_functions, test_rust_exported_functions และ test_c_string_length
    // - union_transmute.rs มี test_union, test_transmute, test_data_converter และ test_transmute_copy
    // - inline_assembly.rs มี test_assembly_examples
}
