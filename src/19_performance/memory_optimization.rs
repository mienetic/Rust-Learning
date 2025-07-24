//! 🧠 Memory Optimization Techniques - Web Development Workshop
//!
//! 🎯 เทคนิคการเพิ่มประสิทธิภาพหน่วยความจำใน Rust สำหรับเวิร์กช็อป
//! 🌟 เหมาะสำหรับนักพัฒนาเว็บที่ต้องการเพิ่มประสิทธิภาพ

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

/// 📐 การจัดการ Memory Layout - Workshop Edition
#[repr(C)]
struct OptimizedStruct {
    // จัดเรียงฟิลด์ตามขนาดเพื่อลด padding
    large_field: u64,    // 8 bytes
    medium_field: u32,   // 4 bytes
    small_field: u16,    // 2 bytes
    tiny_field: u8,      // 1 byte
    flag: bool,          // 1 byte
}

#[repr(C, packed)]
struct PackedStruct {
    a: u8,
    b: u64,
    c: u8,
}

/// 🔤 String Interning สำหรับลดการใช้หน่วยความจำ - Workshop Technique
struct StringInterner {
    strings: HashMap<String, Rc<str>>,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }
    
    fn intern(&mut self, s: &str) -> Rc<str> {
        if let Some(interned) = self.strings.get(s) {
            interned.clone()
        } else {
            let interned: Rc<str> = s.into();
            self.strings.insert(s.to_string(), interned.clone());
            interned
        }
    }
}

/// 🏊 Object Pool Pattern - Workshop Performance Technique
struct ObjectPool<T> {
    objects: Vec<T>,
    create_fn: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    fn new<F>(create_fn: F) -> Self 
    where 
        F: Fn() -> T + 'static,
    {
        Self {
            objects: Vec::new(),
            create_fn: Box::new(create_fn),
        }
    }
    
    fn get(&mut self) -> T {
        self.objects.pop().unwrap_or_else(|| (self.create_fn)())
    }
    
    fn return_object(&mut self, obj: T) {
        self.objects.push(obj);
    }
}

/// 🐄 Copy-on-Write (`CoW`) Pattern - Workshop Memory Technique
#[derive(Clone)]
struct CowData {
    data: Arc<Vec<u8>>,
    is_owned: bool,
}

impl CowData {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(data),
            is_owned: false,
        }
    }
    
    fn make_mut(&mut self) -> &mut Vec<u8> {
        if !self.is_owned || Arc::strong_count(&self.data) > 1 {
            self.data = Arc::new((*self.data).clone());
            self.is_owned = true;
        }
        Arc::get_mut(&mut self.data).unwrap()
    }
    
    fn get(&self) -> &Vec<u8> {
        &self.data
    }
}

/// 🏊‍♂️ Memory Pool Allocator - Workshop Advanced Technique
struct MemoryPool {
    pool: Vec<Vec<u8>>,
    block_size: usize,
}

impl MemoryPool {
    fn new(block_size: usize, initial_blocks: usize) -> Self {
        let mut pool = Vec::with_capacity(initial_blocks);
        for _ in 0..initial_blocks {
            pool.push(vec![0; block_size]);
        }
        
        Self { pool, block_size }
    }
    
    fn allocate(&mut self) -> Option<Vec<u8>> {
        self.pool.pop().or_else(|| {
            Some(vec![0; self.block_size])
        })
    }
    
    fn deallocate(&mut self, mut block: Vec<u8>) {
        if block.len() == self.block_size {
            block.clear();
            block.resize(self.block_size, 0);
            self.pool.push(block);
        }
    }
}

/// 📦 การใช้ Box สำหรับ Large Objects - Workshop Heap Management
struct LargeObject {
    data: [u8; 1024 * 1024], // 1MB
}

fn create_large_object_on_heap() -> Box<LargeObject> {
    Box::new(LargeObject {
        data: [0; 1024 * 1024],
    })
}

/// 🔗 การใช้ Weak References เพื่อหลีกเลี่ยง Circular References - Workshop Pattern
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<std::rc::Weak<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }
    
    fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
}

/// 🏟️ การใช้ Arena Allocator - Workshop Advanced Memory Management
struct Arena {
    data: Vec<u8>,
    offset: usize,
}

impl Arena {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            offset: 0,
        }
    }
    
    fn allocate<T>(&mut self, value: T) -> Option<&mut T> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        
        // Align offset
        let aligned_offset = (self.offset + align - 1) & !(align - 1);
        
        if aligned_offset + size <= self.data.capacity() {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(aligned_offset).cast::<T>();
                ptr.write(value);
                self.offset = aligned_offset + size;
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
    
    const fn reset(&mut self) {
        self.offset = 0;
    }
}

/// 🎯 สาธิตการใช้งาน Memory Optimization - Web Development Workshop
/// 🌟 สำหรับผู้เข้าร่วมเวิร์กช็อป
pub fn demonstrate_memory_optimization() {
    println!("📊 Memory Layout Optimization Workshop:");
    
    // แสดงขนาดของ struct ที่ optimize แล้ว - Workshop Demo
    println!("📐 OptimizedStruct size: {} bytes", std::mem::size_of::<OptimizedStruct>());
    println!("📦 PackedStruct size: {} bytes", std::mem::size_of::<PackedStruct>());
    
    // String Interning
    println!("\n🔤 String Interning Workshop:");
    let mut interner = StringInterner::new();
    let str1 = interner.intern("สวัสดี Workshop");
    let str2 = interner.intern("สวัสดี Workshop");
    println!("🎯 Same string reference: {}", Rc::ptr_eq(&str1, &str2));
    
    // Object Pool
    println!("\n🏊 Object Pool Workshop:");
    let mut pool = ObjectPool::new(|| Vec::<i32>::with_capacity(100));
    let mut vec1 = pool.get();
    vec1.push(42);
    println!("🎯 Vector from pool: {vec1:?}");
    vec1.clear();
    pool.return_object(vec1);
    
    // Copy-on-Write
    println!("\n🐄 Copy-on-Write Workshop:");
    let mut cow1 = CowData::new(vec![1, 2, 3, 4, 5]);
    let cow2 = cow1.clone();
    println!("🎯 Shared data: {:?}", cow1.get());
    
    let mutable_data = cow1.make_mut();
    mutable_data.push(6);
    println!("🔄 Modified data: {:?}", cow1.get());
    println!("📋 Original data: {:?}", cow2.get());
    
    // Memory Pool
    println!("\n🏊‍♂️ Memory Pool Workshop:");
    let mut mem_pool = MemoryPool::new(1024, 5);
    if let Some(block) = mem_pool.allocate() {
        println!("🎯 Allocated block size: {}", block.len());
        mem_pool.deallocate(block);
    }
    
    // Large Object on Heap
    println!("\n📦 Large Object on Heap Workshop:");
    let large_obj = create_large_object_on_heap();
    println!("🎯 Large object created on heap (size: {} bytes)", 
             std::mem::size_of_val(&*large_obj));
    
    // Weak References
    println!("\n🔗 Weak References Workshop (avoiding cycles):");
    let parent = Node::new(1);
    let child = Node::new(2);
    Node::add_child(&parent, child.clone());
    println!("🎯 Parent value: {}", parent.value);
    println!("👶 Child value: {}", child.value);
    
    // Arena Allocator
    println!("\n🏟️ Arena Allocator Workshop:");
    let mut arena = Arena::new(1024);
    if let Some(value) = arena.allocate(42i32) {
        println!("🎯 Allocated value in arena: {value}");
    }
    
    println!("\n✅ Memory Optimization Workshop สำเร็จแล้ว! 🎉");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_interning() {
        let mut interner = StringInterner::new();
        let str1 = interner.intern("test");
        let str2 = interner.intern("test");
        assert!(Rc::ptr_eq(&str1, &str2));
    }
    
    #[test]
    fn test_object_pool() {
        let mut pool = ObjectPool::new(Vec::<i32>::new);
        let vec = pool.get();
        assert_eq!(vec.len(), 0);
    }
    
    #[test]
    fn test_cow_data() {
        let mut cow = CowData::new(vec![1, 2, 3]);
        let original_len = cow.get().len();
        
        let mutable_data = cow.make_mut();
        mutable_data.push(4);
        
        assert_eq!(cow.get().len(), original_len + 1);
    }
    
    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(100, 2);
        let block = pool.allocate().unwrap();
        assert_eq!(block.len(), 100);
        pool.deallocate(block);
    }
}