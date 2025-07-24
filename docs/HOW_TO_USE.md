# 🦀 วิธีการใช้งาน Rust Concepts Learning Project

## 📖 ภาพรวม
โปรเจกต์ Rust Concepts นี้มีเครื่องมือหลายแบบสำหรับการตรวจสอบและทดสอบโค้ดแยกตามหัวข้อ:

1. **Cargo Commands โดยตรง** - คำสั่งพื้นฐานของ Cargo
2. **Makefile** - คำสั่งที่ง่ายและสะดวก
3. **Shell Scripts** - สคริปต์อัตโนมัติ

## 🚀 วิธีการใช้งาน

```bash
cargo run    # รันโปรแกรมเรียนรู้
cargo test   # ทดสอบความเข้าใจ
```

### 1. การใช้ Cargo Commands โดยตรง

#### ตรวจสอบทั้งโปรเจกต์:
```bash
cargo check          # ตรวจสอบ syntax และ type errors
cargo clippy         # ตรวจสอบ code quality
cargo test           # รัน tests ทั้งหมด
cargo run            # รันโปรแกรม
```

#### ตรวจสอบแยกตาม module:
```bash
# ทดสอบเฉพาะ module พื้นฐาน (1-10)
cargo test basics::tests
cargo test ownership::tests
cargo test structs_enums::tests
cargo test functions::tests
cargo test modules::tests
cargo test collections::tests
cargo test error_handling::tests
cargo test generics::tests
cargo test traits::tests
cargo test lifetimes::tests

# ทดสอบ module ขั้นกลาง (11-14)
cargo test async_await::tests
cargo test macros::tests
cargo test testing::tests
cargo test unsafe_rust::tests

# ทดสอบ module ขั้นสูง (15-21)
cargo test smart_pointers::tests
cargo test concurrency::tests
cargo test io_filesystem::tests
cargo test networking::tests
cargo test web_development::tests
cargo test database::tests
cargo test performance::tests

# ทดสอบ module เฉพาะทาง (22-27)
cargo test ffi::tests
cargo test embedded::tests
cargo test devops::tests
cargo test game_development::tests
cargo test blockchain::tests
cargo test mobile_development::tests

# ทดสอบเฉพาะฟังก์ชัน
cargo test test_variables_work
cargo test test_data_types
```

### 2. การใช้ Makefile (แนะนำ ⭐)

#### ดูคำสั่งที่มีให้:
```bash
make help
```

#### คำสั่งพื้นฐาน:
```bash
make check           # ตรวจสอบ syntax
make clippy          # ตรวจสอบ code quality
make test            # รัน tests
make run             # รันโปรแกรม
make all             # รัน check + clippy + test
```

#### ตรวจสอบแยกตามหัวข้อ:

**บทเรียนพื้นฐาน (1-10):**
```bash
make basics          # 🔥 Basics - พื้นฐาน Rust
make ownership       # 🔒 Ownership - Ownership และ Borrowing
make structs         # 📊 Structs & Enums
make functions       # 🚀 Functions - ฟังก์ชันและ Control Flow
make modules         # 📦 Modules - Module System
make collections     # 📦 Collections - Vec, HashMap, etc.
make error           # ⚠️ Error Handling
make generics        # 🔧 Generics
make traits          # 🎭 Traits
make lifetimes       # ⏰ Lifetimes
```

**บทเรียนขั้นกลาง (11-14):**
```bash
make async           # ⚡ Async/Await Programming
make macros          # 🎪 Macros
make testing         # 🧪 Testing
make unsafe          # ⚠️ Unsafe Rust
```

**บทเรียนขั้นสูง (15-21):**
```bash
make smart_pointers  # 🧠 Smart Pointers
make concurrency     # 🔄 Concurrency & Parallelism
make io              # 📁 I/O & File System
make networking      # 🌐 Network Programming
make web             # 🕸️ Web Development
make database        # 🗄️ Database Integration
make performance     # ⚡ Performance Optimization
```

**บทเรียนเฉพาะทาง (22-27):**
```bash
make ffi             # 🔗 Foreign Function Interface
make embedded        # 🔧 Embedded Programming
make devops          # 🚀 DevOps & Deployment
make game            # 🎮 Game Development
make blockchain      # ⛓️ Blockchain Development
make mobile          # 📱 Mobile Development
```

#### คำสั่งเพิ่มเติม:
```bash
make clean           # ลบไฟล์ build
make release         # build แบบ release
make doc             # สร้าง documentation
```

### 3. การใช้ Shell Scripts

#### ให้สิทธิ์ execute (ครั้งแรกเท่านั้น):
```bash
chmod +x scripts/*.sh
```

#### รัน scripts:
```bash
# ตรวจสอบ basics module
./scripts/check_basics.sh

# ตรวจสอบ functions module
./scripts/check_functions.sh

# ตรวจสอบทั้งโปรเจกต์
./scripts/check_all.sh

# เมนูแบบ interactive
./scripts/check_by_topic.sh
```

## 📊 ตัวอย่างผลลัพธ์

### เมื่อรัน `make basics`:
```
🔥 ตรวจสอบ Basics module...
cargo check
    Finished dev profile [unoptimized + debuginfo] target(s) in 0.12s
cargo clippy
    Finished dev profile [unoptimized + debuginfo] target(s) in 1.26s
cargo test basics::tests
    Finished test profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs

running 3 tests
test basics::tests::test_compound_types ... ok
test basics::tests::test_data_types ... ok
test basics::tests::test_variables_work ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out

✅ Basics module ผ่านการตรวจสอบ
```

## 🎯 แนะนำการใช้งาน

### สำหรับการพัฒนาประจำวัน:
1. **ใช้ `make check`** - เร็วที่สุด เพื่อตรวจสอบ syntax
2. **ใช้ `make clippy`** - ตรวจสอบ code quality
3. **ใช้ `make test`** - รัน tests ทั้งหมด

### สำหรับการตรวจสอบเฉพาะหัวข้อ:
1. **ใช้ `make <topic>`** - เช่น `make basics`, `make functions`
2. **ใช้ scripts** - สำหรับการตรวจสอบแบบละเอียด

### สำหรับการตรวจสอบครั้งสุดท้าย:
1. **ใช้ `make all`** - ตรวจสอบทั้งหมด
2. **ใช้ `./scripts/check_all.sh`** - ตรวจสอบแบบละเอียด

## 🔧 การปรับแต่ง

### เพิ่ม Tests สำหรับ Modules ที่ยังไม่มี:
Modules ที่ยังไม่มี tests: `functions`, `ownership`, `structs_enums`

สามารถเพิ่ม tests ได้โดยเพิ่มในไฟล์ `mod.rs` ของแต่ละ module:

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

### แก้ไข Clippy Warnings:
หากต้องการแก้ไข warnings ทั้งหมด:
```bash
cargo clippy --fix --bin "rust_concepts"
```

## 📚 ไฟล์ที่เกี่ยวข้อง

- `CARGO_COMMANDS.md` - คู่มือ cargo commands แบบละเอียด
- `Makefile` - คำสั่ง make ทั้งหมด
- `scripts/` - โฟลเดอร์ scripts
  - `check_basics.sh` - ตรวจสอบ basics
  - `check_functions.sh` - ตรวจสอบ functions
  - `check_all.sh` - ตรวจสอบทั้งหมด
  - `check_by_topic.sh` - เมนูแบบ interactive

## 🎉 สรุป

ตอนนี้คุณมีเครื่องมือครบครันสำหรับการตรวจสอบโค้ด Rust แยกตามหัวข้อแล้ว!

**วิธีที่ง่ายที่สุด:** ใช้ `make help` เพื่อดูคำสั่งทั้งหมด แล้วเลือกใช้ตามต้องการ 🚀