//! 🚀 บทที่ 25: DevOps และ Deployment
//! 
//! บทนี้จะสอนการ Deploy และจัดการ Rust applications
//! ครอบคลุม Docker, CI/CD, Monitoring และ Infrastructure
//! 
//! ## 📚 เนื้อหาในบทนี้:
//! - Docker containerization
//! - CI/CD pipelines
//! - Monitoring และ Logging
//! - Performance optimization
//! - Infrastructure as Code
//! - Security best practices
//! - Load balancing และ Scaling

pub mod docker_deployment;
pub mod cicd_pipelines;
pub mod monitoring_observability;
pub mod performance_optimization;
pub mod infrastructure_as_code;

/// 🎯 ฟังก์ชันหลักสำหรับการเรียนรู้ DevOps
pub fn learn_devops() {
    println!("\n{}", "=".repeat(50));
    println!("🚀 DevOps และ Deployment");
    println!("{}", "=".repeat(50));
    
    println!("📖 ยินดีต้อนรับสู่บทเรียน DevOps!");
    println!("🎯 ในบทนี้เราจะเรียนรู้:");
    println!("   • การ Containerize Rust applications");
    println!("   • การสร้าง CI/CD pipelines");
    println!("   • การ Monitor และ Log applications");
    println!("   • การ Optimize performance");
    println!("   • การจัดการ Infrastructure");
    println!();
    
    // เรียกใช้งานตัวอย่างต่างๆ
    docker_deployment::demonstrate_docker_deployment();
    cicd_pipelines::demonstrate_cicd_pipelines();
    monitoring_observability::demonstrate_monitoring_observability();
    performance_optimization::demonstrate_performance_optimization();
    infrastructure_as_code::demonstrate_infrastructure_as_code();
    
    println!("✅ จบบทเรียน DevOps และ Deployment!");
    println!("🎉 ตอนนี้คุณสามารถ Deploy Rust applications ได้แล้ว!");
}

/// 🧪 ฟังก์ชันสำหรับรันตัวอย่าง DevOps
pub fn run_devops_examples() {
    println!("🚀 กำลังรันตัวอย่าง DevOps...");
    learn_devops();
}