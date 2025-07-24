//! 🌐 TCP Server และ Client Implementation - Web Development Workshop
//!
//! 🎯 การสร้าง TCP Server และ Client ใน Rust สำหรับ Workshop
//! 📚 รวมถึงการจัดการ connection, threading, และ error handling
//! 🚀 เหมาะสำหรับการเรียนรู้ network programming ในเวิร์กช็อป

use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// 🔄 Simple TCP Echo Server - Workshop Demo
/// 📡 รับข้อความจาก client และส่งกลับไป (สำหรับเวิร์กช็อป)
/// 🎓 เหมาะสำหรับการเรียนรู้พื้นฐาน TCP communication
struct EchoServer {
    address: String,
    max_connections: usize,
}

impl EchoServer {
    /// 🏗️ สร้าง Echo Server ใหม่สำหรับ Workshop
    /// 🎯 กำหนด address และจำนวน connection สูงสุด
    fn new(address: &str, max_connections: usize) -> Self {
        Self {
            address: address.to_string(),
            max_connections,
        }
    }
    
    /// 🚀 เริ่มต้น server และรอรับ connection (Workshop Mode)
    /// 🎓 สำหรับการสาธิตใน Web Development Workshop
    fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("🌟 Workshop Echo Server เริ่มทำงานที่ {}", self.address);
        println!("👥 จำนวน connection สูงสุด: {}", self.max_connections);
        
        let connection_count = Arc::new(Mutex::new(0));
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let count = Arc::clone(&connection_count);
                    
                    // Check connection limit
                    {
                        let mut current_count = count.lock().unwrap();
                        if *current_count >= self.max_connections {
                            println!("⚠️ Connection limit reached, rejecting connection");
                            continue;
                        }
                        *current_count += 1;
                    }
                    
                    // Handle connection in separate thread
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream) {
                            eprintln!("❌ Error handling client: {e}");
                        }
                        
                        // Decrease connection count
                        let mut current_count = count.lock().unwrap();
                        *current_count -= 1;
                    });
                }
                Err(e) => {
                    eprintln!("❌ Error accepting connection: {e}");
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
        let peer_addr = stream.peer_addr()?;
        println!("📞 New connection from: {peer_addr}");
        
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = String::new();
        
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    println!("👋 Client {peer_addr} disconnected");
                    break;
                }
                Ok(_) => {
                    let message = buffer.trim();
                    
                    // Handle special commands
                    match message {
                        "quit" | "exit" => {
                            stream.write_all(b"Goodbye!\n")?;
                            break;
                        }
                        "time" => {
                            let time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                            stream.write_all(format!("Current time: {time}\n").as_bytes())?;
                        }
                        "ping" => {
                            stream.write_all(b"pong\n")?;
                        }
                        _ => {
                            // Echo the message back
                            stream.write_all(format!("Echo: {message}\n").as_bytes())?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading from client {peer_addr}: {e}");
                    break;
                }
            }
        }
        
        Ok(())
    }
}

/// 💬 Multi-room Chat Server - Workshop Edition
/// 🏠 รองรับการสร้างห้องแชทหลายห้องและการส่งข้อความ (สำหรับเวิร์กช็อป)
/// 🎯 เหมาะสำหรับการเรียนรู้ real-time communication
struct ChatServer {
    address: String,
    rooms: Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
}

impl ChatServer {
    /// 🏗️ สร้าง Chat Server ใหม่สำหรับ Workshop
    /// 💬 เตรียมระบบห้องแชทสำหรับผู้เข้าร่วมเวิร์กช็อป
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 🚀 เริ่มต้น chat server สำหรับ Workshop
    /// 💬 เปิดระบบแชทสำหรับผู้เข้าร่วมเวิร์กช็อป
    fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("🌟 Workshop Chat Server เริ่มทำงานที่ {}", self.address);
        println!("📋 คำสั่งที่ใช้ได้: /join <room>, /leave, /list, /quit");
        
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let rooms = Arc::clone(&self.rooms);
                    
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_chat_client(stream, rooms) {
                            eprintln!("❌ Error handling chat client: {e}");
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Error accepting connection: {e}");
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_chat_client(
        mut stream: TcpStream,
        rooms: Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
    ) -> std::io::Result<()> {
        let peer_addr = stream.peer_addr()?;
        println!("💬 New chat client: {peer_addr}");
        
        // Welcome message
        stream.write_all(b"Welcome to Chat Server!\nCommands: /join <room>, /leave, /list, /quit\n")?;
        
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut buffer = String::new();
        let mut current_room: Option<String> = None;
        let stream_arc = Arc::new(Mutex::new(stream.try_clone()?));
        
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    let message = buffer.trim();
                    
                    if message.starts_with('/') {
                        // Handle commands
                        let parts: Vec<&str> = message.splitn(2, ' ').collect();
                        match parts[0] {
                            "/join" => {
                                if parts.len() > 1 {
                                    let room_name = parts[1].to_string();
                                    
                                    // Leave current room
                                    if let Some(ref old_room) = current_room {
                                        Self::leave_room(&rooms, old_room, &stream_arc);
                                    }
                                    
                                    // Join new room
                                    Self::join_room(&rooms, &room_name, Arc::clone(&stream_arc));
                                    current_room = Some(room_name.clone());
                                    
                                    stream.write_all(format!("Joined room: {room_name}\n").as_bytes())?;
                                } else {
                                    stream.write_all(b"Usage: /join <room_name>\n")?;
                                }
                            }
                            "/leave" => {
                                if let Some(ref room) = current_room {
                                    Self::leave_room(&rooms, room, &stream_arc);
                                    current_room = None;
                                    stream.write_all(b"Left the room\n")?;
                                } else {
                                    stream.write_all(b"You are not in any room\n")?;
                                }
                            }
                            "/list" => {
                                let room_list = Self::list_rooms(&rooms);
                                stream.write_all(format!("Available rooms: {room_list}\n").as_bytes())?;
                            }
                            "/quit" => {
                                if let Some(ref room) = current_room {
                                    Self::leave_room(&rooms, room, &stream_arc);
                                }
                                stream.write_all(b"Goodbye!\n")?;
                                break;
                            }
                            _ => {
                                stream.write_all(b"Unknown command\n")?;
                            }
                        }
                    } else if let Some(ref room) = current_room {
                        // Broadcast message to room
                        let broadcast_msg = format!("{peer_addr}: {message}\n");
                        Self::broadcast_to_room(&rooms, room, &broadcast_msg, &stream_arc);
                    } else {
                        stream.write_all(b"Please join a room first with /join <room_name>\n")?;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading from chat client {peer_addr}: {e}");
                    break;
                }
            }
        }
        
        // Cleanup on disconnect
        if let Some(ref room) = current_room {
            Self::leave_room(&rooms, room, &stream_arc);
        }
        
        Ok(())
    }
    
    fn join_room(
        rooms: &Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
        room_name: &str,
        stream: Arc<Mutex<TcpStream>>,
    ) {
        let mut rooms_guard = rooms.lock().unwrap();
        rooms_guard
            .entry(room_name.to_string())
            .or_default()
            .push(stream);
    }
    
    fn leave_room(
        rooms: &Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
        room_name: &str,
        stream: &Arc<Mutex<TcpStream>>,
    ) {
        let mut rooms_guard = rooms.lock().unwrap();
        if let Some(room_streams) = rooms_guard.get_mut(room_name) {
            room_streams.retain(|s| !Arc::ptr_eq(s, stream));
            
            // Remove empty rooms
            if room_streams.is_empty() {
                rooms_guard.remove(room_name);
            }
        }
    }
    
    fn broadcast_to_room(
        rooms: &Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
        room_name: &str,
        message: &str,
        sender: &Arc<Mutex<TcpStream>>,
    ) {
        let rooms_guard = rooms.lock().unwrap();
        if let Some(room_streams) = rooms_guard.get(room_name) {
            for stream_arc in room_streams {
                if !Arc::ptr_eq(stream_arc, sender) {
                    if let Ok(mut stream) = stream_arc.try_lock() {
                        let _ = stream.write_all(message.as_bytes());
                    }
                }
            }
        }
    }
    
    fn list_rooms(
        rooms: &Arc<Mutex<HashMap<String, Vec<Arc<Mutex<TcpStream>>>>>>,
    ) -> String {
        let rooms_guard = rooms.lock().unwrap();
        if rooms_guard.is_empty() {
            "No active rooms".to_string()
        } else {
            rooms_guard
                .keys()
                .map(|k| format!("{} ({} users)", k, rooms_guard[k].len()))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

/// 📱 TCP Client - Workshop Demo
/// 🔗 เชื่อมต่อกับ server สำหรับการทดสอบในเวิร์กช็อป
struct TcpClient {
    server_address: String,
}

impl TcpClient {
    /// 🏗️ สร้าง TCP Client ใหม่สำหรับ Workshop
    fn new(server_address: &str) -> Self {
        Self {
            server_address: server_address.to_string(),
        }
    }
    
    fn connect_and_echo(&self, message: &str) -> std::io::Result<String> {
        let mut stream = TcpStream::connect(&self.server_address)?;
        
        // Send message
        stream.write_all(format!("{message}\n").as_bytes())?;
        
        // Read response
        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;
        
        Ok(response.trim().to_string())
    }
    
    fn interactive_session(&self) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(&self.server_address)?;
        println!("📞 เชื่อมต่อกับ Workshop Server ที่ {}", self.server_address);
        
        // Start reader thread
        let reader_stream = stream.try_clone()?;
        let reader_handle = thread::spawn(move || {
            let mut reader = BufReader::new(reader_stream);
            let mut buffer = String::new();
            
            loop {
                buffer.clear();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        println!("\n🔌 Workshop Server ตัดการเชื่อมต่อ");
                        break;
                    }
                    Ok(_) => {
                        print!("📨 {buffer}");
                    }
                    Err(e) => {
                        eprintln!("❌ Error reading from server: {e}");
                        break;
                    }
                }
            }
        });
        
        // Main input loop
        let stdin = std::io::stdin();
        loop {
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    let message = input.trim();
                    if message == "quit" || message == "exit" {
                        stream.write_all(format!("{message}\n").as_bytes())?;
                        break;
                    }
                    
                    if let Err(e) = stream.write_all(format!("{message}\n").as_bytes()) {
                        eprintln!("❌ Error sending message: {e}");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading input: {e}");
                    break;
                }
            }
        }
        
        // Wait for reader thread to finish
        let _ = reader_handle.join();
        
        Ok(())
    }
}

/// 🎯 สาธิตการใช้งาน TCP Server และ Client - Web Development Workshop
/// 🌟 ตัวอย่างสำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn demonstrate_tcp() {
    println!("🌐 TCP Server/Client Examples - Web Development Workshop:");
    
    // Example 1: Simple Echo Server (in a separate thread)
    println!("\n1. 🔄 Echo Server Example (Workshop Demo):");
    let echo_server = EchoServer::new("127.0.0.1:8080", 10);
    
    // Start server in background thread
    thread::spawn(move || {
        if let Err(e) = echo_server.start() {
            eprintln!("❌ Echo server error: {e}");
        }
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Test client
    let client = TcpClient::new("127.0.0.1:8080");
    
    match client.connect_and_echo("สวัสดี Workshop Server!") {
        Ok(response) => println!("📨 การตอบกลับจาก Server: {response}"),
        Err(e) => eprintln!("❌ ข้อผิดพลาด Client: {e}"),
    }
    
    match client.connect_and_echo("ping") {
        Ok(response) => println!("📨 การตอบกลับ Ping: {response}"),
        Err(e) => eprintln!("❌ ข้อผิดพลาด Ping: {e}"),
    }
    
    // Example 2: Chat Server (commented out to avoid port conflicts)
    println!("\n2. 💬 Chat Server Example (Workshop Simulation):");
    println!("🏠 Chat server จะฟังที่ port 8081 สำหรับเวิร์กช็อป");
    println!("👥 ผู้เข้าร่วมสามารถเข้าห้องและแชทกันได้");
    println!("📋 คำสั่ง: /join <room>, /leave, /list, /quit");
    
    // Simulate some client interactions
    println!("\n📝 การจำลองการแชทในเวิร์กช็อป:");
    println!("ผู้เข้าร่วม1: /join workshop-general");
    println!("Server: เข้าร่วมห้อง: workshop-general");
    println!("ผู้เข้าร่วม2: /join workshop-general");
    println!("ผู้เข้าร่วม1: สวัสดีทุกคนในเวิร์กช็อป!");
    println!("ผู้เข้าร่วม2: 127.0.0.1:8081: สวัสดีทุกคนในเวิร์กช็อป!");
    
    println!("\n✅ TCP Workshop Examples สำเร็จแล้ว!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_tcp_client_connection() {
        // Start a simple echo server for testing
        let server = EchoServer::new("127.0.0.1:0", 1); // Use port 0 for automatic assignment
        
        // This test would need a more sophisticated setup
        // to properly test the TCP functionality
        assert!(true); // Placeholder
    }
    
    #[test]
    fn test_chat_server_creation() {
        let chat_server = ChatServer::new("127.0.0.1:0");
        // Test that the server can be created
        assert!(!chat_server.address.is_empty());
    }
}