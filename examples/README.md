# Rust Examples - ตัวอย่างการใช้งาน Rust ในจริง

โฟลเดอร์นี้ประกอบด้วยตัวอย่างการใช้งาน Rust ในสถานการณ์จริง แสดงให้เห็นถึงการประยุกต์ใช้แนวคิดต่างๆ ของ Rust ในการพัฒนาแอปพลิเคชันที่ใช้งานได้จริง

## 📋 รายการตัวอย่าง

### 1. 🖥️ CLI Application (`real_world_cli.rs`)
**Task Manager CLI Application**

ตัวอย่างการสร้างแอปพลิเคชัน Command Line Interface ที่สมบูรณ์

**ฟีเจอร์หลัก:**
- ✅ CLI argument parsing ด้วย `clap`
- 📁 File I/O operations
- 🔧 Error handling ด้วย `anyhow`
- 📊 JSON serialization/deserialization ด้วย `serde`
- 🧪 Unit และ Integration tests
- 📝 Task management (CRUD operations)

**การรัน:**
```bash
cargo run --example real_world_cli -- --help
cargo run --example real_world_cli -- add "Complete project" --priority high
cargo run --example real_world_cli -- list
cargo run --example real_world_cli -- complete <task-id>
```

**สิ่งที่เรียนรู้:**
- การใช้ `clap` สำหรับ CLI parsing
- การจัดการไฟล์และ directories
- Error handling patterns
- Testing strategies

---

### 2. 🌐 Web API (`web_api_example.rs`)
**RESTful API Server with Axum**

ตัวอย่างการสร้าง Web API ด้วย Rust และ Axum framework

**ฟีเจอร์หลัก:**
- 🚀 HTTP server ด้วย `axum`
- 📡 RESTful API endpoints
- 🔄 JSON request/response handling
- 🛡️ Error handling และ middleware
- 🧪 API testing
- 📊 User management system

**การรัน:**
```bash
cargo run --example web_api_example
# Server จะรันที่ http://localhost:3000
```

**API Endpoints:**
- `GET /users` - ดึงรายการผู้ใช้ทั้งหมด
- `POST /users` - สร้างผู้ใช้ใหม่
- `GET /users/:id` - ดึงข้อมูลผู้ใช้ตาม ID
- `PUT /users/:id` - อัปเดตข้อมูลผู้ใช้
- `DELETE /users/:id` - ลบผู้ใช้
- `GET /users/stats` - ดึงสถิติผู้ใช้

**สิ่งที่เรียนรู้:**
- Async programming ด้วย `tokio`
- Web framework patterns
- HTTP handling
- API design principles

---

### 3. 🗄️ Database Integration (`database_example.rs`)
**Database Operations with JSON Storage**

ตัวอย่างการทำงานกับฐานข้อมูล (จำลองด้วย JSON file)

**ฟีเจอร์หลัก:**
- 💾 Database connection และ operations
- 🔍 CRUD operations (Create, Read, Update, Delete)
- 🔎 Search และ filtering
- 📊 Statistics และ aggregations
- 💼 Transaction support
- 🧪 Comprehensive testing

**การรัน:**
```bash
cargo run --example database_example
```

**สิ่งที่เรียนรู้:**
- Database design patterns
- Transaction management
- Data persistence
- Query optimization concepts

---

### 4. 🏗️ Microservices Architecture (`microservices_example.rs`)
**Event-Driven Microservices System**

ตัวอย่างการสร้างระบบ microservices ที่สมบูรณ์

**ฟีเจอร์หลัก:**
- 📡 Event-driven architecture
- 🚌 Message bus สำหรับการสื่อสาร
- 🔍 Service discovery
- ⚡ Circuit breaker pattern
- 🔄 Asynchronous communication
- 🛡️ Fault tolerance
- 🏥 Health checking

**Services ที่รวมอยู่:**
- **User Service** - จัดการข้อมูลผู้ใช้
- **Order Service** - จัดการคำสั่งซื้อ
- **Payment Service** - จัดการการชำระเงิน
- **Notification Service** - จัดการการแจ้งเตือน

**การรัน:**
```bash
cargo run --example microservices_example
```

**สิ่งที่เรียนรู้:**
- Microservices patterns
- Event-driven design
- Service communication
- Fault tolerance strategies
- Distributed systems concepts

---

## 🛠️ Dependencies ที่ใช้

ตัวอย่างเหล่านี้ใช้ libraries ยอดนิยมในระบบนิเวศ Rust:

### Core Libraries
- **`anyhow`** - Error handling ที่ง่ายและยืดหยุ่น
- **`serde`** - Serialization/deserialization
- **`uuid`** - UUID generation
- **`chrono`** - Date และ time handling

### CLI Libraries
- **`clap`** - Command line argument parsing
- **`dirs`** - Directory operations
- **`tempfile`** - Temporary file handling

### Web Libraries
- **`axum`** - Modern web framework
- **`tokio`** - Async runtime
- **`tower`** - Service abstractions
- **`tower-http`** - HTTP middleware

### Development Libraries
- **`tracing-subscriber`** - Logging และ tracing

## 🧪 การทดสอบ

แต่ละตัวอย่างมี tests ที่ครอบคลุม:

```bash
# รัน tests ทั้งหมด
cargo test

# รัน tests สำหรับตัวอย่างเฉพาะ
cargo test --example real_world_cli
cargo test --example web_api_example
cargo test --example database_example
cargo test --example microservices_example

# รัน integration tests
cargo test --test integration_test
```

## 📚 แนวคิดที่ครอบคลุม

### 1. **Error Handling**
- `Result<T, E>` patterns
- `anyhow` สำหรับ error chaining
- Custom error types
- Error propagation

### 2. **Async Programming**
- `async`/`await` syntax
- `tokio` runtime
- Concurrent operations
- Channel communication

### 3. **Testing Strategies**
- Unit tests
- Integration tests
- Property-based testing
- Mocking และ test doubles

### 4. **Design Patterns**
- Repository pattern
- Service layer pattern
- Circuit breaker pattern
- Event-driven architecture

### 5. **Performance**
- Memory management
- Zero-copy operations
- Efficient data structures
- Benchmarking

## 🚀 การเริ่มต้น

1. **ติดตั้ง dependencies:**
   ```bash
   cargo build
   ```

2. **รันตัวอย่างที่สนใจ:**
   ```bash
   cargo run --example <example_name>
   ```

3. **ศึกษา source code:**
   - อ่าน comments ในโค้ด
   - ทำความเข้าใจ structure
   - ทดลองแก้ไขและรัน

4. **รัน tests:**
   ```bash
   cargo test
   ```

## 💡 เคล็ดลับการเรียนรู้

1. **เริ่มจากตัวอย่างง่ายๆ** - เริ่มจาก CLI example ก่อน
2. **อ่านโค้ดทีละส่วน** - ไม่ต้องเข้าใจทั้งหมดในครั้งเดียว
3. **ทดลองแก้ไข** - ลองเปลี่ยนค่าต่างๆ และดูผลลัพธ์
4. **รัน tests บ่อยๆ** - เพื่อให้แน่ใจว่าโค้ดยังทำงานได้
5. **อ่าน documentation** - ของ libraries ที่ใช้

## 🔗 แหล่งข้อมูลเพิ่มเติม

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Clap Documentation](https://docs.rs/clap/latest/clap/)

## 📝 หมายเหตุ

ตัวอย่างเหล่านี้ออกแบบมาเพื่อการเรียนรู้และสาธิต ในการใช้งานจริงควร:

- ใช้ฐานข้อมูลจริง (PostgreSQL, MySQL) แทน JSON files
- เพิ่ม authentication และ authorization
- ใช้ proper logging และ monitoring
- เพิ่ม rate limiting และ security measures
- ใช้ configuration management
- เพิ่ม comprehensive error handling

---

**Happy Coding! 🦀**