//! Inline Assembly ใน Unsafe Rust
//!
//! ไฟล์นี้ประกอบด้วย:
//! - ตัวอย่างการใช้ inline assembly
//! - คำอธิบายเกี่ยวกับ assembly syntax
//! - ข้อควรระวังในการใช้งาน

/// ตัวอย่างการใช้ inline assembly (ต้องใช้ nightly Rust)
pub fn inline_assembly_examples() {
    println!("\n⚙️ === ตัวอย่าง Inline Assembly ===");

    // หมายเหตุ: inline assembly ต้องใช้ nightly Rust
    // และ feature flag #![feature(asm)]

    println!("💡 Inline assembly ใช้สำหรับ:");
    println!("   - การเขียนโค้ดที่เร็วมาก");
    println!("   - การเข้าถึง CPU instructions พิเศษ");
    println!("   - การทำงานกับ hardware โดยตรง");

    // ตัวอย่างง่ายๆ (ไม่ได้ compile จริงเพราะต้องใช้ nightly)
    /*
    let result: u64;
    unsafe {
        asm!(
            "mov {}, 42",
            out(reg) result
        );
    }
    println!("📊 ผลลัพธ์จาก assembly: {}", result);
    */

    println!("⚠️ ตัวอย่างนี้ต้องใช้ nightly Rust และ feature flag");
}

/// ตัวอย่าง assembly syntax patterns
pub fn assembly_syntax_examples() {
    println!("\n📝 === ตัวอย่าง Assembly Syntax ===");

    println!("🔧 รูปแบบ assembly syntax ใน Rust:");
    println!();

    println!("1. การใช้ registers:");
    println!("   asm!(\"mov {{}}, 42\", out(reg) result);");
    println!();

    println!("2. การใช้ specific registers:");
    println!("   asm!(\"mov rax, {{}}\", in(reg) value);");
    println!();

    println!("3. การใช้ memory operands:");
    println!("   asm!(\"mov {{}}, [{{}}]\", out(reg) result, in(reg) ptr);");
    println!();

    println!("4. การใช้ immediate values:");
    println!("   asm!(\"add {{}}, 10\", inout(reg) value);");
    println!();

    println!("5. การใช้ clobbers:");
    println!(
        "   asm!(\"cpuid\", inout(\"eax\") a, out(\"ebx\") b, out(\"ecx\") c, out(\"edx\") d);"
    );
    println!();
}

/// ตัวอย่างการใช้ assembly สำหรับ performance-critical code
pub fn performance_assembly_examples() {
    println!("\n🚀 === ตัวอย่าง Performance Assembly ===");

    println!("💡 กรณีที่ควรใช้ inline assembly:");
    println!("   - การคำนวณที่ต้องการความเร็วสูงมาก");
    println!("   - การใช้ SIMD instructions");
    println!("   - การเข้าถึง CPU features พิเศษ");
    println!("   - การทำงานกับ system calls โดยตรง");
    println!();

    println!("⚠️ ข้อควรระวัง:");
    println!("   - ยากต่อการ debug");
    println!("   - ไม่ portable ระหว่าง architectures");
    println!("   - อาจทำให้ compiler optimizations ลดลง");
    println!("   - ต้องเข้าใจ calling conventions");
    println!();

    // ตัวอย่างการใช้ compiler intrinsics แทน inline assembly
    println!("💡 ทางเลือกที่ปลอดภัยกว่า: Compiler Intrinsics");

    #[cfg(target_arch = "x86_64")]
    {
        // ตัวอย่างการใช้ intrinsics
        println!("   - std::arch::x86_64 สำหรับ x86_64 instructions");
        println!("   - เช่น _mm_add_ps สำหรับ SIMD operations");
    }
}

/// ตัวอย่างการใช้ `global_asm`! (assembly ระดับ module)
pub fn global_assembly_examples() {
    println!("\n🌐 === ตัวอย่าง Global Assembly ===");

    println!("📝 global_asm! ใช้สำหรับ:");
    println!("   - การเขียน assembly functions ทั้งหมด");
    println!("   - การสร้าง startup code");
    println!("   - การทำงานกับ bootloaders");
    println!();

    println!("🔧 ตัวอย่าง syntax:");
    println!("   global_asm!(\"");
    println!("       .global my_function");
    println!("       my_function:");
    println!("           mov rax, 42");
    println!("           ret");
    println!("   \");");
    println!();

    println!("⚠️ ต้องระวังเรื่อง symbol naming และ linking");
}

/// ตัวอย่างการใช้ naked functions
pub fn naked_functions_examples() {
    println!("\n🔥 === ตัวอย่าง Naked Functions ===");

    println!("💡 Naked functions คือ:");
    println!("   - Functions ที่ไม่มี prologue/epilogue");
    println!("   - ต้องจัดการ stack เอง");
    println!("   - ใช้สำหรับ interrupt handlers");
    println!();

    println!("🔧 ตัวอย่าง syntax:");
    println!("   #[naked]");
    println!("   unsafe extern \"C\" fn my_naked_function() {{");
    println!("       asm!(\"ret\", options(noreturn));");
    println!("   }}");
    println!();

    println!("⚠️ Naked functions ต้องใช้ด้วยความระมัดระวังสูง!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_examples() {
        // ทดสอบว่าฟังก์ชันต่างๆ ทำงานได้โดยไม่ panic
        inline_assembly_examples();
        assembly_syntax_examples();
        performance_assembly_examples();
        global_assembly_examples();
        naked_functions_examples();
    }

    // หมายเหตุ: การทดสอบ inline assembly จริงๆ ต้องใช้ nightly Rust
    // และ feature flags ที่เหมาะสม
}
