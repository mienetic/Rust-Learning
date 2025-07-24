# 🚀 คู่มือการสร้างโปรเจกต์ Rust ที่ใช้งานจริง

## 📋 ขั้นตอนการสร้างโปรเจกต์

### 1. การเริ่มต้นโปรเจกต์

```bash
# สร้างโปรเจกต์ใหม่
cargo new my_project --bin  # สำหรับ executable
cargo new my_lib --lib      # สำหรับ library

# หรือสร้างในโฟลเดอร์ปัจจุบัน
cargo init
```

### 2. โครงสร้างโปรเจกต์ที่แนะนำ

```
my_project/
├── Cargo.toml              # ไฟล์ configuration หลัก
├── Cargo.lock              # Lock file สำหรับ dependencies
├── README.md               # คำอธิบายโปรเจกต์
├── .gitignore              # ไฟล์ที่ไม่ต้องการใน git
├── src/                    # โค้ดหลัก
│   ├── main.rs            # Entry point สำหรับ binary
│   ├── lib.rs             # Entry point สำหรับ library
│   ├── config/            # การตั้งค่า
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── models/            # Data structures
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── services/          # Business logic
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   └── product_service.rs
│   ├── handlers/          # HTTP handlers (สำหรับ web app)
│   │   ├── mod.rs
│   │   └── api.rs
│   ├── database/          # Database operations
│   │   ├── mod.rs
│   │   └── connection.rs
│   └── utils/             # Utility functions
│       ├── mod.rs
│       └── helpers.rs
├── tests/                  # Integration tests
│   ├── common/
│   │   └── mod.rs
│   ├── integration_test.rs
│   └── api_test.rs
├── benches/               # Benchmarks
│   └── performance.rs
├── examples/              # ตัวอย่างการใช้งาน
│   └── basic_usage.rs
├── docs/                  # เอกสาร
│   └── api.md
└── scripts/               # Scripts สำหรับ automation
    ├── build.sh
    └── deploy.sh
```

## ⚙️ การตั้งค่า Cargo.toml

### Cargo.toml พื้นฐาน

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"
rust-version = "1.88.0"
authors = ["Your Name <your.email@example.com>"]
description = "A brief description of your project"
license = "MIT OR Apache-2.0"
readme = "README.md"
homepage = "https://github.com/yourusername/my_project"
repository = "https://github.com/yourusername/my_project"
keywords = ["rust", "cli", "tool"]
categories = ["command-line-utilities"]

[dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

# Database (เลือกตามความต้องการ)
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres"] }
# หรือ
diesel = { version = "2.0", features = ["postgres"] }

# Web framework (เลือกตามความต้องการ)
axum = "0.7"
# หรือ
actix-web = "4.0"
# หรือ
warp = "0.3"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Configuration
config = "0.14"
dotenv = "0.15"

# Date and time
chrono = { version = "0.4", features = ["serde"] }

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"
proptest = "1.0"
mockall = "0.12"
wiremock = "0.6"

[build-dependencies]
vergen = "8.0"

# Profile สำหรับ optimization
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = 0
debug = true

# Workspace (สำหรับ multi-crate project)
[workspace]
members = [
    "core",
    "api",
    "cli",
]

[[bin]]
name = "my_project"
path = "src/main.rs"

[[example]]
name = "basic_usage"
path = "examples/basic_usage.rs"

[[bench]]
name = "performance"
harness = false
```

## 🧪 การทดสอบ

### 1. Unit Tests

```rust
// src/models/user.rs
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(User { id, name, email })
        } else {
            Err("Invalid email format".to_string())
        }
    }
    
    pub fn is_valid_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_success() {
        let user = User::new(1, "John".to_string(), "john@example.com".to_string());
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let user = User::new(1, "John".to_string(), "invalid-email".to_string());
        assert!(user.is_err());
        assert_eq!(user.unwrap_err(), "Invalid email format");
    }

    #[test]
    fn test_email_validation() {
        let user = User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        };
        assert!(user.is_valid_email());
        
        let invalid_user = User {
            id: 2,
            name: "Jane".to_string(),
            email: "jane@invalid".to_string(),
        };
        assert!(!invalid_user.is_valid_email());
    }
}
```

### 2. Integration Tests

```rust
// tests/integration_test.rs
use my_project::models::User;
use my_project::services::UserService;

#[tokio::test]
async fn test_user_service_integration() {
    let service = UserService::new();
    
    // Test creating user
    let user = service.create_user("John".to_string(), "john@example.com".to_string()).await;
    assert!(user.is_ok());
    
    let user = user.unwrap();
    assert_eq!(user.name, "John");
    
    // Test finding user
    let found_user = service.find_user_by_id(user.id).await;
    assert!(found_user.is_ok());
    assert_eq!(found_user.unwrap().id, user.id);
}

#[tokio::test]
async fn test_user_service_error_handling() {
    let service = UserService::new();
    
    // Test invalid email
    let result = service.create_user("Invalid".to_string(), "invalid-email".to_string()).await;
    assert!(result.is_err());
    
    // Test non-existent user
    let result = service.find_user_by_id(99999).await;
    assert!(result.is_err());
}
```

### 3. Property-based Testing

```rust
// tests/property_test.rs
use proptest::prelude::*;
use my_project::models::User;

proptest! {
    #[test]
    fn test_user_email_always_contains_at(
        id in 1u32..1000,
        name in "[a-zA-Z]{1,20}",
        local in "[a-zA-Z0-9]{1,10}",
        domain in "[a-zA-Z]{1,10}",
        tld in "[a-zA-Z]{2,4}"
    ) {
        let email = format!("{}@{}.{}", local, domain, tld);
        let user = User::new(id, name, email).unwrap();
        prop_assert!(user.is_valid_email());
    }
}
```

### 4. Benchmark Tests

```rust
// benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_project::models::User;
use my_project::services::UserService;

fn benchmark_user_creation(c: &mut Criterion) {
    c.bench_function("user_creation", |b| {
        b.iter(|| {
            User::new(
                black_box(1),
                black_box("John".to_string()),
                black_box("john@example.com".to_string())
            )
        })
    });
}

fn benchmark_email_validation(c: &mut Criterion) {
    let user = User {
        id: 1,
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    };
    
    c.bench_function("email_validation", |b| {
        b.iter(|| {
            black_box(&user).is_valid_email()
        })
    });
}

criterion_group!(benches, benchmark_user_creation, benchmark_email_validation);
criterion_main!(benches);
```

## 🔧 การตั้งค่าเพิ่มเติม

### 1. Logging

```rust
// src/main.rs
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting application");
    
    // Your application logic here
    
    Ok(())
}
```

### 2. Configuration

```rust
// src/config/settings.rs
use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        
        // Start with default configuration
        s.merge(File::with_name("config/default"))?;
        
        // Add environment-specific settings
        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        
        // Add local configuration (gitignored)
        s.merge(File::with_name("config/local").required(false))?;
        
        // Override with environment variables
        s.merge(Environment::with_prefix("APP"))?;
        
        s.try_into()
    }
}
```

### 3. Error Handling

```rust
// src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Internal server error")]
    Internal,
}

pub type Result<T> = std::result::Result<T, AppError>;
```

## 📝 คำสั่งที่ใช้บ่อย

```bash
# การพัฒนา
cargo check          # ตรวจสอบ syntax
cargo build          # Build โปรเจกต์
cargo run            # รันโปรเจกต์
cargo test           # รันทดสอบ
cargo bench          # รัน benchmarks
cargo doc --open     # สร้างและเปิด documentation

# การทดสอบ
cargo test --lib                    # รัน unit tests
cargo test --test integration_test  # รัน integration test เฉพาะ
cargo test -- --nocapture          # แสดง println! ในการทดสอบ
cargo test --release               # รันทดสอบใน release mode

# Code quality
cargo clippy                       # Linting
cargo fmt                          # Format code
cargo audit                        # ตรวจสอบ security vulnerabilities

# การ deploy
cargo build --release              # Build สำหรับ production
cargo install --path .             # Install binary locally
```

## 🚀 Best Practices

### 1. Code Organization
- แยก business logic ออกจาก presentation layer
- ใช้ modules เพื่อจัดกลุ่มโค้ดที่เกี่ยวข้อง
- ใช้ traits สำหรับ abstraction
- ใช้ Result type สำหรับ error handling

### 2. Testing Strategy
- เขียน unit tests สำหรับ business logic
- เขียน integration tests สำหรับ API endpoints
- ใช้ property-based testing สำหรับ complex logic
- Mock external dependencies

### 3. Performance
- ใช้ `cargo bench` เพื่อวัดประสิทธิภาพ
- Profile โค้ดด้วย `cargo flamegraph`
- ใช้ `cargo bloat` เพื่อตรวจสอบขนาดไฟล์

### 4. Security
- ใช้ `cargo audit` เป็นประจำ
- Validate input data
- ใช้ secure defaults
- ไม่ commit secrets ลง git

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

คู่มือนี้ครอบคลุมการสร้างโปรเจกต์ Rust ที่พร้อมใช้งานจริง รวมถึงการตั้งค่า การทดสอบ และ best practices ที่สำคัญ! 🦀