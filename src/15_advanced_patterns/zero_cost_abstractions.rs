//! Zero-Cost Abstractions Implementation
//!
//! การใช้งาน Zero-Cost Abstractions ใน Rust
//! ครอบคลุม Iterator Patterns, Generic Programming, และ Compile-time Optimizations

use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Zero-cost iterator wrapper
#[derive(Debug)]
pub struct ZeroCostIterator<I> {
    inner: I,
}

impl<I> ZeroCostIterator<I> {
    pub const fn new(iter: I) -> Self {
        Self { inner: iter }
    }
}

impl<I: Iterator> Iterator for ZeroCostIterator<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for ZeroCostIterator<I> {}

impl<I: DoubleEndedIterator> DoubleEndedIterator for ZeroCostIterator<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

/// Zero-cost mathematical operations
pub trait ZeroCostMath<T> {
    fn zero_cost_add(self, other: T) -> T;
    fn zero_cost_multiply(self, other: T) -> T;
    fn zero_cost_square(self) -> T;
}

impl ZeroCostMath<Self> for i32 {
    #[inline(always)]
    fn zero_cost_add(self, other: Self) -> Self {
        self + other
    }
    
    #[inline(always)]
    fn zero_cost_multiply(self, other: Self) -> Self {
        self * other
    }
    
    #[inline(always)]
    fn zero_cost_square(self) -> Self {
        self * self
    }
}

impl ZeroCostMath<Self> for f64 {
    #[inline(always)]
    fn zero_cost_add(self, other: Self) -> Self {
        self + other
    }
    
    #[inline(always)]
    fn zero_cost_multiply(self, other: Self) -> Self {
        self * other
    }
    
    #[inline(always)]
    fn zero_cost_square(self) -> Self {
        self * self
    }
}

/// Zero-cost vector operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Add<Output = T> + Copy> Add for Vector3D<T> {
    type Output = Self;
    
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Vector3D<T> {
    #[inline(always)]
    pub fn scale(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector3D<T> {
    #[inline(always)]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Zero-cost state machine
pub struct StateMachine<State> {
    _state: PhantomData<State>,
}

#[derive(Debug)]
pub struct Idle;

#[derive(Debug)]
pub struct Running;

#[derive(Debug)]
pub struct Stopped;

impl Default for StateMachine<Idle> {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine<Idle> {
    #[must_use] pub const fn new() -> Self {
        Self { _state: PhantomData }
    }
    
    #[inline(always)]
    #[must_use] pub const fn start(self) -> StateMachine<Running> {
        StateMachine { _state: PhantomData }
    }
}

impl StateMachine<Running> {
    #[inline(always)]
    #[must_use] pub const fn stop(self) -> StateMachine<Stopped> {
        StateMachine { _state: PhantomData }
    }
    
    #[inline(always)]
    #[must_use] pub const fn pause(self) -> StateMachine<Idle> {
        StateMachine { _state: PhantomData }
    }
    
    #[inline(always)]
    #[must_use] pub const fn process_data(&self, data: &[u8]) -> usize {
        // Simulate processing
        data.len()
    }
}

impl StateMachine<Stopped> {
    #[inline(always)]
    #[must_use] pub const fn reset(self) -> StateMachine<Idle> {
        StateMachine { _state: PhantomData }
    }
}

/// Zero-cost builder pattern
#[derive(Debug)]
pub struct ConfigBuilder<HasName, HasValue, HasTimeout> {
    name: Option<String>,
    value: Option<i32>,
    timeout: Option<u64>,
    _markers: PhantomData<(HasName, HasValue, HasTimeout)>,
}

#[derive(Debug)]
pub struct Yes;

#[derive(Debug)]
pub struct No;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub value: i32,
    pub timeout: u64,
}

impl Default for ConfigBuilder<No, No, No> {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder<No, No, No> {
    #[must_use] pub const fn new() -> Self {
        Self {
            name: None,
            value: None,
            timeout: None,
            _markers: PhantomData,
        }
    }
}

impl<HasValue, HasTimeout> ConfigBuilder<No, HasValue, HasTimeout> {
    #[inline(always)]
    #[must_use] pub fn name(self, name: String) -> ConfigBuilder<Yes, HasValue, HasTimeout> {
        ConfigBuilder {
            name: Some(name),
            value: self.value,
            timeout: self.timeout,
            _markers: PhantomData,
        }
    }
}

impl<HasName, HasTimeout> ConfigBuilder<HasName, No, HasTimeout> {
    #[inline(always)]
    #[must_use] pub fn value(self, value: i32) -> ConfigBuilder<HasName, Yes, HasTimeout> {
        ConfigBuilder {
            name: self.name,
            value: Some(value),
            timeout: self.timeout,
            _markers: PhantomData,
        }
    }
}

impl<HasName, HasValue> ConfigBuilder<HasName, HasValue, No> {
    #[inline(always)]
    #[must_use] pub fn timeout(self, timeout: u64) -> ConfigBuilder<HasName, HasValue, Yes> {
        ConfigBuilder {
            name: self.name,
            value: self.value,
            timeout: Some(timeout),
            _markers: PhantomData,
        }
    }
}

impl ConfigBuilder<Yes, Yes, Yes> {
    #[inline(always)]
    #[must_use] pub fn build(self) -> Config {
        Config {
            name: self.name.unwrap(),
            value: self.value.unwrap(),
            timeout: self.timeout.unwrap(),
        }
    }
}

/// Zero-cost functional programming constructs
pub const fn zero_cost_map<I, F, U>(iter: I, f: F) -> ZeroCostMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    ZeroCostMap { iter, f }
}

pub const fn zero_cost_filter<I, F>(iter: I, predicate: F) -> ZeroCostFilter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    ZeroCostFilter { iter, predicate }
}

pub fn zero_cost_fold<I, U, F>(iter: I, mut init: U, mut f: F) -> U
where
    I: Iterator,
    F: FnMut(U, I::Item) -> U,
{
    for item in iter {
        init = f(init, item);
    }
    init
}

#[derive(Debug)]
pub struct ZeroCostMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, U> Iterator for ZeroCostMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    type Item = U;
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Debug)]
pub struct ZeroCostFilter<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for ZeroCostFilter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

/// Zero-cost generic algorithms
pub trait ZeroCostAlgorithms {
    type Item;
    
    fn zero_cost_binary_search(&self, target: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord;
    
    fn zero_cost_partition_point<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&Self::Item) -> bool;
}

impl<T> ZeroCostAlgorithms for [T] {
    type Item = T;
    
    #[inline(always)]
    fn zero_cost_binary_search(&self, target: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search(target)
    }
    
    #[inline(always)]
    fn zero_cost_partition_point<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.partition_point(predicate)
    }
}

/// Zero-cost memory layout optimization
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OptimizedStruct {
    pub flag: bool,      // 1 byte
    pub value: u64,      // 8 bytes (aligned)
    pub count: u32,      // 4 bytes
    pub small_flag: bool, // 1 byte
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct PackedStruct {
    pub flag: bool,      // 1 byte
    pub value: u64,      // 8 bytes (no padding)
    pub count: u32,      // 4 bytes
    pub small_flag: bool, // 1 byte
}

/// Zero-cost compile-time computations
#[must_use] pub const fn zero_cost_fibonacci(n: u32) -> u64 {
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

#[must_use] pub const fn zero_cost_factorial(n: u32) -> u64 {
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

#[must_use] pub const fn zero_cost_power(base: u64, exp: u32) -> u64 {
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

/// Zero-cost type-level programming
pub trait TypeLevelNumber {
    const VALUE: usize;
}

#[derive(Debug)]
pub struct Zero;

#[derive(Debug)]
pub struct Succ<N>(PhantomData<N>);

impl TypeLevelNumber for Zero {
    const VALUE: usize = 0;
}

impl<N: TypeLevelNumber> TypeLevelNumber for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;
pub type Four = Succ<Three>;
pub type Five = Succ<Four>;

/// Zero-cost array with compile-time size
#[derive(Debug)]
pub struct TypedArray<T, N: TypeLevelNumber> {
    data: Vec<T>,
    _size: PhantomData<N>,
}

impl<T, N: TypeLevelNumber> Default for TypedArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, N: TypeLevelNumber> TypedArray<T, N> {
    #[must_use] pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(N::VALUE),
            _size: PhantomData,
        }
    }
    
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.data.len() < N::VALUE {
            self.data.push(item);
            Ok(())
        } else {
            Err(item)
        }
    }
    
    #[must_use] pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    
    #[must_use] pub const fn len(&self) -> usize {
        self.data.len()
    }
    
    #[must_use] pub const fn capacity() -> usize {
        N::VALUE
    }
    
    #[must_use] pub const fn is_full(&self) -> bool {
        self.data.len() == N::VALUE
    }
}

/// Zero-cost performance measurement
#[derive(Debug)]
pub struct PerformanceCounter {
    operations: u64,
}

impl Default for PerformanceCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceCounter {
    #[must_use] pub const fn new() -> Self {
        Self { operations: 0 }
    }
    
    #[inline(always)]
    pub const fn increment(&mut self) {
        self.operations += 1;
    }
    
    #[inline(always)]
    pub const fn add(&mut self, count: u64) {
        self.operations += count;
    }
    
    #[must_use] pub const fn get_count(&self) -> u64 {
        self.operations
    }
    
    pub const fn reset(&mut self) {
        self.operations = 0;
    }
}

/// สาธิตการใช้งาน Zero-Cost Abstractions
pub fn demonstrate_zero_cost_abstractions() {
    println!("⚡ Zero-Cost Abstractions Examples:");
    
    // Zero-cost iterators
    println!("\n🔄 Zero-Cost Iterators:");
    println!("{:-<50}", "");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Chain multiple zero-cost operations
    let result: Vec<i32> = zero_cost_map(
        zero_cost_filter(
            data.iter().copied(),
            |&x| x % 2 == 0
        ),
        |x| x * x
    ).collect();
    
    println!("Original: {data:?}");
    println!("Even numbers squared: {result:?}");
    
    // Zero-cost fold
    let sum = zero_cost_fold(
        data.iter().copied(),
        0,
        |acc, x| acc + x
    );
    
    println!("Sum: {sum}");
    
    // Zero-cost mathematical operations
    println!("\n🧮 Zero-Cost Math:");
    println!("{:-<50}", "");
    
    let a = 10i32;
    let b = 20i32;
    
    let sum = a.zero_cost_add(b);
    let product = a.zero_cost_multiply(b);
    let square = a.zero_cost_square();
    
    println!("a = {a}, b = {b}");
    println!("Sum: {sum}");
    println!("Product: {product}");
    println!("Square of a: {square}");
    
    // Zero-cost vector operations
    println!("\n📐 Zero-Cost Vector Operations:");
    println!("{:-<50}", "");
    
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);
    
    let sum_vec = v1.clone() + v2.clone();
    let scaled = v1.clone().scale(2.0);
    let dot_product = v1.clone().dot(v2.clone());
    
    println!("v1: {v1:?}");
    println!("v2: {v2:?}");
    println!("v1 + v2: {sum_vec:?}");
    println!("v1 * 2: {scaled:?}");
    println!("v1 · v2: {dot_product:.2}");
    
    // Zero-cost state machine
    println!("\n🔄 Zero-Cost State Machine:");
    println!("{:-<50}", "");
    
    let machine = StateMachine::<Idle>::new();
    println!("Created idle state machine");
    
    let machine = machine.start();
    println!("Started machine");
    
    let processed = machine.process_data(b"Hello, World!");
    println!("Processed {processed} bytes");
    
    let machine = machine.stop();
    println!("Stopped machine");
    
    let _machine = machine.reset();
    println!("Reset to idle state");
    
    // Zero-cost builder pattern
    println!("\n🔨 Zero-Cost Builder Pattern:");
    println!("{:-<50}", "");
    
    let config = ConfigBuilder::new()
        .name("MyConfig".to_string())
        .value(42)
        .timeout(5000)
        .build();
    
    println!("Built config: {config:?}");
    
    // Compile-time computations
    println!("\n⏱️ Compile-Time Computations:");
    println!("{:-<50}", "");
    
    const FIB_10: u64 = zero_cost_fibonacci(10);
    const FACT_5: u64 = zero_cost_factorial(5);
    const POWER_2_8: u64 = zero_cost_power(2, 8);
    
    println!("Fibonacci(10): {FIB_10} (computed at compile time)");
    println!("Factorial(5): {FACT_5} (computed at compile time)");
    println!("2^8: {POWER_2_8} (computed at compile time)");
    
    // Type-level programming
    println!("\n🎯 Type-Level Programming:");
    println!("{:-<50}", "");
    
    let mut array: TypedArray<i32, Five> = TypedArray::new();
    
    println!("Created array with capacity: {}", TypedArray::<i32, Five>::capacity());
    
    for i in 1..=7 {
        match array.push(i) {
            Ok(()) => println!("Pushed {i}"),
            Err(item) => println!("Failed to push {item} (array full)"),
        }
    }
    
    println!("Array length: {}/{}", array.len(), TypedArray::<i32, Five>::capacity());
    println!("Array is full: {}", array.is_full());
    
    // Memory layout optimization
    println!("\n💾 Memory Layout Optimization:");
    println!("{:-<50}", "");
    
    println!("OptimizedStruct size: {} bytes", std::mem::size_of::<OptimizedStruct>());
    println!("PackedStruct size: {} bytes", std::mem::size_of::<PackedStruct>());
    
    let optimized = OptimizedStruct {
        flag: true,
        value: 0x1234567890ABCDEF,
        count: 42,
        small_flag: false,
    };
    
    let packed = PackedStruct {
        flag: true,
        value: 0x1234567890ABCDEF,
        count: 42,
        small_flag: false,
    };
    
    println!("OptimizedStruct: {optimized:?}");
    println!("PackedStruct: {packed:?}");
    
    // Performance measurement
    println!("\n📊 Performance Measurement:");
    println!("{:-<50}", "");
    
    let mut counter = PerformanceCounter::new();
    
    // Simulate some operations
    for i in 0..1000 {
        counter.increment();
        if i % 100 == 0 {
            counter.add(10); // Bonus operations
        }
    }
    
    println!("Total operations: {}", counter.get_count());
    
    counter.reset();
    println!("After reset: {}", counter.get_count());
    
    // Demonstrate zero-cost nature
    println!("\n✨ Zero-Cost Demonstration:");
    println!("{:-<50}", "");
    
    let large_data: Vec<i32> = (0..1_000_000).collect();
    
    // This chain of operations compiles to highly optimized code
    let processed_count = zero_cost_filter(
        zero_cost_map(
            zero_cost_filter(
                large_data.iter().copied(),
                |&x| x % 2 == 0
            ),
            |x| x * 2
        ),
        |&x| x > 1000
    ).count();
    
    println!("Processed {processed_count} items from 1M elements");
    println!("All operations were zero-cost abstractions!");
    
    println!("\n✅ Zero-cost abstractions demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_cost_math() {
        let a = 5i32;
        let b = 3i32;
        
        assert_eq!(a.zero_cost_add(b), 8);
        assert_eq!(a.zero_cost_multiply(b), 15);
        assert_eq!(a.zero_cost_square(), 25);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        
        let sum = v1.clone() + v2.clone();
        assert_eq!(sum, Vector3D::new(5.0, 7.0, 9.0));
        
        let scaled = v1.clone().scale(2.0);
        assert_eq!(scaled, Vector3D::new(2.0, 4.0, 6.0));
        
        let dot = v1.dot(v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_state_machine() {
        let machine = StateMachine::<Idle>::new();
        let machine = machine.start();
        let processed = machine.process_data(b"test");
        assert_eq!(processed, 4);
        
        let machine = machine.stop();
        let _machine = machine.reset();
    }

    #[test]
    fn test_builder_pattern() {
        let config = ConfigBuilder::new()
            .name("test".to_string())
            .value(123)
            .timeout(1000)
            .build();
        
        assert_eq!(config.name, "test");
        assert_eq!(config.value, 123);
        assert_eq!(config.timeout, 1000);
    }

    #[test]
    fn test_compile_time_computations() {
        const FIB_5: u64 = zero_cost_fibonacci(5);
        const FACT_4: u64 = zero_cost_factorial(4);
        const POWER_3_4: u64 = zero_cost_power(3, 4);
        
        assert_eq!(FIB_5, 5); // 0, 1, 1, 2, 3, 5
        assert_eq!(FACT_4, 24); // 4! = 4 * 3 * 2 * 1 = 24
        assert_eq!(POWER_3_4, 81); // 3^4 = 81
    }

    #[test]
    fn test_typed_array() {
        let mut array: TypedArray<i32, Three> = TypedArray::new();
        
        assert_eq!(TypedArray::<i32, Three>::capacity(), 3);
        assert_eq!(array.len(), 0);
        assert!(!array.is_full());
        
        assert!(array.push(1).is_ok());
        assert!(array.push(2).is_ok());
        assert!(array.push(3).is_ok());
        assert!(array.push(4).is_err()); // Should fail, array is full
        
        assert_eq!(array.len(), 3);
        assert!(array.is_full());
        assert_eq!(array.get(1), Some(&2));
    }

    #[test]
    fn test_zero_cost_functional() {
        let data = [1, 2, 3, 4, 5, 6];
        
        let result: Vec<i32> = zero_cost_map(
            zero_cost_filter(
                data.iter().copied(),
                |&x| x % 2 == 0
            ),
            |x| x * x
        ).collect();
        
        assert_eq!(result, vec![4, 16, 36]); // 2^2, 4^2, 6^2
        
        let sum = zero_cost_fold(
            data.iter().copied(),
            0,
            |acc, x| acc + x
        );
        
        assert_eq!(sum, 21); // 1+2+3+4+5+6 = 21
    }

    #[test]
    fn test_memory_layout() {
        // These tests verify that our structs have expected sizes
        assert!(std::mem::size_of::<OptimizedStruct>() >= 14); // At least the sum of field sizes
        assert_eq!(std::mem::size_of::<PackedStruct>(), 14); // Exactly the sum of field sizes
    }

    #[test]
    fn test_performance_counter() {
        let mut counter = PerformanceCounter::new();
        
        assert_eq!(counter.get_count(), 0);
        
        counter.increment();
        assert_eq!(counter.get_count(), 1);
        
        counter.add(10);
        assert_eq!(counter.get_count(), 11);
        
        counter.reset();
        assert_eq!(counter.get_count(), 0);
    }
}