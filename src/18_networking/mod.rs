//! 🌐 Network Programming Workshop - เวิร์คช็อปการเขียนโปรแกรมเครือข่าย
//!
//! 🚀 บทเรียนเกี่ยวกับการเขียนโปรแกรมเครือข่ายใน Rust สำหรับเวิร์คช็อปพัฒนาเว็บ
//! 📡 ครอบคลุม TCP/UDP, HTTP, WebSocket, และ protocols ต่างๆ

pub mod tcp_server;
pub mod udp_communication;
// pub mod http_client_server;
// pub mod websocket_communication;
// pub mod async_networking;
// pub mod protocol_implementation;
// pub mod network_security;
// pub mod load_balancing;
// pub mod network_monitoring;
// pub mod custom_protocols;

/// 🎯 รันตัวอย่าง Network Programming ทั้งหมดในเวิร์คช็อป
pub fn run_networking_examples() {
    println!("🌐 === Network Programming Workshop in Rust === 🌐");
    println!();

    // 🔌 TCP Server/Client Workshop
    println!("🔌 TCP Server/Client Workshop:");
    tcp_server::demonstrate_tcp();
    
    println!();
    
    // 📡 UDP Communication Workshop
    println!("📡 UDP Communication Workshop:");
    udp_communication::demonstrate_udp();
    
    println!();
    
    // TODO: Implement remaining networking modules
    // HTTP Client/Server
    // println!("🌍 HTTP Client/Server:");
    // http_client_server::demonstrate_http();
    
    // WebSocket Communication
    // println!("🔗 WebSocket Communication:");
    // websocket_communication::demonstrate_websocket();
    
    // Async Networking
    // println!("⚡ Async Networking:");
    // async_networking::demonstrate_async_networking();
    
    // Protocol Implementation
    // println!("📋 Protocol Implementation:");
    // protocol_implementation::demonstrate_protocols();
    
    // Network Security
    // println!("🔒 Network Security:");
    // network_security::demonstrate_security();
    
    // Load Balancing
    // println!("⚖️ Load Balancing:");
    // load_balancing::demonstrate_load_balancing();
    
    // Network Monitoring
    // println!("📊 Network Monitoring:");
    // network_monitoring::demonstrate_monitoring();
    
    // Custom Protocols
    // println!("🛠️ Custom Protocols:");
    // custom_protocols::demonstrate_custom_protocols();
    
    println!("\n✅ Network Programming Workshop examples completed!");
}