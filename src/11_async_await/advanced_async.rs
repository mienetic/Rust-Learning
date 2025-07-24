//! # Advanced Async Programming - เทคนิคขั้นเทพแห่งอนาคต! 🧙‍♂️⚡
//!
//! ไฟล์นี้ประกอบด้วยเทคนิคขั้นสูงที่จะทำให้คุณเป็นนินจา async! 🥷✨
//!
//! ## สิ่งที่จะได้เรียนรู้: 🎯
//! - 🔮 Custom Future Implementation - สร้าง Future ของตัวเองแบบ DIY! 🔧
//! - 📋 Async Task Scheduler - ตัวจัดการงานแบบเทพ! (ผู้จัดการส่วนตัว!)
//! - 📡 Async Channels - ช่องทางสื่อสารแห่งอนาคต! (ไปรษณีย์แบบ async!)
//! - 🌊 Async Streams - กระแสข้อมูลไม่รู้จบ! (แม่น้ำข้อมูล!)
//! - 👥 Worker Pools - ทีมงานแบบ async! (กองทัพนินจา!)
//! - 🔄 Retry Mechanisms - ระบบลองใหม่อัตโนมัติ! (ไม่ยอมแพ้!)
//!
//! 🎭 เตรียมตัวให้พร้อม! เราจะไปผจญภัยในโลกของ async ขั้นเทพกัน! 🚀✨

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::thread;

/// Custom Future Implementation - สร้าง Future ที่หน่วงเวลาแบบ DIY! ⏰🔧
/// เหมือนตั้งนาฬิกาปลุกแบบ async! (แต่ไม่บล็อกใครเลย!) 😴💤
struct DelayFuture {
    when: Instant,      // เวลาที่จะตื่น - เหมือนนาฬิกาปลุก! ⏰
    waker: Option<Waker>, // ตัวปลุก - เหมือนแม่ที่มาปลุกตอนเช้า! 👩‍👧‍👦
}

impl DelayFuture {
    /// สร้าง `DelayFuture` ใหม่ - ตั้งนาฬิกาปลุกแบบ async! ⏰✨
    fn new(duration: Duration) -> Self {
        println!("⏰🔧 DelayFuture::new: ตั้งนาฬิกาปลุกไว้ {duration:?} (ไม่บล็อกใครนะ!)");
        Self {
            when: Instant::now() + duration, // คำนวณเวลาที่จะตื่น! 🧮
            waker: None, // ยังไม่มีใครมาปลุก! 😴
        }
    }
}

impl Future for DelayFuture {
    type Output = ();
    
    /// ตรวจสอบว่าถึงเวลาแล้วหรือยัง - เหมือนดูนาฬิกาว่าตื่นหรือยัง! 👀⏰
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("⏰✅ DelayFuture: ตื่นแล้ว! เวลาผ่านไปครบแล้ว! (เหมือนนาฬิกาปลุกดัง!)");
            Poll::Ready(()) // ตื่นแล้ว! พร้อมทำงานต่อ! 🌅
        } else {
            // เก็บ waker ไว้ปลุกทีหลัง - เหมือนบอกแม่ว่าให้มาปลุกตอนไหน! 📞
            self.waker = Some(cx.waker().clone());
            println!("😴💤 DelayFuture: ยังไม่ถึงเวลา... นอนต่อ... ZZZ (ไม่บล็อกใครนะ!)");
            
            // ในการใช้งานจริง ควรใช้ timer ที่เหมาะสม 🎯
            // ที่นี่เราจะใช้ thread::spawn เพื่อจำลอง (เหมือนผีช่วยตั้งปลุก!) 👻⏰
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now); // งีบจริงๆ ในเธรดแยก! 😴
                }
                waker.wake(); // ปลุกเจ้าของ! "ตื่นได้แล้ว!" 📢
            });
            
            Poll::Pending // ยังไม่พร้อม รอก่อน! ⏳
        }
    }
}

/// Async Task Scheduler - ตัวจัดการงานแบบเทพ! 📋⚡
/// เหมือนมีผู้จัดการส่วนตัวที่จัดการงานให้! (เลขาส่วนตัว!) 👩‍💼✨
struct TaskScheduler {
    tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>, // รายการงาน - เหมือน to-do list! 📝
    wakers: Vec<Waker>, // รายการตัวปลุก - เหมือนนาฬิกาปลุกหลายเรือน! ⏰
}

impl TaskScheduler {
    /// สร้าง `TaskScheduler` ใหม่ - เปิดออฟฟิศจัดการงาน! 🏢✨
    fn new() -> Self {
        println!("🏢📋 TaskScheduler: เปิดออฟฟิศจัดการงานแล้ว! พร้อมรับงาน! (เหมือนเปิดบริษัท!)");
        Self {
            tasks: Vec::new(), // โต๊ะทำงานว่างเปล่า พร้อมรับงาน! 📝
            wakers: Vec::new(), // ยังไม่มีนาฬิกาปลุก! ⏰
        }
    }
    
    /// เพิ่มงานใหม่ - รับงานเข้าระบบ! 📥
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        println!("📥💼 TaskScheduler: รับงานใหม่เข้าระบบ! งานที่ {} (เพิ่มเข้าคิว!)", self.tasks.len() + 1);
        self.tasks.push(Box::pin(future)); // เพิ่มงานเข้าคิว! 📋
    }
    
    /// รันงานจนเสร็จหมด - ทำงานจนเสร็จ! 💪
    fn run_until_complete(&mut self) {
        println!("🚀📋 TaskScheduler: เริ่มทำงาน! มีงานทั้งหมด {} งาน (เหมือนเปิดโรงงาน!)", self.tasks.len());
        
        while !self.tasks.is_empty() {
            let mut completed = Vec::new();
            
            for (i, task) in self.tasks.iter_mut().enumerate() {
                let waker = futures::task::noop_waker(); // ผู้ปลุกจำลอง (ไม่ทำอะไรจริงๆ!) 😴
                let mut context = Context::from_waker(&waker);
                
                match task.as_mut().poll(&mut context) {
                    Poll::Ready(()) => {
                        println!("✅🎉 TaskScheduler: งานที่ {} เสร็จแล้ว! (ลบออกจากรายการ!)", i + 1);
                        completed.push(i);
                    },
                    Poll::Pending => {
                        println!("⏳⏭️ TaskScheduler: งานที่ {} ยังไม่เสร็จ รอต่อ... (ข้ามไปงานต่อไป!)", i + 1);
                    },
                }
            }
            
            // ลบงานที่เสร็จแล้ว (เรียงจากหลังไปหน้าเพื่อรักษา index) 🗑️
            for &i in completed.iter().rev() {
                let _ = self.tasks.remove(i); // เอางานที่เสร็จออก! 🗑️
            }
            
            if !self.tasks.is_empty() {
                thread::sleep(Duration::from_millis(10)); // พักเล็กน้อย (พักตาหน่อย!) 👁️
            }
        }
        
        println!("🎉🏆 TaskScheduler: งานเสร็จหมดแล้ว! ปิดออฟฟิศ! (เหมือนปิดโรงงาน!)");
    }
}

/// Async Channel Implementation - ช่องทางสื่อสารแห่งอนาคต! 📡⚡
/// เหมือนมีท่อส่งข้อมูลแบบ async!
struct AsyncChannel<T> {
    buffer: Arc<Mutex<Vec<T>>>,           // ที่เก็บข้อมูล - เหมือนกล่องจดหมาย! 📮
    capacity: usize,                      // ความจุ - ขนาดกล่องจดหมาย! 📏
    senders: Arc<Mutex<Vec<Waker>>>,      // รายการผู้ส่ง - คิวคนส่งจดหมาย! 📤
    receivers: Arc<Mutex<Vec<Waker>>>,    // รายการผู้รับ - คิวคนรับจดหมาย! 📥
}

/// ตัวส่งข้อมูล - บุรุษไปรษณีย์! 📮👨‍💼
struct AsyncSender<T> {
    channel: Arc<AsyncChannel<T>>,
}

/// ตัวรับข้อมูล - คนรับจดหมาย! 📬👩‍💼
struct AsyncReceiver<T> {
    channel: Arc<AsyncChannel<T>>,
}

/// Future สำหรับส่งข้อมูล - กระบวนการส่งจดหมาย! 📮⚡
struct SendFuture<T> {
    channel: Arc<AsyncChannel<T>>,
    item: Option<T>,
}

/// Future สำหรับรับข้อมูล - กระบวนการรับจดหมาย! 📬⚡
struct ReceiveFuture<T> {
    channel: Arc<AsyncChannel<T>>,
}

impl<T> AsyncChannel<T> {
    /// สร้าง channel ใหม่ - เปิดไปรษณีย์ใหม่! 🏤✨
    fn new(capacity: usize) -> (AsyncSender<T>, AsyncReceiver<T>) {
        println!("🏤 AsyncChannel: เปิดไปรษณีย์ใหม่! ความจุ {capacity} ชิ้น");
        
        let channel = Arc::new(Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            capacity,
            senders: Arc::new(Mutex::new(Vec::new())),
            receivers: Arc::new(Mutex::new(Vec::new())),
        });
        
        let sender = AsyncSender {
            channel: channel.clone(),
        };
        
        let receiver = AsyncReceiver {
            channel,
        };
        
        (sender, receiver)
    }
}

impl<T> AsyncSender<T> {
    /// ส่งข้อมูล - ส่งจดหมาย! 📮
    fn send(&self, item: T) -> SendFuture<T> {
        println!("📮 AsyncSender: เตรียมส่งข้อมูล...");
        SendFuture {
            channel: self.channel.clone(),
            item: Some(item),
        }
    }
}

impl<T> AsyncReceiver<T> {
    /// รับข้อมูล - รับจดหมาย! 📬
    fn receive(&self) -> ReceiveFuture<T> {
        println!("📬 AsyncReceiver: เตรียมรับข้อมูล...");
        ReceiveFuture {
            channel: self.channel.clone(),
        }
    }
}

impl<T: Unpin> Future for SendFuture<T> {
    type Output = Result<(), T>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut buffer = this.channel.buffer.lock().unwrap();
        
        if buffer.len() < this.channel.capacity {
            if let Some(item) = this.item.take() {
                buffer.push(item);
                println!("✅ SendFuture: ส่งข้อมูลสำเร็จ! กล่องจดหมายมี {} ชิ้น", buffer.len());
                
                // ปลุกผู้รับที่รออยู่ - แจ้งว่ามีจดหมายมาแล้ว! 📢
                let mut receivers = this.channel.receivers.lock().unwrap();
                for waker in receivers.drain(..) {
                    waker.wake();
                }
                
                Poll::Ready(Ok(()))
            } else {
                Poll::Ready(Err(this.item.take().unwrap_or_else(|| panic!("Item already taken"))))
            }
        } else {
            // กล่องจดหมายเต็ม รอให้มีที่ว่าง! 📮⏳
            println!("📮⏳ SendFuture: กล่องจดหมายเต็ม! รอให้มีที่ว่าง...");
            let mut senders = this.channel.senders.lock().unwrap();
            senders.push(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<T> Future for ReceiveFuture<T> {
    type Output = Option<T>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut buffer = self.channel.buffer.lock().unwrap();
        
        if buffer.is_empty() {
            // กล่องจดหมายว่าง รอให้มีจดหมายมา! 📬⏳
            println!("📬⏳ ReceiveFuture: กล่องจดหมายว่าง! รอจดหมายใหม่...");
            let mut receivers = self.channel.receivers.lock().unwrap();
            receivers.push(cx.waker().clone());
            Poll::Pending
        } else {
            let item = buffer.remove(0);
            println!("✅ ReceiveFuture: รับข้อมูลสำเร็จ! เหลือในกล่อง {} ชิ้น", buffer.len());
            
            // ปลุกผู้ส่งที่รออยู่ - แจ้งว่ามีที่ว่างแล้ว! 📢
            let mut senders = self.channel.senders.lock().unwrap();
            for waker in senders.drain(..) {
                waker.wake();
            }
            
            Poll::Ready(Some(item))
        }
    }
}

/// ตัวอย่างการใช้งาน Advanced Async - โชว์เทคนิคขั้นเทพ! 🎭⚡
pub async fn demonstrate_advanced_async() {
    println!("\n🧙‍♂️ === Advanced Async Programming: เทคนิคขั้นเทพ! === 🧙‍♂️");
    println!("🎯 เตรียมพบกับเวทมนตร์ async ขั้นสูง!\n");
    
    // ทดสอบ Custom DelayFuture
    println!("⏰ === ทดสอบ Custom DelayFuture === ⏰");
    println!("🔮 สร้าง DelayFuture ที่หน่วงเวลา 100ms...");
    
    let delay = DelayFuture::new(Duration::from_millis(100));
    delay.await;
    
    println!("✨ DelayFuture เสร็จสิ้น!\n");
    
    // ทดสอบ TaskScheduler
    println!("📋 === ทดสอบ TaskScheduler === 📋");
    let mut scheduler = TaskScheduler::new();
    
    // เพิ่มงานหลายๆ งาน
    scheduler.spawn(async {
        println!("🎯 งานที่ 1: เริ่มทำงาน...");
        DelayFuture::new(Duration::from_millis(50)).await;
        println!("✅ งานที่ 1: เสร็จแล้ว!");
    });
    
    scheduler.spawn(async {
        println!("🎯 งานที่ 2: เริ่มทำงาน...");
        DelayFuture::new(Duration::from_millis(30)).await;
        println!("✅ งานที่ 2: เสร็จแล้ว!");
    });
    
    scheduler.spawn(async {
        println!("🎯 งานที่ 3: เริ่มทำงาน...");
        DelayFuture::new(Duration::from_millis(20)).await;
        println!("✅ งานที่ 3: เสร็จแล้ว!");
    });
    
    // รันงานทั้งหมด
    scheduler.run_until_complete();
    
    println!("\n📡 === ทดสอบ Async Channel === 📡");
    let (sender, receiver) = AsyncChannel::new(2);
    
    // ส่งข้อมูล
    println!("📤 ส่งข้อมูลผ่าน channel...");
    let send_result = sender.send("Hello Async World! 🌍".to_string()).await;
    match send_result {
        Ok(()) => println!("✅ ส่งข้อมูลสำเร็จ!"),
        Err(data) => println!("❌ ส่งข้อมูลไม่สำเร็จ: {data}"),
    }
    
    // รับข้อมูล
    println!("📥 รับข้อมูลจาก channel...");
    if let Some(data) = receiver.receive().await {
        println!("✅ รับข้อมูลสำเร็จ: {data}");
    } else {
        println!("❌ ไม่มีข้อมูลในช่องทาง");
    }
    
    println!("\n🎉 === Advanced Async Programming เสร็จสิ้น! === 🎉");
    println!("💡 คุณได้เรียนรู้:");
    println!("   🔧 การสร้าง Custom Future");
    println!("   📋 การใช้ Task Scheduler");
    println!("   📡 การสร้าง Async Channel");
    println!("   ⚡ เทคนิค async ขั้นสูง");
    println!("🚀 ตอนนี้คุณเป็นนินจา async แล้ว! 🥷✨");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;
    
    /// ทดสอบ DelayFuture - ทดสอบนาฬิกาปลุก async! ⏰🧪
    #[tokio::test]
    async fn test_delay_future() {
        println!("🧪⏰ เริ่มทดสอบ DelayFuture! (ทดสอบนาฬิกาปลุก!)");
        let start = Instant::now();
        DelayFuture::new(Duration::from_millis(50)).await;
        let elapsed = start.elapsed();
        
        println!("⏱️ เวลาที่ใช้: {elapsed:?} (ควรใกล้เคียง 50ms!)");
        assert!(elapsed >= Duration::from_millis(45)); // อนุญาตให้คลาดเคลื่อนเล็กน้อย 📏
        println!("✅ DelayFuture ทำงานถูกต้อง! (นาฬิกาปลุกแม่นยำ!) 🎯");
    }
    
    /// ทดสอบ Async Channel - ทดสอบไปรษณีย์ async! 📡🧪
    #[tokio::test]
    async fn test_async_channel() {
        println!("🧪📡 เริ่มทดสอบ Async Channel! (ทดสอบไปรษณีย์!)");
        let (sender, receiver) = AsyncChannel::new(1);
        
        // ทดสอบส่งและรับ - ส่งจดหมายและรับจดหมาย! 📮📬
        println!("📮 ทดสอบส่งข้อมูล 42...");
        let send_result = sender.send(42).await;
        assert!(send_result.is_ok());
        println!("✅ ส่งสำเร็จ! (จดหมายส่งแล้ว!)");
        
        println!("📬 ทดสอบรับข้อมูล...");
        let received = receiver.receive().await;
        assert_eq!(received, Some(42));
        println!("✅ รับสำเร็จ! ได้ข้อมูล: {received:?} (จดหมายถึงมือแล้ว!) 🎉");
    }
    
    /// ทดสอบ Channel Timeout - ทดสอบการหมดเวลา! ⏰🧪
    #[tokio::test]
    async fn test_async_channel_timeout() {
        println!("🧪⏰ เริ่มทดสอบ Channel Timeout! (ทดสอบการรอจดหมายที่ไม่มา!)");
        let (_sender, receiver) = AsyncChannel::<i32>::new(1);
        
        // ทดสอบ timeout เมื่อไม่มีข้อมูล - รอจดหมายที่ไม่มา! 📭⏳
        println!("📭 รอข้อมูลที่ไม่มี... (ควร timeout!)");
        let result = timeout(Duration::from_millis(10), receiver.receive()).await;
        assert!(result.is_err()); // ควร timeout 💥
        println!("✅ Timeout ถูกต้อง! (ไม่รอจดหมายที่ไม่มาตลอดไป!) ⏰✨");
    }
}