//! โปรเจค Rust Concepts Learning - Rust 1.88.0 Edition
//!
//! โปรเจคนี้แสดงแนวคิดต่างๆ ของ Rust programming
//! จัดระเบียบเป็นโมดูลเพื่อการเรียนรู้อย่างเป็นระบบ
//! อัปเดตสำหรับ Rust 1.88.0 และ Edition 2024

use clap::{Arg, Command};
use rust_concepts::{
    async_await, basics, collections, error_handling, functions, generics, lifetimes, macros,
    modules, ownership, structs_enums, testing, traits, unsafe_rust,
};

fn main() {
    // ตรวจสอบ command line arguments
    let matches = Command::new("Rust Concepts Learning")
        .version("0.2.0")
        .author("Rust Learning Project")
        .about("โปรเจค Rust learning ที่สุดยอด - ครอบคลุมแนวคิดทั้งหมด 20 บท")
        .arg(
            Arg::new("chapter")
                .short('c')
                .long("chapter")
                .value_name("NUMBER")
                .help("เรียนรู้เฉพาะบทที่กำหนด (1-27)")
                .value_parser(clap::value_parser!(u8).range(1..=27)),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("แสดงรายการบทเรียนทั้งหมด")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .help("รัน performance benchmarks")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("async")
                .short('a')
                .long("async")
                .help("รัน async examples (บทที่ 11)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // แสดงข้อมูลเวอร์ชั่น Rust
    println!("🦀 Rust Concepts Learning Project - Rust 1.88.0 Edition 🦀");
    println!("📅 Edition 2024 | 🚀 Version 0.2.0");
    println!("{}", "=".repeat(60));

    // ตรวจสอบ flags
    if matches.get_flag("list") {
        show_chapter_list();
        return;
    }

    if matches.get_flag("benchmark") {
        println!("🏃‍♂️ รัน benchmarks ด้วยคำสั่ง: cargo bench");
        return;
    }

    if matches.get_flag("async") {
        println!("🔄 รัน async examples...");
        run_async_chapter();
        return;
    }

    // เรียนรู้ตามบทที่กำหนด หรือทั้งหมด
    if let Some(chapter) = matches.get_one::<u8>("chapter") {
        if *chapter == 11 {
            run_async_chapter();
        } else {
            run_specific_chapter(*chapter);
        }
    } else {
        // รันทั้งหมด
        run_all_examples();
    }
}

/// แสดงรายการบทเรียนทั้งหมด
fn show_chapter_list() {
    println!("📚 รายการบทเรียน Rust Concepts (เรียงตามลำดับการเรียนรู้):");
    println!();
    println!("🔰 บทเรียนพื้นฐาน (1-10):");
    println!("  1. 🔥 พื้นฐาน Rust (Variables, Types, Operators)");
    println!("  2. 🔒 Ownership และ Borrowing");
    println!("  3. 📊 Structs และ Enums");
    println!("  4. 🚀 Functions และ Control Flow");
    println!("  5. 📦 Modules และ Packages");
    println!("  6. 📚 Collections (Vec, HashMap, etc.)");
    println!("  7. ⚠️ Error Handling");
    println!("  8. 🔧 Generics");
    println!("  9. 🎯 Traits");
    println!(" 10. ⏰ Lifetimes");
    println!();
    println!("🎓 บทเรียนขั้นกลาง (11-14):");
    println!(" 11. 🔄 Async/Await Programming");
    println!(" 12. 🎭 Macros");
    println!(" 13. 🧪 Testing");
    println!(" 14. ⚡ Unsafe Rust");
    println!();
    println!("🚀 บทเรียนขั้นสูง (15-21):");
    println!(" 15. 🧠 Smart Pointers");
    println!(" 16. 🔀 Concurrency & Parallelism");
    println!(" 17. 📁 I/O & File System");
    println!(" 18. 🌐 Network Programming");
    println!(" 19. 🕸️ Web Development");
    println!(" 20. 🗄️ Database Integration");
    println!(" 21. ⚡ Performance Optimization");
    println!();
    println!("🎯 บทเรียนเฉพาะทาง (22-27):");
    println!(" 22. 🔗 Foreign Function Interface (FFI)");
    println!(" 23. 🔧 Embedded Programming");
    println!(" 24. 🚀 DevOps & Deployment");
    println!(" 25. 🎮 Game Development");
    println!(" 26. ⛓️ Blockchain Development");
    println!(" 27. 📱 Mobile Development");
    println!();
    println!("💡 ใช้ --chapter <NUMBER> เพื่อเรียนรู้บทเฉพาะ");
    println!("💡 ใช้ --async เพื่อรัน async examples");
    println!("💡 ใช้ --benchmark เพื่อรัน performance tests");
}

/// รันบทเรียนเฉพาะ
fn run_specific_chapter(chapter: u8) {
    match chapter {
        1 => {
            println!("🔥 === บทที่ 1: พื้นฐาน Rust === 🔥");
            basics::run_basics_examples();
        }
        2 => {
            println!("🔒 === บทที่ 2: Ownership และ Borrowing === 🔒");
            ownership::run_ownership_examples();
        }
        3 => {
            println!("📊 === บทที่ 3: Structs และ Enums === 📊");
            structs_enums::run_structs_enums_examples();
        }
        4 => {
            println!("🚀 === บทที่ 4: Functions และ Control Flow === 🚀");
            functions::run_functions_examples();
        }
        5 => {
            println!("📦 === บทที่ 5: Modules และ Packages === 📦");
            modules::run_modules_examples();
        }
        6 => {
            println!("📚 === บทที่ 6: Collections === 📚");
            collections::run_collections_examples();
        }
        7 => {
            println!("⚠️ === บทที่ 7: Error Handling === ⚠️");
            error_handling::run_error_handling_examples();
        }
        8 => {
            println!("🔧 === บทที่ 8: Generics === 🔧");
            generics::run_generics_examples();
        }
        9 => {
            println!("🎯 === บทที่ 9: Traits === 🎯");
            traits::run_traits_examples();
        }
        10 => {
            println!("⏰ === บทที่ 10: Lifetimes === ⏰");
            lifetimes::run_lifetimes_examples();
        }
        12 => {
            println!("🎭 === บทที่ 12: Macros === 🎭");
            macros::run_macros_examples();
        }
        13 => {
            println!("🧪 === บทที่ 13: Testing === 🧪");
            testing::run_testing_examples();
        }
        14 => {
            println!("⚡ === บทที่ 14: Unsafe Rust === ⚡");
            unsafe_rust::run_unsafe_examples();
        }
        15 => {
            println!("🧠 === บทที่ 15: Smart Pointers === 🧠");
            println!("🎯 เรียนรู้ Box, Rc, Arc, RefCell และ Smart Pointers อื่นๆ!");
            rust_concepts::smart_pointers::run_smart_pointers_examples();
        }
        16 => {
            println!("🔀 === บทที่ 16: Concurrency === 🔀");
            println!("⚡ เรียนรู้ Threads, Channels และ Parallel Programming!");
            rust_concepts::concurrency::run_concurrency_examples();
        }
        17 => {
            println!("📁 === บทที่ 17: I/O & File System === 📁");
            println!("📂 เรียนรู้การจัดการไฟล์และ I/O Operations!");
            rust_concepts::io_filesystem::run_io_filesystem_examples();
        }
        18 => {
            println!("🌐 === บทที่ 18: Network Programming === 🌐");
            println!("📡 เรียนรู้การเขียนโปรแกรมเครือข่าย TCP/UDP!");
            rust_concepts::networking::run_networking_examples();
        }
        19 => {
            println!("🕸️ === บทที่ 19: Web Development === 🕸️");
            println!("🌍 เรียนรู้การสร้าง Web Applications และ APIs!");
            rust_concepts::web_development::run_web_development_examples();
        }
        20 => {
            println!("🗄️ === บทที่ 20: Database Integration === 🗄️");
            println!("🗃️ เรียนรู้การเชื่อมต่อฐานข้อมูลและ ORM!");
            rust_concepts::database::run_database_examples();
        }
        21 => {
            println!("⚡ === บทที่ 21: Performance Optimization === ⚡");
            println!("🚀 เรียนรู้เทคนิคเพิ่มประสิทธิภาพแบบเทพ!");
            rust_concepts::performance::run_performance_examples();
        }
        22 => {
            println!("🔗 === บทที่ 22: Foreign Function Interface (FFI) === 🔗");
            println!("🌉 เรียนรู้การเชื่อมต่อกับภาษาอื่น (C/C++)!");
            rust_concepts::ffi::run_ffi_examples();
        }
        23 => {
            println!("🔧 === บทที่ 23: Embedded Programming === 🔧");
            println!("🤖 เรียนรู้การเขียนโปรแกรมสำหรับ Microcontrollers!");
            rust_concepts::embedded::run_embedded_examples();
        }
        24 => {
            println!("🚀 === บทที่ 24: DevOps & Deployment === 🚀");
            println!("🛠️ เรียนรู้ DevOps และ Deployment แบบโปร!");
            rust_concepts::devops::run_devops_examples();
        }
        25 => {
            println!("🎮 === บทที่ 25: Game Development === 🎮");
            println!("🕹️ เรียนรู้การพัฒนาเกมด้วย Rust!");
            rust_concepts::game_development::run_game_development_examples();
        }
        26 => {
            println!("⛓️ === บทที่ 26: Blockchain Development === ⛓️");
            println!("🔗 เรียนรู้เทคโนโลยี Blockchain และ Smart Contracts!");
            rust_concepts::blockchain::run_blockchain_examples();
        }
        27 => {
            println!("📱 === บทที่ 27: Mobile Development === 📱");
            println!("📲 เรียนรู้การพัฒนาแอปมือถือด้วย Rust!");
            rust_concepts::mobile_development::run_mobile_development_examples();
        }
        _ => unreachable!("Invalid chapter number"),
    }

    println!("\n✅ เสร็จสิ้นบทที่ {chapter}!");
}

/// รัน async chapter (บทที่ 11)
#[tokio::main]
async fn run_async_chapter() {
    println!("🔄 === บทที่ 11: Async/Await Programming === 🔄");
    async_await::run_async_examples().await;
    println!("\n✅ เสร็จสิ้นบทที่ 11!");
}

/// รันตัวอย่างทั้งหมด (ตามลำดับการเรียนรู้)
fn run_all_examples() {
    println!("🔥 === บทที่ 1: พื้นฐาน Rust === 🔥");
    basics::run_basics_examples();

    println!("\n\n🔒 === บทที่ 2: Ownership และ Borrowing === 🔒");
    ownership::run_ownership_examples();

    println!("\n\n📊 === บทที่ 3: Structs และ Enums === 📊");
    structs_enums::run_structs_enums_examples();

    println!("\n\n🚀 === บทที่ 4: Functions และ Control Flow === 🚀");
    functions::run_functions_examples();

    println!("\n\n📦 === บทที่ 5: Modules และ Packages === 📦");
    modules::run_modules_examples();

    println!("\n\n📚 === บทที่ 6: Collections === 📚");
    collections::run_collections_examples();

    println!("\n\n⚠️ === บทที่ 7: Error Handling === ⚠️");
    error_handling::run_error_handling_examples();

    println!("\n\n🔧 === บทที่ 8: Generics === 🔧");
    generics::run_generics_examples();

    println!("\n\n🎯 === บทที่ 9: Traits === 🎯");
    traits::run_traits_examples();

    println!("\n\n⏰ === บทที่ 10: Lifetimes === ⏰");
    lifetimes::run_lifetimes_examples();

    println!("\n\n🎭 === บทที่ 12: Macros === 🎭");
    macros::run_macros_examples();

    println!("\n\n🧪 === บทที่ 13: Testing === 🧪");
    testing::run_testing_examples();

    println!("\n\n⚡ === บทที่ 14: Unsafe Rust === ⚡");
    unsafe_rust::run_unsafe_examples();

    println!("\n\n🎊 สำเร็จ! คุณได้เรียนรู้แนวคิดสำคัญของ Rust ครบถ้วนแล้ว! 🎊");
    println!("🚀 ตอนนี้คุณพร้อมที่จะสร้างแอปพลิเคชัน Rust ของตัวเองแล้ว!");
    println!("\n💡 หมายเหตุ: บทที่ 11 (Async/Await) ต้องรันแยกด้วย --async หรือ --chapter 11");
}
