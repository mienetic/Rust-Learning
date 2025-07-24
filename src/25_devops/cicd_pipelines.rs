//! 🔄 CI/CD Pipelines - การสร้าง Continuous Integration/Deployment
//! 
//! โมดูลนี้สาธิตการสร้าง CI/CD pipelines สำหรับ Rust projects
//! รวมถึง GitHub Actions, GitLab CI, และ Jenkins

use std::collections::HashMap;
use std::fmt;

/// 🔄 ประเภทของ CI/CD Platform
#[derive(Debug, Clone, PartialEq)]
pub enum CicdPlatform {
    GitHubActions,
    GitLabCI,
    Jenkins,
    CircleCI,
    TravisCI,
}

impl fmt::Display for CicdPlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CicdPlatform::GitHubActions => write!(f, "GitHub Actions"),
            CicdPlatform::GitLabCI => write!(f, "GitLab CI"),
            CicdPlatform::Jenkins => write!(f, "Jenkins"),
            CicdPlatform::CircleCI => write!(f, "CircleCI"),
            CicdPlatform::TravisCI => write!(f, "Travis CI"),
        }
    }
}

/// 🏗️ โครงสร้าง Pipeline Stage
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub commands: Vec<String>,
    pub dependencies: Vec<String>,
    pub environment: HashMap<String, String>,
    pub artifacts: Vec<String>,
    pub cache_paths: Vec<String>,
}

impl PipelineStage {
    /// สร้าง Pipeline Stage ใหม่
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            commands: Vec::new(),
            dependencies: Vec::new(),
            environment: HashMap::new(),
            artifacts: Vec::new(),
            cache_paths: Vec::new(),
        }
    }
    
    /// เพิ่ม command
    pub fn add_command(mut self, command: &str) -> Self {
        self.commands.push(command.to_string());
        self
    }
    
    /// เพิ่ม dependency
    pub fn add_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }
    
    /// เพิ่ม environment variable
    pub fn add_env(mut self, key: &str, value: &str) -> Self {
        self.environment.insert(key.to_string(), value.to_string());
        self
    }
    
    /// เพิ่ม artifact
    pub fn add_artifact(mut self, artifact: &str) -> Self {
        self.artifacts.push(artifact.to_string());
        self
    }
    
    /// เพิ่ม cache path
    pub fn add_cache(mut self, path: &str) -> Self {
        self.cache_paths.push(path.to_string());
        self
    }
}

/// 🔄 Pipeline Configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub name: String,
    pub platform: CicdPlatform,
    pub triggers: Vec<String>,
    pub stages: Vec<PipelineStage>,
    pub global_env: HashMap<String, String>,
    pub rust_version: String,
    pub target_platforms: Vec<String>,
}

impl PipelineConfig {
    /// สร้าง Pipeline Config ใหม่
    pub fn new(name: &str, platform: CicdPlatform) -> Self {
        Self {
            name: name.to_string(),
            platform,
            triggers: vec!["push".to_string(), "pull_request".to_string()],
            stages: Vec::new(),
            global_env: HashMap::new(),
            rust_version: "1.75".to_string(),
            target_platforms: vec!["ubuntu-latest".to_string()],
        }
    }
    
    /// เพิ่ม stage
    pub fn add_stage(mut self, stage: PipelineStage) -> Self {
        self.stages.push(stage);
        self
    }
    
    /// เพิ่ม trigger
    pub fn add_trigger(mut self, trigger: &str) -> Self {
        self.triggers.push(trigger.to_string());
        self
    }
    
    /// เพิ่ม target platform
    pub fn add_target_platform(mut self, platform: &str) -> Self {
        self.target_platforms.push(platform.to_string());
        self
    }
    
    /// สร้าง pipeline configuration file
    pub fn generate_config(&self) -> String {
        match self.platform {
            CicdPlatform::GitHubActions => self.generate_github_actions(),
            CicdPlatform::GitLabCI => self.generate_gitlab_ci(),
            CicdPlatform::Jenkins => self.generate_jenkinsfile(),
            CicdPlatform::CircleCI => self.generate_circleci(),
            CicdPlatform::TravisCI => self.generate_travis_ci(),
        }
    }
    
    /// สร้าง GitHub Actions workflow
    fn generate_github_actions(&self) -> String {
        let mut yaml = String::new();
        
        yaml.push_str(&format!("name: {}\n\n", self.name));
        
        // Triggers
        yaml.push_str("on:\n");
        for trigger in &self.triggers {
            if trigger == "push" || trigger == "pull_request" {
                yaml.push_str(&format!("  {}:\n", trigger));
                yaml.push_str("    branches: [ main, develop ]\n");
            } else {
                yaml.push_str(&format!("  {}\n", trigger));
            }
        }
        yaml.push_str("\n");
        
        // Environment variables
        if !self.global_env.is_empty() {
            yaml.push_str("env:\n");
            for (key, value) in &self.global_env {
                yaml.push_str(&format!("  {}: {}\n", key, value));
            }
            yaml.push_str("\n");
        }
        
        yaml.push_str("jobs:\n");
        
        // Generate jobs for each stage
        for stage in &self.stages {
            yaml.push_str(&format!("  {}:\n", stage.name.replace(' ', "_").to_lowercase()));
            yaml.push_str(&format!("    name: {}\n", stage.name));
            
            // Platform matrix
            if self.target_platforms.len() > 1 {
                yaml.push_str("    strategy:\n");
                yaml.push_str("      matrix:\n");
                yaml.push_str("        os: [");
                yaml.push_str(&self.target_platforms.join(", "));
                yaml.push_str("]\n");
                yaml.push_str("    runs-on: ${{ matrix.os }}\n");
            } else {
                yaml.push_str(&format!("    runs-on: {}\n", self.target_platforms[0]));
            }
            
            // Dependencies
            if !stage.dependencies.is_empty() {
                yaml.push_str("    needs: [");
                yaml.push_str(&stage.dependencies.join(", "));
                yaml.push_str("]\n");
            }
            
            yaml.push_str("    steps:\n");
            
            // Checkout
            yaml.push_str("      - name: Checkout code\n");
            yaml.push_str("        uses: actions/checkout@v4\n\n");
            
            // Setup Rust
            yaml.push_str("      - name: Setup Rust\n");
            yaml.push_str("        uses: actions-rs/toolchain@v1\n");
            yaml.push_str("        with:\n");
            yaml.push_str(&format!("          toolchain: {}\n", self.rust_version));
            yaml.push_str("          override: true\n");
            yaml.push_str("          components: rustfmt, clippy\n\n");
            
            // Cache
            if !stage.cache_paths.is_empty() {
                yaml.push_str("      - name: Cache dependencies\n");
                yaml.push_str("        uses: actions/cache@v3\n");
                yaml.push_str("        with:\n");
                yaml.push_str("          path: |\n");
                for path in &stage.cache_paths {
                    yaml.push_str(&format!("            {}\n", path));
                }
                yaml.push_str("          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}\n\n");
            }
            
            // Commands
            for (i, command) in stage.commands.iter().enumerate() {
                yaml.push_str(&format!("      - name: {}\n", 
                    if stage.commands.len() == 1 {
                        stage.name.clone()
                    } else {
                        format!("{} - Step {}", stage.name, i + 1)
                    }
                ));
                yaml.push_str(&format!("        run: {}\n", command));
                
                // Environment variables for this step
                if !stage.environment.is_empty() {
                    yaml.push_str("        env:\n");
                    for (key, value) in &stage.environment {
                        yaml.push_str(&format!("          {}: {}\n", key, value));
                    }
                }
                yaml.push_str("\n");
            }
            
            // Artifacts
            if !stage.artifacts.is_empty() {
                yaml.push_str("      - name: Upload artifacts\n");
                yaml.push_str("        uses: actions/upload-artifact@v3\n");
                yaml.push_str("        with:\n");
                yaml.push_str(&format!("          name: {}-artifacts\n", stage.name.replace(' ', "-").to_lowercase()));
                yaml.push_str("          path: |\n");
                for artifact in &stage.artifacts {
                    yaml.push_str(&format!("            {}\n", artifact));
                }
                yaml.push_str("\n");
            }
        }
        
        yaml
    }
    
    /// สร้าง GitLab CI configuration
    fn generate_gitlab_ci(&self) -> String {
        let mut yaml = String::new();
        
        yaml.push_str("# GitLab CI/CD Pipeline\n");
        yaml.push_str(&format!("# Generated for: {}\n\n", self.name));
        
        // Image
        yaml.push_str(&format!("image: rust:{}\n\n", self.rust_version));
        
        // Variables
        if !self.global_env.is_empty() {
            yaml.push_str("variables:\n");
            for (key, value) in &self.global_env {
                yaml.push_str(&format!("  {}: \"{}\"\n", key, value));
            }
            yaml.push_str("\n");
        }
        
        // Cache
        yaml.push_str("cache:\n");
        yaml.push_str("  paths:\n");
        yaml.push_str("    - target/\n");
        yaml.push_str("    - ~/.cargo/\n\n");
        
        // Before script
        yaml.push_str("before_script:\n");
        yaml.push_str("  - apt-get update -qq && apt-get install -y -qq git\n");
        yaml.push_str("  - rustc --version\n");
        yaml.push_str("  - cargo --version\n\n");
        
        // Stages
        yaml.push_str("stages:\n");
        for stage in &self.stages {
            yaml.push_str(&format!("  - {}\n", stage.name.replace(' ', "_").to_lowercase()));
        }
        yaml.push_str("\n");
        
        // Jobs
        for stage in &self.stages {
            let job_name = stage.name.replace(' ', "_").to_lowercase();
            yaml.push_str(&format!("{}:\n", job_name));
            yaml.push_str(&format!("  stage: {}\n", job_name));
            
            if !stage.commands.is_empty() {
                yaml.push_str("  script:\n");
                for command in &stage.commands {
                    yaml.push_str(&format!("    - {}\n", command));
                }
            }
            
            if !stage.artifacts.is_empty() {
                yaml.push_str("  artifacts:\n");
                yaml.push_str("    paths:\n");
                for artifact in &stage.artifacts {
                    yaml.push_str(&format!("      - {}\n", artifact));
                }
                yaml.push_str("    expire_in: 1 week\n");
            }
            
            yaml.push_str("\n");
        }
        
        yaml
    }
    
    /// สร้าง Jenkinsfile
    fn generate_jenkinsfile(&self) -> String {
        let mut jenkinsfile = String::new();
        
        jenkinsfile.push_str("pipeline {\n");
        jenkinsfile.push_str("    agent any\n\n");
        
        // Environment
        if !self.global_env.is_empty() {
            jenkinsfile.push_str("    environment {\n");
            for (key, value) in &self.global_env {
                jenkinsfile.push_str(&format!("        {} = '{}'\n", key, value));
            }
            jenkinsfile.push_str("    }\n\n");
        }
        
        // Tools
        jenkinsfile.push_str("    tools {\n");
        jenkinsfile.push_str(&format!("        rust '{}'\n", self.rust_version));
        jenkinsfile.push_str("    }\n\n");
        
        // Stages
        jenkinsfile.push_str("    stages {\n");
        
        for stage in &self.stages {
            jenkinsfile.push_str(&format!("        stage('{}') {{\n", stage.name));
            jenkinsfile.push_str("            steps {\n");
            
            for command in &stage.commands {
                jenkinsfile.push_str(&format!("                sh '{}'\n", command));
            }
            
            jenkinsfile.push_str("            }\n");
            
            if !stage.artifacts.is_empty() {
                jenkinsfile.push_str("            post {\n");
                jenkinsfile.push_str("                always {\n");
                jenkinsfile.push_str("                    archiveArtifacts artifacts: '");
                jenkinsfile.push_str(&stage.artifacts.join(", "));
                jenkinsfile.push_str("', fingerprint: true\n");
                jenkinsfile.push_str("                }\n");
                jenkinsfile.push_str("            }\n");
            }
            
            jenkinsfile.push_str("        }\n\n");
        }
        
        jenkinsfile.push_str("    }\n");
        jenkinsfile.push_str("}\n");
        
        jenkinsfile
    }
    
    /// สร้าง CircleCI configuration
    fn generate_circleci(&self) -> String {
        let mut yaml = String::new();
        
        yaml.push_str("version: 2.1\n\n");
        
        // Executors
        yaml.push_str("executors:\n");
        yaml.push_str("  rust-executor:\n");
        yaml.push_str("    docker:\n");
        yaml.push_str(&format!("      - image: rust:{}\n\n", self.rust_version));
        
        // Jobs
        yaml.push_str("jobs:\n");
        
        for stage in &self.stages {
            let job_name = stage.name.replace(' ', "_").to_lowercase();
            yaml.push_str(&format!("  {}:\n", job_name));
            yaml.push_str("    executor: rust-executor\n");
            yaml.push_str("    steps:\n");
            yaml.push_str("      - checkout\n");
            
            // Cache
            if !stage.cache_paths.is_empty() {
                yaml.push_str("      - restore_cache:\n");
                yaml.push_str("          keys:\n");
                yaml.push_str("            - cargo-cache-{{ checksum \"Cargo.lock\" }}\n");
            }
            
            // Commands
            for command in &stage.commands {
                yaml.push_str(&format!("      - run: {}\n", command));
            }
            
            // Save cache
            if !stage.cache_paths.is_empty() {
                yaml.push_str("      - save_cache:\n");
                yaml.push_str("          key: cargo-cache-{{ checksum \"Cargo.lock\" }}\n");
                yaml.push_str("          paths:\n");
                for path in &stage.cache_paths {
                    yaml.push_str(&format!("            - {}\n", path));
                }
            }
            
            yaml.push_str("\n");
        }
        
        // Workflows
        yaml.push_str("workflows:\n");
        yaml.push_str("  version: 2\n");
        yaml.push_str(&format!("  {}:\n", self.name.replace(' ', "_").to_lowercase()));
        yaml.push_str("    jobs:\n");
        
        for stage in &self.stages {
            let job_name = stage.name.replace(' ', "_").to_lowercase();
            yaml.push_str(&format!("      - {}\n", job_name));
        }
        
        yaml
    }
    
    /// สร้าง Travis CI configuration
    fn generate_travis_ci(&self) -> String {
        let mut yaml = String::new();
        
        yaml.push_str("language: rust\n\n");
        
        // Rust versions
        yaml.push_str("rust:\n");
        yaml.push_str(&format!("  - {}\n\n", self.rust_version));
        
        // OS matrix
        if self.target_platforms.len() > 1 {
            yaml.push_str("os:\n");
            for platform in &self.target_platforms {
                let os = match platform.as_str() {
                    "ubuntu-latest" => "linux",
                    "macos-latest" => "osx",
                    "windows-latest" => "windows",
                    _ => "linux",
                };
                yaml.push_str(&format!("  - {}\n", os));
            }
            yaml.push_str("\n");
        }
        
        // Cache
        yaml.push_str("cache: cargo\n\n");
        
        // Environment
        if !self.global_env.is_empty() {
            yaml.push_str("env:\n");
            for (key, value) in &self.global_env {
                yaml.push_str(&format!("  - {}={}\n", key, value));
            }
            yaml.push_str("\n");
        }
        
        // Script
        yaml.push_str("script:\n");
        for stage in &self.stages {
            for command in &stage.commands {
                yaml.push_str(&format!("  - {}\n", command));
            }
        }
        
        yaml
    }
}

/// 🎯 สาธิตการทำงานกับ CI/CD Pipelines
pub fn demonstrate_cicd_pipelines() {
    println!("\n🔄 === CI/CD Pipelines Demo ===");
    
    // 1. การสร้าง Pipeline Stages
    println!("\n1️⃣ การสร้าง Pipeline Stages:");
    demonstrate_pipeline_stages();
    
    // 2. GitHub Actions Pipeline
    println!("\n2️⃣ GitHub Actions Pipeline:");
    demonstrate_github_actions();
    
    // 3. GitLab CI Pipeline
    println!("\n3️⃣ GitLab CI Pipeline:");
    demonstrate_gitlab_ci();
    
    // 4. การเปรียบเทียบ CI/CD Platforms
    println!("\n4️⃣ การเปรียบเทียบ CI/CD Platforms:");
    compare_cicd_platforms();
    
    // 5. Best Practices
    println!("\n5️⃣ CI/CD Best Practices:");
    show_cicd_best_practices();
    
    println!("\n✅ จบการสาธิต CI/CD Pipelines!");
}

/// 🏗️ สาธิต Pipeline Stages
fn demonstrate_pipeline_stages() {
    println!("🏗️ การสร้าง Pipeline Stages:");
    
    // Test stage
    let test_stage = PipelineStage::new("Test")
        .add_command("cargo fmt -- --check")
        .add_command("cargo clippy -- -D warnings")
        .add_command("cargo test")
        .add_cache("target/")
        .add_cache("~/.cargo/");
    
    // Build stage
    let build_stage = PipelineStage::new("Build")
        .add_command("cargo build --release")
        .add_dependency("test")
        .add_artifact("target/release/rust_concepts")
        .add_cache("target/");
    
    // Deploy stage
    let deploy_stage = PipelineStage::new("Deploy")
        .add_command("docker build -t rust-app:latest .")
        .add_command("docker push rust-app:latest")
        .add_dependency("build")
        .add_env("DOCKER_REGISTRY", "registry.example.com");
    
    let stages = vec![&test_stage, &build_stage, &deploy_stage];
    
    for stage in stages {
        println!("\n📋 Stage: {}", stage.name);
        println!("   Commands: {} ตัว", stage.commands.len());
        println!("   Dependencies: {:?}", stage.dependencies);
        println!("   Artifacts: {} ไฟล์", stage.artifacts.len());
        println!("   Cache paths: {} paths", stage.cache_paths.len());
    }
}

/// 🐙 สาธิต GitHub Actions
fn demonstrate_github_actions() {
    println!("🐙 GitHub Actions Pipeline:");
    
    let test_stage = PipelineStage::new("Test")
        .add_command("cargo fmt -- --check")
        .add_command("cargo clippy -- -D warnings")
        .add_command("cargo test")
        .add_cache("target/")
        .add_cache("~/.cargo/");
    
    let build_stage = PipelineStage::new("Build")
        .add_command("cargo build --release")
        .add_dependency("test")
        .add_artifact("target/release/rust_concepts");
    
    let config = PipelineConfig::new("Rust CI/CD", CicdPlatform::GitHubActions)
        .add_stage(test_stage)
        .add_stage(build_stage)
        .add_target_platform("ubuntu-latest")
        .add_target_platform("macos-latest");
    
    let workflow = config.generate_config();
    
    println!("📄 .github/workflows/ci.yml:");
    let lines: Vec<&str> = workflow.lines().take(20).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
    
    println!("\n🔧 การใช้งาน GitHub Actions:");
    println!("   • Workflow จะรันอัตโนมัติเมื่อ push หรือ PR");
    println!("   • รองรับ matrix builds สำหรับหลาย OS");
    println!("   • มี marketplace สำหรับ actions สำเร็จรูป");
    println!("   • รองรับ secrets management");
}

/// 🦊 สาธิต GitLab CI
fn demonstrate_gitlab_ci() {
    println!("🦊 GitLab CI Pipeline:");
    
    let test_stage = PipelineStage::new("test")
        .add_command("cargo fmt -- --check")
        .add_command("cargo clippy -- -D warnings")
        .add_command("cargo test");
    
    let build_stage = PipelineStage::new("build")
        .add_command("cargo build --release")
        .add_artifact("target/release/rust_concepts");
    
    let config = PipelineConfig::new("Rust Pipeline", CicdPlatform::GitLabCI)
        .add_stage(test_stage)
        .add_stage(build_stage);
    
    let pipeline = config.generate_config();
    
    println!("📄 .gitlab-ci.yml:");
    let lines: Vec<&str> = pipeline.lines().take(15).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
    
    println!("\n🔧 การใช้งาน GitLab CI:");
    println!("   • รองรับ Docker-in-Docker");
    println!("   • มี built-in container registry");
    println!("   • รองรับ parallel และ sequential jobs");
    println!("   • มี GitLab Pages สำหรับ static sites");
}

/// 📊 เปรียบเทียบ CI/CD Platforms
fn compare_cicd_platforms() {
    println!("📊 การเปรียบเทียบ CI/CD Platforms:");
    
    let platforms = vec![
        (
            CicdPlatform::GitHubActions,
            "ฟรีสำหรับ public repos",
            "ง่ายต่อการใช้งาน, marketplace ใหญ่",
            "จำกัด minutes สำหรับ private repos"
        ),
        (
            CicdPlatform::GitLabCI,
            "ฟรี 400 minutes/month",
            "รองรับ self-hosted runners",
            "UI ซับซ้อนกว่า GitHub Actions"
        ),
        (
            CicdPlatform::Jenkins,
            "ฟรี (self-hosted)",
            "ยืดหยุ่นสูง, plugins เยอะ",
            "ต้องจัดการ infrastructure เอง"
        ),
        (
            CicdPlatform::CircleCI,
            "ฟรี 6,000 minutes/month",
            "Performance ดี, Docker support",
            "ราคาแพงสำหรับ usage สูง"
        ),
    ];
    
    for (platform, pricing, pros, cons) in platforms {
        println!("\n🔧 {}:", platform);
        println!("   💰 ราคา: {}", pricing);
        println!("   ✅ ข้อดี: {}", pros);
        println!("   ❌ ข้อเสีย: {}", cons);
    }
    
    println!("\n🎯 การเลือก CI/CD Platform:");
    println!("   • GitHub Actions: สำหรับ GitHub projects");
    println!("   • GitLab CI: สำหรับ GitLab projects หรือต้องการ self-hosted");
    println!("   • Jenkins: สำหรับ enterprise ที่ต้องการ control เต็มที่");
    println!("   • CircleCI: สำหรับ performance-critical applications");
}

/// 📋 แสดง CI/CD Best Practices
fn show_cicd_best_practices() {
    println!("📋 CI/CD Best Practices:");
    
    let practices = vec![
        ("🚀", "Fast feedback", "ให้ feedback เร็วที่สุดเท่าที่จะทำได้"),
        ("🔄", "Fail fast", "หยุด pipeline ทันทีเมื่อมี error"),
        ("📦", "Artifact management", "เก็บ artifacts และ cache อย่างมีประสิทธิภาพ"),
        ("🔒", "Security scanning", "สแกน vulnerabilities ใน dependencies"),
        ("🧪", "Test automation", "รัน tests ทุกครั้งที่มีการเปลี่ยนแปลง"),
        ("📊", "Monitoring", "ติดตาม pipeline performance และ success rate"),
        ("🔑", "Secrets management", "จัดการ secrets อย่างปลอดภัย"),
        ("📝", "Documentation", "เขียน documentation สำหรับ pipeline"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎯 Rust-specific Best Practices:");
    println!("   • ใช้ cargo fmt และ clippy ใน CI");
    println!("   • Cache target/ และ ~/.cargo/ directories");
    println!("   • รัน tests ด้วย --all-features");
    println!("   • ตรวจสอบ security ด้วย cargo audit");
    println!("   • สร้าง release builds สำหรับ production");
    
    println!("\n🔧 การ Optimize CI/CD Performance:");
    println!("   • ใช้ parallel jobs เมื่อเป็นไปได้");
    println!("   • Cache dependencies อย่างมีประสิทธิภาพ");
    println!("   • ใช้ incremental compilation");
    println!("   • แยก fast tests และ slow tests");
    println!("   • ใช้ matrix builds สำหรับหลาย platform");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_stage() {
        let stage = PipelineStage::new("test")
            .add_command("cargo test")
            .add_dependency("build")
            .add_env("RUST_LOG", "debug")
            .add_artifact("test-results.xml")
            .add_cache("target/");
        
        assert_eq!(stage.name, "test");
        assert_eq!(stage.commands.len(), 1);
        assert_eq!(stage.dependencies.len(), 1);
        assert_eq!(stage.environment.len(), 1);
        assert_eq!(stage.artifacts.len(), 1);
        assert_eq!(stage.cache_paths.len(), 1);
    }
    
    #[test]
    fn test_pipeline_config() {
        let test_stage = PipelineStage::new("test")
            .add_command("cargo test");
        
        let config = PipelineConfig::new("Test Pipeline", CicdPlatform::GitHubActions)
            .add_stage(test_stage)
            .add_trigger("schedule")
            .add_target_platform("windows-latest");
        
        assert_eq!(config.name, "Test Pipeline");
        assert_eq!(config.platform, CicdPlatform::GitHubActions);
        assert_eq!(config.stages.len(), 1);
        assert_eq!(config.triggers.len(), 3); // push, pull_request, schedule
        assert_eq!(config.target_platforms.len(), 2); // ubuntu-latest, windows-latest
    }
    
    #[test]
    fn test_github_actions_generation() {
        let stage = PipelineStage::new("test")
            .add_command("cargo test");
        
        let config = PipelineConfig::new("Test", CicdPlatform::GitHubActions)
            .add_stage(stage);
        
        let workflow = config.generate_config();
        
        assert!(workflow.contains("name: Test"));
        assert!(workflow.contains("on:"));
        assert!(workflow.contains("jobs:"));
        assert!(workflow.contains("cargo test"));
    }
}