//! Concurrency - การเขียนโปรแกรมแบบ Concurrent และ Parallel! ⚡🚀
//! 
//! บทนี้จะสอนเกี่ยวกับ:
//! - Threads และการจัดการ
//! - Channels สำหรับการสื่อสาร
//! - Shared State และ Synchronization
//! - Async/Await Programming
//! - Atomic Operations
//! - Lock-free Programming

use std::thread;
use std::time::Duration;

pub mod practice_concurrency;

pub use practice_concurrency::*;

/// รันตัวอย่าง Concurrency ทั้งหมด
pub fn run_concurrency_examples() {
    println!("⚡ === Concurrency Examples === ⚡");
    
    println!("\n🧵 === Basic Threading === 🧵");
    basic_threading_example();
    
    println!("\n📡 === Message Passing === 📡");
    message_passing_example();
    
    println!("\n🔄 === Shared State === 🔄");
    shared_state_example();
    
    println!("\n⚛️ === Atomic Operations === ⚛️");
    atomic_operations_example();
    
    println!("\n⚡ === แบบฝึกหัด Concurrency === ⚡");
    practice_concurrency::practice_concurrency();
    practice_concurrency::scoped_threads_example();
    practice_concurrency::thread_local_example();
    
    println!("\n✅ Concurrency examples completed!");
}

/// ตัวอย่างการใช้ Threads พื้นฐาน - เริ่มต้นกับ Threading! 🧵✨
fn basic_threading_example() {
    println!("🧵 สร้าง threads และรอให้เสร็จ:");
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("  🔥 Thread {i} เริ่มทำงาน!");
            
            for j in 1..=3 {
                println!("    Thread {i} กำลังทำงาน... ({j}/3)");
                thread::sleep(Duration::from_millis(500));
            }
            
            println!("  ✅ Thread {i} เสร็จงาน!");
            format!("Result from thread {i}")
        });
        
        handles.push(handle);
    }
    
    println!("⏳ รอให้ threads ทั้งหมดเสร็จ...");
    
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("📥 ได้ผลลัพธ์จาก thread {i}: {result}");
    }
    
    println!("🎉 Threading พื้นฐานเสร็จสิ้น!");
}

/// ตัวอย่างการใช้ Message Passing - สื่อสารด้วย Channels! 📡💌
fn message_passing_example() {
    use std::sync::mpsc;
    
    println!("📡 การสื่อสารด้วย channels:");
    
    let (sender, receiver) = mpsc::channel();
    
    // สร้าง producer thread
    let producer = thread::spawn(move || {
        let messages = ["สวัสดี! 👋",
            "ข้อความจาก producer 📨",
            "กำลังส่งข้อมูล... 📊",
            "เสร็จแล้ว! ✅"];
        
        for (i, message) in messages.iter().enumerate() {
            println!("  📤 Producer ส่ง: {message}");
            sender.send(format!("{} (ข้อความที่ {})", message, i + 1)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
        
        println!("  🏁 Producer เสร็จงาน!");
    });
    
    // รับข้อความใน main thread
    println!("  📥 Consumer รอรับข้อความ...");
    
    for received in receiver {
        println!("  📨 Consumer ได้รับ: {received}");
    }
    
    producer.join().unwrap();
    println!("🎉 Message passing เสร็จสิ้น!");
}

/// ตัวอย่างการใช้ Shared State - แชร์ข้อมูลอย่างปลอดภัย! 🔄🔒
fn shared_state_example() {
    use std::sync::{Arc, Mutex};
    
    println!("🔄 การแชร์ state ด้วย Arc<Mutex<T>>:");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("  🔢 Thread {} เพิ่มตัวนับ: {} (รอบที่ {})", i, *num, j + 1);
                drop(num); // ปล่อย lock
                
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = *counter.lock().unwrap();
    println!("🎯 ค่าสุดท้ายของตัวนับ: {final_count}");
    println!("🎉 Shared state เสร็จสิ้น!");
}

/// ตัวอย่างการใช้ Atomic Operations - ไม่ต้อง Lock! ⚛️⚡
fn atomic_operations_example() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    println!("⚛️ การใช้ atomic operations:");
    
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let atomic_counter = Arc::clone(&atomic_counter);
        
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let old_value = atomic_counter.fetch_add(1, Ordering::SeqCst);
                let new_value = old_value + 1;
                println!("  ⚛️ Thread {} เพิ่มค่า: {} -> {} (รอบที่ {})", 
                    i, old_value, new_value, j + 1);
                
                thread::sleep(Duration::from_millis(50));
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = atomic_counter.load(Ordering::SeqCst);
    println!("🎯 ค่าสุดท้าย (atomic): {final_value}");
    println!("🎉 Atomic operations เสร็จสิ้น!");
}