//! Advanced Rust Patterns
//!
//! บทเรียนขั้นสูงเกี่ยวกับ patterns และเทคนิคการเขียนโปรแกรม Rust ระดับสูง
//! ครอบคลุม design patterns, architectural patterns, และ advanced techniques

pub mod builder_pattern;
pub mod command_pattern;
pub mod factory_pattern;
pub mod observer_pattern;
pub mod state_pattern;
pub mod strategy_pattern;
pub mod visitor_pattern;
pub mod newtype_pattern;
pub mod type_state_pattern;
pub mod phantom_types;
pub mod zero_cost_abstractions;
pub mod compile_time_computation;
pub mod practice_advanced_patterns;

/// รันตัวอย่าง Advanced Patterns ทั้งหมด
pub fn run_advanced_patterns_examples() {
    println!("🎨 === Advanced Rust Patterns === 🎨");
    println!();

    // Design Patterns
    println!("📐 Design Patterns:");
    builder_pattern::demonstrate_builder();
    command_pattern::demonstrate_command();
    factory_pattern::demonstrate_factory_pattern();
    observer_pattern::demonstrate_observer();
    state_pattern::demonstrate_state();
    strategy_pattern::demonstrate_strategy();
    visitor_pattern::demonstrate_visitor();
    
    println!();
    
    // Rust-specific Patterns
    println!("🦀 Rust-specific Patterns:");
    newtype_pattern::demonstrate_newtype();
    type_state_pattern::demonstrate_type_state();
    phantom_types::demonstrate_phantom_types();
    zero_cost_abstractions::demonstrate_zero_cost_abstractions();
    compile_time_computation::demonstrate_compile_time_computation();
    
    println!("\n🎭 === แบบฝึกหัด Advanced Patterns === 🎭");
    practice_advanced_patterns::practice_advanced_patterns();
    
    println!("\n✅ Advanced Patterns examples completed!");
}