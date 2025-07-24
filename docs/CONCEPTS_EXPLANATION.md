# คู่มือเรียนรู้ Rust Concepts

โปรเจกต์นี้เป็นคู่มือการเรียนรู้ภาษา Rust ที่ครอบคลุมแนวคิดสำคัญต่างๆ ตั้งแต่พื้นฐานไปจนถึงระดับขั้นสูง

## 📚 เนื้อหาหลัก

### 1. พื้นฐาน (Basics)
- **Variables & Constants**: การประกาศตัวแปรและค่าคงที่
- **Data Types**: ชนิดข้อมูลพื้นฐานใน Rust
- **Operators**: ตัวดำเนินการต่างๆ (arithmetic, comparison, logical)
- **Control Flow**: การควบคุมการทำงาน (if, loop, while, for)
- **Functions**: การสร้างและใช้งานฟังก์ชัน

### 2. Ownership System
- **Ownership**: แนวคิดการเป็นเจ้าของข้อมูลใน Rust
- **Borrowing**: การยืมข้อมูลแบบ reference
- **Lifetimes**: การกำหนดอายุการใช้งานของ reference

### 3. Structs และ Enums
- **Structs**: การสร้างโครงสร้างข้อมูลแบบกำหนดเอง
- **Enums**: การสร้างชนิดข้อมูลที่มีหลายตัวเลือก
- **Pattern Matching**: การจับคู่รูปแบบด้วย match

### 4. Collections
- **Vectors**: อาร์เรย์แบบขยายได้
- **HashMaps**: โครงสร้างข้อมูลแบบ key-value
- **Strings**: การจัดการข้อความ

### 5. Error Handling
- **Result Type**: การจัดการข้อผิดพลาดด้วย Result<T, E>
- **Option Type**: การจัดการค่าที่อาจไม่มีด้วย Option<T>
- **Custom Errors**: การสร้าง error types แบบกำหนดเอง

### 6. Generics และ Traits
- **Generics**: การเขียนโค้ดที่ทำงานกับหลายชนิดข้อมูล
- **Traits**: การกำหนดพฤติกรรมที่ใช้ร่วมกัน
- **Trait Objects**: การใช้ traits แบบ dynamic

### 7. Advanced Features
- **Macros**: การสร้างโค้ดอัตโนมัติ
- **Async/Await**: การเขียนโปรแกรมแบบ asynchronous
- **Unsafe Rust**: การใช้งาน unsafe code
- **Testing**: การเขียนและรัน tests

### 8. Smart Pointers และ Memory Management
- **Box<T>**: การจัดเก็บข้อมูลใน heap
- **Rc<T> และ Arc<T>**: การแชร์ ownership
- **RefCell<T> และ Mutex<T>**: การเปลี่ยนแปลงข้อมูลแบบ interior mutability

### 9. Concurrency และ Parallelism
- **Threads**: การสร้างและจัดการ threads
- **Message Passing**: การสื่อสารระหว่าง threads ด้วย channels
- **Shared State**: การแชร์ข้อมูลระหว่าง threads อย่างปลอดภัย

### 10. I/O และ File System
- **File Operations**: การอ่านและเขียนไฟล์
- **Directory Operations**: การจัดการโฟลเดอร์
- **Standard I/O**: การรับ input และแสดง output

### 11. Network Programming
- **TCP/UDP**: การเขียนโปรแกรม network ระดับต่ำ
- **HTTP Client/Server**: การสร้าง web services
- **WebSocket**: การสื่อสารแบบ real-time

### 12. Web Development
- **Web Frameworks**: การใช้ Actix-web, Warp, Axum
- **REST APIs**: การสร้าง RESTful services
- **Template Engines**: การสร้าง dynamic web pages

### 13. Database Integration
- **SQL Databases**: การเชื่อมต่อกับ PostgreSQL, MySQL
- **NoSQL Databases**: การใช้งาน MongoDB, Redis
- **ORM**: การใช้ Diesel และ SQLx

### 14. Performance Optimization
- **Profiling**: การวิเคราะห์ performance
- **Memory Optimization**: การจัดการ memory อย่างมีประสิทธิภาพ
- **Benchmarking**: การวัดและเปรียบเทียบ performance

### 15. Specialized Development
- **FFI**: การเชื่อมต่อกับภาษาอื่น (C/C++)
- **Embedded Programming**: การเขียนโปรแกรมสำหรับ microcontrollers
- **Game Development**: การสร้างเกมด้วย Bevy engine
- **Blockchain Development**: การพัฒนา smart contracts
- **Mobile Development**: การสร้างแอปมือถือด้วย Tauri
- **DevOps**: การ deploy และจัดการ infrastructure

## 🚀 วิธีการใช้งาน

### รันโปรแกรมหลัก
```bash
cargo run
```

### รัน tests
```bash
cargo test
```

### ตรวจสอบโค้ด
```bash
cargo check
cargo clippy
```

### รันตัวอย่างเฉพาะ
```bash
cargo run --example real_world_cli
cargo run --example web_api_example
```

## 📁 โครงสร้างโปรเจกต์

```
src/
├── 01_basics/              # พื้นฐานภาษา Rust
├── 02_ownership/           # Ownership และ Borrowing
├── 03_structs_enums/       # Structs และ Enums
├── 04_functions/           # Functions และ Closures
├── 05_modules/             # Module System
├── 06_collections/         # Collections (Vec, HashMap, etc.)
├── 07_error_handling/      # Error Handling
├── 08_generics/            # Generics
├── 09_traits/              # Traits
├── 10_lifetimes/           # Lifetimes
├── 11_async_await/         # Async Programming
├── 12_macros/              # Macros
├── 13_testing/             # Testing
├── 14_unsafe_rust/         # Unsafe Rust
├── 15_smart_pointers/      # Smart Pointers
├── 16_concurrency/         # Concurrency และ Parallelism
├── 17_io_filesystem/       # I/O และ File System
├── 18_networking/          # Network Programming
├── 19_web_development/     # Web Development
├── 20_database/            # Database Integration
├── 21_performance/         # Performance Optimization
├── 22_ffi/                 # Foreign Function Interface
├── 23_embedded/            # Embedded Programming
├── 24_devops/              # DevOps และ Deployment
├── 25_game_development/    # Game Development
├── 26_blockchain/          # Blockchain Development
├── 27_mobile_development/  # Mobile Development
├── lib.rs                 # Library root
└── main.rs                # Main executable
```

## 🎯 จุดเด่นของโปรเจกต์

1. **ครอบคลุมทุกแนวคิดสำคัญ**: ตั้งแต่พื้นฐานไปจนถึงขั้นสูง
2. **ตัวอย่างที่ใช้งานได้จริง**: โค้ดทุกส่วนสามารถรันและทดสอบได้
3. **คอมเมนต์ภาษาไทย**: อธิบายแนวคิดด้วยภาษาไทยที่เข้าใจง่าย
4. **Tests ครอบคลุม**: มี unit tests และ integration tests
5. **Real-world Examples**: ตัวอย่างการใช้งานจริงเช่น CLI, Web API

## 📖 แนวทางการเรียนรู้

### สำหรับผู้เริ่มต้น (บทที่ 1-10)
1. เริ่มจาก `01_basics/` เพื่อเรียนรู้พื้นฐาน
2. ศึกษา `02_ownership/` เพื่อเข้าใจ ownership system
3. ต่อด้วย `03_structs_enums/` และ `04_functions/`
4. เรียนรู้ `05_modules/` และ `06_collections/`
5. ศึกษา `07_error_handling/` เพื่อจัดการข้อผิดพลาด
6. เรียนรู้ `08_generics/`, `09_traits/`, และ `10_lifetimes/`

### สำหรับผู้ที่มีพื้นฐาน (บทที่ 11-14)
1. ศึกษา `11_async_await/` สำหรับ asynchronous programming
2. เรียนรู้ `12_macros/` เพื่อสร้างโค้ดอัตโนมัติ
3. ลองใช้ `13_testing/` เพื่อเขียน tests ที่มีคุณภาพ
4. สำรวจ `14_unsafe_rust/` สำหรับ advanced memory management

### สำหรับผู้ที่ต้องการความเชี่ยวชาญ (บทที่ 15-21)
1. ศึกษา `15_smart_pointers/` เพื่อเข้าใจ advanced memory management
2. เรียนรู้ `16_concurrency/` สำหรับ parallel programming
3. ลองใช้ `17_io_filesystem/` และ `18_networking/`
4. สำรวจ `19_web_development/` และ `20_database/`
5. ศึกษา `21_performance/` เพื่อ optimization

### สำหรับการพัฒนาเฉพาะทาง (บทที่ 22-27)
1. `22_ffi/` - สำหรับการเชื่อมต่อกับภาษาอื่น
2. `23_embedded/` - สำหรับ embedded systems
3. `24_devops/` - สำหรับ deployment และ infrastructure
4. `25_game_development/` - สำหรับการพัฒนาเกม
5. `26_blockchain/` - สำหรับ blockchain development
6. `27_mobile_development/` - สำหรับการพัฒนาแอปมือถือ

## 🔧 เครื่องมือและการตั้งค่า

### Dependencies หลัก
- `tokio`: สำหรับ async runtime
- `serde`: สำหรับ serialization/deserialization
- `clap`: สำหรับ command-line parsing
- `anyhow`: สำหรับ error handling

### Development Tools
- `cargo-watch`: สำหรับ auto-reload
- `cargo-expand`: สำหรับดู macro expansion
- `cargo-audit`: สำหรับตรวจสอบ security vulnerabilities

## 💡 Tips การเรียนรู้

1. **อ่านโค้ดและรันทดสอบ**: ลองรันโค้ดแต่ละส่วนเพื่อดูผลลัพธ์
2. **แก้ไขและทดลอง**: ลองแก้ไขโค้ดเพื่อดูว่าเกิดอะไรขึ้น
3. **อ่าน error messages**: Rust compiler ให้ error messages ที่มีประโยชน์มาก
4. **ใช้ `cargo doc --open`**: เพื่อดู documentation ของโปรเจกต์
5. **ศึกษา examples/**: ดูตัวอย่างการใช้งานจริงใน `examples/`

## 🤝 การมีส่วนร่วม

หากต้องการปรับปรุงหรือเพิ่มเติมเนื้อหา:
1. Fork โปรเจกต์
2. สร้าง branch ใหม่
3. เพิ่มเติมหรือแก้ไขเนื้อหา
4. เขียน tests สำหรับโค้ดใหม่
5. ส่ง Pull Request

## 📚 แหล่งเรียนรู้เพิ่มเติม

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (สำหรับ unsafe Rust)
- [Async Book](https://rust-lang.github.io/async-book/) (สำหรับ async programming)

---

**หมายเหตุ**: โปรเจกต์นี้ออกแบบมาเพื่อการเรียนรู้ หากต้องการใช้ในการพัฒนาจริง ควรปรับปรุงและเพิ่มเติมตามความเหมาะสม