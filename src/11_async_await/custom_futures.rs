//! Custom Futures Implementation - โรงงานผลิต Future แห่งอนาคต! 🏭⚡
//!
//! 🎯 **เป้าหมายการเรียนรู้:**
//! ยินดีต้อนรับสู่โรงงานผลิต Future ที่ทันสมัยที่สุด! 🏭✨
//! ที่นี่คุณจะได้เรียนรู้การสร้าง Future แบบกำหนดเองที่เจ๋งมาก! 🚀
//!
//! 🛠️ **สิ่งที่จะได้เรียนรู้:**
//! - 🎨 การสร้าง custom Future trait implementations - ออกแบบ Future ในแบบของคุณ!
//! - 📌 การใช้ Pin และ Context - เครื่องมือสำคัญของนักพัฒนา Future!
//! - 🔄 ตัวอย่าง state machine ใน Future - เครื่องจักรสถานะที่ฉลาด!
//! - 🔔 การทำงานกับ Waker - ระบบปลุกที่ไม่เคยพลาด!

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// ตัวอย่างการสร้าง custom Future - เครื่องนับถอยหลังแห่งอนาคต! 🚀⏰
struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    const fn new(count: u32) -> Self {
        Self { count }
    }
}

impl Future for CountdownFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready("🎉🚀 เครื่องนับถอยหลังเสร็จแล้ว! ยิงจรวดได้เลย! 🌟✨".to_string())
        } else {
            println!("⏰🔥 นับถอยหลัง: {} (เตรียมตัวให้พร้อม!) 🚀", self.count);
            self.count -= 1;

            // Wake up the task to poll again - ปลุกระบบให้ทำงานต่อ! 🔔
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub async fn custom_future_example() {
    println!("\n🔧🎨 === ตัวอย่าง Custom Future: เครื่องนับถอยหลังแห่งอนาคต! === 🎨🔧");
    println!("🌟 เตรียมพบกับการสร้าง Future แบบกำหนดเองที่เจ๋งที่สุด! (ทำเองได้!) 🛠️✨");

    let countdown = CountdownFuture::new(3);
    let result = countdown.await;
    println!("📢🎊 {result}");
}

/// ตัวอย่าง Timer Future ที่ใช้ระบบเวลาจริง - นาฬิกาแห่งอนาคต! ⏰🚀
struct TimerFuture {
    start_time: Option<Instant>,
    duration: Duration,
}

impl TimerFuture {
    const fn new(duration: Duration) -> Self {
        Self {
            start_time: None,
            duration,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // เริ่มจับเวลาในการ poll ครั้งแรก - เปิดนาฬิกาแห่งอนาคต! ⏰
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
            println!("⏱️🚀 เปิดนาฬิกาแห่งอนาคต! จับเวลา {:?} (แม่นยำระดับนาโน!) 🎯", self.duration);
        }

        let elapsed = self.start_time.unwrap().elapsed();

        if elapsed >= self.duration {
            println!("✅🎉 นาฬิกาแห่งอนาคตเสร็จแล้ว! ใช้เวลา {elapsed:?} (แม่นยำเป๊ะ!) 🏆⚡");
            Poll::Ready(())
        } else {
            println!("⏳💫 กำลังนับเวลา... เหลือ {:?} (อดใจหน่อยนะ!) 🕐✨", self.duration - elapsed);

            // ในการใช้งานจริง ควรใช้ proper timer system
            // แต่ที่นี่เราจะ wake ทันทีเพื่อให้เห็นการทำงาน - ระบบปลุกอัตโนมัติ! 🔔
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub async fn timer_future_example() {
    println!("\n⏰🎨 === ตัวอย่าง Timer Future: นาฬิกาแห่งอนาคต! === 🎨⏰");
    println!("🌟 เตรียมพบกับนาฬิกาที่แม่นยำที่สุดในจักรวาล! (ไม่เคยผิดพลาด!) 🎯✨");

    let timer = TimerFuture::new(Duration::from_millis(100));
    timer.await;
    println!("🎯🎊 นาฬิกาแห่งอนาคตเสร็จสิ้น! เวลาผ่านไปอย่างสวยงาม! 🌈⚡");
}

/// ตัวอย่าง State Machine Future - เครื่องจักรสถานะแห่งอนาคต! 🤖⚡
#[derive(Debug)]
enum TaskState {
    Starting,
    Processing(u32),
    Finishing,
    Done,
}

struct StateMachineFuture {
    state: TaskState,
    max_steps: u32,
}

impl StateMachineFuture {
    const fn new(max_steps: u32) -> Self {
        Self {
            state: TaskState::Starting,
            max_steps,
        }
    }
}

impl Future for StateMachineFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &self.state {
                TaskState::Starting => {
                    println!("🚀🤖 เปิดเครื่องจักรสถานะแห่งอนาคต! (ระบบออนไลน์!) ⚡✨");
                    self.state = TaskState::Processing(0);
                    // Continue to next state immediately - เปลี่ยนสถานะทันที! 🔄
                }
                TaskState::Processing(step) => {
                    println!("⚙️🔥 เครื่องจักรกำลังประมวลผล step: {}/{} (โหมดเทอร์โบ!) 💨🎯", step, self.max_steps);

                    if *step >= self.max_steps {
                        self.state = TaskState::Finishing;
                    } else {
                        self.state = TaskState::Processing(step + 1);
                        // Yield control back to executor - ส่งคืนการควบคุม! 🎮
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                }
                TaskState::Finishing => {
                    println!("🏁🎊 เครื่องจักรกำลังจบงาน... (เก็บของเครื่องมือ!) 🧰✨");
                    self.state = TaskState::Done;
                    // Continue to next state immediately - ไปสถานะสุดท้าย! 🎯
                }
                TaskState::Done => {
                    return Poll::Ready("✅🏆 เครื่องจักรสถานะเสร็จสมบูรณ์! ทำงานได้อย่างสวยงาม! 🌟🎉".to_string());
                }
            }
        }
    }
}

pub async fn state_machine_future_example() {
    println!("\n🔄🤖 === ตัวอย่าง State Machine Future: เครื่องจักรสถานะแห่งอนาคต! === 🤖🔄");
    println!("🌟 เตรียมพบกับเครื่องจักรที่ฉลาดที่สุด! (เปลี่ยนสถานะได้อย่างลื่นไหล!) 🎯✨");

    let state_machine = StateMachineFuture::new(3);
    let result = state_machine.await;
    println!("📢🎊 {result}");
}

/// ตัวอย่าง Future ที่รวม multiple async operations - สายการผลิตแห่งอนาคต! 🏭⚡
struct CombinedFuture {
    step: u32,
    results: Vec<String>,
}

impl CombinedFuture {
    const fn new() -> Self {
        Self {
            step: 0,
            results: Vec::new(),
        }
    }
}

impl Future for CombinedFuture {
    type Output = Vec<String>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.step {
            0 => {
                println!("📋🚀 Step 1: เตรียมข้อมูล (เปิดเครื่องจักร!) ⚙️✨");
                self.results.push("ข้อมูลเตรียมพร้อม - คุณภาพเยี่ยม!".to_string());
                self.step = 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            1 => {
                println!("📋🔥 Step 2: ประมวลผล (โหมดเทอร์โบ!) 💨🎯");
                self.results.push("ประมวลผลเสร็จ - ความเร็วสูงสุด!".to_string());
                self.step = 2;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            2 => {
                println!("📋🎊 Step 3: สรุปผล (ตรวจสอบคุณภาพ!) 🔍✅");
                self.results.push("สรุปผลเสร็จ - ผ่านมาตรฐาน!".to_string());
                self.step = 3;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            _ => {
                println!("✅🏆 ทุกขั้นตอนเสร็จสิ้น! สายการผลิตทำงานสมบูรณ์! 🌟🎉");
                Poll::Ready(self.results.clone())
            }
        }
    }
}

pub async fn combined_future_example() {
    println!("\n🔗🏭 === ตัวอย่าง Combined Future: สายการผลิตแห่งอนาคต! === 🏭🔗");
    println!("🌟 เตรียมพบกับสายการผลิตที่ทันสมัยที่สุด! (ทุกขั้นตอนลงตัว!) 🎯✨");

    let combined = CombinedFuture::new();
    let results = combined.await;

    println!("📊🎉 ผลลัพธ์ทั้งหมดจากสายการผลิต:");
    for (i, result) in results.iter().enumerate() {
        println!("   🏆 {}. {} 🌟", i + 1, result);
    }
    println!("🎊✨ สายการผลิตแห่งอนาคตทำงานได้อย่างสมบูรณ์แบบ! 🚀🌈");
}

/// ฟังก์ชันรวมตัวอย่างทั้งหมด - ทัวร์โรงงานผลิต Future แห่งอนาคต! 🏭🎓
pub async fn run_custom_futures_examples() {
    println!("\n🛠️🎓 === ทัวร์โรงงานผลิต Future แห่งอนาคต! === 🎓🛠️");
    println!("🌟 ยินดีต้อนรับสู่การเดินทางสู่โลกแห่งการสร้าง Future! (ประสบการณ์ที่ไม่ลืม!) 🚀✨");

    println!("\n🎨 จุดแรก: เครื่องนับถอยหลังแห่งอนาคต! 🚀⏰");
    custom_future_example().await;
    
    println!("\n🎨 จุดที่สอง: นาฬิกาแห่งอนาคต! ⏰🚀");
    timer_future_example().await;
    
    println!("\n🎨 จุดที่สาม: เครื่องจักรสถานะแห่งอนาคต! 🤖⚡");
    state_machine_future_example().await;
    
    println!("\n🎨 จุดสุดท้าย: สายการผลิตแห่งอนาคต! 🏭⚡");
    combined_future_example().await;

    println!("\n🎉🏆 === จบทัวร์แล้ว! คุณเป็นมาสเตอร์ Future แล้ว! === 🏆🎉");
    println!("\n💡🌟 สิ่งที่คุณเรียนรู้ได้ (ทักษะใหม่ที่เจ๋งมาก!):");
    println!("   🎨⚡ การสร้าง custom Future trait - ออกแบบ Future ในแบบของคุณ!");
    println!("   📌⚡ การใช้ Pin และ Context - เครื่องมือสำคัญของนักพัฒนา!");
    println!("   🔄⚡ การจัดการ state ใน Future - เครื่องจักรสถานะที่ฉลาด!");
    println!("   🔔⚡ การใช้ Waker สำหรับ polling - ระบบปลุกที่ไม่เคยพลาด!");
    println!("\n🌈✨ ขอแสดงความยินดี! คุณพร้อมสร้าง Future แบบกำหนดเองแล้ว! 🚀🎊");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_countdown_future() {
        let countdown = CountdownFuture::new(2);
        let result = countdown.await;
        assert!(result.contains("เสร็จแล้ว"));
    }

    #[test]
    async fn test_timer_future() {
        let start = Instant::now();
        let timer = TimerFuture::new(Duration::from_millis(50));
        timer.await;
        let elapsed = start.elapsed();

        // Timer ควรใช้เวลาอย่างน้อย 50ms
        assert!(elapsed >= Duration::from_millis(45)); // ให้ tolerance เล็กน้อย
    }

    #[test]
    async fn test_state_machine_future() {
        let state_machine = StateMachineFuture::new(2);
        let result = state_machine.await;
        assert!(result.contains("เสร็จสมบูรณ์"));
    }

    #[test]
    async fn test_combined_future() {
        let combined = CombinedFuture::new();
        let results = combined.await;

        assert_eq!(results.len(), 3);
        assert!(results[0].contains("เตรียมพร้อม"));
        assert!(results[1].contains("ประมวลผล"));
        assert!(results[2].contains("สรุปผล"));
    }
}
