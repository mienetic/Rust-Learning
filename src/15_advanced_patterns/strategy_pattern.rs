//! Strategy Pattern Implementation
//!
//! การใช้งาน Strategy Pattern ใน Rust
//! ครอบคลุม Algorithm Selection, Payment Processing, และ Sorting Strategies

// use std::collections::HashMap;
// use std::fmt;

/// Strategy trait สำหรับ sorting algorithms
pub trait SortStrategy<T> {
    fn sort(&self, data: &mut [T]);
    fn name(&self) -> &'static str;
}

/// Bubble Sort Strategy
#[derive(Debug, Clone)]
pub struct BubbleSort;

impl<T: PartialOrd + Clone> SortStrategy<T> for BubbleSort {
    fn sort(&self, data: &mut [T]) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Bubble Sort"
    }
}

/// Quick Sort Strategy
#[derive(Debug, Clone)]
pub struct QuickSort;

impl<T: PartialOrd + Clone> SortStrategy<T> for QuickSort {
    fn sort(&self, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        self.quick_sort_recursive(data, 0, data.len() - 1);
    }

    fn name(&self) -> &'static str {
        "Quick Sort"
    }
}

impl QuickSort {
    fn quick_sort_recursive<T: PartialOrd>(&self, data: &mut [T], low: usize, high: usize) {
        if low < high {
            let pivot = self.partition(data, low, high);
            if pivot > 0 {
                self.quick_sort_recursive(data, low, pivot - 1);
            }
            self.quick_sort_recursive(data, pivot + 1, high);
        }
    }

    fn partition<T: PartialOrd>(&self, data: &mut [T], low: usize, high: usize) -> usize {
        let mut i = low;
        for j in low..high {
            if data[j] <= data[high] {
                data.swap(i, j);
                i += 1;
            }
        }
        data.swap(i, high);
        i
    }
}

/// Merge Sort Strategy
#[derive(Debug, Clone)]
pub struct MergeSort;

impl<T: PartialOrd + Clone> SortStrategy<T> for MergeSort {
    fn sort(&self, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        let mut temp = data.to_vec();
        self.merge_sort_recursive(data, &mut temp, 0, data.len() - 1);
    }

    fn name(&self) -> &'static str {
        "Merge Sort"
    }
}

impl MergeSort {
    fn merge_sort_recursive<T: PartialOrd + Clone>(
        &self,
        data: &mut [T],
        temp: &mut [T],
        left: usize,
        right: usize,
    ) {
        if left < right {
            let mid = left + (right - left) / 2;
            self.merge_sort_recursive(data, temp, left, mid);
            self.merge_sort_recursive(data, temp, mid + 1, right);
            self.merge(data, temp, left, mid, right);
        }
    }

    fn merge<T: PartialOrd + Clone>(
        &self,
        data: &mut [T],
        temp: &mut [T],
        left: usize,
        mid: usize,
        right: usize,
    ) {
        for i in left..=right {
            temp[i] = data[i].clone();
        }

        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        while i <= mid && j <= right {
            if temp[i] <= temp[j] {
                data[k] = temp[i].clone();
                i += 1;
            } else {
                data[k] = temp[j].clone();
                j += 1;
            }
            k += 1;
        }

        while i <= mid {
            data[k] = temp[i].clone();
            i += 1;
            k += 1;
        }

        while j <= right {
            data[k] = temp[j].clone();
            j += 1;
            k += 1;
        }
    }
}

/// Sorter context ที่ใช้ strategy
pub struct Sorter<T> {
    strategy: Box<dyn SortStrategy<T>>,
}

impl<T> Sorter<T> {
    #[must_use] pub fn new(strategy: Box<dyn SortStrategy<T>>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn SortStrategy<T>>) {
        self.strategy = strategy;
    }

    pub fn sort(&self, data: &mut [T]) {
        println!("🔄 Using {} algorithm", self.strategy.name());
        self.strategy.sort(data);
    }

    #[must_use] pub fn get_strategy_name(&self) -> &'static str {
        self.strategy.name()
    }
}

/// Payment processing strategies
#[derive(Debug, Clone)]
pub struct PaymentAmount {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct PaymentResult {
    pub success: bool,
    pub transaction_id: String,
    pub message: String,
    pub fee: f64,
}

/// Payment strategy trait
pub trait PaymentStrategy {
    fn process_payment(&self, amount: &PaymentAmount) -> PaymentResult;
    fn calculate_fee(&self, amount: f64) -> f64;
    fn name(&self) -> &'static str;
}

/// Credit Card Payment Strategy
#[derive(Debug, Clone)]
pub struct CreditCardPayment {
    pub card_number: String,
    pub cardholder_name: String,
    pub expiry_date: String,
    pub cvv: String,
}

impl CreditCardPayment {
    #[must_use] pub const fn new(card_number: String, cardholder_name: String, expiry_date: String, cvv: String) -> Self {
        Self {
            card_number,
            cardholder_name,
            expiry_date,
            cvv,
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn process_payment(&self, amount: &PaymentAmount) -> PaymentResult {
        let fee = self.calculate_fee(amount.amount);
        let total = amount.amount + fee;
        
        // Simulate payment processing
        let success = amount.amount > 0.0 && amount.amount <= 10000.0;
        
        PaymentResult {
            success,
            transaction_id: format!("CC_{}", rand::random::<u32>()),
            message: if success {
                format!("Credit card payment of {:.2} {} processed successfully", total, amount.currency)
            } else {
                "Credit card payment failed".to_string()
            },
            fee,
        }
    }

    fn calculate_fee(&self, amount: f64) -> f64 {
        amount.mul_add(0.029, 0.30) // 2.9% + $0.30
    }

    fn name(&self) -> &'static str {
        "Credit Card"
    }
}

/// `PayPal` Payment Strategy
#[derive(Debug, Clone)]
pub struct PayPalPayment {
    pub email: String,
    pub password: String,
}

impl PayPalPayment {
    #[must_use] pub const fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn process_payment(&self, amount: &PaymentAmount) -> PaymentResult {
        let fee = self.calculate_fee(amount.amount);
        let total = amount.amount + fee;
        
        // Simulate payment processing
        let success = amount.amount > 0.0 && self.email.contains('@');
        
        PaymentResult {
            success,
            transaction_id: format!("PP_{}", rand::random::<u32>()),
            message: if success {
                format!("PayPal payment of {:.2} {} processed successfully", total, amount.currency)
            } else {
                "PayPal payment failed".to_string()
            },
            fee,
        }
    }

    fn calculate_fee(&self, amount: f64) -> f64 {
        amount.mul_add(0.034, 0.49) // 3.4% + $0.49
    }

    fn name(&self) -> &'static str {
        "PayPal"
    }
}

/// Bank Transfer Payment Strategy
#[derive(Debug, Clone)]
pub struct BankTransferPayment {
    pub account_number: String,
    pub routing_number: String,
    pub bank_name: String,
}

impl BankTransferPayment {
    #[must_use] pub const fn new(account_number: String, routing_number: String, bank_name: String) -> Self {
        Self {
            account_number,
            routing_number,
            bank_name,
        }
    }
}

impl PaymentStrategy for BankTransferPayment {
    fn process_payment(&self, amount: &PaymentAmount) -> PaymentResult {
        let fee = self.calculate_fee(amount.amount);
        let total = amount.amount + fee;
        
        // Simulate payment processing
        let success = amount.amount > 0.0 && amount.amount >= 10.0; // Minimum $10
        
        PaymentResult {
            success,
            transaction_id: format!("BT_{}", rand::random::<u32>()),
            message: if success {
                format!("Bank transfer of {:.2} {} processed successfully", total, amount.currency)
            } else {
                "Bank transfer failed (minimum $10 required)".to_string()
            },
            fee,
        }
    }

    fn calculate_fee(&self, amount: f64) -> f64 {
        if amount >= 1000.0 {
            0.0 // Free for large transfers
        } else {
            5.0 // Flat $5 fee
        }
    }

    fn name(&self) -> &'static str {
        "Bank Transfer"
    }
}

/// Payment processor context
pub struct PaymentProcessor {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentProcessor {
    #[must_use] pub fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy;
    }

    #[must_use] pub fn process_payment(&self, amount: &PaymentAmount) -> PaymentResult {
        println!("💳 Processing payment using {}", self.strategy.name());
        let result = self.strategy.process_payment(amount);
        
        if result.success {
            println!("✅ {}", result.message);
            println!("📄 Transaction ID: {}", result.transaction_id);
            println!("💰 Fee: ${:.2}", result.fee);
        } else {
            println!("❌ {}", result.message);
        }
        
        result
    }

    #[must_use] pub fn calculate_fee(&self, amount: f64) -> f64 {
        self.strategy.calculate_fee(amount)
    }

    #[must_use] pub fn get_strategy_name(&self) -> &'static str {
        self.strategy.name()
    }
}

/// Compression strategies
pub trait CompressionStrategy {
    fn compress(&self, data: &str) -> String;
    fn decompress(&self, data: &str) -> String;
    fn name(&self) -> &'static str;
}

/// ZIP Compression Strategy
#[derive(Debug, Clone)]
pub struct ZipCompression;

impl CompressionStrategy for ZipCompression {
    fn compress(&self, data: &str) -> String {
        // Simulate ZIP compression
        format!("ZIP[{}]", data.len())
    }

    fn decompress(&self, data: &str) -> String {
        // Simulate ZIP decompression
        if data.starts_with("ZIP[") && data.ends_with(']') {
            let size_str = &data[4..data.len()-1];
            if let Ok(size) = size_str.parse::<usize>() {
                return "x".repeat(size);
            }
        }
        data.to_string()
    }

    fn name(&self) -> &'static str {
        "ZIP"
    }
}

/// GZIP Compression Strategy
#[derive(Debug, Clone)]
pub struct GzipCompression;

impl CompressionStrategy for GzipCompression {
    fn compress(&self, data: &str) -> String {
        // Simulate GZIP compression
        format!("GZIP[{}]", data.len())
    }

    fn decompress(&self, data: &str) -> String {
        // Simulate GZIP decompression
        if data.starts_with("GZIP[") && data.ends_with(']') {
            let size_str = &data[5..data.len()-1];
            if let Ok(size) = size_str.parse::<usize>() {
                return "x".repeat(size);
            }
        }
        data.to_string()
    }

    fn name(&self) -> &'static str {
        "GZIP"
    }
}

/// LZ4 Compression Strategy
#[derive(Debug, Clone)]
pub struct Lz4Compression;

impl CompressionStrategy for Lz4Compression {
    fn compress(&self, data: &str) -> String {
        // Simulate LZ4 compression
        format!("LZ4[{}]", data.len())
    }

    fn decompress(&self, data: &str) -> String {
        // Simulate LZ4 decompression
        if data.starts_with("LZ4[") && data.ends_with(']') {
            let size_str = &data[4..data.len()-1];
            if let Ok(size) = size_str.parse::<usize>() {
                return "x".repeat(size);
            }
        }
        data.to_string()
    }

    fn name(&self) -> &'static str {
        "LZ4"
    }
}

/// File compressor context
pub struct FileCompressor {
    strategy: Box<dyn CompressionStrategy>,
}

impl FileCompressor {
    #[must_use] pub fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn CompressionStrategy>) {
        self.strategy = strategy;
    }

    #[must_use] pub fn compress_file(&self, filename: &str, data: &str) -> String {
        println!("🗜️  Compressing {} using {}", filename, self.strategy.name());
        let compressed = self.strategy.compress(data);
        println!("📦 Original size: {} bytes", data.len());
        println!("📦 Compressed size: {} bytes", compressed.len());
        compressed
    }

    #[must_use] pub fn decompress_file(&self, filename: &str, compressed_data: &str) -> String {
        println!("📂 Decompressing {} using {}", filename, self.strategy.name());
        let decompressed = self.strategy.decompress(compressed_data);
        println!("📦 Compressed size: {} bytes", compressed_data.len());
        println!("📦 Decompressed size: {} bytes", decompressed.len());
        decompressed
    }

    #[must_use] pub fn get_strategy_name(&self) -> &'static str {
        self.strategy.name()
    }
}

// Simple random number generator for demo
mod rand {
    use std::cell::Cell;
    
    thread_local! {
        static SEED: Cell<u32> = const { Cell::new(1) };
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<u32>
    {
        SEED.with(|seed| {
            let current = seed.get();
            let next = current.wrapping_mul(1103515245).wrapping_add(12345);
            seed.set(next);
            T::from(next)
        })
    }
}

/// สาธิตการใช้งาน Strategy Pattern
pub fn demonstrate_strategy() {
    println!("🎯 Strategy Pattern Examples:");
    
    // Sorting Strategies Example
    println!("\n🔢 Sorting Strategies:");
    let mut data1 = vec![64, 34, 25, 12, 22, 11, 90];
    let mut data2 = data1.clone();
    let mut data3 = data1.clone();
    
    println!("Original data: {data1:?}");
    
    // Bubble Sort
    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data1);
    println!("After {}: {:?}", sorter.get_strategy_name(), data1);
    
    // Quick Sort
    sorter.set_strategy(Box::new(QuickSort));
    sorter.sort(&mut data2);
    println!("After {}: {:?}", sorter.get_strategy_name(), data2);
    
    // Merge Sort
    sorter.set_strategy(Box::new(MergeSort));
    sorter.sort(&mut data3);
    println!("After {}: {:?}", sorter.get_strategy_name(), data3);
    
    // Payment Processing Example
    println!("\n💳 Payment Processing Strategies:");
    
    let amount = PaymentAmount {
        amount: 100.0,
        currency: "USD".to_string(),
    };
    
    // Credit Card Payment
    let credit_card = CreditCardPayment::new(
        "1234-5678-9012-3456".to_string(),
        "John Doe".to_string(),
        "12/25".to_string(),
        "123".to_string(),
    );
    
    let mut processor = PaymentProcessor::new(Box::new(credit_card));
    let _ = processor.process_payment(&amount);
    
    // PayPal Payment
    let paypal = PayPalPayment::new(
        "john.doe@email.com".to_string(),
        "password123".to_string(),
    );
    
    processor.set_strategy(Box::new(paypal));
    let _ = processor.process_payment(&amount);
    
    // Bank Transfer Payment
    let bank_transfer = BankTransferPayment::new(
        "123456789".to_string(),
        "987654321".to_string(),
        "First National Bank".to_string(),
    );
    
    processor.set_strategy(Box::new(bank_transfer));
    let _ = processor.process_payment(&amount);
    
    // Test with small amount (should fail for bank transfer)
    let small_amount = PaymentAmount {
        amount: 5.0,
        currency: "USD".to_string(),
    };
    let _ = processor.process_payment(&small_amount);
    
    // Compression Strategies Example
    println!("\n🗜️  Compression Strategies:");
    
    let file_data = "This is a sample file content that we want to compress using different algorithms. It contains some repetitive text to demonstrate compression efficiency.";
    
    // ZIP Compression
    let mut compressor = FileCompressor::new(Box::new(ZipCompression));
    let compressed_zip = compressor.compress_file("document.txt", file_data);
    let _decompressed_zip = compressor.decompress_file("document.txt", &compressed_zip);
    
    // GZIP Compression
    compressor.set_strategy(Box::new(GzipCompression));
    let compressed_gzip = compressor.compress_file("document.txt", file_data);
    let _decompressed_gzip = compressor.decompress_file("document.txt", &compressed_gzip);
    
    // LZ4 Compression
    compressor.set_strategy(Box::new(Lz4Compression));
    let compressed_lz4 = compressor.compress_file("document.txt", file_data);
    let _decompressed_lz4 = compressor.decompress_file("document.txt", &compressed_lz4);
    
    // Fee Comparison
    println!("\n💰 Payment Fee Comparison for $1000:");
    let large_amount = 1000.0;
    
    let cc_fee = CreditCardPayment::new(
        "1234-5678-9012-3456".to_string(),
        "John Doe".to_string(),
        "12/25".to_string(),
        "123".to_string(),
    ).calculate_fee(large_amount);
    
    let pp_fee = PayPalPayment::new(
        "john.doe@email.com".to_string(),
        "password123".to_string(),
    ).calculate_fee(large_amount);
    
    let bt_fee = BankTransferPayment::new(
        "123456789".to_string(),
        "987654321".to_string(),
        "First National Bank".to_string(),
    ).calculate_fee(large_amount);
    
    println!("Credit Card fee: ${cc_fee:.2}");
    println!("PayPal fee: ${pp_fee:.2}");
    println!("Bank Transfer fee: ${bt_fee:.2}");
    
    println!("\n✅ Strategy pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let strategy = BubbleSort;
        strategy.sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let strategy = QuickSort;
        strategy.sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let strategy = MergeSort;
        strategy.sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_credit_card_payment() {
        let strategy = CreditCardPayment::new(
            "1234-5678-9012-3456".to_string(),
            "John Doe".to_string(),
            "12/25".to_string(),
            "123".to_string(),
        );
        
        let amount = PaymentAmount {
            amount: 100.0,
            currency: "USD".to_string(),
        };
        
        let result = strategy.process_payment(&amount);
        assert!(result.success);
        assert!(result.transaction_id.starts_with("CC_"));
    }

    #[test]
    fn test_payment_fee_calculation() {
        let cc_strategy = CreditCardPayment::new(
            "1234-5678-9012-3456".to_string(),
            "John Doe".to_string(),
            "12/25".to_string(),
            "123".to_string(),
        );
        
        let fee = cc_strategy.calculate_fee(100.0);
        assert_eq!(fee, 100.0f64.mul_add(0.029, 0.30));
    }

    #[test]
    fn test_compression_strategies() {
        let data = "Hello, World!";
        
        let zip_strategy = ZipCompression;
        let compressed = zip_strategy.compress(data);
        assert_eq!(compressed, "ZIP[13]");
        
        let decompressed = zip_strategy.decompress(&compressed);
        assert_eq!(decompressed.len(), data.len());
    }

    #[test]
    fn test_sorter_context() {
        let mut data = vec![3, 1, 4, 1, 5];
        let sorter = Sorter::new(Box::new(BubbleSort));
        
        assert_eq!(sorter.get_strategy_name(), "Bubble Sort");
        sorter.sort(&mut data);
        assert_eq!(data, vec![1, 1, 3, 4, 5]);
    }
}