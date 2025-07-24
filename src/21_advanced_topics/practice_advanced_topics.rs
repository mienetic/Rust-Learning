//! Practice Advanced Topics - แบบฝึกหัดหัวข้อขั้นสูงใน Rust 🚀🔬
//!
//! บทนี้จะฝึกฝนหัวข้อขั้นสูงใน Rust:
//! - Unsafe Rust และ Raw Pointers
//! - Foreign Function Interface (FFI)
//! - Custom Allocators
//! - Procedural Macros
//! - Compiler Plugins
//! - Advanced Type System Features
//! - Memory Layout และ Optimization
//! - Advanced Concurrency Patterns

use std::alloc::{GlobalAlloc, Layout, System};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ===== 1. Unsafe Rust และ Raw Pointers =====

/// ตัวอย่างการใช้ Unsafe Rust
pub struct UnsafeVector<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Default for UnsafeVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UnsafeVector<T> {
    #[must_use] pub const fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    #[must_use] pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout).cast::<T>() };
        
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }
    
    pub const fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.add(self.len)))
            }
        }
    }
    
    #[must_use] pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    #[must_use] pub const fn len(&self) -> usize {
        self.len
    }
    
    #[must_use] pub const fn capacity(&self) -> usize {
        self.capacity
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        
        let new_ptr = if self.capacity == 0 {
            unsafe { std::alloc::alloc(new_layout).cast::<T>() }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::realloc(
                    self.ptr.cast::<u8>(),
                    old_layout,
                    new_layout.size(),
                ).cast::<T>()
            }
        };
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for UnsafeVector<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.ptr.add(i));
                }
            }
            
            // Deallocate memory
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr.cast::<u8>(), layout);
            }
        }
    }
}

// ===== 2. Foreign Function Interface (FFI) =====

/// C functions declarations
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
}

/// Rust functions exported to C
#[unsafe(no_mangle)]
pub const extern "C" fn rust_add_advanced(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub const extern "C" fn rust_string_length_advanced(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    
    unsafe {
        let c_str = CStr::from_ptr(s);
        c_str.to_bytes().len()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_create_string(content: *const c_char) -> *mut c_char {
    if content.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let c_str = CStr::from_ptr(content);
        let rust_str = c_str.to_str().unwrap_or("");
        let new_string = format!("Rust: {rust_str}");
        
        match CString::new(new_string) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// FFI Helper functions
pub struct FFIHelper;

impl FFIHelper {
    #[must_use] pub fn call_c_strlen(s: &str) -> usize {
        let c_string = CString::new(s).unwrap();
        unsafe {
            strlen(c_string.as_ptr())
        }
    }
    
    #[must_use] pub fn call_c_strcmp(s1: &str, s2: &str) -> i32 {
        let c_string1 = CString::new(s1).unwrap();
        let c_string2 = CString::new(s2).unwrap();
        
        unsafe {
            strcmp(c_string1.as_ptr(), c_string2.as_ptr())
        }
    }
    
    #[must_use] pub fn allocate_c_memory(size: usize) -> *mut u8 {
        unsafe {
            malloc(size).cast::<u8>()
        }
    }
    
    pub fn free_c_memory(ptr: *mut u8) {
        if !ptr.is_null() {
            unsafe {
                free(ptr.cast::<std::ffi::c_void>());
            }
        }
    }
}

// ===== 3. Custom Allocators =====

/// Custom allocator ที่ track การใช้ memory
pub struct TrackingAllocator {
    allocated: AtomicUsize,
    deallocated: AtomicUsize,
    peak_usage: AtomicUsize,
}

impl Default for TrackingAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackingAllocator {
    #[must_use] pub const fn new() -> Self {
        Self {
            allocated: AtomicUsize::new(0),
            deallocated: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
        }
    }
    
    pub fn allocated(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }
    
    pub fn deallocated(&self) -> usize {
        self.deallocated.load(Ordering::Relaxed)
    }
    
    pub fn current_usage(&self) -> usize {
        self.allocated() - self.deallocated()
    }
    
    pub fn peak_usage(&self) -> usize {
        self.peak_usage.load(Ordering::Relaxed)
    }
    
    fn update_peak(&self, current: usize) {
        let mut peak = self.peak_usage.load(Ordering::Relaxed);
        while current > peak {
            match self.peak_usage.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(new_peak) => peak = new_peak,
            }
        }
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            let size = layout.size();
            self.allocated.fetch_add(size, Ordering::Relaxed);
            let current = self.current_usage();
            self.update_peak(current);
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.deallocated.fetch_add(layout.size(), Ordering::Relaxed);
    }
}

// Global instance of tracking allocator
#[global_allocator]
static TRACKING_ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

// ===== 4. Advanced Type System Features =====

/// Higher-Kinded Types simulation
pub trait HKT<T> {
    type Applied;
}

pub struct VecHKT;

impl<T> HKT<T> for VecHKT {
    type Applied = Vec<T>;
}

pub struct OptionHKT;

impl<T> HKT<T> for OptionHKT {
    type Applied = Option<T>;
}

/// Type-level programming with const generics
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    #[must_use] pub fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }
    
    pub const fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < ROWS && col < COLS {
            Some(&self.data[row][col])
        } else {
            None
        }
    }
    
    pub const fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row < ROWS && col < COLS {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    
    pub fn multiply<const OTHER_COLS: usize>(
        &self,
        other: &Matrix<T, COLS, OTHER_COLS>,
    ) -> Matrix<T, ROWS, OTHER_COLS>
    where
        T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
    {
        let mut result = Matrix::<T, ROWS, OTHER_COLS>::new();
        
        for i in 0..ROWS {
            for j in 0..OTHER_COLS {
                let mut sum = T::default();
                for k in 0..COLS {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        
        result
    }
}

/// Phantom Types for type safety
pub struct TypedId<T> {
    id: u64,
    _phantom: PhantomData<T>,
}

impl<T> TypedId<T> {
    #[must_use] pub const fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    #[must_use] pub const fn value(&self) -> u64 {
        self.id
    }
}

impl<T> Clone for TypedId<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for TypedId<T> {}

// Type markers
pub struct User;
pub struct Product;
pub struct Order;

pub type UserId = TypedId<User>;
pub type ProductId = TypedId<Product>;
pub type OrderId = TypedId<Order>;

// ===== 5. Advanced Concurrency Patterns =====

/// Lock-free data structures
pub struct LockFreeStack<T> {
    head: std::sync::atomic::AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Default for LockFreeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LockFreeStack<T> {
    #[must_use] pub const fn new() -> Self {
        Self {
            head: std::sync::atomic::AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
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
    
    pub fn pop(&self) -> Option<T> {
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

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

/// Actor Pattern Implementation
pub trait Actor {
    type Message;
    
    fn handle(&mut self, message: Self::Message);
}

pub struct ActorSystem<A: Actor> {
    actor: Arc<Mutex<A>>,
    sender: std::sync::mpsc::Sender<A::Message>,
    _handle: std::thread::JoinHandle<()>,
}

impl<A: Actor + Send + 'static> ActorSystem<A>
where
    A::Message: Send + 'static,
{
    pub fn new(actor: A) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let actor = Arc::new(Mutex::new(actor));
        let actor_clone = Arc::clone(&actor);
        
        let handle = std::thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                if let Ok(mut actor) = actor_clone.lock() {
                    actor.handle(message);
                }
            }
        });
        
        Self {
            actor,
            sender,
            _handle: handle,
        }
    }
    
    pub fn send(&self, message: A::Message) -> Result<(), std::sync::mpsc::SendError<A::Message>> {
        self.sender.send(message)
    }
}

// Example Actor
pub struct CounterActor {
    count: i32,
}

pub enum CounterMessage {
    Increment,
    Decrement,
    Get(std::sync::mpsc::Sender<i32>),
}

impl Default for CounterActor {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterActor {
    #[must_use] pub const fn new() -> Self {
        Self { count: 0 }
    }
}

impl Actor for CounterActor {
    type Message = CounterMessage;
    
    fn handle(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
            CounterMessage::Get(sender) => {
                let _ = sender.send(self.count);
            }
        }
    }
}

// ===== 6. Memory Layout Optimization =====

/// Packed structs for memory efficiency
#[repr(packed)]
pub struct PackedStruct {
    a: u8,
    b: u32,
    c: u16,
}

/// Aligned structs for performance
#[repr(align(64))] // Cache line alignment
pub struct AlignedStruct {
    data: [u8; 64],
}

/// Zero-cost abstractions
pub struct ZeroCostWrapper<T>(T);

impl<T> ZeroCostWrapper<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
    
    pub fn into_inner(self) -> T {
        self.0
    }
    
    pub const fn as_ref(&self) -> &T {
        &self.0
    }
    
    pub const fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

// ===== 7. Advanced Trait Patterns =====

/// Associated Type Families
pub trait Iterator2 {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}

/// Higher-Ranked Trait Bounds (HRTB)
pub fn apply_to_all<F>(f: F) -> impl Fn(&str) -> String
where
    F: for<'a> Fn(&'a str) -> String,
{
    move |s| f(s)
}

/// Trait Objects with Associated Types
pub trait Drawable {
    type Canvas;
    
    fn draw(&self, canvas: &mut Self::Canvas);
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Canvas {
    pixels: Vec<Vec<char>>,
}

impl Canvas {
    #[must_use] pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![' '; width]; height],
        }
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, ch: char) {
        if y < self.pixels.len() && x < self.pixels[y].len() {
            self.pixels[y][x] = ch;
        }
    }
    
    pub fn display(&self) {
        for row in &self.pixels {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

impl Drawable for Circle {
    type Canvas = Canvas;
    
    fn draw(&self, canvas: &mut Self::Canvas) {
        // Simplified circle drawing
        let center_x = canvas.pixels[0].len() / 2;
        let center_y = canvas.pixels.len() / 2;
        
        for y in 0..canvas.pixels.len() {
            for x in 0..canvas.pixels[y].len() {
                let dx = (x as f64 - center_x as f64).abs();
                let dy = (y as f64 - center_y as f64).abs();
                let distance = dx.hypot(dy);
                
                if (distance - self.radius).abs() < 1.0 {
                    canvas.set_pixel(x, y, 'O');
                }
            }
        }
    }
}

impl Drawable for Rectangle {
    type Canvas = Canvas;
    
    fn draw(&self, canvas: &mut Self::Canvas) {
        let start_x = (canvas.pixels[0].len() as f64 - self.width) as usize / 2;
        let start_y = (canvas.pixels.len() as f64 - self.height) as usize / 2;
        let end_x = start_x + self.width as usize;
        let end_y = start_y + self.height as usize;
        
        for y in start_y..end_y.min(canvas.pixels.len()) {
            for x in start_x..end_x.min(canvas.pixels[y].len()) {
                if y == start_y || y == end_y - 1 || x == start_x || x == end_x - 1 {
                    canvas.set_pixel(x, y, '#');
                }
            }
        }
    }
}

// ===== Main Practice Function =====

pub fn practice_advanced_topics() {
    println!("🚀 === Practice Advanced Topics: ฝึกฝนหัวข้อขั้นสูงใน Rust! === 🚀\n");
    
    // 1. Unsafe Rust
    println!("⚠️ === Unsafe Rust Examples === ⚠️");
    let mut unsafe_vec = UnsafeVector::new();
    unsafe_vec.push(1);
    unsafe_vec.push(2);
    unsafe_vec.push(3);
    
    // 🔍 ตรวจสอบข้อมูลใน UnsafeVector แบบปลอดภัย! 📊
    println!("UnsafeVector length: {}", unsafe_vec.len());
    println!("UnsafeVector capacity: {}", unsafe_vec.capacity());
    
    if let Some(value) = unsafe_vec.get(1) {
        println!("Value at index 1: {value}");
    }
    
    while let Some(value) = unsafe_vec.pop() {
        println!("Popped: {value}");
    }
    
    // 2. FFI Examples
    println!("\n🔗 === FFI Examples === 🔗");
    let test_string = "Hello, FFI!";
    let c_length = FFIHelper::call_c_strlen(test_string);
    println!("C strlen result: {c_length}");
    
    let cmp_result = FFIHelper::call_c_strcmp("hello", "world");
    println!("C strcmp result: {cmp_result}");
    
    // Test exported functions
    let sum = rust_add_advanced(5, 3);
    println!("Rust add result: {sum}");
    
    // 3. Custom Allocator
    println!("\n📊 === Memory Tracking === 📊");
    println!("Initial allocated: {} bytes", TRACKING_ALLOCATOR.allocated());
    println!("Initial deallocated: {} bytes", TRACKING_ALLOCATOR.deallocated());
    println!("Current usage: {} bytes", TRACKING_ALLOCATOR.current_usage());
    
    // Allocate some memory
    let _vec: Vec<i32> = (0..1000).collect();
    println!("After allocation - Current usage: {} bytes", TRACKING_ALLOCATOR.current_usage());
    println!("Peak usage: {} bytes", TRACKING_ALLOCATOR.peak_usage());
    
    // 4. Advanced Type System
    println!("\n🔢 === Advanced Type System === 🔢");
    let mut matrix: Matrix<i32, 2, 3> = Matrix::new();
    matrix.set(0, 0, 1).unwrap();
    matrix.set(0, 1, 2).unwrap();
    matrix.set(0, 2, 3).unwrap();
    matrix.set(1, 0, 4).unwrap();
    matrix.set(1, 1, 5).unwrap();
    matrix.set(1, 2, 6).unwrap();
    
    println!("Matrix element (0,1): {:?}", matrix.get(0, 1));
    
    // Typed IDs
    let user_id = UserId::new(123);
    let product_id = ProductId::new(456);
    
    println!("User ID: {}", user_id.value());
    println!("Product ID: {}", product_id.value());
    
    // 5. Lock-free Stack
    println!("\n🔒 === Lock-free Concurrency === 🔒");
    let stack = LockFreeStack::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(value) = stack.pop() {
        println!("Popped from lock-free stack: {value}");
    }
    
    // 6. Actor System
    println!("\n🎭 === Actor System === 🎭");
    let counter = CounterActor::new();
    let actor_system = ActorSystem::new(counter);
    
    actor_system.send(CounterMessage::Increment).unwrap();
    actor_system.send(CounterMessage::Increment).unwrap();
    actor_system.send(CounterMessage::Decrement).unwrap();
    
    let (sender, receiver) = std::sync::mpsc::channel();
    actor_system.send(CounterMessage::Get(sender)).unwrap();
    
    if let Ok(count) = receiver.recv() {
        println!("Counter value: {count}");
    }
    
    // 7. Memory Layout
    println!("\n💾 === Memory Layout === 💾");
    println!("PackedStruct size: {}", mem::size_of::<PackedStruct>());
    println!("AlignedStruct size: {}", mem::size_of::<AlignedStruct>());
    println!("AlignedStruct alignment: {}", mem::align_of::<AlignedStruct>());
    
    let wrapper = ZeroCostWrapper::new(42);
    println!("ZeroCostWrapper value: {}", wrapper.as_ref());
    println!("ZeroCostWrapper size: {}", mem::size_of_val(&wrapper));
    println!("Original type size: {}", mem::size_of::<i32>());
    
    // 8. Drawing with Trait Objects
    println!("\n🎨 === Drawing System === 🎨");
    let mut canvas = Canvas::new(20, 10);
    
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 8.0, height: 4.0 };
    
    circle.draw(&mut canvas);
    println!("Circle:");
    canvas.display();
    
    let mut canvas2 = Canvas::new(20, 10);
    rectangle.draw(&mut canvas2);
    println!("\nRectangle:");
    canvas2.display();
    
    println!("\n🎉 === Advanced Topics Practice Complete! === 🎉");
}

// ===== Advanced Patterns and Techniques =====

pub fn advanced_patterns_examples() {
    println!("\n🔬 === Advanced Patterns === 🔬");
    
    println!("\n📝 Advanced Pattern Techniques:");
    println!("  • Unsafe Rust for performance-critical code");
    println!("  • FFI for interoperability with C libraries");
    println!("  • Custom allocators for memory management");
    println!("  • Type-level programming with const generics");
    println!("  • Phantom types for compile-time safety");
    println!("  • Lock-free data structures for concurrency");
    println!("  • Actor pattern for message-passing concurrency");
    println!("  • Memory layout optimization");
    println!("  • Zero-cost abstractions");
    println!("  • Higher-ranked trait bounds (HRTB)");
    
    println!("\n⚠️ Safety Considerations:");
    println!("  • Always validate unsafe code thoroughly");
    println!("  • Use unsafe only when necessary for performance");
    println!("  • Prefer safe abstractions over raw unsafe code");
    println!("  • Document safety invariants clearly");
    println!("  • Use tools like Miri for unsafe code testing");
    
    println!("\n🚀 Performance Tips:");
    println!("  • Profile before optimizing");
    println!("  • Use const generics for compile-time optimization");
    println!("  • Consider memory layout for cache efficiency");
    println!("  • Use lock-free structures for high-contention scenarios");
    println!("  • Minimize allocations in hot paths");
    
    println!("\n🔧 Advanced Tools:");
    println!("  • cargo-expand: View macro expansions");
    println!("  • cargo-asm: View generated assembly");
    println!("  • cargo-bloat: Analyze binary size");
    println!("  • valgrind: Memory debugging");
    println!("  • perf: Performance profiling");
    println!("  • miri: Undefined behavior detection");
}

pub fn practice_advanced_topics_extended() {
    println!("🚀 === Extended Advanced Topics Practice === 🚀\n");
    
    advanced_patterns_examples();
    
    println!("\n🎉 === Extended Advanced Topics Practice Complete! === 🎉");
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_vector() {
        let mut vec = UnsafeVector::new();
        assert_eq!(vec.len(), 0);
        
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_ffi_functions() {
        let result = rust_add_advanced(5, 3);
        assert_eq!(result, 8);
        
        let length = FFIHelper::call_c_strlen("hello");
        assert_eq!(length, 5);
        
        let cmp = FFIHelper::call_c_strcmp("abc", "abc");
        assert_eq!(cmp, 0);
    }

    #[test]
    fn test_matrix() {
        let mut matrix: Matrix<i32, 2, 2> = Matrix::new();
        
        matrix.set(0, 0, 1).unwrap();
        matrix.set(0, 1, 2).unwrap();
        matrix.set(1, 0, 3).unwrap();
        matrix.set(1, 1, 4).unwrap();
        
        assert_eq!(matrix.get(0, 0), Some(&1));
        assert_eq!(matrix.get(1, 1), Some(&4));
        assert_eq!(matrix.get(2, 0), None);
    }

    #[test]
    fn test_typed_ids() {
        let user_id = UserId::new(123);
        let product_id = ProductId::new(456);
        
        assert_eq!(user_id.value(), 123);
        assert_eq!(product_id.value(), 456);
        
        // This should not compile (type safety):
        // let _: UserId = product_id; // Error!
    }

    #[test]
    fn test_lock_free_stack() {
        let stack = LockFreeStack::new();
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_actor_system() {
        let counter = CounterActor::new();
        let actor_system = ActorSystem::new(counter);
        
        actor_system.send(CounterMessage::Increment).unwrap();
        actor_system.send(CounterMessage::Increment).unwrap();
        
        let (sender, receiver) = std::sync::mpsc::channel();
        actor_system.send(CounterMessage::Get(sender)).unwrap();
        
        let count = receiver.recv().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_memory_layout() {
        // Test that packed struct is smaller
        assert!(mem::size_of::<PackedStruct>() <= 7); // u8 + u32 + u16 = 7 bytes
        
        // Test alignment
        assert_eq!(mem::align_of::<AlignedStruct>(), 64);
        
        // Test zero-cost wrapper
        assert_eq!(mem::size_of::<ZeroCostWrapper<i32>>(), mem::size_of::<i32>());
    }

    #[test]
    fn test_canvas_drawing() {
        let mut canvas = Canvas::new(10, 10);
        let circle = Circle { radius: 2.0 };
        
        circle.draw(&mut canvas);
        
        // Check that some pixels were set (simplified test)
        let mut has_circle_pixels = false;
        for row in &canvas.pixels {
            for &pixel in row {
                if pixel == 'O' {
                    has_circle_pixels = true;
                    break;
                }
            }
        }
        assert!(has_circle_pixels);
    }

    #[test]
    fn test_tracking_allocator() {
        let initial_allocated = TRACKING_ALLOCATOR.allocated();
        
        // Allocate more memory to ensure the test passes
        let _vec1: Vec<i32> = vec![1; 1000];
        let _vec2: Vec<String> = vec!["test".to_string(); 100];
        let _vec3: Vec<u64> = (0..500).collect();
        
        let after_allocated = TRACKING_ALLOCATOR.allocated();
        assert!(after_allocated > initial_allocated, 
                "Expected after_allocated ({}) > initial_allocated ({})", 
                after_allocated, initial_allocated);
    }
}