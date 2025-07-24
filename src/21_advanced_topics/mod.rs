//! บทที่ 22: Advanced Topics - หัวข้อขั้นสูงใน Rust 🚀
//!
//! บทนี้ครอบคลุมหัวข้อขั้นสูงที่จำเป็นสำหรับการพัฒนา Rust ในระดับ Expert:
//! - Unsafe Rust และการจัดการ Memory ระดับต่ำ
//! - Foreign Function Interface (FFI)
//! - Custom Allocators และ Memory Layout Optimization
//! - Advanced Type System (HKT, Const Generics, Phantom Types)
//! - Advanced Concurrency Patterns
//! - Zero-cost Abstractions และ Performance Optimization
//! - Advanced Trait Patterns และ Type-level Programming

pub mod practice_advanced_topics;

pub use practice_advanced_topics::*;

use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::marker::PhantomData;
use std::mem;

/// ตัวอย่าง Unsafe Rust - Raw Pointer Operations
pub fn unsafe_pointer_operations() {
    println!("\n=== Unsafe Pointer Operations ===");
    
    let mut x = 42;
    let raw_ptr = &raw mut x;
    
    unsafe {
        *raw_ptr = 100;
        println!("Value through raw pointer: {}", *raw_ptr);
    }
    
    // Pointer arithmetic
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    
    unsafe {
        for i in 0..arr.len() {
            let val = *ptr.add(i);
            println!("arr[{i}] = {val}");
        }
    }
}

/// ตัวอย่าง Custom Allocator
struct TrackingAllocator {
    allocated: AtomicUsize,
}

impl TrackingAllocator {
    const fn new() -> Self {
        Self {
            allocated: AtomicUsize::new(0),
        }
    }
    
    fn allocated_bytes(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 🔥 เรียกใช้ allocator ระบบแบบปลอดภัย! 🛡️
        let ptr = unsafe { std::alloc::System.alloc(layout) };
        if !ptr.is_null() {
            self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // 🗑️ คืนหน่วยความจำแบบปลอดภัย! ♻️
        unsafe { std::alloc::System.dealloc(ptr, layout) };
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

/// ตัวอย่าง Phantom Types
struct TypedId<T> {
    id: u64,
    _phantom: PhantomData<T>,
}

struct User;
struct Product;

impl<T> TypedId<T> {
    const fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    const fn value(&self) -> u64 {
        self.id
    }
}

pub fn phantom_types_example() {
    println!("\n=== Phantom Types Example ===");
    
    let user_id: TypedId<User> = TypedId::new(123);
    let product_id: TypedId<Product> = TypedId::new(456);
    
    println!("User ID: {}", user_id.value());
    println!("Product ID: {}", product_id.value());
    
    // This would cause a compile error:
    // let _wrong: TypedId<User> = product_id; // Type mismatch!
}

/// ตัวอย่าง Const Generics
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }
    
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }
    
    const fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row >= ROWS || col >= COLS {
            return Err("Index out of bounds");
        }
        self.data[row][col] = value;
        Ok(())
    }
}

pub fn const_generics_example() {
    println!("\n=== Const Generics Example ===");
    
    let mut matrix: Matrix<i32, 3, 3> = Matrix::new();
    
    matrix.set(0, 0, 1).unwrap();
    matrix.set(1, 1, 5).unwrap();
    matrix.set(2, 2, 9).unwrap();
    
    for row in 0..3 {
        for col in 0..3 {
            if let Some(value) = matrix.get(row, col) {
                print!("{value:3} ");
            }
        }
        println!();
    }
}

/// ตัวอย่าง Advanced Trait Patterns - Associated Type Families
trait Iterator2 {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}

trait Collect<T> {
    type Output;
    
    fn collect(iter: T) -> Self::Output;
}

/// ตัวอย่าง Higher-Ranked Trait Bounds (HRTB)
fn apply_to_all<F>(f: F) -> impl Fn(&str) -> String
where
    F: for<'a> Fn(&'a str) -> String,
{
    move |s| f(s)
}

pub fn hrtb_example() {
    println!("\n=== Higher-Ranked Trait Bounds Example ===");
    
    let uppercase = apply_to_all(|s: &str| s.to_uppercase());
    let result = uppercase("hello world");
    println!("Uppercase: {result}");
}

/// ตัวอย่าง Memory Layout Optimization
#[repr(C)]
struct OptimizedStruct {
    flag: bool,     // 1 byte
    value: u64,     // 8 bytes (aligned to 8-byte boundary)
    count: u32,     // 4 bytes
    // Total: 24 bytes due to padding
}

#[repr(C, packed)]
struct PackedStruct {
    flag: bool,     // 1 byte
    value: u64,     // 8 bytes (no alignment)
    count: u32,     // 4 bytes
    // Total: 13 bytes (no padding)
}

pub fn memory_layout_example() {
    println!("\n=== Memory Layout Example ===");
    
    println!("OptimizedStruct size: {} bytes", mem::size_of::<OptimizedStruct>());
    println!("PackedStruct size: {} bytes", mem::size_of::<PackedStruct>());
    
    println!("bool size: {} bytes", mem::size_of::<bool>());
    println!("u64 size: {} bytes", mem::size_of::<u64>());
    println!("u32 size: {} bytes", mem::size_of::<u32>());
}

/// ตัวอย่าง Lock-free Data Structure
use std::sync::atomic::AtomicPtr;

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    const fn new() -> Self {
        Self {
            head: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: std::ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }
            
            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            match self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let data = unsafe { Box::from_raw(head).data };
                    return Some(data);
                }
                Err(_) => continue,
            }
        }
    }
}

pub fn lock_free_example() {
    println!("\n=== Lock-free Data Structure Example ===");
    
    let stack = LockFreeStack::new();
    
    // Push some values
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Pop values
    while let Some(value) = stack.pop() {
        println!("Popped: {value}");
    }
}

/// ตัวอย่าง Zero-cost Abstractions
trait IntoIterator2 {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}

struct Range2 {
    start: usize,
    end: usize,
}

struct RangeIter {
    current: usize,
    end: usize,
}

impl Iterator for RangeIter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl IntoIterator2 for Range2 {
    type Item = usize;
    type IntoIter = RangeIter;
    
    fn into_iter(self) -> Self::IntoIter {
        RangeIter {
            current: self.start,
            end: self.end,
        }
    }
}

pub fn zero_cost_abstractions_example() {
    println!("\n=== Zero-cost Abstractions Example ===");
    
    let range = Range2 { start: 0, end: 5 };
    let sum: usize = range.into_iter()
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .sum();
    
    println!("Sum of even squares: {sum}");
}

/// ฟังก์ชันหลักที่รวมตัวอย่างทั้งหมด
pub fn run_advanced_topics_examples() {
    println!("🚀 Advanced Topics Examples");
    println!("{}", "=".repeat(50));
    
    unsafe_pointer_operations();
    phantom_types_example();
    const_generics_example();
    hrtb_example();
    memory_layout_example();
    lock_free_example();
    zero_cost_abstractions_example();
    
    // เรียกใช้ตัวอย่างจาก practice_advanced_topics
    println!("\n📚 Practice Examples:");
    practice_advanced_topics();
    advanced_patterns_examples();
    
    println!("\n✅ Advanced Topics examples completed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phantom_types() {
        let user_id: TypedId<User> = TypedId::new(123);
        assert_eq!(user_id.value(), 123);
    }
    
    #[test]
    fn test_const_generics() {
        let mut matrix: Matrix<i32, 2, 2> = Matrix::new();
        matrix.set(0, 0, 42).unwrap();
        assert_eq!(matrix.get(0, 0), Some(&42));
    }
    
    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        stack.push(1);
        stack.push(2);
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
    
    #[test]
    fn test_memory_layout() {
        // Test that packed struct is smaller
        assert!(mem::size_of::<PackedStruct>() < mem::size_of::<OptimizedStruct>());
    }
}