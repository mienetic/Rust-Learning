//! แบบฝึกหัด Concurrency - สนามรบของ Threads และ Async! ⚡🚀
//! ที่นี่เราจะฝึกการเขียนโปรแกรมแบบ concurrent และ parallel!

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, mpsc, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::cell::RefCell;

/// ระบบดาวน์โหลดไฟล์แบบ Concurrent - โปรเจกต์ขั้นสูง! 📥⚡
/// ใช้ threads, channels, และ shared state

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: usize,
    pub url: String,
    pub size_mb: usize,
    pub priority: u8,
}

impl DownloadTask {
    #[must_use] pub const fn new(id: usize, url: String, size_mb: usize, priority: u8) -> Self {
        Self { id, url, size_mb, priority }
    }
    
    /// จำลองการดาวน์โหลด - ใช้เวลาตามขนาดไฟล์
    pub fn download(&self) -> Result<String, String> {
        let download_time = Duration::from_millis((self.size_mb * 100) as u64);
        thread::sleep(download_time);
        
        // จำลองความล้มเหลว 10%
        if self.id % 10 == 0 {
            Err(format!("Failed to download {}", self.url))
        } else {
            Ok(format!("Downloaded {} ({} MB)", self.url, self.size_mb))
        }
    }
}

/// สถิติการดาวน์โหลด - ใช้ Atomic types สำหรับ thread safety! 📊⚡
#[derive(Debug)]
pub struct DownloadStats {
    pub total_downloads: AtomicUsize,
    pub successful_downloads: AtomicUsize,
    pub failed_downloads: AtomicUsize,
    pub total_mb_downloaded: AtomicUsize,
    pub is_running: AtomicBool,
}

impl Default for DownloadStats {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadStats {
    #[must_use] pub const fn new() -> Self {
        Self {
            total_downloads: AtomicUsize::new(0),
            successful_downloads: AtomicUsize::new(0),
            failed_downloads: AtomicUsize::new(0),
            total_mb_downloaded: AtomicUsize::new(0),
            is_running: AtomicBool::new(false),
        }
    }
    
    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
    }
    
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
    
    pub fn record_success(&self, size_mb: usize) {
        self.total_downloads.fetch_add(1, Ordering::SeqCst);
        self.successful_downloads.fetch_add(1, Ordering::SeqCst);
        self.total_mb_downloaded.fetch_add(size_mb, Ordering::SeqCst);
    }
    
    pub fn record_failure(&self) {
        self.total_downloads.fetch_add(1, Ordering::SeqCst);
        self.failed_downloads.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get_summary(&self) -> (usize, usize, usize, usize, bool) {
        (
            self.total_downloads.load(Ordering::SeqCst),
            self.successful_downloads.load(Ordering::SeqCst),
            self.failed_downloads.load(Ordering::SeqCst),
            self.total_mb_downloaded.load(Ordering::SeqCst),
            self.is_running.load(Ordering::SeqCst),
        )
    }
}

/// Download Manager ที่ใช้ Thread Pool และ Channels! 🎯🔄
pub struct DownloadManager {
    stats: Arc<DownloadStats>,
    max_concurrent: usize,
}

impl DownloadManager {
    #[must_use] pub fn new(max_concurrent: usize) -> Self {
        Self {
            stats: Arc::new(DownloadStats::new()),
            max_concurrent,
        }
    }
    
    pub fn download_batch(&self, tasks: Vec<DownloadTask>) {
        println!("🚀 เริ่มดาวน์โหลด {} ไฟล์ด้วย {} threads!", tasks.len(), self.max_concurrent);
        
        self.stats.start();
        let start_time = Instant::now();
        
        // สร้าง channel สำหรับส่งงาน
        let (task_sender, task_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        
        // ส่งงานทั้งหมดไปยัง channel
        for task in tasks {
            task_sender.send(task).unwrap();
        }
        drop(task_sender); // ปิด channel
        
        // สร้าง worker threads
        let task_receiver = Arc::new(Mutex::new(task_receiver));
        let mut handles = vec![];
        
        for worker_id in 0..self.max_concurrent {
            let task_receiver = Arc::clone(&task_receiver);
            let result_sender = result_sender.clone();
            let stats = Arc::clone(&self.stats);
            
            let handle = thread::spawn(move || {
                println!("👷 Worker {worker_id} เริ่มทำงาน!");
                
                while let Ok(task) = {
                    let receiver = task_receiver.lock().unwrap();
                    receiver.recv()
                } {
                    println!("📥 Worker {} กำลังดาวน์โหลด: {} ({} MB)", 
                        worker_id, task.url, task.size_mb);
                    
                    match task.download() {
                        Ok(result) => {
                            stats.record_success(task.size_mb);
                            result_sender.send((task.id, Ok(result))).unwrap();
                            println!("✅ Worker {} สำเร็จ: {}", worker_id, task.url);
                        }
                        Err(error) => {
                            stats.record_failure();
                            result_sender.send((task.id, Err(error.clone()))).unwrap();
                            println!("❌ Worker {} ล้มเหลว: {} - {}", worker_id, task.url, error);
                        }
                    }
                }
                
                println!("👷 Worker {worker_id} เสร็จงาน!");
            });
            
            handles.push(handle);
        }
        
        drop(result_sender); // ปิด channel
        
        // รอรับผลลัพธ์
        let mut results = HashMap::new();
        for (task_id, result) in result_receiver {
            results.insert(task_id, result);
        }
        
        // รอให้ worker threads เสร็จ
        for handle in handles {
            handle.join().unwrap();
        }
        
        self.stats.stop();
        let duration = start_time.elapsed();
        
        // แสดงผลสรุป
        let (total, success, failed, total_mb, _) = self.stats.get_summary();
        println!("\n📊 === สรุปผลการดาวน์โหลด === 📊");
        println!("⏱️ เวลาที่ใช้: {:.2} วินาที", duration.as_secs_f64());
        println!("📁 ไฟล์ทั้งหมด: {total} ไฟล์");
        println!("✅ สำเร็จ: {success} ไฟล์");
        println!("❌ ล้มเหลว: {failed} ไฟล์");
        println!("💾 ขนาดรวม: {total_mb} MB");
        println!("🚀 ความเร็วเฉลี่ย: {:.2} MB/s", 
            total_mb as f64 / duration.as_secs_f64());
        
        if failed > 0 {
            println!("\n❌ ไฟล์ที่ล้มเหลว:");
            for (task_id, result) in &results {
                if let Err(error) = result {
                    println!("  📁 Task {task_id}: {error}");
                }
            }
        }
    }
    
    #[must_use] pub fn get_stats(&self) -> Arc<DownloadStats> {
        Arc::clone(&self.stats)
    }
}

/// ระบบ Chat Room แบบ Multi-threaded - สนทนาแบบ Real-time! 💬⚡
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub content: String,
    pub timestamp: Instant,
}

impl ChatMessage {
    #[must_use] pub fn new(user: String, content: String) -> Self {
        Self {
            user,
            content,
            timestamp: Instant::now(),
        }
    }
}

/// Chat Room ที่ใช้ `RwLock` สำหรับ concurrent access! 💬🔒
pub struct ChatRoom {
    messages: Arc<RwLock<Vec<ChatMessage>>>,
    users: Arc<RwLock<Vec<String>>>,
    message_count: Arc<AtomicUsize>,
}

impl Default for ChatRoom {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatRoom {
    #[must_use] pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
            users: Arc::new(RwLock::new(Vec::new())),
            message_count: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn join_user(&self, username: String) -> Result<(), String> {
        let mut users = self.users.write().unwrap();
        
        if users.contains(&username) {
            return Err(format!("User {username} already exists"));
        }
        
        users.push(username.clone());
        println!("👋 {} เข้าร่วมห้องแชท! (ผู้ใช้ทั้งหมด: {})", username, users.len());
        Ok(())
    }
    
    pub fn leave_user(&self, username: &str) {
        let mut users = self.users.write().unwrap();
        users.retain(|user| user != username);
        println!("👋 {} ออกจากห้องแชท! (ผู้ใช้ทั้งหมด: {})", username, users.len());
    }
    
    pub fn send_message(&self, user: String, content: String) {
        let message = ChatMessage::new(user, content);
        
        {
            let mut messages = self.messages.write().unwrap();
            messages.push(message.clone());
        }
        
        let count = self.message_count.fetch_add(1, Ordering::SeqCst) + 1;
        println!("💬 [{}] {}: {} (ข้อความที่ {})", 
            count, message.user, message.content, count);
    }
    
    #[must_use] pub fn get_recent_messages(&self, limit: usize) -> Vec<ChatMessage> {
        let messages = self.messages.read().unwrap();
        let start = if messages.len() > limit { messages.len() - limit } else { 0 };
        messages[start..].to_vec()
    }
    
    #[must_use] pub fn get_user_count(&self) -> usize {
        self.users.read().unwrap().len()
    }
    
    #[must_use] pub fn get_message_count(&self) -> usize {
        self.message_count.load(Ordering::SeqCst)
    }
    
    #[must_use] pub fn get_users(&self) -> Vec<String> {
        self.users.read().unwrap().clone()
    }
}

/// Producer-Consumer Pattern ด้วย Channels - โรงงานผลิตข้อมูล! 🏭📦
pub fn producer_consumer_example() {
    println!("\n🏭 === Producer-Consumer Pattern: โรงงานผลิตข้อมูล! === 🏭");
    
    let (sender, receiver) = mpsc::channel();
    let num_producers = 3;
    let num_consumers = 2;
    let items_per_producer = 5;
    
    // สร้าง Producers
    let mut producer_handles = vec![];
    for producer_id in 0..num_producers {
        let sender = sender.clone();
        
        let handle = thread::spawn(move || {
            println!("🏭 Producer {producer_id} เริ่มผลิต!");
            
            for item_id in 0..items_per_producer {
                let item = format!("Item-{producer_id}-{item_id}");
                println!("📦 Producer {producer_id} ผลิต: {item}");
                sender.send(item).unwrap();
                
                // จำลองเวลาในการผลิต (ลดเวลาลง)
                thread::sleep(Duration::from_millis(50));
            }
            
            println!("🏭 Producer {producer_id} เสร็จงาน!");
        });
        
        producer_handles.push(handle);
    }
    
    drop(sender); // ปิด channel เมื่อ producers เสร็จ
    
    // สร้าง Consumers
    let receiver = Arc::new(Mutex::new(receiver));
    let mut consumer_handles = vec![];
    
    for consumer_id in 0..num_consumers {
        let receiver = Arc::clone(&receiver);
        
        let handle = thread::spawn(move || {
            println!("🛒 Consumer {consumer_id} เริ่มบริโภค!");
            let mut consumed_count = 0;
            
            while let Ok(item) = {
                let receiver = receiver.lock().unwrap();
                receiver.recv()
            } {
                println!("🍽️ Consumer {consumer_id} บริโภค: {item}");
                consumed_count += 1;
                
                // จำลองเวลาในการบริโภค (ลดเวลาลง)
                thread::sleep(Duration::from_millis(75));
            }
            
            println!("🛒 Consumer {consumer_id} เสร็จงาน! (บริโภค {consumed_count} รายการ)");
        });
        
        consumer_handles.push(handle);
    }
    
    // รอให้ producer threads เสร็จก่อน
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    // รอให้ consumer threads เสร็จ
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("🎉 Producer-Consumer เสร็จสิ้น!");
}

/// Parallel Computing Example - คำนวณแบบขนาน! 🧮⚡
pub fn parallel_computing_example() {
    println!("\n🧮 === Parallel Computing: คำนวณแบบขนาน! === 🧮");
    
    let numbers: Vec<u64> = (1..=1000000).collect();
    let chunk_size = numbers.len() / 4;
    
    println!("🔢 คำนวณผลรวมของตัวเลข 1-1,000,000 แบบขนาน!");
    
    let start_time = Instant::now();
    
    // แบ่งงานเป็น chunks
    let chunks: Vec<Vec<u64>> = numbers.chunks(chunk_size)
        .map(<[u64]>::to_vec)
        .collect();
    
    // สร้าง threads สำหรับคำนวณแต่ละ chunk
    let mut handles = vec![];
    
    for (chunk_id, chunk) in chunks.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            println!("🧮 Thread {} เริ่มคำนวณ {} ตัวเลข", chunk_id, chunk.len());
            
            let sum: u64 = chunk.iter().sum();
            
            println!("✅ Thread {chunk_id} เสร็จ! ผลรวม: {sum}");
            sum
        });
        
        handles.push(handle);
    }
    
    // รวมผลลัพธ์จากทุก threads
    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().unwrap();
    }
    
    let duration = start_time.elapsed();
    
    println!("\n📊 === ผลลัพธ์การคำนวณแบบขนาน === 📊");
    println!("🎯 ผลรวมทั้งหมด: {total_sum}");
    println!("⏱️ เวลาที่ใช้: {:.2} มิลลิวินาที", duration.as_millis());
    
    // เปรียบเทียบกับการคำนวณแบบ sequential
    let start_time = Instant::now();
    let sequential_sum: u64 = (1..=1000000).sum();
    let sequential_duration = start_time.elapsed();
    
    println!("\n🐌 === การคำนวณแบบ Sequential === 🐌");
    println!("🎯 ผลรวม: {sequential_sum}");
    println!("⏱️ เวลาที่ใช้: {:.2} มิลลิวินาที", sequential_duration.as_millis());
    
    let speedup = sequential_duration.as_nanos() as f64 / duration.as_nanos() as f64;
    println!("🚀 ความเร็วเพิ่มขึ้น: {speedup:.2}x");
}

/// Deadlock Prevention Example - ป้องกันการติดขัด! 🔒⚠️
pub fn deadlock_prevention_example() {
    println!("\n🔒 === Deadlock Prevention: ป้องกันการติดขัด! === 🔒");
    
    let resource_a = Arc::new(Mutex::new("Resource A"));
    let resource_b = Arc::new(Mutex::new("Resource B"));
    
    println!("⚠️ สาธิตการป้องกัน Deadlock ด้วยการเรียงลำดับ locks!");
    
    // Thread 1: ขอ A แล้วค่อยขอ B
    let resource_a1 = Arc::clone(&resource_a);
    let resource_b1 = Arc::clone(&resource_b);
    
    let handle1 = thread::spawn(move || {
        println!("🧵 Thread 1: พยายามขอ Resource A...");
        let _lock_a = resource_a1.lock().unwrap();
        println!("🔓 Thread 1: ได้ Resource A แล้ว!");
        
        thread::sleep(Duration::from_millis(50));
        
        println!("🧵 Thread 1: พยายามขอ Resource B...");
        let _lock_b = resource_b1.lock().unwrap();
        println!("🔓 Thread 1: ได้ Resource B แล้ว!");
        
        thread::sleep(Duration::from_millis(100));
        println!("✅ Thread 1: เสร็จงาน!");
    });
    
    // Thread 2: ขอ A แล้วค่อยขอ B (ลำดับเดียวกัน - ป้องกัน deadlock)
    let resource_a2 = Arc::clone(&resource_a);
    let resource_b2 = Arc::clone(&resource_b);
    
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(25));
        
        println!("🧵 Thread 2: พยายามขอ Resource A...");
        let _lock_a = resource_a2.lock().unwrap();
        println!("🔓 Thread 2: ได้ Resource A แล้ว!");
        
        thread::sleep(Duration::from_millis(50));
        
        println!("🧵 Thread 2: พยายามขอ Resource B...");
        let _lock_b = resource_b2.lock().unwrap();
        println!("🔓 Thread 2: ได้ Resource B แล้ว!");
        
        thread::sleep(Duration::from_millis(100));
        println!("✅ Thread 2: เสร็จงาน!");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("🎉 ไม่มี Deadlock เกิดขึ้น! (เพราะใช้ลำดับ locks เดียวกัน)");
}

/// Work Stealing Example - ขโมยงานแบบถูกกฎหมาย! 😈⚡
pub fn work_stealing_example() {
    println!("\n😈 === Work Stealing: ขโมยงานแบบถูกกฎหมาย! === 😈");
    
    let num_workers = 4;
    let total_tasks = 20;
    
    // สร้าง work queues สำหรับแต่ละ worker
    let mut work_queues = Vec::new();
    let mut queue_handles = Vec::new();
    
    for _ in 0..num_workers {
        let (sender, receiver) = mpsc::channel();
        work_queues.push(sender);
        queue_handles.push(Arc::new(Mutex::new(receiver)));
    }
    
    // แจกงานให้ workers แบบไม่เท่ากัน (จำลองการกระจายงานที่ไม่สมดุล)
    for task_id in 0..total_tasks {
        let worker_id = if task_id < 15 { 0 } else { task_id % num_workers }; // Worker 0 ได้งานเยอะ
        work_queues[worker_id].send(format!("Task-{task_id}")).unwrap();
    }
    
    // ปิด channels
    drop(work_queues);
    
    // สร้าง shared work pool สำหรับ work stealing
    let shared_work = Arc::new(Mutex::new(Vec::<String>::new()));
    let completed_tasks = Arc::new(AtomicUsize::new(0));
    
    // สร้าง workers
    let mut worker_handles = vec![];
    
    for worker_id in 0..num_workers {
        let queue = Arc::clone(&queue_handles[worker_id]);
        let shared_work = Arc::clone(&shared_work);
        let completed_tasks = Arc::clone(&completed_tasks);
        
        let handle = thread::spawn(move || {
            println!("👷 Worker {worker_id} เริ่มทำงาน!");
            let mut local_completed = 0;
            
            // ทำงานจาก local queue ก่อน
            while let Ok(task) = {
                let queue = queue.lock().unwrap();
                queue.recv()
            } {
                println!("🔨 Worker {worker_id} ทำงาน: {task}");
                thread::sleep(Duration::from_millis(100)); // จำลองการทำงาน
                local_completed += 1;
                completed_tasks.fetch_add(1, Ordering::SeqCst);
            }
            
            // เมื่อ local queue หมด ลองขโมยงานจาก shared pool
            loop {
                let stolen_task = {
                    let mut shared = shared_work.lock().unwrap();
                    shared.pop()
                };
                
                if let Some(task) = stolen_task {
                    println!("😈 Worker {worker_id} ขโมยงาน: {task}");
                    thread::sleep(Duration::from_millis(100));
                    local_completed += 1;
                    completed_tasks.fetch_add(1, Ordering::SeqCst);
                } else {
                    // ไม่มีงานให้ขโมย ลองรอสักหน่อย
                    thread::sleep(Duration::from_millis(50));
                    
                    // ถ้างานเสร็จหมดแล้ว ออกจาก loop
                    if completed_tasks.load(Ordering::SeqCst) >= total_tasks {
                        break;
                    }
                }
            }
            
            println!("✅ Worker {worker_id} เสร็จงาน! (ทำงาน {local_completed} งาน)");
            local_completed
        });
        
        worker_handles.push(handle);
    }
    
    // รอให้ workers เสร็จ
    let mut total_completed = 0;
    for handle in worker_handles {
        total_completed += handle.join().unwrap();
    }
    
    println!("\n📊 === สรุปผล Work Stealing === 📊");
    println!("🎯 งานทั้งหมด: {total_tasks} งาน");
    println!("✅ งานที่เสร็จ: {total_completed} งาน");
    println!("🎉 Work Stealing ช่วยให้การทำงานเสร็จเร็วขึ้น!");
}

/// ฟังก์ชันหลักสำหรับทดสอบ Concurrency - โชว์ทุกเทคนิค! 🎭⚡
pub fn practice_concurrency() {
    println!("⚡ === แบบฝึกหัด Concurrency: สนามรบของ Threads และ Async! === ⚡");
    
    // ทดสอบ Download Manager
    println!("\n📥 === Download Manager: ระบบดาวน์โหลดแบบ Concurrent! === 📥");
    
    let download_manager = DownloadManager::new(3);
    
    let download_tasks = vec![
        DownloadTask::new(1, "https://example.com/file1.zip".to_string(), 5, 1),
        DownloadTask::new(2, "https://example.com/file2.pdf".to_string(), 3, 2),
        DownloadTask::new(3, "https://example.com/file3.mp4".to_string(), 10, 1),
        DownloadTask::new(4, "https://example.com/file4.jpg".to_string(), 2, 3),
        DownloadTask::new(5, "https://example.com/file5.doc".to_string(), 4, 2),
        DownloadTask::new(6, "https://example.com/file6.mp3".to_string(), 6, 1),
        DownloadTask::new(7, "https://example.com/file7.txt".to_string(), 1, 3),
        DownloadTask::new(8, "https://example.com/file8.xlsx".to_string(), 3, 2),
    ];
    
    download_manager.download_batch(download_tasks);
    
    // ทดสอบ Chat Room
    println!("\n💬 === Chat Room: สนทนาแบบ Multi-threaded! === 💬");
    
    let chat_room = Arc::new(ChatRoom::new());
    
    // เพิ่มผู้ใช้
    chat_room.join_user("Alice".to_string()).unwrap();
    chat_room.join_user("Bob".to_string()).unwrap();
    chat_room.join_user("Charlie".to_string()).unwrap();
    
    // สร้าง threads สำหรับผู้ใช้แต่ละคน
    let mut chat_handles = vec![];
    
    // Alice thread
    let chat_room_alice = Arc::clone(&chat_room);
    let alice_handle = thread::spawn(move || {
        chat_room_alice.send_message("Alice".to_string(), "สวัสดีทุกคน! 👋".to_string());
        thread::sleep(Duration::from_millis(100));
        chat_room_alice.send_message("Alice".to_string(), "วันนี้เป็นยังไงบ้าง? 😊".to_string());
        thread::sleep(Duration::from_millis(200));
        chat_room_alice.send_message("Alice".to_string(), "ฉันกำลังเรียน Rust อยู่! 🦀".to_string());
    });
    
    // Bob thread
    let chat_room_bob = Arc::clone(&chat_room);
    let bob_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        chat_room_bob.send_message("Bob".to_string(), "สวัสดี Alice! 👋".to_string());
        thread::sleep(Duration::from_millis(150));
        chat_room_bob.send_message("Bob".to_string(), "ดีมาก! กำลังทำโปรเจกต์อยู่ 💻".to_string());
        thread::sleep(Duration::from_millis(100));
        chat_room_bob.send_message("Bob".to_string(), "Rust เจ๋งมาก! 🚀".to_string());
    });
    
    // Charlie thread
    let chat_room_charlie = Arc::clone(&chat_room);
    let charlie_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(75));
        chat_room_charlie.send_message("Charlie".to_string(), "เฮ้ย! มีใครอยู่มั้ย? 🤔".to_string());
        thread::sleep(Duration::from_millis(250));
        chat_room_charlie.send_message("Charlie".to_string(), "ผมก็เรียน Rust เหมือนกัน! 📚".to_string());
        thread::sleep(Duration::from_millis(100));
        chat_room_charlie.send_message("Charlie".to_string(), "มาช่วยกันทำโปรเจกต์มั้ย? 🤝".to_string());
    });
    
    chat_handles.push(alice_handle);
    chat_handles.push(bob_handle);
    chat_handles.push(charlie_handle);
    
    // รอให้การสนทนาเสร็จ
    for handle in chat_handles {
        handle.join().unwrap();
    }
    
    // แสดงสถิติ chat room
    println!("\n📊 === สถิติ Chat Room === 📊");
    println!("👥 ผู้ใช้ทั้งหมด: {} คน", chat_room.get_user_count());
    println!("💬 ข้อความทั้งหมด: {} ข้อความ", chat_room.get_message_count());
    println!("👤 รายชื่อผู้ใช้: {:?}", chat_room.get_users());
    
    // แสดงข้อความล่าสุด
    println!("\n📝 === ข้อความล่าสุด 5 ข้อความ === 📝");
    let recent_messages = chat_room.get_recent_messages(5);
    for (i, message) in recent_messages.iter().enumerate() {
        println!("  {}. [{}] {}: {}", i + 1, 
            message.timestamp.elapsed().as_millis(), 
            message.user, message.content);
    }
    
    // ผู้ใช้ออกจากห้อง
    chat_room.leave_user("Charlie");
    
    // ทดสอบ patterns อื่นๆ
    producer_consumer_example();
    parallel_computing_example();
    deadlock_prevention_example();
    work_stealing_example();
    
    println!("\n🎉 จบแบบฝึกหัด Concurrency! (เป็นนักสู้ Multi-threading แล้ว! ⚡👨‍💻)");
}

/// ตัวอย่างการใช้ Scoped Threads - Threads ที่ปลอดภัย! 🔒🧵
pub fn scoped_threads_example() {
    println!("\n🔒 === Scoped Threads: Threads ที่ปลอดภัย! === 🔒");
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut results = vec![0; data.len()];
    
    println!("🧮 คำนวณกำลังสองของตัวเลขแบบ parallel!");
    
    // ใช้ scoped threads เพื่อให้สามารถ borrow data ได้
    thread::scope(|s| {
        let chunk_size = data.len() / 2;
        
        // แบ่งงานเป็น 2 ส่วน
        let (left_data, right_data) = data.split_at(chunk_size);
        let (left_results, right_results) = results.split_at_mut(chunk_size);
        
        // Thread สำหรับครึ่งแรก
        let handle1 = s.spawn(|| {
            println!("🧵 Thread 1: คำนวณ {} ตัวเลขแรก", left_data.len());
            for (i, &value) in left_data.iter().enumerate() {
                left_results[i] = value * value;
                thread::sleep(Duration::from_millis(50));
            }
            println!("✅ Thread 1: เสร็จแล้ว!");
        });
        
        // Thread สำหรับครึ่งหลัง
        let handle2 = s.spawn(|| {
            println!("🧵 Thread 2: คำนวณ {} ตัวเลขหลัง", right_data.len());
            for (i, &value) in right_data.iter().enumerate() {
                right_results[i] = value * value;
                thread::sleep(Duration::from_millis(50));
            }
            println!("✅ Thread 2: เสร็จแล้ว!");
        });
        
        // รอให้ threads เสร็จ (ไม่ต้อง join เพราะ scope จะรอให้อัตโนมัติ)
    });
    
    println!("\n📊 === ผลลัพธ์ === 📊");
    for (i, (&original, &squared)) in data.iter().zip(results.iter()).enumerate() {
        println!("  {}. {} ^ 2 = {}", i + 1, original, squared);
    }
    
    println!("🎉 Scoped Threads ช่วยให้เราใช้ references ได้อย่างปลอดภัย!");
}

/// ตัวอย่างการใช้ Thread Local Storage - ข้อมูลส่วนตัวของ Thread! 🗄️🧵
pub fn thread_local_example() {
    println!("\n🗄️ === Thread Local Storage: ข้อมูลส่วนตัวของ Thread! === 🗄️");
    
    thread_local! {
        static COUNTER: RefCell<usize> = const { RefCell::new(0) };
        static THREAD_NAME: RefCell<String> = const { RefCell::new(String::new()) };
    }
    
    let mut handles = vec![];
    
    for thread_id in 0..3 {
        let handle = thread::spawn(move || {
            // ตั้งชื่อ thread
            THREAD_NAME.with(|name| {
                *name.borrow_mut() = format!("Worker-{thread_id}");
            });
            
            // นับจำนวนการทำงาน
            for i in 0..5 {
                COUNTER.with(|counter| {
                    let mut count = counter.borrow_mut();
                    *count += 1;
                    
                    THREAD_NAME.with(|name| {
                        println!("🧵 {} ทำงานครั้งที่ {} (รอบที่ {})", 
                            name.borrow(), *count, i + 1);
                    });
                });
                
                thread::sleep(Duration::from_millis(100));
            }
            
            // แสดงผลสรุป
            COUNTER.with(|counter| {
                let count = counter.borrow();
                THREAD_NAME.with(|name| {
                    println!("✅ {} เสร็จงาน! ทำงานทั้งหมด {} ครั้ง", 
                        name.borrow(), *count);
                });
            });
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("🎉 Thread Local Storage ช่วยให้แต่ละ thread มีข้อมูลส่วนตัว!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_download_task() {
        let task = DownloadTask::new(1, "test.zip".to_string(), 5, 1);
        assert_eq!(task.id, 1);
        assert_eq!(task.url, "test.zip");
        assert_eq!(task.size_mb, 5);
        assert_eq!(task.priority, 1);
    }
    
    #[test]
    fn test_download_stats() {
        let stats = DownloadStats::new();
        
        stats.record_success(10);
        stats.record_failure();
        
        let (total, success, failed, total_mb, _) = stats.get_summary();
        assert_eq!(total, 2);
        assert_eq!(success, 1);
        assert_eq!(failed, 1);
        assert_eq!(total_mb, 10);
    }
    
    #[test]
    fn test_chat_room() {
        let chat_room = ChatRoom::new();
        
        chat_room.join_user("Alice".to_string()).unwrap();
        chat_room.join_user("Bob".to_string()).unwrap();
        
        assert_eq!(chat_room.get_user_count(), 2);
        
        chat_room.send_message("Alice".to_string(), "Hello".to_string());
        assert_eq!(chat_room.get_message_count(), 1);
        
        chat_room.leave_user("Alice");
        assert_eq!(chat_room.get_user_count(), 1);
    }
    
    #[test]
    fn test_atomic_operations() {
        let counter = Arc::new(AtomicUsize::new(0));
        
        let handles: Vec<_> = (0..10).map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }
    
    #[test]
    fn test_channel_communication() {
        let (sender, receiver) = mpsc::channel();
        
        thread::spawn(move || {
            for i in 0..5 {
                sender.send(i).unwrap();
            }
        });
        
        let mut received = vec![];
        for _ in 0..5 {
            received.push(receiver.recv().unwrap());
        }
        
        assert_eq!(received, vec![0, 1, 2, 3, 4]);
    }
}