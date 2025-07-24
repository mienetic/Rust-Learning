//! 📡 UDP Communication Implementation - Web Development Workshop
//!
//! 🎯 การสื่อสารแบบ UDP (User Datagram Protocol) ใน Rust สำหรับเวิร์กช็อป
//! 🚀 เหมาะสำหรับการเรียนรู้ network programming แบบ connectionless

use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// 🖥️ UDP Server implementation - Workshop Edition
/// 📡 รองรับการสื่อสารแบบ UDP สำหรับผู้เข้าร่วมเวิร์กช็อป
struct UdpServer {
    socket: UdpSocket,
    clients: Arc<Mutex<HashMap<SocketAddr, ClientInfo>>>,
    running: Arc<Mutex<bool>>,
}

/// 👤 ข้อมูลผู้เข้าร่วมเวิร์กช็อป
#[derive(Debug, Clone)]
struct ClientInfo {
    last_seen: Instant,
    message_count: usize,
}

impl UdpServer {
    /// 🏗️ สร้าง UDP Server ใหม่สำหรับ Workshop
    /// 🎯 เตรียมระบบสำหรับผู้เข้าร่วมเวิร์กช็อป
    fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(100)))?;
        
        Ok(Self {
            socket,
            clients: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// 🚀 เริ่มต้น UDP Server สำหรับ Workshop
    /// 🌟 เปิดรับการเชื่อมต่อจากผู้เข้าร่วมเวิร์กช็อป
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        *self.running.lock().unwrap() = true;
        println!("🌟 Workshop UDP Server เริ่มทำงานที่ {}", self.socket.local_addr()?);
        
        let mut buffer = [0; 1024];
        
        while *self.running.lock().unwrap() {
            match self.socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("📨 Received from {src}: {message}");
                    
                    // Update client info
                    {
                        let mut clients = self.clients.lock().unwrap();
                        let client_info = clients.entry(src).or_insert(ClientInfo {
                            last_seen: Instant::now(),
                            message_count: 0,
                        });
                        client_info.last_seen = Instant::now();
                        client_info.message_count += 1;
                    }
                    
                    // Handle different message types
                    self.handle_message(&message, src)?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        eprintln!("❌ Error receiving UDP message: {e}");
                    }
                }
            }
            
            // Clean up inactive clients
            self.cleanup_inactive_clients();
        }
        
        Ok(())
    }
    
    fn handle_message(&self, message: &str, src: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = message.trim().split(':').collect();
        
        match parts.first() {
            Some(&"PING") => {
                self.send_to(src, "PONG")?;
            }
            Some(&"ECHO") => {
                if let Some(text) = parts.get(1) {
                    let response = format!("ECHO:{text}");
                    self.send_to(src, &response)?;
                }
            }
            Some(&"TIME") => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let response = format!("TIME:{now}");
                self.send_to(src, &response)?;
            }
            Some(&"STATS") => {
                let stats = self.get_client_stats(src);
                self.send_to(src, &stats)?;
            }
            Some(&"BROADCAST") => {
                if let Some(text) = parts.get(1) {
                    self.broadcast_message(&format!("BROADCAST:{src}:{text}"), Some(src))?;
                }
            }
            Some(&"LIST") => {
                let client_list = self.get_client_list();
                self.send_to(src, &client_list)?;
            }
            _ => {
                self.send_to(src, "ERROR:Unknown command")?;
            }
        }
        
        Ok(())
    }
    
    fn send_to(&self, addr: SocketAddr, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.send_to(message.as_bytes(), addr)?;
        println!("📤 Sent to {addr}: {message}");
        Ok(())
    }
    
    fn broadcast_message(&self, message: &str, exclude: Option<SocketAddr>) -> Result<(), Box<dyn std::error::Error>> {
        let clients = self.clients.lock().unwrap();
        for &addr in clients.keys() {
            if Some(addr) != exclude {
                if let Err(e) = self.socket.send_to(message.as_bytes(), addr) {
                    eprintln!("❌ Failed to send broadcast to {addr}: {e}");
                }
            }
        }
        println!("📢 Broadcasted: {message}");
        Ok(())
    }
    
    fn get_client_stats(&self, addr: SocketAddr) -> String {
        let clients = self.clients.lock().unwrap();
        if let Some(info) = clients.get(&addr) {
            format!("STATS:messages={},last_seen={:.2}s_ago", 
                   info.message_count, 
                   info.last_seen.elapsed().as_secs_f64())
        } else {
            "STATS:not_found".to_string()
        }
    }
    
    fn get_client_list(&self) -> String {
        let clients = self.clients.lock().unwrap();
        let client_addrs: Vec<String> = clients.keys()
            .map(std::string::ToString::to_string)
            .collect();
        format!("LIST:{}", client_addrs.join(","))
    }
    
    fn cleanup_inactive_clients(&self) {
        let mut clients = self.clients.lock().unwrap();
        let timeout = Duration::from_secs(30);
        
        clients.retain(|addr, info| {
            let is_active = info.last_seen.elapsed() < timeout;
            if !is_active {
                println!("🧹 Removed inactive client: {addr}");
            }
            is_active
        });
    }
    
    fn stop(&self) {
        *self.running.lock().unwrap() = false;
        println!("🛑 UDP Server stopped");
    }
    
    fn get_client_count(&self) -> usize {
        self.clients.lock().unwrap().len()
    }
}

/// 📱 UDP Client implementation - Workshop Edition
/// 🔗 เชื่อมต่อกับ UDP Server สำหรับการทดสอบในเวิร์กช็อป
struct UdpClient {
    socket: UdpSocket,
    server_addr: SocketAddr,
}

impl UdpClient {
    /// 🏗️ สร้าง UDP Client ใหม่สำหรับ Workshop
    /// 📡 เตรียมเชื่อมต่อกับ Workshop Server
    fn new(server_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available port
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        
        let server_addr = server_addr.parse()?;
        
        Ok(Self {
            socket,
            server_addr,
        })
    }
    
    fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Send message
        self.socket.send_to(message.as_bytes(), self.server_addr)?;
        println!("📤 Sent: {message}");
        
        // Receive response
        let mut buffer = [0; 1024];
        let (size, _) = self.socket.recv_from(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..size]).to_string();
        println!("📨 Received: {response}");
        
        Ok(response)
    }
    
    fn ping(&self) -> Result<Duration, Box<dyn std::error::Error>> {
        let start = Instant::now();
        let response = self.send_message("PING")?;
        let duration = start.elapsed();
        
        if response == "PONG" {
            Ok(duration)
        } else {
            Err(format!("Unexpected response: {response}").into())
        }
    }
    
    fn echo(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let message = format!("ECHO:{text}");
        let response = self.send_message(&message)?;
        
        if let Some(echo_text) = response.strip_prefix("ECHO:") {
            Ok(echo_text.to_string())
        } else {
            Err(format!("Unexpected response: {response}").into())
        }
    }
    
    fn get_server_time(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self.send_message("TIME")?;
        
        if let Some(time_str) = response.strip_prefix("TIME:") {
            Ok(time_str.parse()?)
        } else {
            Err(format!("Unexpected response: {response}").into())
        }
    }
    
    fn get_stats(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.send_message("STATS")?;
        
        if let Some(stats) = response.strip_prefix("STATS:") {
            Ok(stats.to_string())
        } else {
            Err(format!("Unexpected response: {response}").into())
        }
    }
    
    fn broadcast(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let broadcast_msg = format!("BROADCAST:{message}");
        self.send_message(&broadcast_msg)?;
        Ok(())
    }
    
    fn list_clients(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = self.send_message("LIST")?;
        
        if let Some(list_str) = response.strip_prefix("LIST:") {
            if list_str.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(list_str.split(',').map(std::string::ToString::to_string).collect())
            }
        } else {
            Err(format!("Unexpected response: {response}").into())
        }
    }
}

/// 📁 UDP File Transfer - Workshop Demo
/// 🚀 การส่งไฟล์แบบ UDP สำหรับการเรียนรู้ในเวิร์กช็อป
struct UdpFileTransfer {
    socket: UdpSocket,
    chunk_size: usize,
}

impl UdpFileTransfer {
    /// 🏗️ สร้าง UDP File Transfer ใหม่สำหรับ Workshop
    /// 📦 กำหนดขนาด chunk สำหรับการส่งไฟล์
    fn new(addr: &str, chunk_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_secs(10)))?;
        
        Ok(Self {
            socket,
            chunk_size,
        })
    }
    
    fn send_file(&self, file_data: &[u8], dest: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let total_chunks = file_data.len().div_ceil(self.chunk_size);
        
        // Send file header
        let header = format!("FILE_START:{}:{}", file_data.len(), total_chunks);
        self.socket.send_to(header.as_bytes(), dest)?;
        
        // Wait for acknowledgment
        let mut buffer = [0; 1024];
        let (size, _) = self.socket.recv_from(&mut buffer)?;
        let ack = String::from_utf8_lossy(&buffer[..size]);
        
        if ack != "ACK_START" {
            return Err("Failed to get start acknowledgment".into());
        }
        
        // Send file chunks
        for (chunk_id, chunk) in file_data.chunks(self.chunk_size).enumerate() {
            let chunk_header = format!("CHUNK:{}:{}", chunk_id, chunk.len());
            
            // Send chunk header
            self.socket.send_to(chunk_header.as_bytes(), dest)?;
            
            // Send chunk data
            self.socket.send_to(chunk, dest)?;
            
            // Wait for chunk acknowledgment
            let (size, _) = self.socket.recv_from(&mut buffer)?;
            let ack = String::from_utf8_lossy(&buffer[..size]);
            
            if ack != format!("ACK_CHUNK:{chunk_id}") {
                return Err(format!("Failed to get chunk {chunk_id} acknowledgment").into());
            }
            
            println!("📦 Sent chunk {}/{}", chunk_id + 1, total_chunks);
        }
        
        // Send end marker
        self.socket.send_to(b"FILE_END", dest)?;
        
        println!("✅ File transfer completed");
        Ok(())
    }
    
    fn receive_file(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        let mut file_data = Vec::new();
        
        // Receive file header
        let (size, src) = self.socket.recv_from(&mut buffer)?;
        let header = String::from_utf8_lossy(&buffer[..size]);
        
        if !header.starts_with("FILE_START:") {
            return Err("Invalid file header".into());
        }
        
        let parts: Vec<&str> = header.split(':').collect();
        let file_size: usize = parts[1].parse()?;
        let total_chunks: usize = parts[2].parse()?;
        
        println!("📥 Receiving file: {file_size} bytes, {total_chunks} chunks");
        
        // Send start acknowledgment
        self.socket.send_to(b"ACK_START", src)?;
        
        // Receive chunks
        for expected_chunk in 0..total_chunks {
            // Receive chunk header
            let (size, _) = self.socket.recv_from(&mut buffer)?;
            let chunk_header = String::from_utf8_lossy(&buffer[..size]);
            
            if !chunk_header.starts_with("CHUNK:") {
                return Err("Invalid chunk header".into());
            }
            
            let parts: Vec<&str> = chunk_header.split(':').collect();
            let chunk_id: usize = parts[1].parse()?;
            let chunk_size: usize = parts[2].parse()?;
            
            if chunk_id != expected_chunk {
                return Err(format!("Expected chunk {expected_chunk}, got {chunk_id}").into());
            }
            
            // Receive chunk data
            let mut chunk_buffer = vec![0; chunk_size];
            let (received_size, _) = self.socket.recv_from(&mut chunk_buffer)?;
            
            if received_size != chunk_size {
                return Err(format!("Expected {chunk_size} bytes, got {received_size}").into());
            }
            
            file_data.extend_from_slice(&chunk_buffer);
            
            // Send chunk acknowledgment
            let ack = format!("ACK_CHUNK:{chunk_id}");
            self.socket.send_to(ack.as_bytes(), src)?;
            
            println!("📦 Received chunk {}/{}", chunk_id + 1, total_chunks);
        }
        
        // Receive end marker
        let (size, _) = self.socket.recv_from(&mut buffer)?;
        let end_marker = String::from_utf8_lossy(&buffer[..size]);
        
        if end_marker != "FILE_END" {
            return Err("Invalid end marker".into());
        }
        
        if file_data.len() != file_size {
            return Err(format!("Expected {} bytes, got {}", file_size, file_data.len()).into());
        }
        
        println!("✅ File received successfully");
        Ok(file_data)
    }
}

/// 🎯 สาธิตการใช้งาน UDP Communication - Web Development Workshop
/// 🌟 ตัวอย่างสำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn demonstrate_udp() {
    println!("🌐 UDP Communication Examples - Web Development Workshop:");
    
    // Start server in a separate thread
    let server = match UdpServer::new("127.0.0.1:8080") {
        Ok(server) => Arc::new(server),
        Err(e) => {
            println!("❌ ไม่สามารถสร้าง server: {e}");
            return;
        }
    };
    let server_clone = server.clone();
    
    let server_handle = thread::spawn(move || {
        if let Err(e) = server_clone.start() {
            eprintln!("❌ Server error: {e}");
        }
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Create clients
    println!("\n👥 สร้าง UDP clients สำหรับเวิร์กช็อป:");
    
    let client1 = match UdpClient::new("127.0.0.1:8080") {
        Ok(client) => client,
        Err(e) => {
            println!("❌ ไม่สามารถสร้าง client1: {e}");
            return;
        }
    };
    let client2 = match UdpClient::new("127.0.0.1:8080") {
        Ok(client) => client,
        Err(e) => {
            println!("❌ ไม่สามารถสร้าง client2: {e}");
            return;
        }
    };
    
    // Test ping
    println!("\n🏓 ทดสอบ ping ในเวิร์กช็อป:");
    match client1.ping() {
        Ok(duration) => println!("✅ Ping สำเร็จ: {:.2}ms", duration.as_secs_f64() * 1000.0),
        Err(e) => println!("❌ Ping ล้มเหลว: {e}"),
    }
    
    // Test echo
    println!("\n🔊 ทดสอบ echo ในเวิร์กช็อป:");
    match client1.echo("สวัสดี Workshop UDP!") {
        Ok(response) => println!("✅ การตอบกลับ Echo: {response}"),
        Err(e) => println!("❌ Echo ล้มเหลว: {e}"),
    }
    
    // Test server time
    println!("\n⏰ ขอเวลาจาก Workshop Server:");
    match client1.get_server_time() {
        Ok(timestamp) => println!("✅ เวลาของ Server: {timestamp}"),
        Err(e) => println!("❌ การขอเวลาล้มเหลว: {e}"),
    }
    
    // Test stats
    println!("\n📊 ขอสถิติผู้เข้าร่วมเวิร์กช็อป:");
    match client1.get_stats() {
        Ok(stats) => println!("✅ สถิติผู้เข้าร่วม: {stats}"),
        Err(e) => println!("❌ การขอสถิติล้มเหลว: {e}"),
    }
    
    // Test client list
    println!("\n📋 ดูรายชื่อผู้เข้าร่วมเวิร์กช็อป:");
    
    // Make sure both clients are registered
    let _ = client2.ping();
    
    match client1.list_clients() {
        Ok(clients) => {
            println!("✅ ผู้เข้าร่วมที่เชื่อมต่อ: {}", clients.len());
            for client in clients {
                println!("  - {client}");
            }
        }
        Err(e) => println!("❌ การขอรายชื่อล้มเหลว: {e}"),
    }
    
    // Test broadcast
    println!("\n📢 ทดสอบการส่งข้อความหาทุกคนในเวิร์กช็อป:");
    if let Err(e) = client1.broadcast("สวัสดีทุกคนในเวิร์กช็อป!") {
        println!("❌ การส่งข้อความล้มเหลว: {e}");
    }
    
    // Give some time for message processing
    thread::sleep(Duration::from_millis(100));
    
    println!("\n📊 สถิติ Workshop Server:");
    println!("ผู้เข้าร่วมที่ใช้งานอยู่: {}", server.get_client_count());
    
    // Test file transfer
    println!("\n📁 ทดสอบการส่งไฟล์ในเวิร์กช็อป:");
    
    let file_data = "นี่คือไฟล์ทดสอบสำหรับ Workshop UDP transfer. มีข้อมูลหลาย chunk เพื่อทดสอบ protocol การส่งไฟล์.".as_bytes();
    
    // Start file receiver in a separate thread
    let receiver_handle = thread::spawn(|| {
        let file_transfer = match UdpFileTransfer::new("127.0.0.1:8081", 32) {
            Ok(transfer) => transfer,
            Err(e) => {
                println!("❌ ไม่สามารถสร้าง file receiver: {e}");
                return;
            }
        };
        match file_transfer.receive_file() {
            Ok(data) => {
                println!("✅ รับไฟล์สำเร็จ: {} bytes", data.len());
                println!("เนื้อหา: {}", String::from_utf8_lossy(&data));
            }
            Err(e) => println!("❌ การรับไฟล์ล้มเหลว: {e}"),
        }
    });
    
    // Give receiver time to start
    thread::sleep(Duration::from_millis(100));
    
    // Send file
    let sender = match UdpFileTransfer::new("127.0.0.1:8082", 32) {
        Ok(sender) => sender,
        Err(e) => {
            println!("❌ ไม่สามารถสร้าง file sender: {e}");
            return;
        }
    };
    let target_addr = match "127.0.0.1:8081".parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("❌ ไม่สามารถ parse address: {e}");
            return;
        }
    };
    match sender.send_file(file_data, target_addr) {
        Ok(()) => println!("✅ ส่งไฟล์สำเร็จ"),
        Err(e) => println!("❌ การส่งไฟล์ล้มเหลว: {e}"),
    }
    
    // Wait for file transfer to complete
    let _ = receiver_handle.join();
    
    // Stop server
    server.stop();
    
    // Wait for server to stop
    thread::sleep(Duration::from_millis(200));
    
    println!("\n✅ UDP Workshop Communication สำเร็จแล้ว!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_udp_server_creation() {
        let server = UdpServer::new("127.0.0.1:0");
        assert!(server.is_ok());
    }
    
    #[test]
    fn test_udp_client_creation() {
        let client = UdpClient::new("127.0.0.1:8080");
        assert!(client.is_ok());
    }
    
    #[test]
    fn test_file_transfer_creation() {
        let transfer = UdpFileTransfer::new("127.0.0.1:0", 1024);
        assert!(transfer.is_ok());
    }
}