//! Compile-Time Computation Implementation
//!
//! การใช้งาน Compile-Time Computation ใน Rust
//! ครอบคลุม const functions, const generics, และ compile-time evaluation

use std::marker::PhantomData;

/// Compile-time mathematical computations
#[must_use] pub const fn const_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;
            
            while i <= n {
                let temp = a + b;
                a = b;
                b = temp;
                i += 1;
            }
            
            b
        }
    }
}

#[must_use] pub const fn const_factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => {
            let mut result = 1;
            let mut i = 2;
            
            while i <= n {
                result *= i as u64;
                i += 1;
            }
            
            result
        }
    }
}

#[must_use] pub const fn const_power(base: u64, exp: u32) -> u64 {
    match exp {
        0 => 1,
        1 => base,
        _ => {
            let mut result = 1;
            let mut base_power = base;
            let mut remaining_exp = exp;
            
            while remaining_exp > 0 {
                if remaining_exp % 2 == 1 {
                    result *= base_power;
                }
                base_power *= base_power;
                remaining_exp /= 2;
            }
            
            result
        }
    }
}

#[must_use] pub const fn const_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[must_use] pub const fn const_lcm(a: u64, b: u64) -> u64 {
    (a * b) / const_gcd(a, b)
}

/// Compile-time string operations
#[must_use] pub const fn const_str_len(s: &str) -> usize {
    s.len()
}

#[must_use] pub const fn const_str_is_empty(s: &str) -> bool {
    s.is_empty()
}

#[must_use] pub const fn const_str_first_char(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        None
    } else {
        Some(bytes[0])
    }
}

#[must_use] pub const fn const_str_last_char(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        None
    } else {
        Some(bytes[bytes.len() - 1])
    }
}

/// Compile-time array operations
#[must_use] pub const fn const_array_sum<const N: usize>(arr: &[i32; N]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    
    while i < N {
        sum += arr[i];
        i += 1;
    }
    
    sum
}

#[must_use] pub const fn const_array_max<const N: usize>(arr: &[i32; N]) -> Option<i32> {
    if N == 0 {
        return None;
    }
    
    let mut max = arr[0];
    let mut i = 1;
    
    while i < N {
        if arr[i] > max {
            max = arr[i];
        }
        i += 1;
    }
    
    Some(max)
}

#[must_use] pub const fn const_array_min<const N: usize>(arr: &[i32; N]) -> Option<i32> {
    if N == 0 {
        return None;
    }
    
    let mut min = arr[0];
    let mut i = 1;
    
    while i < N {
        if arr[i] < min {
            min = arr[i];
        }
        i += 1;
    }
    
    Some(min)
}

#[must_use] pub const fn const_array_contains<const N: usize>(arr: &[i32; N], target: i32) -> bool {
    let mut i = 0;
    
    while i < N {
        if arr[i] == target {
            return true;
        }
        i += 1;
    }
    
    false
}

/// Compile-time type-level computations
pub trait ConstCompute {
    const VALUE: u64;
}

#[derive(Debug)]
pub struct ConstFib<const N: u32>;

impl<const N: u32> ConstCompute for ConstFib<N> {
    const VALUE: u64 = const_fibonacci(N);
}

#[derive(Debug)]
pub struct ConstFact<const N: u32>;

impl<const N: u32> ConstCompute for ConstFact<N> {
    const VALUE: u64 = const_factorial(N);
}

#[derive(Debug)]
pub struct ConstPow<const BASE: u64, const EXP: u32>;

impl<const BASE: u64, const EXP: u32> ConstCompute for ConstPow<BASE, EXP> {
    const VALUE: u64 = const_power(BASE, EXP);
}

/// Compile-time matrix operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstMatrix<const ROWS: usize, const COLS: usize> {
    data: [[i32; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> ConstMatrix<ROWS, COLS> {
    #[must_use] pub const fn new(data: [[i32; COLS]; ROWS]) -> Self {
        Self { data }
    }
    
    #[must_use] pub const fn zero() -> Self {
        Self { data: [[0; COLS]; ROWS] }
    }
    
    #[must_use] pub const fn get(&self, row: usize, col: usize) -> Option<i32> {
        if row < ROWS && col < COLS {
            Some(self.data[row][col])
        } else {
            None
        }
    }
    
    #[must_use] pub const fn transpose(&self) -> ConstMatrix<COLS, ROWS> {
        let mut result = [[0; ROWS]; COLS];
        let mut i = 0;
        
        while i < ROWS {
            let mut j = 0;
            while j < COLS {
                result[j][i] = self.data[i][j];
                j += 1;
            }
            i += 1;
        }
        
        ConstMatrix { data: result }
    }
    
    #[must_use] pub const fn add(&self, other: &Self) -> Self {
        let mut result = [[0; COLS]; ROWS];
        let mut i = 0;
        
        while i < ROWS {
            let mut j = 0;
            while j < COLS {
                result[i][j] = self.data[i][j] + other.data[i][j];
                j += 1;
            }
            i += 1;
        }
        
        Self { data: result }
    }
    
    #[must_use] pub const fn scalar_multiply(&self, scalar: i32) -> Self {
        let mut result = [[0; COLS]; ROWS];
        let mut i = 0;
        
        while i < ROWS {
            let mut j = 0;
            while j < COLS {
                result[i][j] = self.data[i][j] * scalar;
                j += 1;
            }
            i += 1;
        }
        
        Self { data: result }
    }
}

impl<const N: usize> ConstMatrix<N, N> {
    #[must_use] pub const fn identity() -> Self {
        let mut result = [[0; N]; N];
        let mut i = 0;
        
        while i < N {
            result[i][i] = 1;
            i += 1;
        }
        
        Self { data: result }
    }
    
    #[must_use] pub const fn trace(&self) -> i32 {
        let mut sum = 0;
        let mut i = 0;
        
        while i < N {
            sum += self.data[i][i];
            i += 1;
        }
        
        sum
    }
}

/// Compile-time hash computation
#[must_use] pub const fn const_hash_djb2(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut hash: u32 = 5381;
    let mut i = 0;
    
    while i < bytes.len() {
        hash = hash.wrapping_mul(33).wrapping_add(bytes[i] as u32);
        i += 1;
    }
    
    hash
}

#[must_use] pub const fn const_hash_fnv1a(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut hash: u32 = 2166136261;
    let mut i = 0;
    
    while i < bytes.len() {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(16777619);
        i += 1;
    }
    
    hash
}

/// Compile-time bit operations
#[must_use] pub const fn const_popcount(mut n: u64) -> u32 {
    let mut count = 0;
    
    while n != 0 {
        count += 1;
        n &= n - 1; // Remove the lowest set bit
    }
    
    count
}

#[must_use] pub const fn const_leading_zeros(n: u64) -> u32 {
    if n == 0 {
        return 64;
    }
    
    let mut count = 0;
    let mut mask = 1u64 << 63;
    
    while (n & mask) == 0 {
        count += 1;
        mask >>= 1;
    }
    
    count
}

#[must_use] pub const fn const_trailing_zeros(n: u64) -> u32 {
    if n == 0 {
        return 64;
    }
    
    let mut count = 0;
    let mut mask = 1u64;
    
    while (n & mask) == 0 {
        count += 1;
        mask <<= 1;
    }
    
    count
}

#[must_use] pub const fn const_reverse_bits(mut n: u64) -> u64 {
    let mut result = 0;
    let mut i = 0;
    
    while i < 64 {
        result = (result << 1) | (n & 1);
        n >>= 1;
        i += 1;
    }
    
    result
}

/// Compile-time lookup tables
#[must_use] pub const fn generate_sin_table<const N: usize>() -> [f64; N] {
    let mut table = [0.0; N];
    let mut i = 0;
    
    while i < N {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI) / (N as f64);
        // Simple sine approximation for compile-time
        table[i] = angle - (angle * angle * angle) / 6.0 + (angle * angle * angle * angle * angle) / 120.0;
        i += 1;
    }
    
    table
}

#[must_use] pub const fn generate_prime_table<const N: usize>() -> [bool; N] {
    let mut is_prime = [true; N];
    
    if N > 0 {
        is_prime[0] = false;
    }
    if N > 1 {
        is_prime[1] = false;
    }
    
    let mut i = 2;
    while i * i < N {
        if is_prime[i] {
            let mut j = i * i;
            while j < N {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    
    is_prime
}

/// Compile-time configuration
#[derive(Debug)]
pub struct CompileTimeConfig<const BUFFER_SIZE: usize, const MAX_CONNECTIONS: usize, const TIMEOUT_MS: u64> {
    _phantom: PhantomData<()>,
}

impl<const BUFFER_SIZE: usize, const MAX_CONNECTIONS: usize, const TIMEOUT_MS: u64> Default for CompileTimeConfig<BUFFER_SIZE, MAX_CONNECTIONS, TIMEOUT_MS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BUFFER_SIZE: usize, const MAX_CONNECTIONS: usize, const TIMEOUT_MS: u64> 
    CompileTimeConfig<BUFFER_SIZE, MAX_CONNECTIONS, TIMEOUT_MS> 
{
    #[must_use] pub const fn new() -> Self {
        Self { _phantom: PhantomData }
    }
    
    #[must_use] pub const fn buffer_size() -> usize {
        BUFFER_SIZE
    }
    
    #[must_use] pub const fn max_connections() -> usize {
        MAX_CONNECTIONS
    }
    
    #[must_use] pub const fn timeout_ms() -> u64 {
        TIMEOUT_MS
    }
    
    #[must_use] pub const fn total_memory_needed() -> usize {
        BUFFER_SIZE * MAX_CONNECTIONS
    }
    
    #[must_use] pub const fn is_valid_config() -> bool {
        BUFFER_SIZE > 0 && MAX_CONNECTIONS > 0 && TIMEOUT_MS > 0
    }
}

/// Compile-time assertions
macro_rules! const_assert {
    ($condition:expr) => {
        const _: () = assert!($condition);
    };
}

macro_rules! const_assert_eq {
    ($left:expr, $right:expr) => {
        const _: () = assert!($left == $right);
    };
}

/// Compile-time format string validation
#[must_use] pub const fn validate_format_string(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut brace_count = 0;
    
    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                    i += 2; // Skip escaped brace
                    continue;
                }
                brace_count += 1;
            }
            b'}' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'}' {
                    i += 2; // Skip escaped brace
                    continue;
                }
                if brace_count == 0 {
                    return false; // Unmatched closing brace
                }
                brace_count -= 1;
            }
            _ => {}
        }
        i += 1;
    }
    
    brace_count == 0
}

/// สาธิตการใช้งาน Compile-Time Computation
pub fn demonstrate_compile_time_computation() {
    println!("⚡ Compile-Time Computation Examples:");
    
    // Compile-time mathematical computations
    println!("\n🧮 Compile-Time Math:");
    println!("{:-<50}", "");
    
    const FIB_10: u64 = const_fibonacci(10);
    const FIB_20: u64 = const_fibonacci(20);
    const FACT_10: u64 = const_factorial(10);
    const POW_2_16: u64 = const_power(2, 16);
    const GCD_48_18: u64 = const_gcd(48, 18);
    const LCM_12_8: u64 = const_lcm(12, 8);
    
    println!("Fibonacci(10): {FIB_10} (computed at compile time)");
    println!("Fibonacci(20): {FIB_20} (computed at compile time)");
    println!("Factorial(10): {FACT_10} (computed at compile time)");
    println!("2^16: {POW_2_16} (computed at compile time)");
    println!("GCD(48, 18): {GCD_48_18} (computed at compile time)");
    println!("LCM(12, 8): {LCM_12_8} (computed at compile time)");
    
    // Compile-time string operations
    println!("\n📝 Compile-Time String Operations:");
    println!("{:-<50}", "");
    
    const HELLO: &str = "Hello, World!";
    const HELLO_LEN: usize = const_str_len(HELLO);
    const HELLO_FIRST: Option<u8> = const_str_first_char(HELLO);
    const HELLO_LAST: Option<u8> = const_str_last_char(HELLO);
    const EMPTY_CHECK: bool = const_str_is_empty("");
    
    println!("String: \"{HELLO}\"");
    println!("Length: {HELLO_LEN} (computed at compile time)");
    if let Some(first) = HELLO_FIRST {
        println!("First char: '{}' (computed at compile time)", first as char);
    }
    if let Some(last) = HELLO_LAST {
        println!("Last char: '{}' (computed at compile time)", last as char);
    }
    println!("Empty string is empty: {EMPTY_CHECK} (computed at compile time)");
    
    // Compile-time array operations
    println!("\n📊 Compile-Time Array Operations:");
    println!("{:-<50}", "");
    
    const NUMBERS: [i32; 5] = [10, 5, 8, 3, 12];
    const ARRAY_SUM: i32 = const_array_sum(&NUMBERS);
    const ARRAY_MAX: Option<i32> = const_array_max(&NUMBERS);
    const ARRAY_MIN: Option<i32> = const_array_min(&NUMBERS);
    const CONTAINS_8: bool = const_array_contains(&NUMBERS, 8);
    const CONTAINS_15: bool = const_array_contains(&NUMBERS, 15);
    
    println!("Array: {NUMBERS:?}");
    println!("Sum: {ARRAY_SUM} (computed at compile time)");
    if let Some(max) = ARRAY_MAX {
        println!("Max: {max} (computed at compile time)");
    }
    if let Some(min) = ARRAY_MIN {
        println!("Min: {min} (computed at compile time)");
    }
    println!("Contains 8: {CONTAINS_8} (computed at compile time)");
    println!("Contains 15: {CONTAINS_15} (computed at compile time)");
    
    // Type-level computations
    println!("\n🎯 Type-Level Computations:");
    println!("{:-<50}", "");
    
    println!("Fibonacci(15) via type: {}", ConstFib::<15>::VALUE);
    println!("Factorial(8) via type: {}", ConstFact::<8>::VALUE);
    println!("3^10 via type: {}", ConstPow::<3, 10>::VALUE);
    
    // Compile-time matrix operations
    println!("\n🔢 Compile-Time Matrix Operations:");
    println!("{:-<50}", "");
    
    const MATRIX_A: ConstMatrix<2, 3> = ConstMatrix::new([
        [1, 2, 3],
        [4, 5, 6]
    ]);
    
    const MATRIX_B: ConstMatrix<2, 3> = ConstMatrix::new([
        [7, 8, 9],
        [10, 11, 12]
    ]);
    
    const MATRIX_SUM: ConstMatrix<2, 3> = MATRIX_A.add(&MATRIX_B);
    const MATRIX_SCALED: ConstMatrix<2, 3> = MATRIX_A.scalar_multiply(2);
    const MATRIX_TRANSPOSED: ConstMatrix<3, 2> = MATRIX_A.transpose();
    
    println!("Matrix A: {:?}", MATRIX_A.data);
    println!("Matrix B: {:?}", MATRIX_B.data);
    println!("A + B: {:?} (computed at compile time)", MATRIX_SUM.data);
    println!("A * 2: {:?} (computed at compile time)", MATRIX_SCALED.data);
    println!("A^T: {:?} (computed at compile time)", MATRIX_TRANSPOSED.data);
    
    const IDENTITY_3X3: ConstMatrix<3, 3> = ConstMatrix::identity();
    const IDENTITY_TRACE: i32 = IDENTITY_3X3.trace();
    
    println!("3x3 Identity: {:?}", IDENTITY_3X3.data);
    println!("Identity trace: {IDENTITY_TRACE} (computed at compile time)");
    
    // Compile-time hash computations
    println!("\n🔐 Compile-Time Hash Functions:");
    println!("{:-<50}", "");
    
    const HASH_DJB2: u32 = const_hash_djb2("Hello, World!");
    const HASH_FNV1A: u32 = const_hash_fnv1a("Hello, World!");
    
    println!("DJB2 hash of \"Hello, World!\": 0x{HASH_DJB2:08X} (computed at compile time)");
    println!("FNV-1a hash of \"Hello, World!\": 0x{HASH_FNV1A:08X} (computed at compile time)");
    
    // Compile-time bit operations
    println!("\n🔢 Compile-Time Bit Operations:");
    println!("{:-<50}", "");
    
    const NUMBER: u64 = 0b1010110100110101;
    const POPCOUNT: u32 = const_popcount(NUMBER);
    const LEADING_ZEROS: u32 = const_leading_zeros(NUMBER);
    const TRAILING_ZEROS: u32 = const_trailing_zeros(NUMBER);
    const REVERSED: u64 = const_reverse_bits(NUMBER);
    
    println!("Number: 0b{NUMBER:016b} ({NUMBER})");
    println!("Population count: {POPCOUNT} (computed at compile time)");
    println!("Leading zeros: {LEADING_ZEROS} (computed at compile time)");
    println!("Trailing zeros: {TRAILING_ZEROS} (computed at compile time)");
    println!("Reversed bits: 0b{REVERSED:016b} (computed at compile time)");
    
    // Compile-time lookup tables
    println!("\n📋 Compile-Time Lookup Tables:");
    println!("{:-<50}", "");
    
    const PRIMES_100: [bool; 100] = generate_prime_table::<100>();
    
    print!("Primes up to 100: ");
    for i in 2..100 {
        if PRIMES_100[i] {
            print!("{i} ");
        }
    }
    println!("(computed at compile time)");
    
    // Compile-time configuration
    println!("\n⚙️ Compile-Time Configuration:");
    println!("{:-<50}", "");
    
    type MyConfig = CompileTimeConfig<1024, 100, 5000>;
    
    println!("Buffer size: {} bytes", MyConfig::buffer_size());
    println!("Max connections: {}", MyConfig::max_connections());
    println!("Timeout: {} ms", MyConfig::timeout_ms());
    println!("Total memory needed: {} bytes", MyConfig::total_memory_needed());
    println!("Config is valid: {}", MyConfig::is_valid_config());
    
    // Format string validation
    println!("\n✅ Compile-Time Format String Validation:");
    println!("{:-<50}", "");
    
    const VALID_FORMAT: bool = validate_format_string("Hello, {}! You have {} messages.");
    const INVALID_FORMAT: bool = validate_format_string("Hello, {}! You have } messages.");
    
    println!("\"Hello, {{}}! You have {{}} messages.\" is valid: {VALID_FORMAT}");
    println!("\"Hello, {{}}! You have }} messages.\" is valid: {INVALID_FORMAT}");
    
    println!("\n✅ All compile-time computations completed!");
}

// Compile-time assertions examples
const_assert!(const_fibonacci(10) == 55);
const_assert!(const_factorial(5) == 120);
const_assert!(const_power(2, 8) == 256);
const_assert_eq!(const_gcd(48, 18), 6);
const_assert_eq!(const_lcm(12, 8), 24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_fibonacci() {
        assert_eq!(const_fibonacci(0), 0);
        assert_eq!(const_fibonacci(1), 1);
        assert_eq!(const_fibonacci(10), 55);
        assert_eq!(const_fibonacci(20), 6765);
    }

    #[test]
    fn test_const_factorial() {
        assert_eq!(const_factorial(0), 1);
        assert_eq!(const_factorial(1), 1);
        assert_eq!(const_factorial(5), 120);
        assert_eq!(const_factorial(10), 3628800);
    }

    #[test]
    fn test_const_power() {
        assert_eq!(const_power(2, 0), 1);
        assert_eq!(const_power(2, 1), 2);
        assert_eq!(const_power(2, 8), 256);
        assert_eq!(const_power(3, 4), 81);
    }

    #[test]
    fn test_const_gcd_lcm() {
        assert_eq!(const_gcd(48, 18), 6);
        assert_eq!(const_gcd(17, 13), 1);
        assert_eq!(const_lcm(12, 8), 24);
        assert_eq!(const_lcm(17, 13), 221);
    }

    #[test]
    fn test_const_string_operations() {
        assert_eq!(const_str_len("hello"), 5);
        assert_eq!(const_str_len(""), 0);
        assert!(const_str_is_empty(""));
        assert!(!const_str_is_empty("hello"));
        assert_eq!(const_str_first_char("hello"), Some(b'h'));
        assert_eq!(const_str_first_char(""), None);
        assert_eq!(const_str_last_char("hello"), Some(b'o'));
        assert_eq!(const_str_last_char(""), None);
    }

    #[test]
    fn test_const_array_operations() {
        const ARR: [i32; 5] = [1, 3, 2, 5, 4];
        
        assert_eq!(const_array_sum(&ARR), 15);
        assert_eq!(const_array_max(&ARR), Some(5));
        assert_eq!(const_array_min(&ARR), Some(1));
        assert!(const_array_contains(&ARR, 3));
        assert!(!const_array_contains(&ARR, 6));
        
        const EMPTY: [i32; 0] = [];
        assert_eq!(const_array_max(&EMPTY), None);
        assert_eq!(const_array_min(&EMPTY), None);
    }

    #[test]
    fn test_const_matrix_operations() {
        const MATRIX: ConstMatrix<2, 2> = ConstMatrix::new([
            [1, 2],
            [3, 4]
        ]);
        
        const OTHER: ConstMatrix<2, 2> = ConstMatrix::new([
            [5, 6],
            [7, 8]
        ]);
        
        const SUM: ConstMatrix<2, 2> = MATRIX.add(&OTHER);
        const SCALED: ConstMatrix<2, 2> = MATRIX.scalar_multiply(2);
        const TRANSPOSED: ConstMatrix<2, 2> = MATRIX.transpose();
        
        assert_eq!(SUM.data, [[6, 8], [10, 12]]);
        assert_eq!(SCALED.data, [[2, 4], [6, 8]]);
        assert_eq!(TRANSPOSED.data, [[1, 3], [2, 4]]);
        
        const IDENTITY: ConstMatrix<3, 3> = ConstMatrix::identity();
        assert_eq!(IDENTITY.data, [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(IDENTITY.trace(), 3);
    }

    #[test]
    fn test_const_hash_functions() {
        const HASH1: u32 = const_hash_djb2("test");
        const HASH2: u32 = const_hash_djb2("test");
        const HASH3: u32 = const_hash_djb2("different");
        
        assert_eq!(HASH1, HASH2); // Same input should produce same hash
        assert_ne!(HASH1, HASH3); // Different input should produce different hash
        
        const FNV1: u32 = const_hash_fnv1a("test");
        const FNV2: u32 = const_hash_fnv1a("test");
        const FNV3: u32 = const_hash_fnv1a("different");
        
        assert_eq!(FNV1, FNV2);
        assert_ne!(FNV1, FNV3);
    }

    #[test]
    fn test_const_bit_operations() {
        const NUM: u64 = 0b1010110100110101;
        
        assert_eq!(const_popcount(NUM), 9);
        assert_eq!(const_popcount(0), 0);
        assert_eq!(const_popcount(u64::MAX), 64);
        
        assert_eq!(const_leading_zeros(0), 64);
        assert_eq!(const_leading_zeros(1), 63);
        
        assert_eq!(const_trailing_zeros(0), 64);
        assert_eq!(const_trailing_zeros(1), 0);
        assert_eq!(const_trailing_zeros(8), 3);
    }

    #[test]
    fn test_compile_time_config() {
        type TestConfig = CompileTimeConfig<512, 50, 3000>;
        
        assert_eq!(TestConfig::buffer_size(), 512);
        assert_eq!(TestConfig::max_connections(), 50);
        assert_eq!(TestConfig::timeout_ms(), 3000);
        assert_eq!(TestConfig::total_memory_needed(), 25600);
        assert!(TestConfig::is_valid_config());
        
        type InvalidConfig = CompileTimeConfig<0, 10, 1000>;
        assert!(!InvalidConfig::is_valid_config());
    }

    #[test]
    fn test_format_string_validation() {
        assert!(validate_format_string("Hello, {}!"));
        assert!(validate_format_string("Hello, {} and {}!"));
        assert!(validate_format_string("No placeholders"));
        assert!(validate_format_string("Escaped {{braces}}"));
        assert!(!validate_format_string("Unmatched }"));
        assert!(!validate_format_string("Unmatched {"));
        assert!(!validate_format_string("Mixed { and }}"));
    }

    #[test]
    fn test_type_level_computations() {
        assert_eq!(ConstFib::<10>::VALUE, 55);
        assert_eq!(ConstFact::<5>::VALUE, 120);
        assert_eq!(ConstPow::<2, 8>::VALUE, 256);
    }

    #[test]
    fn test_prime_table() {
        const PRIMES: [bool; 20] = generate_prime_table::<20>();
        
        assert!(!PRIMES[0]); // 0 is not prime
        assert!(!PRIMES[1]); // 1 is not prime
        assert!(PRIMES[2]);  // 2 is prime
        assert!(PRIMES[3]);  // 3 is prime
        assert!(!PRIMES[4]); // 4 is not prime
        assert!(PRIMES[5]);  // 5 is prime
        assert!(!PRIMES[6]); // 6 is not prime
        assert!(PRIMES[7]);  // 7 is prime
        assert!(!PRIMES[8]); // 8 is not prime
        assert!(!PRIMES[9]); // 9 is not prime
        assert!(!PRIMES[10]); // 10 is not prime
        assert!(PRIMES[11]); // 11 is prime
        assert!(!PRIMES[12]); // 12 is not prime
        assert!(PRIMES[13]); // 13 is prime
        assert!(!PRIMES[14]); // 14 is not prime
        assert!(!PRIMES[15]); // 15 is not prime
        assert!(!PRIMES[16]); // 16 is not prime
        assert!(PRIMES[17]); // 17 is prime
        assert!(!PRIMES[18]); // 18 is not prime
        assert!(PRIMES[19]); // 19 is prime
    }
}