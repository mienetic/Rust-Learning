//! 🚀 SIMD Vectorization Workshop - การประมวลผลแบบขนาน! 🔢
//!
//! ยินดีต้อนรับสู่ Workshop การเรียนรู้ SIMD Vectorization! 🎉
//! เหมือนการทำงานหลายอย่างพร้อมกันในเวลาเดียว - เร็วกว่าเดิมหลายเท่า! ⚡
//!
//! 🎯 สิ่งที่จะได้เรียนรู้:
//! - 🔢 SIMD (Single Instruction, Multiple Data)
//! - ⚡ Vectorized Operations
//! - 🚀 Platform-specific Optimizations
//! - 📊 Performance Comparisons
//!
//! หมายเหตุ: นี่คือการจำลอง SIMD เพื่อการศึกษา! 📚

// Note: This is a simulation of SIMD operations for educational purposes
// Real SIMD would require nightly Rust and target-specific intrinsics

/// 🔢 SIMD Math - การคำนวณแบบขนาน!
/// เหมือนการมีเครื่องคิดเลขหลายเครื่องทำงานพร้อมกัน! 🧮
pub struct SimdMath;

impl SimdMath {
    /// Add two arrays using SIMD-style vectorized operations
    pub fn add_arrays_f32(a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        
        let chunks = a.len() / 4;
        let remainder = a.len() % 4;
        
        // Process 4 elements at a time (simulating SIMD)
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD by processing 4 elements in parallel-style loop
            for j in 0..4 {
                result[start + j] = a[start + j] + b[start + j];
            }
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result[i] = a[i] + b[i];
        }
    }
    
    /// Multiply two arrays using SIMD-style vectorized operations
    pub fn multiply_arrays_f32(a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        
        let chunks = a.len() / 4;
        let remainder = a.len() % 4;
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD multiplication
            for j in 0..4 {
                result[start + j] = a[start + j] * b[start + j];
            }
        }
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result[i] = a[i] * b[i];
        }
    }
    
    /// Calculate dot product using SIMD-style vectorized operations
    #[must_use] pub fn dot_product_f32(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        
        let chunks = a.len() / 4;
        let remainder = a.len() % 4;
        let mut sum_array = [0.0f32; 4];
        
        // SIMD-style dot product
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD by processing 4 elements
            for j in 0..4 {
                sum_array[j] += a[start + j] * b[start + j];
            }
        }
        
        // Sum the SIMD-style result
        let mut result = sum_array.iter().sum::<f32>();
        
        // Handle remaining elements
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result += a[i] * b[i];
        }
        
        result
    }
    
    /// Calculate sum of array using SIMD-style vectorized operations
    #[must_use] pub fn sum_f32(array: &[f32]) -> f32 {
        let chunks = array.len() / 4;
        let remainder = array.len() % 4;
        let mut sum_array = [0.0f32; 4];
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD sum
            for j in 0..4 {
                sum_array[j] += array[start + j];
            }
        }
        
        let mut result = sum_array.iter().sum::<f32>();
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result += array[i];
        }
        
        result
    }
    
    /// Find maximum value using SIMD-style vectorized operations
    #[must_use] pub fn max_f32(array: &[f32]) -> f32 {
        if array.is_empty() {
            return f32::NEG_INFINITY;
        }
        
        let chunks = array.len() / 4;
        let remainder = array.len() % 4;
        let mut max_array = [f32::NEG_INFINITY; 4];
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD max
            for j in 0..4 {
                max_array[j] = max_array[j].max(array[start + j]);
            }
        }
        
        let mut result = max_array.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result = result.max(array[i]);
        }
        
        result
    }
    
    /// Find minimum value using SIMD-style vectorized operations
    #[must_use] pub fn min_f32(array: &[f32]) -> f32 {
        if array.is_empty() {
            return f32::INFINITY;
        }
        
        let chunks = array.len() / 4;
        let remainder = array.len() % 4;
        let mut min_array = [f32::INFINITY; 4];
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD min
            for j in 0..4 {
                min_array[j] = min_array[j].min(array[start + j]);
            }
        }
        
        let mut result = min_array.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result = result.min(array[i]);
        }
        
        result
    }
}

/// 🔢 SIMD Integer Math - การคำนวณจำนวนเต็มแบบขนาน!
/// เหมือนการนับของหลายคนพร้อมกัน! 🧮
pub struct SimdIntegerMath;

impl SimdIntegerMath {
    /// Add two i32 arrays using SIMD-style vectorized operations
    pub fn add_arrays_i32(a: &[i32], b: &[i32], result: &mut [i32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());
        
        let chunks = a.len() / 4;
        let remainder = a.len() % 4;
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD addition
            for j in 0..4 {
                result[start + j] = a[start + j] + b[start + j];
            }
        }
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result[i] = a[i] + b[i];
        }
    }
    
    /// Count set bits using SIMD-style vectorized operations
    #[must_use] pub fn popcount_u32(array: &[u32]) -> u32 {
        let chunks = array.len() / 4;
        let remainder = array.len() % 4;
        let mut total = 0u32;
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD bit counting
            for j in 0..4 {
                total += array[start + j].count_ones();
            }
        }
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            total += array[i].count_ones();
        }
        
        total
    }
    
    /// Find maximum i32 using SIMD-style vectorized operations
    #[must_use] pub fn max_i32(array: &[i32]) -> i32 {
        if array.is_empty() {
            return i32::MIN;
        }
        
        let chunks = array.len() / 4;
        let remainder = array.len() % 4;
        let mut max_array = [i32::MIN; 4];
        
        for i in 0..chunks {
            let start = i * 4;
            // Simulate SIMD max
            for j in 0..4 {
                max_array[j] = max_array[j].max(array[start + j]);
            }
        }
        
        let mut result = max_array.iter().fold(i32::MIN, |a, &b| a.max(b));
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            result = result.max(array[i]);
        }
        
        result
    }
}

/// SIMD-optimized string operations
pub struct SimdStringOps;

impl SimdStringOps {
    /// Count occurrences of a byte in a byte array using SIMD-style operations
    #[must_use] pub fn count_byte(haystack: &[u8], needle: u8) -> usize {
        let chunks = haystack.len() / 16;
        let remainder = haystack.len() % 16;
        let mut count = 0;
        
        for i in 0..chunks {
            let start = i * 16;
            // Simulate SIMD byte comparison
            for j in 0..16 {
                if haystack[start + j] == needle {
                    count += 1;
                }
            }
        }
        
        // Handle remainder
        for i in (chunks * 16)..(chunks * 16 + remainder) {
            if haystack[i] == needle {
                count += 1;
            }
        }
        
        count
    }
    
    /// Find first occurrence of a byte using SIMD-style operations
    #[must_use] pub fn find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
        let chunks = haystack.len() / 16;
        let remainder = haystack.len() % 16;
        
        for i in 0..chunks {
            let start = i * 16;
            // Simulate SIMD byte search
            for j in 0..16 {
                if haystack[start + j] == needle {
                    return Some(start + j);
                }
            }
        }
        
        // Handle remainder
        ((chunks * 16)..(chunks * 16 + remainder)).find(|&i| haystack[i] == needle)
    }
    
    /// Convert ASCII to uppercase using SIMD-style operations
    pub fn ascii_to_uppercase(input: &[u8], output: &mut [u8]) {
        assert_eq!(input.len(), output.len());
        
        let chunks = input.len() / 16;
        let remainder = input.len() % 16;
        
        for i in 0..chunks {
            let start = i * 16;
            // Simulate SIMD case conversion
            for j in 0..16 {
                output[start + j] = if input[start + j] >= b'a' && input[start + j] <= b'z' {
                    input[start + j] - (b'a' - b'A')
                } else {
                    input[start + j]
                };
            }
        }
        
        // Handle remainder
        for i in (chunks * 16)..(chunks * 16 + remainder) {
            output[i] = if input[i] >= b'a' && input[i] <= b'z' {
                input[i] - (b'a' - b'A')
            } else {
                input[i]
            };
        }
    }
}

/// SIMD-optimized matrix operations
pub struct SimdMatrix;

impl SimdMatrix {
    /// Matrix multiplication using SIMD-style operations (simplified 4x4 case)
    #[must_use] pub fn multiply_4x4_f32(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0.0f32; 4]; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                // Simulate SIMD dot product
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += a[i][k] * b[k][j];
                }
                result[i][j] = sum;
            }
        }
        
        result
    }
    
    /// Matrix-vector multiplication using SIMD-style operations
    #[must_use] pub fn multiply_matrix_vector_f32(matrix: &[[f32; 4]; 4], vector: &[f32; 4]) -> [f32; 4] {
        let mut result = [0.0f32; 4];
        
        for i in 0..4 {
            // Simulate SIMD dot product
            let mut sum = 0.0;
            for j in 0..4 {
                sum += matrix[i][j] * vector[j];
            }
            result[i] = sum;
        }
        
        result
    }
    
    /// Transpose a 4x4 matrix using SIMD-style operations
    #[must_use] pub fn transpose_4x4_f32(matrix: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0.0f32; 4]; 4];
        
        // Simulate SIMD transpose operations
        for i in 0..4 {
            for j in 0..4 {
                result[j][i] = matrix[i][j];
            }
        }
        
        result
    }
}

/// SIMD-optimized image processing operations
pub struct SimdImageProcessing;

impl SimdImageProcessing {
    /// Convert RGB to grayscale using SIMD
    pub fn rgb_to_grayscale(rgb: &[u8], grayscale: &mut [u8]) {
        assert_eq!(rgb.len() % 3, 0);
        assert_eq!(grayscale.len(), rgb.len() / 3);
        
        let pixels = rgb.len() / 3;
        
        // Weights for RGB to grayscale conversion (scaled by 256 for integer math)
        let r_weight = 77u16;  // 0.299 * 256
        let g_weight = 151u16; // 0.587 * 256
        let b_weight = 28u16;  // 0.114 * 256
        
        for i in 0..pixels {
            let r = u16::from(rgb[i * 3]);
            let g = u16::from(rgb[i * 3 + 1]);
            let b = u16::from(rgb[i * 3 + 2]);
            
            let gray = (r * r_weight + g * g_weight + b * b_weight) >> 8;
            grayscale[i] = gray as u8;
        }
    }
    
    /// Apply brightness adjustment using SIMD-style operations
    pub fn adjust_brightness(image: &[u8], output: &mut [u8], brightness: i16) {
        assert_eq!(image.len(), output.len());
        
        let chunks = image.len() / 16;
        let remainder = image.len() % 16;
        
        for i in 0..chunks {
            let start = i * 16;
            // Simulate SIMD brightness adjustment
            for j in 0..16 {
                let adjusted = (i16::from(image[start + j]) + brightness).clamp(0, 255);
                output[start + j] = adjusted as u8;
            }
        }
        
        // Handle remainder
        for i in (chunks * 16)..(chunks * 16 + remainder) {
            let adjusted = (i16::from(image[i]) + brightness).clamp(0, 255);
            output[i] = adjusted as u8;
        }
    }
    
    /// Apply contrast adjustment using SIMD-style operations
    pub fn adjust_contrast(image: &[u8], output: &mut [u8], contrast: f32) {
        assert_eq!(image.len(), output.len());
        
        let chunks = image.len() / 4;
        let remainder = image.len() % 4;
        let offset = 128.0 * (1.0 - contrast);
        
        for i in 0..chunks {
            let start = i * 4;
            
            // Simulate SIMD contrast adjustment
            for j in 0..4 {
                let pixel = f32::from(image[start + j]);
                let adjusted = pixel.mul_add(contrast, offset).clamp(0.0, 255.0);
                output[start + j] = adjusted as u8;
            }
        }
        
        // Handle remainder
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            let adjusted = f32::from(image[i]).mul_add(contrast, offset).clamp(0.0, 255.0);
            output[i] = adjusted as u8;
        }
    }
}

/// SIMD-optimized sorting and searching
pub struct SimdSort;

impl SimdSort {
    /// Parallel comparison for sorting networks (simulated)
    pub fn compare_and_swap_f32(a: &mut [f32; 4], b: &mut [f32; 4]) {
        for i in 0..4 {
            let min_val = a[i].min(b[i]);
            let max_val = a[i].max(b[i]);
            a[i] = min_val;
            b[i] = max_val;
        }
    }
    
    /// Sort 4 elements using SIMD-style sorting network
    pub fn sort_4_f32(values: &mut [f32; 4]) {
        // Sorting network for 4 elements
        // Stage 1: Compare (0,1) and (2,3)
        if values[0] > values[1] {
            values.swap(0, 1);
        }
        if values[2] > values[3] {
            values.swap(2, 3);
        }
        
        // Stage 2: Compare (0,2) and (1,3)
        if values[0] > values[2] {
            values.swap(0, 2);
        }
        if values[1] > values[3] {
            values.swap(1, 3);
        }
        
        // Stage 3: Compare (1,2)
        if values[1] > values[2] {
            values.swap(1, 2);
        }
    }
    
    /// Binary search using SIMD for multiple queries
    #[must_use] pub fn binary_search_multiple_f32(sorted_array: &[f32], queries: &[f32; 4]) -> [Option<usize>; 4] {
        let mut results = [None; 4];
        
        for (i, &query) in queries.iter().enumerate() {
            results[i] = sorted_array.binary_search_by(|&x| {
                x.partial_cmp(&query).unwrap_or(std::cmp::Ordering::Equal)
            }).ok();
        }
        
        results
    }
}

/// Performance benchmarking utilities
pub struct SimdBenchmark;

impl SimdBenchmark {
    /// Benchmark SIMD vs scalar addition
    #[must_use] pub fn benchmark_addition(size: usize, iterations: usize) -> (std::time::Duration, std::time::Duration) {
        let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
        let mut result_simd = vec![0.0f32; size];
        let mut result_scalar = vec![0.0f32; size];
        
        // Benchmark simulated SIMD version
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            SimdMath::add_arrays_f32(&a, &b, &mut result_simd);
        }
        let simd_time = start.elapsed();
        
        // Benchmark scalar version
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for i in 0..size {
                result_scalar[i] = a[i] + b[i];
            }
        }
        let scalar_time = start.elapsed();
        
        (simd_time, scalar_time)
    }
    
    /// Benchmark SIMD vs scalar dot product
    #[must_use] pub fn benchmark_dot_product(size: usize, iterations: usize) -> (std::time::Duration, std::time::Duration) {
        let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
        
        // Benchmark simulated SIMD version
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = SimdMath::dot_product_f32(&a, &b);
        }
        let simd_time = start.elapsed();
        
        // Benchmark scalar version
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut sum = 0.0f32;
            for i in 0..size {
                sum += a[i] * b[i];
            }
        }
        let scalar_time = start.elapsed();
        
        (simd_time, scalar_time)
    }
    
    /// Compare simulated SIMD vs scalar performance for array addition
    #[must_use] pub fn compare_add_performance(size: usize) -> (std::time::Duration, std::time::Duration) {
        let a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();
        let mut result_simd = vec![0.0f32; size];
        let mut result_scalar = vec![0.0f32; size];
        
        // Simulated SIMD version (processing 4 elements at a time)
        let start = std::time::Instant::now();
        let chunks = size / 4;
        for i in 0..chunks {
            let start_idx = i * 4;
            // Simulate SIMD addition of 4 elements
            for j in 0..4 {
                result_simd[start_idx + j] = a[start_idx + j] + b[start_idx + j];
            }
        }
        // Handle remainder
        for i in (chunks * 4)..size {
            result_simd[i] = a[i] + b[i];
        }
        let simd_time = start.elapsed();
        
        // Scalar version
        let start = std::time::Instant::now();
        for i in 0..size {
            result_scalar[i] = a[i] + b[i];
        }
        let scalar_time = start.elapsed();
        
        (simd_time, scalar_time)
    }
}

/// สาธิตการใช้งาน SIMD vectorization
/// 🚀 สาธิต SIMD Vectorization Workshop!
/// เหมือนการมีทีมงานหลายคนทำงานพร้อมกัน! 👥⚡
pub fn demonstrate_simd_vectorization() {
    println!("🎉 ยินดีต้อนรับสู่ SIMD Vectorization Workshop! 🚀");
    println!("เหมือนการทำงานหลายอย่างพร้อมกันในเวลาเดียว! ⚡\n");
    
    // SIMD mathematical operations
    println!("🧮 การคำนวณทางคณิตศาสตร์แบบ SIMD:");
    println!("เหมือนการมีเครื่องคิดเลขหลายเครื่องทำงานพร้อมกัน! 🔢");
    println!("{:-<50}", "");
    
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut result = vec![0.0; 8];
    
    // SIMD addition
    SimdMath::add_arrays_f32(&a, &b, &mut result);
    println!("➕ การบวกแบบ SIMD - เหมือนการบวกหลายคู่พร้อมกัน!");
    println!("  A: {a:?}");
    println!("  B: {b:?}");
    println!("  A + B: {result:?}");
    
    // SIMD multiplication
    SimdMath::multiply_arrays_f32(&a, &b, &mut result);
    println!("\n✖️ การคูณแบบ SIMD - เหมือนการคูณหลายคู่พร้อมกัน!");
    println!("  A * B: {result:?}");
    
    // SIMD dot product
    let dot_product = SimdMath::dot_product_f32(&a, &b);
    println!("\n🎯 Dot Product แบบ SIMD - การคำนวณแบบรวมพลัง!: {dot_product}");
    
    // SIMD aggregations
    let sum = SimdMath::sum_f32(&a);
    let max_val = SimdMath::max_f32(&a);
    let min_val = SimdMath::min_f32(&a);
    
    println!("\n📊 การรวมข้อมูลแบบ SIMD - เหมือนการสรุปผลหลายอย่างพร้อมกัน!");
    println!("  ผลรวม (Sum): {sum}");
    println!("  ค่าสูงสุด (Max): {max_val}");
    println!("  ค่าต่ำสุด (Min): {min_val}");
    
    // SIMD integer operations
    println!("\n🔢 การคำนวณจำนวนเต็มแบบ SIMD:");
    println!("เหมือนการนับของหลายคนพร้อมกัน! 🧮");
    println!("{:-<50}", "");
    
    let int_a = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let int_b = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let mut int_result = vec![0; 8];
    
    SimdIntegerMath::add_arrays_i32(&int_a, &int_b, &mut int_result);
    println!("➕ การบวกจำนวนเต็มแบบ SIMD - เหมือนการบวกหลายคู่พร้อมกัน!");
    println!("  A: {int_a:?}");
    println!("  B: {int_b:?}");
    println!("  A + B: {int_result:?}");
    
    // Bit counting
    let bit_array = vec![0b11110000u32, 0b10101010u32, 0b11111111u32, 0b00000001u32];
    let total_bits = SimdIntegerMath::popcount_u32(&bit_array);
    println!("\n🔢 การนับ Bit แบบ SIMD - เหมือนการนับหลายอย่างพร้อมกัน!");
    println!("  Array: {:?}", bit_array.iter().map(|x| format!("{x:08b}")).collect::<Vec<_>>());
    println!("  จำนวน Bit ที่เป็น 1: {total_bits}");
    
    // SIMD string operations
    println!("\n📝 การจัดการข้อความแบบ SIMD:");
    println!("เหมือนการอ่านหลายบรรทัดพร้อมกัน! 📖");
    println!("{:-<50}", "");
    
    let text = b"Hello, SIMD world! This is a test string for SIMD operations.";
    
    // Count specific byte
    let space_count = SimdStringOps::count_byte(text, b' ');
    let letter_s_count = SimdStringOps::count_byte(text, b's');
    
    println!("🔍 การนับตัวอักษรแบบ SIMD - เหมือนการหาของหลายอย่างพร้อมกัน!");
    println!("  ข้อความ: \"{}\"", std::str::from_utf8(text).unwrap());
    println!("  ช่องว่าง (Spaces): {space_count}");
    println!("  ตัวอักษร 's': {letter_s_count}");
    
    // Find byte position
    if let Some(pos) = SimdStringOps::find_byte(text, b'S') {
        println!("  ตำแหน่งแรกของ 'S': {pos}");
    }
    
    // ASCII case conversion
    let input = b"hello world from simd";
    let mut output = vec![0u8; input.len()];
    SimdStringOps::ascii_to_uppercase(input, &mut output);
    
    println!("\n🔄 การแปลงตัวพิมพ์แบบ SIMD - เหมือนการเปลี่ยนหลายตัวพร้อมกัน!");
    println!("  ข้อความเดิม:  \"{}\"", std::str::from_utf8(input).unwrap());
    println!("  ข้อความใหม่: \"{}\"", std::str::from_utf8(&output).unwrap());
    
    // SIMD matrix operations
    println!("\n🔢 การคำนวณเมทริกซ์แบบ SIMD:");
    println!("เหมือนการคำนวณตารางหลายช่องพร้อมกัน! 📊");
    println!("{:-<50}", "");
    
    let matrix_a = [
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    
    let matrix_b = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    
    let result_matrix = SimdMatrix::multiply_4x4_f32(&matrix_a, &matrix_b);
    
    println!("✖️ การคูณเมทริกซ์แบบ SIMD (A * Identity):");
    for (i, row) in result_matrix.iter().enumerate() {
        println!("  แถว {i}: {row:?}");
    }
    
    // Matrix-vector multiplication
    let vector = [1.0, 2.0, 3.0, 4.0];
    let result_vector = SimdMatrix::multiply_matrix_vector_f32(&matrix_a, &vector);
    
    println!("\n🎯 การคูณเมทริกซ์-เวกเตอร์แบบ SIMD:");
    println!("  เวกเตอร์: {vector:?}");
    println!("  ผลลัพธ์: {result_vector:?}");
    
    // Matrix transpose
    let transposed = SimdMatrix::transpose_4x4_f32(&matrix_a);
    println!("\n🔄 การหมุนเมทริกซ์แบบ SIMD (Transpose):");
    for (i, row) in transposed.iter().enumerate() {
        println!("  แถว {i}: {row:?}");
    }
    
    // SIMD image processing
    println!("\n🖼️ การประมวลผลภาพแบบ SIMD:");
    println!("เหมือนการแต่งภาพหลายจุดพร้อมกัน! 🎨");
    println!("{:-<50}", "");
    
    // RGB to grayscale conversion
    let rgb_data = vec![
        255, 0, 0,    // Red pixel
        0, 255, 0,    // Green pixel
        0, 0, 255,    // Blue pixel
        255, 255, 0,  // Yellow pixel
        255, 255, 255, // White pixel
    ];
    
    let mut grayscale_data = vec![0u8; rgb_data.len() / 3];
    SimdImageProcessing::rgb_to_grayscale(&rgb_data, &mut grayscale_data);
    
    println!("🎨 การแปลง RGB เป็นสีเทาแบบ SIMD:");
    for i in 0..grayscale_data.len() {
        let r = rgb_data[i * 3];
        let g = rgb_data[i * 3 + 1];
        let b = rgb_data[i * 3 + 2];
        println!("  RGB({}, {}, {}) -> สีเทา({})", r, g, b, grayscale_data[i]);
    }
    
    // Brightness adjustment
    let image_data = vec![100, 150, 200, 50, 75, 125, 175, 225];
    let mut bright_data = vec![0u8; image_data.len()];
    SimdImageProcessing::adjust_brightness(&image_data, &mut bright_data, 50);
    
    println!("\n💡 การปรับความสว่างแบบ SIMD (+50):");
    println!("  ภาพเดิม: {image_data:?}");
    println!("  ภาพใหม่: {bright_data:?}");
    
    // SIMD sorting
    println!("\n🔄 การเรียงลำดับแบบ SIMD:");
    println!("เหมือนการจัดของหลายชิ้นพร้อมกัน! 📋");
    println!("{:-<50}", "");
    
    let mut values = [3.7, 1.2, 4.8, 2.1];
    println!("📊 การเรียงลำดับ 4 ตัวเลขแบบ SIMD:");
    println!("  ก่อน: {values:?}");
    
    SimdSort::sort_4_f32(&mut values);
    println!("  หลัง: {values:?}");
    
    // Performance comparison
    println!("\n⚡ การเปรียบเทียบประสิทธิภาพ:");
    println!("ดูกันว่า SIMD เร็วกว่าแบบปกติเท่าไหร่! 🏃‍♂️💨");
    println!("{:-<50}", "");
    
    let size = 10000;
    let iterations = 1000;
    
    // Benchmark addition
    let (simd_time, scalar_time) = SimdBenchmark::benchmark_addition(size, iterations);
    println!("➕ การทดสอบการบวก ({size} ตัวเลข, {iterations} รอบ):");
    println!("  SIMD:   {simd_time:?}");
    println!("  แบบปกติ: {scalar_time:?}");
    
    if scalar_time > simd_time {
        let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
        println!("  🚀 SIMD เร็วกว่า {speedup:.2} เท่า!");
    }
    
    // Benchmark dot product
    let (simd_time, scalar_time) = SimdBenchmark::benchmark_dot_product(size, iterations);
    println!("\n🎯 การทดสอบ Dot Product ({size} ตัวเลข, {iterations} รอบ):");
    println!("  SIMD:   {simd_time:?}");
    println!("  แบบปกติ: {scalar_time:?}");
    
    if scalar_time > simd_time {
        let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
        println!("  🚀 SIMD เร็วกว่า {speedup:.2} เท่า!");
    }
    
    println!("\n🎉 ยินดีด้วย! คุณได้เรียนรู้ SIMD Vectorization เรียบร้อยแล้ว!");
    println!("💡 ตอนนี้คุณรู้วิธีทำงานหลายอย่างพร้อมกันแล้ว! ⚡");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_math_operations() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![2.0, 3.0, 4.0, 5.0];
        let mut result = vec![0.0; 4];
        
        // Test addition
        SimdMath::add_arrays_f32(&a, &b, &mut result);
        assert_eq!(result, vec![3.0, 5.0, 7.0, 9.0]);
        
        // Test multiplication
        SimdMath::multiply_arrays_f32(&a, &b, &mut result);
        assert_eq!(result, vec![2.0, 6.0, 12.0, 20.0]);
        
        // Test dot product
        let dot = SimdMath::dot_product_f32(&a, &b);
        assert_eq!(dot, 40.0); // 1*2 + 2*3 + 3*4 + 4*5 = 2 + 6 + 12 + 20 = 40
        
        // Test aggregations
        assert_eq!(SimdMath::sum_f32(&a), 10.0);
        assert_eq!(SimdMath::max_f32(&a), 4.0);
        assert_eq!(SimdMath::min_f32(&a), 1.0);
    }

    #[test]
    fn test_simd_integer_operations() {
        let a = vec![1, 2, 3, 4];
        let b = vec![10, 20, 30, 40];
        let mut result = vec![0; 4];
        
        SimdIntegerMath::add_arrays_i32(&a, &b, &mut result);
        assert_eq!(result, vec![11, 22, 33, 44]);
        
        let max_val = SimdIntegerMath::max_i32(&a);
        assert_eq!(max_val, 4);
        
        // Test popcount
        let bits = vec![0b1111u32, 0b1010u32, 0b1100u32];
        let count = SimdIntegerMath::popcount_u32(&bits);
        assert_eq!(count, 8); // 4 + 2 + 2 = 8
    }

    #[test]
    fn test_simd_string_operations() {
        let text = b"Hello world";
        
        // Test byte counting
        let l_count = SimdStringOps::count_byte(text, b'l');
        assert_eq!(l_count, 3);
        
        let space_count = SimdStringOps::count_byte(text, b' ');
        assert_eq!(space_count, 1);
        
        // Test byte finding
        let pos = SimdStringOps::find_byte(text, b'w');
        assert_eq!(pos, Some(6));
        
        let not_found = SimdStringOps::find_byte(text, b'x');
        assert_eq!(not_found, None);
        
        // Test case conversion
        let input = b"hello";
        let mut output = vec![0u8; input.len()];
        SimdStringOps::ascii_to_uppercase(input, &mut output);
        assert_eq!(&output, b"HELLO");
    }

    #[test]
    fn test_simd_matrix_operations() {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        
        let matrix = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];
        
        // Test matrix multiplication with identity
        let result = SimdMatrix::multiply_4x4_f32(&matrix, &identity);
        assert_eq!(result, matrix);
        
        // Test matrix-vector multiplication
        let vector = [1.0, 0.0, 0.0, 0.0];
        let result = SimdMatrix::multiply_matrix_vector_f32(&matrix, &vector);
        assert_eq!(result, [1.0, 5.0, 9.0, 13.0]);
        
        // Test transpose
        let transposed = SimdMatrix::transpose_4x4_f32(&matrix);
        assert_eq!(transposed[0], [1.0, 5.0, 9.0, 13.0]);
        assert_eq!(transposed[1], [2.0, 6.0, 10.0, 14.0]);
    }

    #[test]
    fn test_simd_image_processing() {
        // Test RGB to grayscale
        let rgb = vec![255, 0, 0, 0, 255, 0, 0, 0, 255]; // Red, Green, Blue
        let mut gray = vec![0u8; 3];
        
        SimdImageProcessing::rgb_to_grayscale(&rgb, &mut gray);
        
        // Check that red, green, and blue have different grayscale values
        assert!(gray[0] > 0); // Red should have some brightness
        assert!(gray[1] > gray[0]); // Green should be brighter than red
        assert!(gray[2] < gray[0]); // Blue should be darker than red
        
        // Test brightness adjustment
        let image = vec![100, 150, 200];
        let mut bright = vec![0u8; 3];
        
        SimdImageProcessing::adjust_brightness(&image, &mut bright, 50);
        assert_eq!(bright, vec![150, 200, 250]);
        
        // Test brightness with saturation
        SimdImageProcessing::adjust_brightness(&image, &mut bright, 100);
        assert_eq!(bright, vec![200, 250, 255]); // 300 clamped to 255
    }

    #[test]
    fn test_simd_sorting() {
        let mut values = [4.0, 1.0, 3.0, 2.0];
        SimdSort::sort_4_f32(&mut values);
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0]);
        
        let mut values = [1.5, 2.5, 1.0, 3.0];
        SimdSort::sort_4_f32(&mut values);
        assert_eq!(values, [1.0, 1.5, 2.5, 3.0]);
    }

    #[test]
    fn test_empty_arrays() {
        assert_eq!(SimdMath::max_f32(&[]), f32::NEG_INFINITY);
        assert_eq!(SimdMath::min_f32(&[]), f32::INFINITY);
        assert_eq!(SimdMath::sum_f32(&[]), 0.0);
        
        assert_eq!(SimdIntegerMath::max_i32(&[]), i32::MIN);
        assert_eq!(SimdIntegerMath::popcount_u32(&[]), 0);
    }

    #[test]
    fn test_single_element_arrays() {
        assert_eq!(SimdMath::max_f32(&[42.0]), 42.0);
        assert_eq!(SimdMath::min_f32(&[42.0]), 42.0);
        assert_eq!(SimdMath::sum_f32(&[42.0]), 42.0);
        
        assert_eq!(SimdIntegerMath::max_i32(&[42]), 42);
    }
}