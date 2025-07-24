//! Performance Benchmarks สำหรับ Rust Concepts
//!
//! ไฟล์นี้ใช้สำหรับวัดประสิทธิภาพของโค้ดต่างๆ ในโปรเจกต์
//! ใช้ criterion crate สำหรับการ benchmark

use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// Benchmark สำหรับการทำงานของ collections
fn benchmark_collections(c: &mut Criterion) {
    c.bench_function("vector_operations", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(black_box(i));
            }
            vec.sort_unstable();
            vec
        });
    });

    c.bench_function("hashmap_operations", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for i in 0..1000 {
                map.insert(black_box(i), black_box(i * 2));
            }
            map
        });
    });
}

/// Benchmark สำหรับการทำงานของ string operations
fn benchmark_strings(c: &mut Criterion) {
    c.bench_function("string_concatenation", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..100 {
                result.push_str(&black_box(i).to_string());
            }
            result
        });
    });

    c.bench_function("string_formatting", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..100 {
                result.push_str(&format!("Number: {}", black_box(i)));
            }
            result
        });
    });
}

/// Benchmark สำหรับการทำงานของ iterators
fn benchmark_iterators(c: &mut Criterion) {
    let data: Vec<i32> = (0..10000).collect();

    c.bench_function("for_loop", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &item in &data {
                sum += black_box(item);
            }
            sum
        });
    });

    c.bench_function("iterator_sum", |b| {
        b.iter(|| data.iter().map(|&x| black_box(x)).sum::<i32>());
    });

    c.bench_function("iterator_filter_map", |b| {
        b.iter(|| {
            data.iter()
                .filter(|&&x| x % 2 == 0)
                .map(|&x| black_box(x * 2))
                .sum::<i32>()
        });
    });
}

/// Benchmark สำหรับการทำงานของ memory allocation
fn benchmark_memory(c: &mut Criterion) {
    c.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(black_box(i));
            }
            vec
        });
    });

    c.bench_function("vec_without_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(black_box(i));
            }
            vec
        });
    });
}

criterion_group!(
    benches,
    benchmark_collections,
    benchmark_strings,
    benchmark_iterators,
    benchmark_memory
);
criterion_main!(benches);
