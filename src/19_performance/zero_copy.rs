//! 🚀 Zero-Copy Techniques Workshop - เทคนิคการจัดการข้อมูลแบบไม่ต้องคัดลอก! 🎯
//!
//! ยินดีต้อนรับสู่ Workshop การเรียนรู้เทคนิค Zero-Copy ใน Rust! 🎉
//! เหมือนการส่งต่อข้อมูลแบบไม่ต้องถ่ายเอกสาร - ประหยัดเวลาและหน่วยความจำ! 📋
//!
//! 🎯 สิ่งที่จะได้เรียนรู้:
//! - 🗺️ Memory Mapping และการแชร์ Buffer
//! - 📝 Zero-Copy String Operations
//! - ⚡ การถ่ายโอนข้อมูลอย่างมีประสิทธิภาพ
//! - 🌐 Network Buffer Management
//!
//! เทคนิคเหล่านี้สำคัญมากสำหรับแอปพลิเคชันที่ต้องการประสิทธิภาพสูง! 🚀

use std::io::{self, Read, Write};
use std::slice;
use std::ptr;
use std::mem;
// use std::marker::PhantomData;
// use std::ops::{Deref, DerefMut};

/// 📦 Zero-Copy Buffer - ตัวจัดการข้อมูลแบบไม่ต้องคัดลอก!
/// เหมือนการแชร์ของเล่นกับเพื่อนโดยไม่ต้องซื้อใหม่! 🎁
pub struct ZeroCopyBuffer<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
    owned: bool,
}

impl<T> ZeroCopyBuffer<T> {
    /// Create a new owned buffer
    #[must_use] pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let data = unsafe { std::alloc::alloc(layout).cast::<T>() };
        
        assert!(!data.is_null(), "Failed to allocate memory");
        
        Self {
            data,
            len: 0,
            capacity,
            owned: true,
        }
    }
    
    /// Create a buffer from existing data without copying
    pub const unsafe fn from_raw_parts(data: *mut T, len: usize, capacity: usize) -> Self {
        Self {
            data,
            len,
            capacity,
            owned: false,
        }
    }
    
    /// Create a view into existing slice without copying
    pub const fn from_slice(slice: &[T]) -> Self {
        Self {
            data: slice.as_ptr().cast_mut(),
            len: slice.len(),
            capacity: slice.len(),
            owned: false,
        }
    }
    
    /// Create a mutable view into existing slice without copying
    pub const fn from_mut_slice(slice: &mut [T]) -> Self {
        Self {
            data: slice.as_mut_ptr(),
            len: slice.len(),
            capacity: slice.len(),
            owned: false,
        }
    }
    
    /// Get the length of the buffer
    #[must_use] pub const fn len(&self) -> usize {
        self.len
    }
    
    /// Check if the buffer is empty
    #[must_use] pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Get the capacity of the buffer
    #[must_use] pub const fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get a raw pointer to the data
    #[must_use] pub const fn as_ptr(&self) -> *const T {
        self.data
    }
    
    /// Get a mutable raw pointer to the data
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }
    
    /// Get a slice view of the buffer
    #[must_use] pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }
    
    /// Get a mutable slice view of the buffer
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data, self.len) }
    }
    
    /// Push an element to the buffer (only if owned and has capacity)
    pub const fn push(&mut self, value: T) -> Result<(), T> {
        if !self.owned || self.len >= self.capacity {
            return Err(value);
        }
        
        unsafe {
            ptr::write(self.data.add(self.len), value);
        }
        self.len += 1;
        Ok(())
    }
    
    /// Pop an element from the buffer
    pub const fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        
        self.len -= 1;
        unsafe {
            Some(ptr::read(self.data.add(self.len)))
        }
    }
    
    /// Create a zero-copy slice of this buffer
    #[must_use] pub const fn slice(&self, start: usize, end: usize) -> Option<Self> {
        if start > end || end > self.len {
            return None;
        }
        
        Some(unsafe {
            Self::from_raw_parts(
                self.data.add(start),
                end - start,
                end - start,
            )
        })
    }
    
    /// Split the buffer at the given index
    #[must_use] pub const fn split_at(&self, mid: usize) -> Option<(Self, Self)> {
        if mid > self.len {
            return None;
        }
        
        let left = unsafe {
            Self::from_raw_parts(self.data, mid, mid)
        };
        
        let right = unsafe {
            Self::from_raw_parts(
                self.data.add(mid),
                self.len - mid,
                self.len - mid,
            )
        };
        
        Some((left, right))
    }
}

impl<T: Clone> ZeroCopyBuffer<T> {
    /// Clone the buffer (this will copy the data)
    #[must_use] pub fn clone_data(&self) -> Self {
        let mut new_buffer = Self::new(self.len);
        
        for i in 0..self.len {
            unsafe {
                let value = ptr::read(self.data.add(i));
                let _ = new_buffer.push(value.clone());
                mem::forget(value); // Prevent double drop
            }
        }
        
        new_buffer
    }
}

impl<T> Drop for ZeroCopyBuffer<T> {
    fn drop(&mut self) {
        if self.owned && !self.data.is_null() {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.data.add(i));
                }
            }
            
            // Deallocate memory
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.data.cast::<u8>(), layout);
            }
        }
    }
}

unsafe impl<T: Send> Send for ZeroCopyBuffer<T> {}
unsafe impl<T: Sync> Sync for ZeroCopyBuffer<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ZeroCopyBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZeroCopyBuffer")
            .field("len", &self.len)
            .field("capacity", &self.capacity)
            .field("owned", &self.owned)
            .field("data", &self.as_slice())
            .finish()
    }
}

/// 📝 Zero-Copy String - การจัดการข้อความแบบไม่ต้องคัดลอก!
/// เหมือนการอ่านหนังสือโดยไม่ต้องถ่ายเอกสาร! 📚
#[derive(Debug)]
pub struct ZeroCopyStr<'a> {
    data: &'a [u8],
}

impl<'a> ZeroCopyStr<'a> {
    /// Create from a string slice without copying
    #[must_use] pub const fn from_str(s: &'a str) -> Self {
        Self {
            data: s.as_bytes(),
        }
    }
    
    /// Create from byte slice without copying
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, std::str::Utf8Error> {
        std::str::from_utf8(bytes)?;
        Ok(Self { data: bytes })
    }
    
    /// Get the length in bytes
    #[must_use] pub const fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if empty
    #[must_use] pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get as string slice
    #[must_use] pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.data) }
    }
    
    /// Get as byte slice
    #[must_use] pub const fn as_bytes(&self) -> &[u8] {
        self.data
    }
    
    /// Create a substring without copying
    #[must_use] pub fn substring(&self, start: usize, end: usize) -> Option<Self> {
        if start > end || end > self.data.len() {
            return None;
        }
        
        // Ensure we don't split UTF-8 characters
        let substr_bytes = &self.data[start..end];
        if std::str::from_utf8(substr_bytes).is_ok() {
            Some(ZeroCopyStr { data: substr_bytes })
        } else {
            None
        }
    }
    
    /// Split at whitespace without copying
    #[must_use] pub const fn split_whitespace(&self) -> ZeroCopyStrSplitWhitespace<'a> {
        ZeroCopyStrSplitWhitespace {
            data: self.data,
            pos: 0,
        }
    }
    
    /// Find a pattern and return zero-copy slices
    #[must_use] pub fn find_pattern(&self, pattern: &str) -> Vec<Self> {
        let mut results = Vec::new();
        let pattern_bytes = pattern.as_bytes();
        let mut start = 0;
        
        while let Some(pos) = self.find_bytes_at(pattern_bytes, start) {
            if pos > start {
                if let Some(substr) = self.substring(start, pos) {
                    results.push(substr);
                }
            }
            start = pos + pattern_bytes.len();
        }
        
        // Add remaining part
        if start < self.data.len() {
            if let Some(substr) = self.substring(start, self.data.len()) {
                results.push(substr);
            }
        }
        
        results
    }
    
    fn find_bytes_at(&self, pattern: &[u8], start: usize) -> Option<usize> {
        if start >= self.data.len() || pattern.is_empty() {
            return None;
        }
        
        (start..=(self.data.len().saturating_sub(pattern.len()))).find(|&i| self.data[i..].starts_with(pattern))
    }
}

/// Iterator for zero-copy string splitting
pub struct ZeroCopyStrSplitWhitespace<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ZeroCopyStrSplitWhitespace<'a> {
    type Item = ZeroCopyStr<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.pos < self.data.len() && self.data[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        
        if self.pos >= self.data.len() {
            return None;
        }
        
        let start = self.pos;
        
        // Find end of word
        while self.pos < self.data.len() && !self.data[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        
        let word_bytes = &self.data[start..self.pos];
        if std::str::from_utf8(word_bytes).is_ok() {
            Some(ZeroCopyStr { data: word_bytes })
        } else {
            self.next() // Skip invalid UTF-8 and try next
        }
    }
}

/// 📖 Zero-Copy Reader - ตัวอ่านข้อมูลแบบไม่ต้องคัดลอก!
/// เหมือนการอ่านหนังสือทีละบรรทัดโดยไม่ต้องเขียนใหม่! 📄
pub struct ZeroCopyReader<R> {
    inner: R,
    buffer: ZeroCopyBuffer<u8>,
    pos: usize,
    end: usize,
}

impl<R: Read> ZeroCopyReader<R> {
    pub fn new(reader: R, buffer_size: usize) -> Self {
        Self {
            inner: reader,
            buffer: ZeroCopyBuffer::new(buffer_size),
            pos: 0,
            end: 0,
        }
    }
    
    pub fn read_line(&mut self) -> io::Result<Option<ZeroCopyStr>> {
        loop {
            // Look for newline in current buffer
            for i in self.pos..self.end {
                if unsafe { *self.buffer.as_ptr().add(i) } == b'\n' {
                    let line_bytes = unsafe {
                        slice::from_raw_parts(
                            self.buffer.as_ptr().add(self.pos),
                            i - self.pos,
                        )
                    };
                    
                    self.pos = i + 1;
                    
                    return match std::str::from_utf8(line_bytes) {
                        Ok(_) => Ok(Some(ZeroCopyStr { data: line_bytes })),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid UTF-8 in line",
                        )),
                    };
                }
            }
            
            // Need to read more data
            if self.pos > 0 {
                // Move remaining data to beginning of buffer
                let remaining = self.end - self.pos;
                unsafe {
                    ptr::copy(
                        self.buffer.as_ptr().add(self.pos),
                        self.buffer.as_mut_ptr(),
                        remaining,
                    );
                }
                self.end = remaining;
                self.pos = 0;
            }
            
            if self.end >= self.buffer.capacity() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Line too long for buffer",
                ));
            }
            
            // Read more data
            let bytes_read = {
                let buf_slice = unsafe {
                    slice::from_raw_parts_mut(
                        self.buffer.as_mut_ptr().add(self.end),
                        self.buffer.capacity() - self.end,
                    )
                };
                self.inner.read(buf_slice)?
            };
            
            if bytes_read == 0 {
                // EOF
                if self.pos < self.end {
                    let line_bytes = unsafe {
                        slice::from_raw_parts(
                            self.buffer.as_ptr().add(self.pos),
                            self.end - self.pos,
                        )
                    };
                    
                    self.pos = self.end;
                    
                    return match std::str::from_utf8(line_bytes) {
                        Ok(_) => Ok(Some(ZeroCopyStr { data: line_bytes })),
                        Err(_) => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid UTF-8 in line",
                        )),
                    };
                }
                return Ok(None);
            }
            
            self.end += bytes_read;
        }
    }
    
    pub fn read_chunk(&mut self, size: usize) -> io::Result<Option<ZeroCopyStr>> {
        if self.end - self.pos >= size {
            let chunk_bytes = unsafe {
                slice::from_raw_parts(
                    self.buffer.as_ptr().add(self.pos),
                    size,
                )
            };
            
            self.pos += size;
            
            return match std::str::from_utf8(chunk_bytes) {
                Ok(_) => Ok(Some(ZeroCopyStr { data: chunk_bytes })),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid UTF-8 in chunk",
                )),
            };
        }
        
        Ok(None)
    }
}

/// 🗺️ Zero-Copy Memory Map - การจำลอง Memory Mapping!
/// เหมือนการทำแผนที่หน่วยความจำแบบไม่ต้องวาดใหม่! 🗺️
#[derive(Debug)]
pub struct ZeroCopyMemoryMap {
    data: *mut u8,
    size: usize,
    owned: bool,
}

impl ZeroCopyMemoryMap {
    /// Create a new memory map
    #[must_use] pub fn new(size: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
        let data = unsafe { std::alloc::alloc(layout) };
        
        assert!(!data.is_null(), "Failed to allocate memory");
        
        Self {
            data,
            size,
            owned: true,
        }
    }
    
    /// Create from existing memory without copying
    pub const unsafe fn from_raw_parts(data: *mut u8, size: usize) -> Self {
        Self {
            data,
            size,
            owned: false,
        }
    }
    
    /// Get the size of the mapping
    #[must_use] pub const fn size(&self) -> usize {
        self.size
    }
    
    /// Get a slice view of the mapping
    #[must_use] pub const fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data, self.size) }
    }
    
    /// Get a mutable slice view of the mapping
    pub const fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.data, self.size) }
    }
    
    /// Create a zero-copy view into a region
    #[must_use] pub const fn view(&self, offset: usize, len: usize) -> Option<Self> {
        if offset + len > self.size {
            return None;
        }
        
        Some(unsafe {
            Self::from_raw_parts(
                self.data.add(offset),
                len,
            )
        })
    }
    
    /// Copy data from another mapping without intermediate copying
    pub const fn copy_from(&mut self, other: &Self, src_offset: usize, dst_offset: usize, len: usize) -> Result<(), &'static str> {
        if src_offset + len > other.size || dst_offset + len > self.size {
            return Err("Copy would exceed bounds");
        }
        
        unsafe {
            ptr::copy_nonoverlapping(
                other.data.add(src_offset),
                self.data.add(dst_offset),
                len,
            );
        }
        
        Ok(())
    }
}

impl Drop for ZeroCopyMemoryMap {
    fn drop(&mut self) {
        if self.owned && !self.data.is_null() {
            let layout = std::alloc::Layout::from_size_align(self.size, 1).unwrap();
            unsafe {
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
}

unsafe impl Send for ZeroCopyMemoryMap {}
unsafe impl Sync for ZeroCopyMemoryMap {}

/// 🌐 Zero-Copy Network Buffer - บัฟเฟอร์เครือข่ายแบบไม่ต้องคัดลอก!
/// เหมือนการส่งข้อความหลายๆ ข้อความในซองเดียว! 📮
pub struct ZeroCopyNetworkBuffer {
    buffers: Vec<ZeroCopyBuffer<u8>>,
    total_len: usize,
}

impl Default for ZeroCopyNetworkBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCopyNetworkBuffer {
    #[must_use] pub const fn new() -> Self {
        Self {
            buffers: Vec::new(),
            total_len: 0,
        }
    }
    
    /// Add a buffer without copying
    pub fn add_buffer(&mut self, buffer: ZeroCopyBuffer<u8>) {
        self.total_len += buffer.len();
        self.buffers.push(buffer);
    }
    
    /// Get total length across all buffers
    #[must_use] pub const fn total_len(&self) -> usize {
        self.total_len
    }
    
    /// Read data from the network buffer without copying
    #[must_use] pub fn read_at(&self, mut offset: usize, len: usize) -> Option<Vec<&[u8]>> {
        if offset + len > self.total_len {
            return None;
        }
        
        let mut result = Vec::new();
        let mut remaining = len;
        
        for buffer in &self.buffers {
            if offset >= buffer.len() {
                offset -= buffer.len();
                continue;
            }
            
            let start = offset;
            let end = std::cmp::min(start + remaining, buffer.len());
            let slice = &buffer.as_slice()[start..end];
            
            result.push(slice);
            remaining -= slice.len();
            offset = 0;
            
            if remaining == 0 {
                break;
            }
        }
        
        Some(result)
    }
    
    /// Write data to a writer using vectored I/O
    pub fn write_vectored<W: Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut total_written = 0;
        
        for buffer in &self.buffers {
            let written = writer.write(buffer.as_slice())?;
            total_written += written;
            
            if written < buffer.len() {
                break; // Partial write
            }
        }
        
        Ok(total_written)
    }
}

/// 🎯 สาธิตการใช้งาน Zero-Copy Techniques - Workshop Demo!
pub fn demonstrate_zero_copy() {
    println!("🚀 ยินดีต้อนรับสู่ Zero-Copy Techniques Workshop! 🎉");
    println!("เหมือนการเรียนรู้เทคนิคการแชร์ข้อมูลแบบไม่ต้องคัดลอก! 📋\n");
    
    // Zero-copy buffer operations
    println!("📦 Zero-Copy Buffer Operations - การจัดการบัฟเฟอร์แบบไม่ต้องคัดลอก:");
    println!("{:-<60}", "");
    
    let mut buffer = ZeroCopyBuffer::<i32>::new(10);
    
    // Add some data
    for i in 0..5 {
        buffer.push(i).unwrap();
    }
    
    println!("🎯 บัฟเฟอร์ต้นฉบับ: {:?}", buffer.as_slice());
    
    // Create zero-copy slices
    if let Some(slice1) = buffer.slice(0, 3) {
        println!("✂️ ชิ้นส่วน [0..3]: {:?} (ไม่มีการคัดลอก!)", slice1.as_slice());
    }
    
    if let Some(slice2) = buffer.slice(2, 5) {
        println!("✂️ ชิ้นส่วน [2..5]: {:?} (แชร์หน่วยความจำเดียวกัน!)", slice2.as_slice());
    }
    
    // Split buffer
    if let Some((left, right)) = buffer.split_at(3) {
        println!("🔄 แบ่งซ้าย: {:?}", left.as_slice());
        println!("🔄 แบ่งขวา: {:?}", right.as_slice());
    }
    
    // Zero-copy string operations
    println!("\n📝 Zero-Copy String Operations - การจัดการข้อความแบบไม่ต้องคัดลอก:");
    println!("{:-<60}", "");
    
    let text = "สวัสดี Workshop Zero-Copy ใน Rust เทคนิคการแชร์ข้อมูล";
    let zero_copy_str = ZeroCopyStr::from_str(text);
    
    println!("🎯 ข้อความต้นฉบับ: \"{}\"", zero_copy_str.as_str());
    println!("📏 ความยาว: {} ไบต์", zero_copy_str.len());
    
    // Create substring without copying
    if let Some(substring) = zero_copy_str.substring(0, 15) {
        println!("✂️ ข้อความย่อย [0..15]: \"{}\" (ไม่มีการคัดลอก!)", substring.as_str());
    }
    
    // Split by whitespace without copying
    println!("📝 แยกคำ (zero-copy split):");
    for (i, word) in zero_copy_str.split_whitespace().enumerate() {
        println!("  {}: \"{}\" 🔗", i, word.as_str());
    }
    
    // Find pattern and split
    let pattern_text = "มะม่วง,กล้วย,ส้ม,แอปเปิ้ล";
    let pattern_str = ZeroCopyStr::from_str(pattern_text);
    let parts = pattern_str.find_pattern(",");
    
    println!("\n🔍 แยกรูปแบบ \"{pattern_text}\" ด้วย \",\":");
    for (i, part) in parts.iter().enumerate() {
        println!("  {}: \"{}\" 🍎", i, part.as_str());
    }
    
    // Zero-copy memory mapping
    println!("\n🗺️ Zero-Copy Memory Mapping - การจำลอง Memory Mapping:");
    println!("{:-<60}", "");
    
    let mut memory_map = ZeroCopyMemoryMap::new(1024);
    
    // Write some data
    let data = "สวัสดี Zero-Copy Memory Mapping Workshop!".as_bytes();
    memory_map.as_mut_slice()[..data.len()].copy_from_slice(data);
    
    println!("💾 เขียนลง Memory Map: \"{}\"", 
             std::str::from_utf8(&memory_map.as_slice()[..data.len()]).unwrap());
    
    // Create a view without copying
    if let Some(view) = memory_map.view(0, data.len()) {
        println!("👀 มุมมอง Memory Map: \"{}\" (ไม่มีการคัดลอก!)", 
                 std::str::from_utf8(view.as_slice()).unwrap());
    }
    
    // Copy between memory maps
    let mut another_map = ZeroCopyMemoryMap::new(1024);
    another_map.copy_from(&memory_map, 0, 0, data.len()).unwrap();
    
    println!("🔄 คัดลอกไปยัง Map อื่น: \"{}\"", 
             std::str::from_utf8(&another_map.as_slice()[..data.len()]).unwrap());
    
    // Zero-copy network buffer
    println!("\n🌐 Zero-Copy Network Buffer - บัฟเฟอร์เครือข่ายแบบไม่ต้องคัดลอก:");
    println!("{:-<60}", "");
    
    let mut network_buffer = ZeroCopyNetworkBuffer::new();
    
    // Add multiple buffers without copying
    let mut buf1 = ZeroCopyBuffer::<u8>::new(100); // เพิ่มขนาด buffer สำหรับข้อความไทย
    let data1 = "แพ็กเก็ตแรก".as_bytes();
    for &byte in data1 {
        if let Err(e) = buf1.push(byte) {
            println!("❌ ไม่สามารถเพิ่มข้อมูลใน buf1: {}", e);
            break;
        }
    }
    
    let mut buf2 = ZeroCopyBuffer::<u8>::new(100); // เพิ่มขนาด buffer สำหรับข้อความไทย
    let data2 = " แพ็กเก็ตสอง".as_bytes();
    for &byte in data2 {
        if let Err(e) = buf2.push(byte) {
            println!("❌ ไม่สามารถเพิ่มข้อมูลใน buf2: {}", e);
            break;
        }
    }
    
    let mut buf3 = ZeroCopyBuffer::<u8>::new(100); // เพิ่มขนาด buffer สำหรับข้อความไทย
    let data3 = " แพ็กเก็ตสาม".as_bytes();
    for &byte in data3 {
        if let Err(e) = buf3.push(byte) {
            println!("❌ ไม่สามารถเพิ่มข้อมูลใน buf3: {}", e);
            break;
        }
    }
    
    network_buffer.add_buffer(buf1);
    network_buffer.add_buffer(buf2);
    network_buffer.add_buffer(buf3);
    
    println!("📊 ความยาวรวมของ Network Buffer: {} ไบต์", network_buffer.total_len());
    
    // Read from network buffer without copying
    if let Some(slices) = network_buffer.read_at(5, 15) {
        print!("📖 อ่านจาก offset 5, ความยาว 15: \"");
        for slice in slices {
            print!("{}", std::str::from_utf8(slice).unwrap_or("<invalid>"));
        }
        println!("\" (ไม่มีการคัดลอก!)");
    }
    
    // Demonstrate zero-copy with existing data
    println!("\n🔗 Zero-Copy กับข้อมูลที่มีอยู่แล้ว:");
    println!("{:-<60}", "");
    
    let existing_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Create zero-copy buffer from existing slice
    let zero_copy_view = ZeroCopyBuffer::from_slice(&existing_data);
    println!("🎯 มุมมอง Zero-Copy ของข้อมูลที่มีอยู่: {:?}", zero_copy_view.as_slice());
    
    // Create multiple views of the same data
    if let Some(view1) = zero_copy_view.slice(0, 5) {
        println!("👀 มุมมอง 1 [0..5]: {:?}", view1.as_slice());
    }
    
    if let Some(view2) = zero_copy_view.slice(5, 10) {
        println!("👀 มุมมอง 2 [5..10]: {:?}", view2.as_slice());
    }
    
    // Performance comparison
    println!("\n⚡ การเปรียบเทียบประสิทธิภาพ:");
    println!("{:-<60}", "");
    
    let large_data: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();
    
    // Measure zero-copy operations
    let start = std::time::Instant::now();
    let zero_copy_buffer = ZeroCopyBuffer::from_slice(&large_data);
    let _view1 = zero_copy_buffer.slice(0, 500_000).unwrap();
    let _view2 = zero_copy_buffer.slice(500_000, 1_000_000).unwrap();
    let zero_copy_time = start.elapsed();
    
    // Measure copying operations
    let start = std::time::Instant::now();
    let _copy1 = large_data[0..500_000].to_vec();
    let _copy2 = large_data[500_000..1_000_000].to_vec();
    let copy_time = start.elapsed();
    
    println!("⚡ การดำเนินการ Zero-Copy: {zero_copy_time:?}");
    println!("📋 การดำเนินการแบบคัดลอก: {copy_time:?}");
    
    if copy_time > zero_copy_time {
        let speedup = copy_time.as_nanos() as f64 / zero_copy_time.as_nanos() as f64;
        println!("🚀 Zero-Copy เร็วกว่า {speedup:.2} เท่า!");
    }
    
    println!("\n✅ สาธิต Zero-Copy Techniques เสร็จสิ้น! 🎉");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_buffer() {
        let mut buffer = ZeroCopyBuffer::<i32>::new(5);
        
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 5);
        assert!(buffer.is_empty());
        
        // Test push
        for i in 0..5 {
            assert!(buffer.push(i).is_ok());
        }
        
        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());
        assert!(buffer.push(5).is_err()); // Should fail, buffer full
        
        // Test slice
        let slice = buffer.slice(1, 4).unwrap();
        assert_eq!(slice.as_slice(), &[1, 2, 3]);
        
        // Test split
        let (left, right) = buffer.split_at(3).unwrap();
        assert_eq!(left.as_slice(), &[0, 1, 2]);
        assert_eq!(right.as_slice(), &[3, 4]);
        
        // Test pop
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.len(), 4);
    }

    #[test]
    fn test_zero_copy_str() {
        let text = "Hello, world!";
        let zero_str = ZeroCopyStr::from_str(text);
        
        assert_eq!(zero_str.len(), text.len());
        assert_eq!(zero_str.as_str(), text);
        assert!(!zero_str.is_empty());
        
        // Test substring
        let substr = zero_str.substring(0, 5).unwrap();
        assert_eq!(substr.as_str(), "Hello");
        
        // Test split whitespace
        let words: Vec<String> = zero_str.split_whitespace()
            .map(|w| w.as_str().to_string())
            .collect();
        assert_eq!(words, vec!["Hello,", "world!"]);
        
        // Test pattern finding
        let csv = ZeroCopyStr::from_str("a,b,c,d");
        let parts = csv.find_pattern(",");
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].as_str(), "a");
        assert_eq!(parts[1].as_str(), "b");
        assert_eq!(parts[2].as_str(), "c");
        assert_eq!(parts[3].as_str(), "d");
    }

    #[test]
    fn test_zero_copy_memory_map() {
        let mut map = ZeroCopyMemoryMap::new(100);
        
        assert_eq!(map.size(), 100);
        
        // Write some data
        let data = b"Hello, memory map!";
        map.as_mut_slice()[..data.len()].copy_from_slice(data);
        
        // Verify data
        assert_eq!(&map.as_slice()[..data.len()], data);
        
        // Test view
        let view = map.view(0, data.len()).unwrap();
        assert_eq!(view.as_slice(), data);
        
        // Test copy between maps
        let mut other_map = ZeroCopyMemoryMap::new(100);
        other_map.copy_from(&map, 0, 0, data.len()).unwrap();
        assert_eq!(&other_map.as_slice()[..data.len()], data);
    }

    #[test]
    fn test_zero_copy_network_buffer() {
        let mut network_buf = ZeroCopyNetworkBuffer::new();
        
        // Create and add buffers
        let mut buf1 = ZeroCopyBuffer::<u8>::new(10);
        for &byte in b"Hello" {
            buf1.push(byte).unwrap();
        }
        
        let mut buf2 = ZeroCopyBuffer::<u8>::new(10);
        for &byte in b" World" {
            buf2.push(byte).unwrap();
        }
        
        network_buf.add_buffer(buf1);
        network_buf.add_buffer(buf2);
        
        assert_eq!(network_buf.total_len(), 11);
        
        // Test reading
        let slices = network_buf.read_at(0, 5).unwrap();
        assert_eq!(slices.len(), 1);
        assert_eq!(slices[0], b"Hello");
        
        let slices = network_buf.read_at(3, 5).unwrap();
        let combined: Vec<u8> = slices.iter().flat_map(|&s| s.iter()).copied().collect();
        assert_eq!(&combined, b"lo Wo");
    }

    #[test]
    fn test_buffer_from_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = ZeroCopyBuffer::from_slice(&data);
        
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
        
        // Test that it's truly zero-copy (same memory)
        assert_eq!(buffer.as_ptr(), data.as_ptr());
    }

    #[test]
    fn test_str_from_bytes() {
        let bytes = b"Hello, bytes!";
        let zero_str = ZeroCopyStr::from_bytes(bytes).unwrap();
        
        assert_eq!(zero_str.as_str(), "Hello, bytes!");
        assert_eq!(zero_str.as_bytes(), bytes);
        
        // Test invalid UTF-8
        let invalid_bytes = &[0xFF, 0xFE, 0xFD];
        assert!(ZeroCopyStr::from_bytes(invalid_bytes).is_err());
    }

    #[test]
    fn test_buffer_clone_data() {
        let mut original = ZeroCopyBuffer::<i32>::new(5);
        for i in 0..3 {
            original.push(i).unwrap();
        }
        
        let cloned = original.clone_data();
        
        assert_eq!(original.as_slice(), cloned.as_slice());
        assert_ne!(original.as_ptr(), cloned.as_ptr()); // Different memory
    }
}