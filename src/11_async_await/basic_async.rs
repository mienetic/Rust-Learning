//! Basic Async/Await Examples - โรงเรียนสอนการทำงานแบบไม่ซิงค์! 🎭⚡
//!
//! ไฟล์นี้ประกอบด้วย:
//! - ตัวอย่างการใช้ async/await พื้นฐาน (เหมือนเรียนรู้การทำหลายอย่างพร้อมกัน!)
//! - การเปรียบเทียบ sequential vs concurrent execution (แข่งขันระหว่างคนทำงานทีละอย่าง vs ทำพร้อมกัน!)
//! - การใช้ `tokio::join`! macro (เวทมนตร์รวมพลัง!)

use std::time::Duration;
use tokio::time::sleep;

/// ตัวอย่างการใช้ async/await พื้นฐาน - โรงเรียนสอนการรอคอย! 🎓⏰
/// เรียนรู้ศิลปะการทำงานแบบไม่ซิงค์ เหมือนเป็นนักแสดงที่เล่นหลายบทพร้อมกัน! 🎭
#[allow(clippy::items_after_statements)]
pub async fn basic_async_example() {
    println!("\n🚀 === ตัวอย่าง Async/Await พื้นฐาน: โรงเรียนสอนการรอคอย! === 🚀");

    async fn slow_operation(name: &str, duration: u64) -> String {
        println!("⏳ เริ่มงาน: {name} (เหมือนเชฟเริ่มทำอาหาร!)");
        sleep(Duration::from_millis(duration)).await;  // รอแบบอดทน (เหมือนรอข้าวสุก!)
        println!("✅ เสร็จงาน: {name} (เสิร์ฟแล้ว!)");
        format!("ผลลัพธ์จาก {name} (จานอาหารสำเร็จรูป!)")
    }

    // รันงานแบบ sequential - เหมือนคนทำงานคนเดียวทำทีละอย่าง! 👤⏳
    println!("📋 รันงานแบบ Sequential (ทำทีละอย่างแบบคนขยัน!):");
    let start = std::time::Instant::now();  // เริ่มจับเวลา (เหมือนกรรมการแข่งขัน!)

    let result1 = slow_operation("งานที่ 1 (งานหนักแบบ heavy task!)", 1000).await;
    let result2 = slow_operation("งานที่ 2 (งานกลางแบบ medium task!)", 800).await;
    let result3 = slow_operation("งานที่ 3 (งานเบาแบบ light task!)", 600).await;

    println!("⏱️ เวลาที่ใช้ (Sequential): {:?} (รวมกันเยอะมาก!)", start.elapsed());
    println!("📊 ผลลัพธ์: {result1}, {result2}, {result3} (ทำเสร็จทีละอย่าง!)");
}

/// ตัวอย่างการใช้ join! สำหรับรัน tasks พร้อมกัน - ทีมงานซุปเปอร์ฮีโร่! 🦸‍♂️⚡
/// เรียนรู้การทำงานแบบ concurrent เหมือนทีม Avengers ที่ต่อสู้พร้อมกัน! 🦸‍♀️🦸‍♂️
#[allow(clippy::items_after_statements)]
pub async fn concurrent_tasks_example() {
    println!("\n⚡ === ตัวอย่าง Concurrent Tasks: ทีมงานซุปเปอร์ฮีโร่! === ⚡");

    async fn async_task(id: u32, duration: u64) -> u32 {
        println!("🏃 Task {id} เริ่มทำงาน (ซุปเปอร์ฮีโร่คนที่ {id} ออกปฏิบัติการ!)");
        sleep(Duration::from_millis(duration)).await;  // ใช้พลังพิเศษ (รอให้พลังชาร์จ!)
        println!("✅ Task {id} เสร็จแล้ว (ภารกิจสำเร็จ!)");
        id * 10  // คะแนนพลังที่ได้ (เหมือนคะแนนเกม!)
    }

    // รันงานแบบ concurrent ด้วย join! - เหมือนทีมซุปเปอร์ฮีโร่ออกปฏิบัติการพร้อมกัน! 🦸‍♂️🦸‍♀️🦸‍♂️
    println!("📋 รันงานแบบ Concurrent (join! - เวทมนตร์รวมพลัง!):");
    let start = std::time::Instant::now();  // เริ่มจับเวลาการต่อสู้ (เหมือนนาฬิกานับถอยหลัง!)

    let (result1, result2, result3) =
        tokio::join!(async_task(1, 1000), async_task(2, 800), async_task(3, 600));  // รวมพลังทั้ง 3 คน!

    println!("⏱️ เวลาที่ใช้ (Concurrent): {:?} (เร็วกว่าเยอะ!)", start.elapsed());
    println!("📊 ผลลัพธ์: {result1}, {result2}, {result3} (คะแนนรวมของทีม!)");
}

/// ตัวอย่างการใช้ spawn สำหรับ background tasks - โรงงานผลิตงานเบื้องหลัง! 🏭👻
/// เรียนรู้การสร้าง background tasks เหมือนมีผีช่วยทำงานเบื้องหลัง! 👻💼
pub async fn spawn_tasks_example() {
    println!("\n🎯 === ตัวอย่าง Spawn Tasks: โรงงานผลิตงานเบื้องหลัง! === 🎯");

    // สร้าง background tasks - เหมือนจ้างผีมาช่วยทำงาน! 👻💼
    let task1 = tokio::spawn(async {
        sleep(Duration::from_millis(500)).await;  // ผีคนที่ 1 ทำงานปานกลาง (เหมือนผีขยัน!)
        "Task 1 เสร็จแล้ว (ผีคนที่ 1 รายงานตัว!)"
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_millis(300)).await;  // ผีคนที่ 2 ทำงานเร็ว (เหมือนผีเร่งรีบ!)
        "Task 2 เสร็จแล้ว (ผีคนที่ 2 รายงานตัว!)"
    });

    let task3 = tokio::spawn(async {
        sleep(Duration::from_millis(700)).await;  // ผีคนที่ 3 ทำงานช้า (เหมือนผีขี้เกียจ!)
        "Task 3 เสร็จแล้ว (ผีคนที่ 3 รายงานตัว!)"
    });

    // รอผลลัพธ์จาก tasks - เหมือนรอผีมารายงานตัว! 👻📋
    match tokio::try_join!(task1, task2, task3) {
        Ok((result1, result2, result3)) => {
            println!("✅ ทุก tasks เสร็จแล้ว (ผีทั้งหมดรายงานตัวครบ!):");
            println!("   - {result1}");
            println!("   - {result2}");
            println!("   - {result3}");
        }
        Err(e) => println!("❌ มี task ที่ล้มเหลว: {e} (ผีคนไหนหายไป!)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_concurrent_execution() {
        let start = std::time::Instant::now();  // เริ่มจับเวลาการแข่งขัน! 🏁

        let (result1, result2) = tokio::join!(
            async {
                sleep(Duration::from_millis(100)).await;  // นักแข่งคนที่ 1! 🏃‍♂️
                "test1"  // ผลลัพธ์จากนักแข่งคนที่ 1! 🏆
            },
            async {
                sleep(Duration::from_millis(100)).await;  // นักแข่งคนที่ 2! 🏃‍♀️
                "test2"  // ผลลัพธ์จากนักแข่งคนที่ 2! 🏆
            }
        );

        let elapsed = start.elapsed();  // เวลาที่ใช้ในการแข่งขัน! ⏱️

        assert_eq!(result1, "test1");  // ตรวจสอบผลลัพธ์นักแข่งคนที่ 1! ✅
        assert_eq!(result2, "test2");  // ตรวจสอบผลลัพธ์นักแข่งคนที่ 2! ✅
        // Concurrent execution ควรใช้เวลาน้อยกว่า 200ms (เร็วกว่าการวิ่งแยกกัน!)
        assert!(elapsed < Duration::from_millis(200));
    }

    #[test]
    async fn test_spawn_tasks() {
        let task = tokio::spawn(async {
            sleep(Duration::from_millis(50)).await;  // ผีช่วยงานทำงาน 50ms! 👻
            42  // เลขมงคล! 🍀
        });

        let result = task.await.unwrap();  // รอผีมารายงานผล! 👻📋
        assert_eq!(result, 42);  // ตรวจสอบเลขมงคล! ✨
    }
}