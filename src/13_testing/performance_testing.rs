//! # Performance Testing Examples - ห้องทดลองประสิทธิภาพนักสืบ! ⚡🕵️‍♂️
//!
//! ตัวอย่างการทดสอบประสิทธิภาพและ benchmark testing
//! เหมือนการวัดความเร็วของเครื่องมือสืบสวนต่างๆ! 🔍⏱️
//!
//! 🎯 **เป้าหมาย**: เรียนรู้การวัดและเปรียบเทียบประสิทธิภาพ!

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// ฟังก์ชันสำหรับคำนวณ Fibonacci แบบ recursive - วิธีนักสืบมือใหม่! 🐌🔍
#[must_use]  // 🚨 ต้องใช้ผลลัพธ์!
pub fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// ฟังก์ชันสำหรับคำนวณ Fibonacci แบบ iterative - วิธีนักสืบมืออาชีพ! ⚡🔍
#[must_use]  // 🚨 ต้องใช้ผลลัพธ์!
pub fn fibonacci_iterative(n: u32) -> u64 {
    if n <= 1 {
        return u64::from(n);
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

/// ฟังก์ชันสำหรับคำนวณ Fibonacci แบบ memoization - วิธีนักสืบอัจฉริยะ! 🧠💾
/// 
/// # หมายเหตุ
/// ใช้ `#[allow(clippy::implicit_hasher)]` เพราะ `HashMap`<u32, u64> เป็น type ที่ชัดเจนแล้ว
/// และไม่จำเป็นต้องใช้ generic hasher ในกรณีนี้
#[allow(clippy::implicit_hasher)]  // 🤫 ปิดเสียง warning
pub fn fibonacci_memoized(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if let Some(&result) = memo.get(&n) {
        return result;  // 💾 พบหลักฐานในความจำ - ไม่ต้องคำนวณใหม่!
    }

    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo),
    };

    memo.insert(n, result);  // 💾 เก็บหลักฐานไว้ในความจำ
    result  // 📊 ส่งคืนผลลัพธ์
}

/// ฟังก์ชันสำหรับการเรียงลำดับแบบ bubble sort - วิธีนักสืบมือใหม่! 🫧🐌
pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// ฟังก์ชันสำหรับการเรียงลำดับแบบ quick sort - วิธีนักสืบมืออาชีพ! ⚡🎯
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

/// ฟังก์ชันสำหรับวัดเวลาการทำงาน - นาฬิกานักสืบ! ⏱️🔍
pub fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();  // ⏰ เริ่มจับเวลา
    let result = f();            // 🔍 ทำการสืบสวน
    let duration = start.elapsed();  // ⏱️ คำนวณเวลาที่ใช้
    (result, duration)  // 📊 ส่งคืนผลลัพธ์และเวลา
}

/// ฟังก์ชันสำหรับ benchmark หลายรอบ - การทดสอบความแข็งแกร่ง! 🏃‍♂️📊
pub fn benchmark<F>(name: &str, iterations: usize, mut f: F)
where
    F: FnMut(),
{
    let mut total_duration = Duration::new(0, 0);
    let mut min_duration = Duration::from_secs(u64::MAX);
    let mut max_duration = Duration::new(0, 0);

    for _ in 0..iterations {
        let ((), duration) = measure_time(&mut f);
        total_duration += duration;

        if duration < min_duration {
            min_duration = duration;
        }
        if duration > max_duration {
            max_duration = duration;
        }
    }

    // ใช้ #[allow(clippy::cast_possible_truncation)] เพราะ iterations มาจาก parameter
    // และมั่นใจว่าจะไม่เกิน u32::MAX ในการใช้งานจริง
    #[allow(clippy::cast_possible_truncation)]
    let avg_duration = total_duration / iterations as u32;

    println!("📊🔍 Benchmark: {name}");
    println!("   🔄 Iterations: {iterations} (รอบการทดสอบ)");
    println!("   ⏱️ Average: {avg_duration:?} (เวลาเฉลี่ย)");
    println!("   ⚡ Min: {min_duration:?} (เร็วที่สุด)");
    println!("   🐌 Max: {max_duration:?} (ช้าที่สุด)");
    println!("   📈 Total: {total_duration:?} (เวลารวม)");
}

/// ตัวอย่างการใช้งาน Performance testing - การแข่งขันความเร็วนักสืบ! ⚡🕵️‍♂️
pub fn performance_testing_examples() {
    println!("⚡✨ === Performance Testing Examples - ห้องทดลองประสิทธิภาพ! === ✨⚡");
    println!("🕵️‍♂️ เริ่มการแข่งขันความเร็วของเครื่องมือสืบสวน! 🏁");

    // 🔢 ทดสอบ Fibonacci algorithms - การแข่งขันคำนวณลำดับ!
    println!("\n🔢🏁 === Fibonacci Performance Comparison - การแข่งขันคำนวณ! === 🏁🔢");

    let n = 30;

    // 🐌 Recursive (ช้า) - วิธีนักสืบมือใหม่!
    let (result_recursive, time_recursive) = measure_time(|| fibonacci_recursive(n));
    println!("🐌🔍 Recursive: fibonacci({n}) = {result_recursive} (เวลา: {time_recursive:?}) [นักสืบมือใหม่]");

    // ⚡ Iterative (เร็ว) - วิธีนักสืบมืออาชีพ!
    let (result_iterative, time_iterative) = measure_time(|| fibonacci_iterative(n));
    println!("⚡🔍 Iterative: fibonacci({n}) = {result_iterative} (เวลา: {time_iterative:?}) [นักสืบมืออาชีพ]");

    // 🧠 Memoized (เร็วสำหรับการเรียกซ้ำ) - วิธีนักสืบอัจฉริยะ!
    let (result_memoized, time_memoized) = measure_time(|| {
        let mut memo = HashMap::new();
        fibonacci_memoized(n, &mut memo)
    });
    println!("🧠💾 Memoized: fibonacci({n}) = {result_memoized} (เวลา: {time_memoized:?}) [นักสืบอัจฉริยะ]");

    // เปรียบเทียบความเร็ว
    if time_iterative < time_recursive {
        // ใช้ #[allow(clippy::cast_precision_loss)] เพราะการแปลง u128 เป็น f64
        // อาจสูญเสียความแม่นยำ แต่ยอมรับได้สำหรับการแสดงผลอัตราส่วน
        #[allow(clippy::cast_precision_loss)]
        let speedup = time_recursive.as_nanos() as f64 / time_iterative.as_nanos() as f64;
        println!("📈🏆 นักสืบมืออาชีพเร็วกว่านักสืบมือใหม่ {speedup:.2} เท่า!");
    }

    // 📊 ทดสอบ Sorting algorithms - การแข่งขันจัดเรียงหลักฐาน!
    println!("\n📊🏁 === Sorting Performance Comparison - การแข่งขันจัดเรียง! === 🏁📊");

    let size = 1000;
    let data: Vec<i32> = (0..size).rev().collect(); // 📋 หลักฐานที่ยุ่งเหยิง (เรียงจากมากไปน้อย)

    // 🫧 Bubble Sort - วิธีนักสืบมือใหม่!
    let mut bubble_data = data.clone();
    let ((), bubble_time) = measure_time(|| bubble_sort(&mut bubble_data));
    println!("🫧🐌 Bubble Sort: {size} elements (เวลา: {bubble_time:?}) [นักสืบมือใหม่]");

    // ⚡ Quick Sort - วิธีนักสืบมืออาชีพ!
    let mut quick_data = data.clone();
    let ((), quick_time) = measure_time(|| quick_sort(&mut quick_data));
    println!("⚡🎯 Quick Sort: {size} elements (เวลา: {quick_time:?}) [นักสืบมืออาชีพ]");

    // Built-in Sort
    // ใช้ #[allow(clippy::collection_is_never_read)] เพราะ builtin_data ถูกใช้เพื่อ benchmark
        // การ sort เท่านั้น ไม่ได้อ่านค่าหลังจากนั้น ซึ่งเป็นจุดประสงค์ของการทดสอบ
        #[allow(clippy::collection_is_never_read)]
        let mut builtin_data = data;
    let ((), builtin_time) = measure_time(|| builtin_data.sort_unstable());
    println!("🦀✨ Built-in Sort: {size} elements (เวลา: {builtin_time:?}) [เครื่องมือ Rust มืออาชีพ]");

    // เปรียบเทียบความเร็ว
    if quick_time < bubble_time {
        // ใช้ #[allow(clippy::cast_precision_loss)] เพราะการแปลง u128 เป็น f64
        // อาจสูญเสียความแม่นยำ แต่ยอมรับได้สำหรับการแสดงผลอัตราส่วน
        #[allow(clippy::cast_precision_loss)]
        let speedup = bubble_time.as_nanos() as f64 / quick_time.as_nanos() as f64;
        println!("📈🏆 นักสืบมืออาชีพเร็วกว่านักสืบมือใหม่ {speedup:.2} เท่า!");
    }

    // 🏃 Benchmark การคำนวณง่ายๆ - การทดสอบความแข็งแกร่ง!
    println!("\n🏃‍♂️💪 === Simple Benchmark - การทดสอบความแข็งแกร่ง! === 💪🏃‍♂️");

    benchmark("Vector push - การเก็บหลักฐาน", 10000, || {
        // ใช้ #[allow(clippy::collection_is_never_read)] เพราะ vec ถูกสร้างเพื่อ benchmark
        // การ push เท่านั้น ไม่ได้อ่านค่า ซึ่งเป็นจุดประสงค์ของการทดสอบประสิทธิภาพ
        #[allow(clippy::collection_is_never_read)]
        let mut vec = Vec::new();  // 📦 กล่องหลักฐานใหม่
        for i in 0..100 {
            vec.push(i);  // 📋 เก็บหลักฐานชิ้นที่ {i}
        }
    });

    benchmark("HashMap insert - การจัดเก็บหลักฐาน", 10000, || {
        // ใช้ #[allow(clippy::collection_is_never_read)] เพราะ map ถูกสร้างเพื่อ benchmark
        // การ insert เท่านั้น ไม่ได้อ่านค่า ซึ่งเป็นจุดประสงค์ของการทดสอบประสิทธิภาพ
        #[allow(clippy::collection_is_never_read)]
        let mut map = HashMap::new();  // 🗂️ ตู้เก็บหลักฐานใหม่
        for i in 0..100 {
            map.insert(i, i * 2);  // 🔑 เก็บหลักฐาน key: {i}, value: {i*2}
        }
    });
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]  // 🧪 ทดสอบนักสืบมือใหม่
    #[allow(clippy::missing_panics_doc)]
    pub fn test_fibonacci_recursive() {
        assert_eq!(fibonacci_recursive(0), 0);   // 🔍 ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_recursive(1), 1);   // 🔍 ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_recursive(5), 5);   // 🔍 ตรวจสอบกรณีปกติ
        assert_eq!(fibonacci_recursive(10), 55); // 🔍 ตรวจสอบกรณีซับซ้อน
    }

    #[test]  // 🧪 ทดสอบนักสืบมืออาชีพ
    #[allow(clippy::missing_panics_doc)]
    pub fn test_fibonacci_iterative() {
        assert_eq!(fibonacci_iterative(0), 0);   // ⚡ ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_iterative(1), 1);   // ⚡ ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_iterative(5), 5);   // ⚡ ตรวจสอบกรณีปกติ
        assert_eq!(fibonacci_iterative(10), 55); // ⚡ ตรวจสอบกรณีซับซ้อน
    }

    #[test]  // 🧪 ทดสอบนักสืบอัจฉริยะ
    #[allow(clippy::missing_panics_doc)]
    pub fn test_fibonacci_memoized() {
        let mut memo = HashMap::new();  // 💾 เตรียมความจำ
        assert_eq!(fibonacci_memoized(0, &mut memo), 0);   // 🧠 ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_memoized(1, &mut memo), 1);   // 🧠 ตรวจสอบกรณีฐาน
        assert_eq!(fibonacci_memoized(5, &mut memo), 5);   // 🧠 ตรวจสอบกรณีปกติ
        assert_eq!(fibonacci_memoized(10, &mut memo), 55); // 🧠 ตรวจสอบกรณีซับซ้อน
    }

    #[test]  // 🧪 ทดสอบการจัดเรียงแบบมือใหม่
    #[allow(clippy::missing_panics_doc)]
    pub fn test_bubble_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];  // 📋 หลักฐานที่ยุ่งเหยิง
        bubble_sort(&mut arr);  // 🫧 จัดเรียงแบบมือใหม่
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);  // ✅ ตรวจสอบผลลัพธ์
    }

    #[test]  // 🧪 ทดสอบการจัดเรียงแบบมืออาชีพ
    #[allow(clippy::missing_panics_doc)]
    pub fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];  // 📋 หลักฐานที่ยุ่งเหยิง
        quick_sort(&mut arr);  // ⚡ จัดเรียงแบบมืออาชีพ
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);  // ✅ ตรวจสอบผลลัพธ์
    }

    #[test]  // 🧪 ทดสอบนาฬิกานักสืบ
    #[allow(clippy::missing_panics_doc)]
    pub fn test_measure_time() {
        let (result, duration) = measure_time(|| {
            let mut sum = 0;
            for i in 1..=100 {
                sum += i;
            }
            sum
        });

        assert_eq!(result, 5050);  // ✅ ตรวจสอบผลลัพธ์
        assert!(duration.as_nanos() > 0);  // ⏱️ ตรวจสอบเวลา
    }

    #[test]  // 🧪 ทดสอบการแข่งขันความเร็ว
    #[allow(clippy::missing_panics_doc)]
    pub fn test_performance_comparison() {
        // 🏁 ทดสอบว่านักสืบมืออาชีพเร็วกว่านักสืบมือใหม่
        let n = 20;
        
        let (_, recursive_time) = measure_time(|| fibonacci_recursive(n));  // 🐌 วัดเวลานักสืบมือใหม่
        let (_, iterative_time) = measure_time(|| fibonacci_iterative(n));  // ⚡ วัดเวลานักสืบมืออาชีพ
        
        // 🏆 นักสืบมืออาชีพควรเร็วกว่านักสืบมือใหม่
        assert!(iterative_time <= recursive_time);
    }

    #[test]  // 🧪 ทดสอบการแข่งขันความเร็วการจัดเรียง
    #[allow(clippy::missing_panics_doc)]
    pub fn test_sorting_performance_comparison() {
        let size = 100; // ใช้ขนาดเล็กกว่าเพื่อให้ test เร็ว
        let data: Vec<i32> = (0..size).rev().collect();

        let mut bubble_data = data.clone();
        let mut quick_data = data;

        let ((), bubble_time) = measure_time(|| bubble_sort(&mut bubble_data));
        let ((), quick_time) = measure_time(|| quick_sort(&mut quick_data));

        // ทั้งคู่ต้องเรียงลำดับถูกต้อง
        assert!(bubble_data.windows(2).all(|w| w[0] <= w[1]));
        assert!(quick_data.windows(2).all(|w| w[0] <= w[1]));

        println!("📊 Sorting {size} elements:");
        println!("   🫧 Bubble Sort: {bubble_time:?}");
        println!("   ⚡ Quick Sort: {quick_time:?}");
    }

    #[test]  // 🧪 ทดสอบห้องทดลองประสิทธิภาพ
    #[allow(clippy::missing_panics_doc)]
    pub fn test_performance_testing_functions() {
        // 🔍 ทดสอบว่าห้องทดลองนักสืบทำงานได้โดยไม่ panic
        performance_testing_examples();
        println!("✅🎉 การทดสอบห้องทดลองประสิทธิภาพเสร็จสิ้น! 🏆🕵️‍♂️");
    }
}