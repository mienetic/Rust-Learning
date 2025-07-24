# 🦀 Cargo Commands สำหรับ Rust Concepts Project

คู่มือการใช้งาน cargo commands สำหรับการตรวจสอบและทดสอบโค้ดแยกตามหัวข้อ

## 📋 คำสั่ง Cargo พื้นฐาน

### 🔍 การตรวจสอบโค้ด (Code Checking)

#### 1. `cargo check` - ตรวจสอบ syntax และ type errors
```bash
# ตรวจสอบทั้งโปรเจกต์
cargo check

# ตรวจสอบเฉพาะ module ที่ต้องการ
cargo check --bin rust_concepts

# ตรวจสอบพร้อม verbose output
cargo check --verbose
```

#### 2. `cargo clippy` - ตรวจสอบ code quality และ best practices
```bash
# รัน clippy ทั้งโปรเจกต์
cargo clippy

# รัน clippy โดยไม่แสดง warnings
cargo clippy -- -A warnings

# รัน clippy แบบ strict
cargo clippy -- -D warnings

# รัน clippy สำหรับ specific target
cargo clippy --bin rust_concepts
```

### 🧪 การทดสอบ (Testing)

#### 3. `cargo test` - รันการทดสอบ
```bash
# รัน test ทั้งหมด
cargo test

# รัน test เฉพาะ module
cargo test basics
cargo test functions
cargo test ownership
cargo test structs_enums
cargo test error_handling
cargo test collections
cargo test generics
cargo test traits
cargo test lifetimes
cargo test modules

# รัน test เฉพาะฟังก์ชัน
cargo test test_variables_work
cargo test test_data_types

# รัน test แบบ verbose
cargo test -- --nocapture

# รัน test แบบ single thread
cargo test -- --test-threads=1
```

### 🚀 การรันโปรแกรม (Running)

#### 4. `cargo run` - รันโปรแกรม
```bash
# รันโปรแกรมหลัก
cargo run

# รันแบบ release mode (optimized)
cargo run --release

# รันพร้อม arguments
cargo run -- arg1 arg2
```

## 📚 การรัน Commands แยกตามหัวข้อ

### 🔥 บทที่ 1: พื้นฐาน Rust (Basics)
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test basics::tests
cargo test test_variables_work
cargo test test_data_types
cargo test test_compound_types
```

### 🚀 บทที่ 2: Functions และ Control Flow
```bash
# ตรวจสอบ functions module
cargo check --lib
cargo clippy --lib

# ทดสอบ functions (ยังไม่มี tests - สามารถเพิ่มได้)
cargo test functions

# รันเฉพาะส่วน functions
# (ต้องแก้ไข main.rs ให้เรียกเฉพาะ functions::learn_functions())
```

### 🔒 บทที่ 3: Ownership และ Borrowing
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ (ยังไม่มี tests - สามารถเพิ่มได้)
cargo test ownership
```

### 📊 บทที่ 4: Structs และ Enums
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ (ยังไม่มี tests - สามารถเพิ่มได้)
cargo test structs_enums
```

### ⚠️ บทที่ 5: Error Handling
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test error_handling::tests
cargo test test_option_some
cargo test test_option_none
cargo test test_result_ok
cargo test test_result_err
```

### 📦 บทที่ 6: Collections
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test collections::tests
cargo test test_vector_operations
cargo test test_hashmap_operations
cargo test test_hashset_operations
```

### 🔧 บทที่ 7: Generic Types
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test generics::tests
cargo test test_generic_largest
cargo test test_generic_point
cargo test test_generic_stack
```

### 🎭 บทที่ 8: Traits
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test traits::tests
cargo test test_trait_implementation
cargo test test_trait_bounds
cargo test test_trait_objects
```

### ⏰ บทที่ 9: Lifetimes
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test lifetimes::tests
cargo test test_longest_function
cargo test test_struct_with_lifetime
cargo test test_lifetime_elision
```

### 📦 บทที่ 10: Module System
```bash
# ตรวจสอบ
cargo check --lib
cargo clippy --lib

# ทดสอบ
cargo test modules::practice_modules::tests
cargo test test_module_visibility
cargo test test_use_statements
cargo test test_reexporting
```

## 🛠️ การสร้าง Scripts สำหรับแต่ละหัวข้อ

### สร้างไฟล์ script สำหรับแต่ละหัวข้อ:

#### `scripts/check_basics.sh`
```bash
#!/bin/bash
echo "🔥 ตรวจสอบ Basics Module"
cargo check --lib
cargo clippy --lib
cargo test basics::tests
echo "✅ Basics Module ผ่านการตรวจสอบ"
```

#### `scripts/check_functions.sh`
```bash
#!/bin/bash
echo "🚀 ตรวจสอบ Functions Module"
cargo check --lib
cargo clippy --lib
# cargo test functions # เมื่อมี tests แล้ว
echo "✅ Functions Module ผ่านการตรวจสอบ"
```

#### `scripts/check_all.sh`
```bash
#!/bin/bash
echo "🦀 ตรวจสอบทั้งโปรเจกต์"
cargo check
cargo clippy
cargo test
echo "🎉 ทั้งโปรเจกต์ผ่านการตรวจสอบ"
```

## 📝 Tips และ Best Practices

1. **ใช้ `cargo check` ก่อนเสมอ** - เร็วกว่า `cargo build`
2. **รัน `cargo clippy` เป็นประจำ** - ช่วยปรับปรุง code quality
3. **เขียน tests สำหรับทุก module** - ใช้ `cargo test` เพื่อตรวจสอบ
4. **ใช้ `cargo run --release`** - สำหรับ production builds
5. **ใช้ `--verbose`** - เมื่อต้องการ debug information

## 🎯 การเพิ่ม Tests สำหรับ Modules ที่ยังไม่มี

สำหรับ modules ที่ยังไม่มี tests (functions, ownership, structs_enums) สามารถเพิ่ม tests ได้โดย:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // test implementation
        assert_eq!(expected, actual);
    }
}
```

แล้วรันด้วย `cargo test module_name::tests`