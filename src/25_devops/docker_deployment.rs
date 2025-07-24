//! 🐳 Docker Deployment - การ Deploy ด้วย Docker
//! 
//! โมดูลนี้สาธิตการสร้าง Docker images และ containers
//! สำหรับ Rust applications

use std::collections::HashMap;
use std::fmt;

/// 🐳 โครงสร้าง Docker Configuration
#[derive(Debug, Clone)]
pub struct DockerConfig {
    pub image_name: String,
    pub tag: String,
    pub base_image: String,
    pub working_dir: String,
    pub exposed_ports: Vec<u16>,
    pub environment_vars: HashMap<String, String>,
    pub volumes: Vec<String>,
}

impl DockerConfig {
    /// สร้าง Docker config ใหม่
    pub fn new(image_name: &str, tag: &str) -> Self {
        Self {
            image_name: image_name.to_string(),
            tag: tag.to_string(),
            base_image: "rust:1.75-slim".to_string(),
            working_dir: "/app".to_string(),
            exposed_ports: vec![8080],
            environment_vars: HashMap::new(),
            volumes: Vec::new(),
        }
    }
    
    /// เพิ่ม environment variable
    pub fn add_env(mut self, key: &str, value: &str) -> Self {
        self.environment_vars.insert(key.to_string(), value.to_string());
        self
    }
    
    /// เพิ่ม port
    pub fn add_port(mut self, port: u16) -> Self {
        self.exposed_ports.push(port);
        self
    }
    
    /// เพิ่ม volume
    pub fn add_volume(mut self, volume: &str) -> Self {
        self.volumes.push(volume.to_string());
        self
    }
    
    /// สร้าง Dockerfile
    pub fn generate_dockerfile(&self) -> String {
        let mut dockerfile = String::new();
        
        // Base image
        dockerfile.push_str(&format!("FROM {}\n\n", self.base_image));
        
        // Working directory
        dockerfile.push_str(&format!("WORKDIR {}\n\n", self.working_dir));
        
        // Install dependencies
        dockerfile.push_str("# Install dependencies\n");
        dockerfile.push_str("RUN apt-get update && apt-get install -y \\\n");
        dockerfile.push_str("    pkg-config \\\n");
        dockerfile.push_str("    libssl-dev \\\n");
        dockerfile.push_str("    && rm -rf /var/lib/apt/lists/*\n\n");
        
        // Copy Cargo files
        dockerfile.push_str("# Copy Cargo files\n");
        dockerfile.push_str("COPY Cargo.toml Cargo.lock ./\n\n");
        
        // Create dummy main.rs for dependency caching
        dockerfile.push_str("# Create dummy main.rs for dependency caching\n");
        dockerfile.push_str("RUN mkdir src && echo 'fn main() {}' > src/main.rs\n");
        dockerfile.push_str("RUN cargo build --release && rm -rf src\n\n");
        
        // Copy source code
        dockerfile.push_str("# Copy source code\n");
        dockerfile.push_str("COPY src ./src\n\n");
        
        // Build application
        dockerfile.push_str("# Build application\n");
        dockerfile.push_str("RUN cargo build --release\n\n");
        
        // Environment variables
        if !self.environment_vars.is_empty() {
            dockerfile.push_str("# Environment variables\n");
            for (key, value) in &self.environment_vars {
                dockerfile.push_str(&format!("ENV {}={}\n", key, value));
            }
            dockerfile.push_str("\n");
        }
        
        // Expose ports
        if !self.exposed_ports.is_empty() {
            dockerfile.push_str("# Expose ports\n");
            for port in &self.exposed_ports {
                dockerfile.push_str(&format!("EXPOSE {}\n", port));
            }
            dockerfile.push_str("\n");
        }
        
        // Command
        dockerfile.push_str("# Run application\n");
        dockerfile.push_str("CMD [\"./target/release/rust_concepts\"]\n");
        
        dockerfile
    }
    
    /// สร้าง docker-compose.yml
    pub fn generate_docker_compose(&self) -> String {
        let mut compose = String::new();
        
        compose.push_str("version: '3.8'\n\n");
        compose.push_str("services:\n");
        compose.push_str(&format!("  {}:\n", self.image_name.replace(':', "_")));
        compose.push_str(&format!("    image: {}:{}\n", self.image_name, self.tag));
        compose.push_str("    build: .\n");
        
        // Ports
        if !self.exposed_ports.is_empty() {
            compose.push_str("    ports:\n");
            for port in &self.exposed_ports {
                compose.push_str(&format!("      - \"{}:{}\"\n", port, port));
            }
        }
        
        // Environment variables
        if !self.environment_vars.is_empty() {
            compose.push_str("    environment:\n");
            for (key, value) in &self.environment_vars {
                compose.push_str(&format!("      {}: {}\n", key, value));
            }
        }
        
        // Volumes
        if !self.volumes.is_empty() {
            compose.push_str("    volumes:\n");
            for volume in &self.volumes {
                compose.push_str(&format!("      - {}\n", volume));
            }
        }
        
        compose.push_str("    restart: unless-stopped\n");
        
        compose
    }
}

/// 🏗️ โครงสร้าง Build Strategy
#[derive(Debug, Clone)]
pub enum BuildStrategy {
    SingleStage,
    MultiStage,
    Distroless,
}

impl fmt::Display for BuildStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildStrategy::SingleStage => write!(f, "Single Stage"),
            BuildStrategy::MultiStage => write!(f, "Multi Stage"),
            BuildStrategy::Distroless => write!(f, "Distroless"),
        }
    }
}

/// 🏗️ Docker Builder
#[derive(Debug)]
pub struct DockerBuilder {
    config: DockerConfig,
    strategy: BuildStrategy,
}

impl DockerBuilder {
    /// สร้าง Docker Builder ใหม่
    pub fn new(config: DockerConfig, strategy: BuildStrategy) -> Self {
        Self { config, strategy }
    }
    
    /// สร้าง Dockerfile ตาม strategy
    pub fn generate_optimized_dockerfile(&self) -> String {
        match self.strategy {
            BuildStrategy::SingleStage => self.config.generate_dockerfile(),
            BuildStrategy::MultiStage => self.generate_multistage_dockerfile(),
            BuildStrategy::Distroless => self.generate_distroless_dockerfile(),
        }
    }
    
    /// สร้าง Multi-stage Dockerfile
    fn generate_multistage_dockerfile(&self) -> String {
        let mut dockerfile = String::new();
        
        // Build stage
        dockerfile.push_str("# Build stage\n");
        dockerfile.push_str("FROM rust:1.75 as builder\n\n");
        dockerfile.push_str(&format!("WORKDIR {}\n\n", self.config.working_dir));
        
        // Copy and build
        dockerfile.push_str("# Copy Cargo files\n");
        dockerfile.push_str("COPY Cargo.toml Cargo.lock ./\n");
        dockerfile.push_str("COPY src ./src\n\n");
        
        dockerfile.push_str("# Build application\n");
        dockerfile.push_str("RUN cargo build --release\n\n");
        
        // Runtime stage
        dockerfile.push_str("# Runtime stage\n");
        dockerfile.push_str("FROM debian:bookworm-slim\n\n");
        
        // Install runtime dependencies
        dockerfile.push_str("RUN apt-get update && apt-get install -y \\\n");
        dockerfile.push_str("    ca-certificates \\\n");
        dockerfile.push_str("    libssl3 \\\n");
        dockerfile.push_str("    && rm -rf /var/lib/apt/lists/*\n\n");
        
        dockerfile.push_str(&format!("WORKDIR {}\n\n", self.config.working_dir));
        
        // Copy binary from builder
        dockerfile.push_str("# Copy binary from builder\n");
        dockerfile.push_str(&format!(
            "COPY --from=builder {}/target/release/rust_concepts ./\n\n",
            self.config.working_dir
        ));
        
        // Environment variables
        if !self.config.environment_vars.is_empty() {
            dockerfile.push_str("# Environment variables\n");
            for (key, value) in &self.config.environment_vars {
                dockerfile.push_str(&format!("ENV {}={}\n", key, value));
            }
            dockerfile.push_str("\n");
        }
        
        // Expose ports
        for port in &self.config.exposed_ports {
            dockerfile.push_str(&format!("EXPOSE {}\n", port));
        }
        
        dockerfile.push_str("\nCMD [\"./rust_concepts\"]\n");
        
        dockerfile
    }
    
    /// สร้าง Distroless Dockerfile
    fn generate_distroless_dockerfile(&self) -> String {
        let mut dockerfile = String::new();
        
        // Build stage
        dockerfile.push_str("# Build stage\n");
        dockerfile.push_str("FROM rust:1.75 as builder\n\n");
        dockerfile.push_str(&format!("WORKDIR {}\n\n", self.config.working_dir));
        
        dockerfile.push_str("COPY Cargo.toml Cargo.lock ./\n");
        dockerfile.push_str("COPY src ./src\n\n");
        
        dockerfile.push_str("# Build static binary\n");
        dockerfile.push_str("RUN cargo build --release --target x86_64-unknown-linux-musl\n\n");
        
        // Runtime stage with distroless
        dockerfile.push_str("# Runtime stage\n");
        dockerfile.push_str("FROM gcr.io/distroless/static\n\n");
        
        dockerfile.push_str(&format!("WORKDIR {}\n\n", self.config.working_dir));
        
        // Copy static binary
        dockerfile.push_str("COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_concepts ./\n\n");
        
        // Environment variables
        for (key, value) in &self.config.environment_vars {
            dockerfile.push_str(&format!("ENV {}={}\n", key, value));
        }
        
        // Expose ports
        for port in &self.config.exposed_ports {
            dockerfile.push_str(&format!("EXPOSE {}\n", port));
        }
        
        dockerfile.push_str("\nENTRYPOINT [\"./rust_concepts\"]\n");
        
        dockerfile
    }
    
    /// สร้าง .dockerignore
    pub fn generate_dockerignore(&self) -> String {
        let mut dockerignore = String::new();
        
        dockerignore.push_str("# Rust\n");
        dockerignore.push_str("target/\n");
        dockerignore.push_str("Cargo.lock\n");
        dockerignore.push_str("\n");
        
        dockerignore.push_str("# IDE\n");
        dockerignore.push_str(".vscode/\n");
        dockerignore.push_str(".idea/\n");
        dockerignore.push_str("*.swp\n");
        dockerignore.push_str("*.swo\n");
        dockerignore.push_str("\n");
        
        dockerignore.push_str("# Git\n");
        dockerignore.push_str(".git/\n");
        dockerignore.push_str(".gitignore\n");
        dockerignore.push_str("\n");
        
        dockerignore.push_str("# Documentation\n");
        dockerignore.push_str("README.md\n");
        dockerignore.push_str("docs/\n");
        dockerignore.push_str("\n");
        
        dockerignore.push_str("# Docker\n");
        dockerignore.push_str("Dockerfile*\n");
        dockerignore.push_str("docker-compose*\n");
        dockerignore.push_str(".dockerignore\n");
        
        dockerignore
    }
}

/// 🎯 สาธิตการทำงานกับ Docker Deployment
pub fn demonstrate_docker_deployment() {
    println!("\n🐳 === Docker Deployment Demo ===");
    
    // 1. การสร้าง Docker Configuration
    println!("\n1️⃣ การสร้าง Docker Configuration:");
    demonstrate_docker_config();
    
    // 2. Build Strategies
    println!("\n2️⃣ Docker Build Strategies:");
    demonstrate_build_strategies();
    
    // 3. Docker Compose
    println!("\n3️⃣ Docker Compose Configuration:");
    demonstrate_docker_compose();
    
    // 4. Best Practices
    println!("\n4️⃣ Docker Best Practices:");
    show_docker_best_practices();
    
    println!("\n✅ จบการสาธิต Docker Deployment!");
}

/// 🔧 สาธิต Docker Configuration
fn demonstrate_docker_config() {
    println!("🔧 การสร้าง Docker Configuration:");
    
    let config = DockerConfig::new("rust-app", "latest")
        .add_env("RUST_LOG", "info")
        .add_env("DATABASE_URL", "postgres://user:pass@db:5432/myapp")
        .add_port(8080)
        .add_port(9090)
        .add_volume("./data:/app/data");
    
    println!("📋 Docker Configuration:");
    println!("   Image: {}:{}", config.image_name, config.tag);
    println!("   Base: {}", config.base_image);
    println!("   Ports: {:?}", config.exposed_ports);
    println!("   Environment Variables: {} ตัว", config.environment_vars.len());
    
    // แสดง Dockerfile
    println!("\n📄 Generated Dockerfile:");
    let dockerfile = config.generate_dockerfile();
    let lines: Vec<&str> = dockerfile.lines().take(10).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
}

/// 🏗️ สาธิต Build Strategies
fn demonstrate_build_strategies() {
    println!("🏗️ Docker Build Strategies:");
    
    let config = DockerConfig::new("rust-app", "latest")
        .add_env("RUST_LOG", "info");
    
    let strategies = vec![
        BuildStrategy::SingleStage,
        BuildStrategy::MultiStage,
        BuildStrategy::Distroless,
    ];
    
    for strategy in strategies {
        println!("\n📦 {} Build:", strategy);
        let builder = DockerBuilder::new(config.clone(), strategy.clone());
        
        match strategy {
            BuildStrategy::SingleStage => {
                println!("   ✅ ง่ายต่อการ debug");
                println!("   ❌ Image size ใหญ่");
                println!("   📏 ขนาดประมาณ: ~1.5GB");
            },
            BuildStrategy::MultiStage => {
                println!("   ✅ Image size เล็กกว่า");
                println!("   ✅ แยก build และ runtime");
                println!("   📏 ขนาดประมาณ: ~100MB");
            },
            BuildStrategy::Distroless => {
                println!("   ✅ Image size เล็กที่สุด");
                println!("   ✅ ปลอดภัยที่สุด");
                println!("   📏 ขนาดประมาณ: ~20MB");
            },
        }
        
        let dockerfile = builder.generate_optimized_dockerfile();
        let line_count = dockerfile.lines().count();
        println!("   📄 Dockerfile: {} บรรทัด", line_count);
    }
}

/// 🐙 สาธิต Docker Compose
fn demonstrate_docker_compose() {
    println!("🐙 Docker Compose Configuration:");
    
    let config = DockerConfig::new("rust-web-app", "v1.0.0")
        .add_env("DATABASE_URL", "postgres://user:password@postgres:5432/myapp")
        .add_env("REDIS_URL", "redis://redis:6379")
        .add_port(8080);
    
    let compose = config.generate_docker_compose();
    
    println!("📄 docker-compose.yml:");
    let lines: Vec<&str> = compose.lines().take(15).collect();
    for line in lines {
        println!("   {}", line);
    }
    
    println!("\n🔧 การใช้งาน Docker Compose:");
    println!("   docker-compose up -d          # รันในโหมด detached");
    println!("   docker-compose logs -f        # ดู logs แบบ real-time");
    println!("   docker-compose down           # หยุดและลบ containers");
    println!("   docker-compose build --no-cache # build ใหม่");
}

/// 📋 แสดง Docker Best Practices
fn show_docker_best_practices() {
    println!("📋 Docker Best Practices:");
    
    let practices = vec![
        ("🎯", "Multi-stage builds", "ใช้ multi-stage เพื่อลด image size"),
        ("📦", "Layer caching", "เรียง COPY commands ให้ cache ได้ดี"),
        ("🔒", "Non-root user", "รันแอปด้วย non-root user"),
        ("🏷️", "Specific tags", "ใช้ specific version tags แทน 'latest'"),
        ("🧹", ".dockerignore", "ใช้ .dockerignore เพื่อลด build context"),
        ("💾", "Health checks", "เพิ่ม health checks สำหรับ containers"),
        ("🔍", "Security scanning", "สแกน vulnerabilities ใน images"),
        ("📊", "Resource limits", "ตั้งค่า memory และ CPU limits"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎯 การ Optimize Docker Images:");
    println!("   • ใช้ Alpine หรือ Distroless base images");
    println!("   • ลบ build dependencies ใน runtime stage");
    println!("   • ใช้ static linking สำหรับ Rust binaries");
    println!("   • Compress layers ด้วย --squash");
    
    println!("\n🔧 Docker Commands ที่มีประโยชน์:");
    println!("   docker system prune -a        # ลบ unused images/containers");
    println!("   docker image ls --format table # แสดง images ในรูปแบบตาราง");
    println!("   docker stats                   # ดู resource usage");
    println!("   docker exec -it <container> sh # เข้าไปใน container");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_docker_config() {
        let config = DockerConfig::new("test-app", "v1.0.0")
            .add_env("TEST_ENV", "test_value")
            .add_port(3000);
        
        assert_eq!(config.image_name, "test-app");
        assert_eq!(config.tag, "v1.0.0");
        assert!(config.exposed_ports.contains(&3000));
        assert_eq!(config.environment_vars.get("TEST_ENV"), Some(&"test_value".to_string()));
    }
    
    #[test]
    fn test_dockerfile_generation() {
        let config = DockerConfig::new("test-app", "latest");
        let dockerfile = config.generate_dockerfile();
        
        assert!(dockerfile.contains("FROM rust:1.75-slim"));
        assert!(dockerfile.contains("WORKDIR /app"));
        assert!(dockerfile.contains("EXPOSE 8080"));
        assert!(dockerfile.contains("CMD"));
    }
    
    #[test]
    fn test_docker_builder() {
        let config = DockerConfig::new("test-app", "latest");
        let builder = DockerBuilder::new(config, BuildStrategy::MultiStage);
        
        let dockerfile = builder.generate_optimized_dockerfile();
        assert!(dockerfile.contains("FROM rust:1.75 as builder"));
        assert!(dockerfile.contains("FROM debian:bookworm-slim"));
    }
}