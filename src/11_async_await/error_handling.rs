//! Async Error Handling - ศูนย์กู้ภัยแห่งอนาคต! 🚨⚡
//!
//! 🎯 **เป้าหมายการเรียนรู้:**
//! ยินดีต้อนรับสู่ศูนย์กู้ภัยที่ทันสมัยที่สุด! 🚨✨
//! ที่นี่คุณจะได้เรียนรู้การจัดการข้อผิดพลาดในโลก async อย่างมืออาชีพ! 🚀
//!
//! 🛠️ **สิ่งที่จะได้เรียนรู้:**
//! - 🚨 การจัดการ error ใน async context - ทีมกู้ภัยที่ไม่เคยพลาด!
//! - ✅ การใช้ Result กับ async functions - ระบบรายงานผลที่แม่นยำ!
//! - 📡 Error propagation ใน async code - ส่งสัญญาณขอความช่วยเหลือ!
//! - 🔄 การใช้ `try_join`! สำหรับ error handling - ทีมงานที่ช่วยเหลือกัน!
//!
//! ⚠️ **หมายเหตุสำคัญ (ข้อมูลเทคนิค):**
//! - 📦 `use tokio::time::sleep;` และ `use std::time::Duration;` อาจแสดง warning "unused import"
//! - 🔧 เหตุผล: imports เหล่านี้ใช้ในฟังก์ชัน async ที่ถูกเรียกจาก main module
//! - ⚡ การลบ imports เหล่านี้จะทำให้โค้ดไม่สามารถ compile ได้
//! - 🤖 Warnings เหล่านี้เกิดจากการที่ Rust analyzer ไม่สามารถตรวจสอบการใช้งานใน async context ได้อย่างสมบูรณ์

use std::time::Duration;
use tokio::time::sleep;

/// ตัวอย่างการจัดการ error ใน async context - ทีมกู้ภัยในภารกิจ! 🚨🚀
#[allow(clippy::items_after_statements)]
pub async fn async_error_handling() {
    println!("\n⚠️🚨 === ตัวอย่าง Error Handling ใน Async: ทีมกู้ภัยในภารกิจ! === 🚨⚠️");
    println!("🌟 เตรียมพบกับทีมกู้ภัยที่พร้อมรับมือทุกสถานการณ์! (ไม่มีภารกิจไหนยาก!) 🚁✨");

    async fn risky_operation(should_fail: bool) -> Result<String, &'static str> {
        sleep(Duration::from_millis(200)).await;

        if should_fail {
            Err("🚨 เกิดเหตุฉุกเฉิน! ต้องการความช่วยเหลือ! 🆘")
        } else {
            Ok("✅ ภารกิจสำเร็จ! ทุกคนปลอดภัย! 🎉".to_string())
        }
    }

    // ทดสอบ error handling - ส่งทีมกู้ภัยออกปฏิบัติการ! 🚁
    println!("🚁 ส่งทีมกู้ภัย 3 หน่วยออกปฏิบัติการ! (พร้อมรับมือทุกสถานการณ์!) 🎯");
    let results = tokio::join!(
        risky_operation(false),
        risky_operation(true),
        risky_operation(false)
    );

    #[allow(clippy::tuple_array_conversions)]
    let results_vec = [results.0, results.1, results.2];
    println!("📊 รายงานผลการปฏิบัติการ:");
    for (i, result) in results_vec.into_iter().enumerate() {
        match result {
            Ok(msg) => println!("✅🏆 หน่วยกู้ภัย {}: {} 🌟", i + 1, msg),
            Err(err) => println!("❌🚨 หน่วยกู้ภัย {}: {} 📡", i + 1, err),
        }
    }
    println!("🎊 ภารกิจเสร็จสิ้น! ทีมกู้ภัยกลับฐานแล้ว! 🏠✨");
}

/// ตัวอย่างการใช้ `try_join`! สำหรับ early error return - ทีมงานที่ช่วยเหลือกัน! 🤝⚡
#[allow(clippy::items_after_statements)]
pub async fn try_join_error_handling() {
    println!("\n🔄🤝 === ตัวอย่าง try_join! Error Handling: ทีมงานที่ช่วยเหลือกัน! === 🤝🔄");
    println!("🌟 เตรียมพบกับทีมงานที่ทำงานร่วมกัน! (หากใครล้มเหลว ทุกคนหยุดช่วยกัน!) 🚀✨");

    async fn fallible_task(id: u32, should_fail: bool) -> Result<String, String> {
        sleep(Duration::from_millis(100 * u64::from(id))).await;

        if should_fail {
            Err(format!("🚨 สมาชิกทีม {id} ต้องการความช่วยเหลือ! 🆘"))
        } else {
            Ok(format!("✅ สมาชิกทีม {id} ทำงานสำเร็จ! 🎉"))
        }
    }

    // ทดสอบกรณีที่ทุก task สำเร็จ - ทีมงานที่สมบูรณ์แบบ! 🏆
    println!("📋🏆 ทดสอบกรณีทีมงานที่สมบูรณ์แบบ:");
    match tokio::try_join!(
        fallible_task(1, false),
        fallible_task(2, false),
        fallible_task(3, false)
    ) {
        Ok((result1, result2, result3)) => {
            println!("✅🎊 ทุกสมาชิกทีมทำงานสำเร็จ! {result1}, {result2}, {result3} 🌟");
        }
        Err(e) => println!("❌🚨 ทีมงานมีปัญหา: {e} 📡"),
    }

    // ทดสอบกรณีที่มี task ล้มเหลว - ทีมงานที่ช่วยเหลือกัน! 🤝
    println!("\n📋🤝 ทดสอบกรณีทีมงานที่ช่วยเหลือกัน:");
    match tokio::try_join!(
        fallible_task(1, false),
        fallible_task(2, true), // สมาชิกคนนี้ต้องการความช่วยเหลือ! 🆘
        fallible_task(3, false)
    ) {
        Ok((result1, result2, result3)) => {
            println!("✅🎊 ทุกสมาชิกทีมทำงานสำเร็จ! {result1}, {result2}, {result3} 🌟");
        }
        Err(e) => println!("❌🤝 ทีมงานหยุดช่วยเหลือสมาชิก (คาดหวัง): {e} 💪"),
    }
    println!("🎉 นี่คือพลังของทีมงานที่ดี! ไม่ทิ้งใครไว้ข้างหลัง! 🌈✨");
}

/// ตัวอย่างการ propagate errors ใน async functions - สายการผลิตแห่งอนาคต! 🏭⚡
#[allow(clippy::items_after_statements)]
pub async fn error_propagation_example() {
    println!("\n🔄🏭 === ตัวอย่าง Error Propagation: สายการผลิตแห่งอนาคต! === 🏭🔄");
    println!("🌟 เตรียมพบกับสายการผลิตที่ต้องผ่านทุกขั้นตอน! (หากขั้นตอนใดล้มเหลว จะหยุดทันที!) 🚀✨");

    async fn step1() -> Result<String, &'static str> {
        sleep(Duration::from_millis(100)).await;
        Ok("🔧 ขั้นตอนที่ 1: เตรียมวัตถุดิบ สำเร็จ!".to_string())
    }

    async fn step2() -> Result<String, &'static str> {
        sleep(Duration::from_millis(100)).await;
        Err("⚠️ ขั้นตอนที่ 2: เครื่องจักรขัดข้อง!")
    }

    async fn step3() -> Result<String, &'static str> {
        sleep(Duration::from_millis(100)).await;
        Ok("📦 ขั้นตอนที่ 3: บรรจุภัณฑ์ สำเร็จ!".to_string())
    }

    async fn multi_step_operation() -> Result<String, &'static str> {
        let result1 = step1().await?; // ขั้นตอนแรก 🔧
        let result2 = step2().await?; // จะหยุดที่นี่เมื่อเครื่องจักรขัดข้อง! ⚠️
        let result3 = step3().await?; // ไม่มีโอกาสถึงขั้นตอนนี้ 📦

        Ok(format!("{result1} → {result2} → {result3}"))
    }

    match multi_step_operation().await {
        Ok(result) => println!("✅🏆 สายการผลิตทำงานสำเร็จ: {result} 🎉"),
        Err(e) => println!("❌🚨 สายการผลิตหยุดทำงาน: {e} 🔧"),
    }
    println!("🎯 นี่คือการทำงานของ Error Propagation! หยุดทันทีเมื่อพบปัญหา! 🌈✨");
}

/// ตัวอย่างการใช้ timeout กับ async operations - นาฬิกาจับเวลาแห่งอนาคต! ⏰⚡
#[allow(clippy::items_after_statements)]
pub async fn timeout_example() {
    println!("\n⏰🚀 === ตัวอย่าง Timeout Handling: นาฬิกาจับเวลาแห่งอนาคต! === 🚀⏰");
    println!("🌟 เตรียมพบกับการแข่งขันกับเวลา! (หากช้าเกินไป จะถูกหยุดทันที!) 🏃‍♂️✨");

    async fn slow_operation(duration: u64) -> Result<String, &'static str> {
         sleep(Duration::from_millis(duration)).await;
         Ok("🎯 ภารกิจเสร็จสิ้น! 🏁".to_string())
     }

    // ทดสอบ operation ที่เสร็จทันเวลา - นักวิ่งที่รวดเร็ว! 🏃‍♂️💨
    println!("📋🏃‍♂️ ทดสอบนักวิ่งที่รวดเร็ว (เวลาจำกัด 500ms):");
    match tokio::time::timeout(Duration::from_millis(500), slow_operation(300)).await {
        Ok(Ok(result)) => println!("✅🏆 เสร็จทันเวลา! {result} 🎉"),
        Ok(Err(e)) => println!("❌🚨 ภารกิจล้มเหลว: {e} 📡"),
        Err(_) => println!("⏰🚨 หมดเวลา! นักวิ่งช้าเกินไป! 🐌"),
    }

    // ทดสอบ operation ที่ timeout - นักวิ่งที่ช้า! 🐌
    println!("\n📋🐌 ทดสอบนักวิ่งที่ช้า (เวลาจำกัด 200ms):");
    match tokio::time::timeout(Duration::from_millis(200), slow_operation(500)).await {
        Ok(Ok(result)) => println!("✅🏆 เสร็จทันเวลา! {result} 🎉"),
        Ok(Err(e)) => println!("❌🚨 ภารกิจล้มเหลว: {e} 📡"),
        Err(_) => println!("⏰🚨 หมดเวลา! นักวิ่งช้าเกินไป! (คาดหวัง) 🐌💔"),
    }
    println!("🎯 นี่คือพลังของ Timeout! ไม่ให้ใครมาทำให้เราช้า! 🌈✨");
}

/// รันตัวอย่างทั้งหมดของ error handling - เปิดศูนย์กู้ภัยแห่งอนาคต! 🚑⚡
pub async fn run_error_handling_examples() {
    println!("🚀🚑 === เปิดศูนย์กู้ภัยแห่งอนาคต! === 🚑🚀");
    println!("🌟 ยินดีต้อนรับสู่ศูนย์ฝึกอบรมทีมกู้ภัยมืออาชีพ! 🎓✨");

    async_error_handling().await;
    try_join_error_handling().await;
    error_propagation_example().await;
    timeout_example().await;

    println!("\n🎉🏆 === ศูนย์กู้ภัยแห่งอนาคตปิดการฝึกอบรม! === 🏆🎉");
    println!("🌈 ขอบคุณที่เข้าร่วมการฝึกอบรม! ตอนนี้คุณพร้อมเป็นทีมกู้ภัยมืออาชีพแล้ว! 🚀✨");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_error_handling() {
        // Test helper function - async signature needed for consistency with real async operations
        #[allow(clippy::unused_async)]
        async fn test_operation(should_fail: bool) -> Result<i32, &'static str> {
            if should_fail {
                Err("test error")
            } else {
                Ok(42)
            }
        }

        // ทดสอบกรณีสำเร็จ
        let result = test_operation(false).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        // ทดสอบกรณีล้มเหลว
        let result = test_operation(true).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "test error");
    }

    #[test]
    async fn test_try_join_success() {
        // Test helper function - async signature needed for try_join! macro compatibility
        #[allow(clippy::unused_async)]
        async fn success_task(value: i32) -> Result<i32, &'static str> {
            Ok(value * 2)
        }

        let result = tokio::try_join!(success_task(1), success_task(2), success_task(3));

        assert!(result.is_ok());
        let (r1, r2, r3) = result.unwrap();
        assert_eq!((r1, r2, r3), (2, 4, 6));
    }

    #[test]
    async fn test_try_join_failure() {
        // Test helper function - async signature needed for try_join! macro compatibility
        #[allow(clippy::unused_async)]
        async fn maybe_fail_task(should_fail: bool) -> Result<i32, &'static str> {
            if should_fail { Err("failed") } else { Ok(42) }
        }

        let result = tokio::try_join!(
            maybe_fail_task(false),
            maybe_fail_task(true), // นี่จะล้มเหลว
            maybe_fail_task(false)
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "failed");
    }

    #[test]
    async fn test_timeout() {
        async fn quick_task() -> Result<i32, &'static str> {
            sleep(Duration::from_millis(10)).await;
            Ok(42)
        }

        async fn slow_task() -> Result<i32, &'static str> {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        }

        // ทดสอบ task ที่เสร็จทันเวลา
        let result = tokio::time::timeout(Duration::from_millis(100), quick_task()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), 42);

        // ทดสอบ task ที่ timeout
        let result = tokio::time::timeout(Duration::from_millis(50), slow_task()).await;
        assert!(result.is_err()); // timeout error
    }
}