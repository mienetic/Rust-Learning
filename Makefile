# 🦀 Makefile สำหรับ Rust Concepts Project
# ใช้งาน: make <target>

.PHONY: help check clippy test run clean all basics functions ownership structs error collections generics traits lifetimes modules iterators closures smart-ptrs concurrency async macros unsafe ffi testing performance patterns web cli devops gamedev blockchain mobile quality fix warnings ci release doc script-basics script-functions script-all script-interactive script-quality setup-hooks

# Default target
help:
	@echo "🦀 Rust Concepts Project - Available Commands (2025 Edition)"
	@echo ""
	@echo "📋 การตรวจสอบพื้นฐาน:"
	@echo "  make check     - ตรวจสอบ syntax และ type errors"
	@echo "  make clippy    - ตรวจสอบ code quality"
	@echo "  make test      - รัน unit tests ทั้งหมด"
	@echo "  make run       - รันโปรแกรม"
	@echo "  make all       - รัน check + clippy + test"
	@echo ""
	@echo "🆕 Rust Quality Check v2025:"
	@echo "  make quality   - ตรวจสอบคุณภาพโค้ดแบบครอบคลุม"
	@echo "  make fix       - แก้ไข warnings อัตโนมัติ"
	@echo "  make warnings  - แสดง warnings ทั้งหมด"
	@echo "  make ci        - รัน CI checks ทั้งหมด"
	@echo ""
	@echo "📚 การตรวจสอบแยกตามหัวข้อ:"
	@echo "  📖 บทเรียนพื้นฐาน (1-10):"
	@echo "    make basics      - บทที่ 1: ตรวจสอบ Basics module"
	@echo "    make functions   - บทที่ 2: ตรวจสอบ Functions module"
	@echo "    make ownership   - บทที่ 3: ตรวจสอบ Ownership module"
	@echo "    make structs     - บทที่ 4: ตรวจสอบ Structs & Enums module"
	@echo "    make error       - บทที่ 5: ตรวจสอบ Error Handling module"
	@echo "    make collections - บทที่ 6: ตรวจสอบ Collections module"
	@echo "    make generics    - บทที่ 7: ตรวจสอบ Generics module"
	@echo "    make traits      - บทที่ 8: ตรวจสอบ Traits module"
	@echo "    make lifetimes   - บทที่ 9: ตรวจสอบ Lifetimes module"
	@echo "    make modules     - บทที่ 10: ตรวจสอบ Modules module"
	@echo ""
	@echo "  📚 บทเรียนขั้นกลาง (11-14):"
	@echo "    make iterators   - บทที่ 11: ตรวจสอบ Iterators module"
	@echo "    make closures    - บทที่ 12: ตรวจสอบ Closures module"
	@echo "    make smart-ptrs  - บทที่ 13: ตรวจสอบ Smart Pointers module"
	@echo "    make concurrency - บทที่ 14: ตรวจสอบ Concurrency module"
	@echo ""
	@echo "  🚀 บทเรียนขั้นสูง (15-21):"
	@echo "    make async       - บทที่ 15: ตรวจสอบ Async Programming module"
	@echo "    make macros      - บทที่ 16: ตรวจสอบ Macros module"
	@echo "    make unsafe      - บทที่ 17: ตรวจสอบ Unsafe Rust module"
	@echo "    make ffi         - บทที่ 18: ตรวจสอบ FFI module"
	@echo "    make testing     - บทที่ 19: ตรวจสอบ Testing module"
	@echo "    make performance - บทที่ 20: ตรวจสอบ Performance module"
	@echo "    make patterns    - บทที่ 21: ตรวจสอบ Design Patterns module"
	@echo ""
	@echo "  🎯 บทเรียนเฉพาะทาง (22-27):"
	@echo "    make web         - บทที่ 22: ตรวจสอบ Web Development module"
	@echo "    make cli         - บทที่ 23: ตรวจสอบ CLI Applications module"
	@echo "    make devops      - บทที่ 24: ตรวจสอบ DevOps module"
	@echo "    make gamedev     - บทที่ 25: ตรวจสอบ Game Development module"
	@echo "    make blockchain  - บทที่ 26: ตรวจสอบ Blockchain module"
	@echo "    make mobile      - บทที่ 27: ตรวจสอบ Mobile Development module"
	@echo ""
	@echo "🛠️  อื่นๆ:"
	@echo "  make clean     - ลบไฟล์ที่สร้างจากการ build"
	@echo "  make release   - build แบบ release mode"
	@echo "  make doc       - สร้าง documentation"
	@echo ""
	@echo "🔧 Scripts และ Setup:"
	@echo "  make script-quality - รัน Quality Check Script v2025"
	@echo "  make setup-hooks    - ติดตั้ง Pre-commit Hooks"
	@echo "  make script-all     - รัน script ตรวจสอบทั้งโปรเจกต์"

# การตรวจสอบพื้นฐาน
check:
	@echo "🔍 ตรวจสอบ syntax และ type errors..."
	cargo check

clippy:
	@echo "🔍 ตรวจสอบ code quality..."
	cargo clippy

test:
	@echo "🧪 รัน unit tests..."
	cargo test

run:
	@echo "🚀 รันโปรแกรม..."
	cargo run

all: check clippy test
	@echo "✅ ตรวจสอบทั้งหมดเสร็จสิ้น!"

# Rust Project Quality Check Script v2025
quality:
	@echo "🔍 ตรวจสอบคุณภาพโค้ดแบบครอบคลุม (Rust 2025)..."
	cargo check --all-targets
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check
	cargo test
	@echo "✅ Quality check เสร็จสิ้น!"

fix:
	@echo "🔧 แก้ไข warnings อัตโนมัติ..."
	cargo clippy --fix --allow-dirty
	cargo fmt
	@echo "✅ Auto-fix เสร็จสิ้น!"

warnings:
	@echo "⚠️ แสดง warnings ทั้งหมด..."
	cargo clippy --all-targets

ci: quality
	@echo "✅ CI checks ผ่านทั้งหมด!"

# การตรวจสอบแยกตามหัวข้อ
basics:
	@echo "🔥 ตรวจสอบ Basics module..."
	cargo check
	cargo clippy
	cargo test basics::tests
	@echo "✅ Basics module ผ่านการตรวจสอบ"

functions:
	@echo "🚀 ตรวจสอบ Functions module..."
	cargo check
	cargo clippy
	@echo "💡 Functions module ยังไม่มี unit tests"
	@echo "✅ Functions module ผ่านการตรวจสอบพื้นฐาน"

ownership:
	@echo "🔒 ตรวจสอบ Ownership module..."
	cargo check
	cargo clippy
	@echo "💡 Ownership module ยังไม่มี unit tests"
	@echo "✅ Ownership module ผ่านการตรวจสอบพื้นฐาน"

structs:
	@echo "📊 ตรวจสอบ Structs & Enums module..."
	cargo check
	cargo clippy
	@echo "💡 Structs & Enums module ยังไม่มี unit tests"
	@echo "✅ Structs & Enums module ผ่านการตรวจสอบพื้นฐาน"

error:
	@echo "⚠️ ตรวจสอบ Error Handling module..."
	cargo check
	cargo clippy
	cargo test error_handling::tests
	@echo "✅ Error Handling module ผ่านการตรวจสอบ"

collections:
	@echo "📦 ตรวจสอบ Collections module..."
	cargo check
	cargo clippy
	cargo test collections::tests
	@echo "✅ Collections module ผ่านการตรวจสอบ"

generics:
	@echo "🔧 ตรวจสอบ Generics module..."
	cargo check
	cargo clippy
	cargo test generics::tests
	@echo "✅ Generics module ผ่านการตรวจสอบ"

traits:
	@echo "🎭 ตรวจสอบ Traits module..."
	cargo check
	cargo clippy
	cargo test traits::tests
	@echo "✅ Traits module ผ่านการตรวจสอบ"

lifetimes:
	@echo "⏰ ตรวจสอบ Lifetimes module..."
	cargo check
	cargo clippy
	cargo test lifetimes::tests
	@echo "✅ Lifetimes module ผ่านการตรวจสอบ"

modules:
	@echo "📦 ตรวจสอบ Modules module..."
	cargo check
	cargo clippy
	cargo test modules::practice_modules::tests
	@echo "✅ Modules module ผ่านการตรวจสอบ"

# บทเรียนขั้นกลาง (11-14)
iterators:
	@echo "🔄 ตรวจสอบ Iterators module..."
	cargo check
	cargo clippy
	cargo test iterators::tests
	@echo "✅ Iterators module ผ่านการตรวจสอบ"

closures:
	@echo "🎯 ตรวจสอบ Closures module..."
	cargo check
	cargo clippy
	cargo test closures::tests
	@echo "✅ Closures module ผ่านการตรวจสอบ"

smart-ptrs:
	@echo "🧠 ตรวจสอบ Smart Pointers module..."
	cargo check
	cargo clippy
	cargo test smart_pointers::tests
	@echo "✅ Smart Pointers module ผ่านการตรวจสอบ"

concurrency:
	@echo "⚡ ตรวจสอบ Concurrency module..."
	cargo check
	cargo clippy
	cargo test concurrency::tests
	@echo "✅ Concurrency module ผ่านการตรวจสอบ"

# บทเรียนขั้นสูง (15-21)
async:
	@echo "🚀 ตรวจสอบ Async Programming module..."
	cargo check
	cargo clippy
	cargo test async_programming::tests
	@echo "✅ Async Programming module ผ่านการตรวจสอบ"

macros:
	@echo "🎭 ตรวจสอบ Macros module..."
	cargo check
	cargo clippy
	cargo test macros::tests
	@echo "✅ Macros module ผ่านการตรวจสอบ"

unsafe:
	@echo "⚠️ ตรวจสอบ Unsafe Rust module..."
	cargo check
	cargo clippy
	cargo test unsafe_rust::tests
	@echo "✅ Unsafe Rust module ผ่านการตรวจสอบ"

ffi:
	@echo "🔗 ตรวจสอบ FFI module..."
	cargo check
	cargo clippy
	cargo test ffi::tests
	@echo "✅ FFI module ผ่านการตรวจสอบ"

testing:
	@echo "🧪 ตรวจสอบ Testing module..."
	cargo check
	cargo clippy
	cargo test testing_advanced::tests
	@echo "✅ Testing module ผ่านการตรวจสอบ"

performance:
	@echo "⚡ ตรวจสอบ Performance module..."
	cargo check
	cargo clippy
	cargo test performance::tests
	@echo "✅ Performance module ผ่านการตรวจสอบ"

patterns:
	@echo "🎨 ตรวจสอบ Design Patterns module..."
	cargo check
	cargo clippy
	cargo test design_patterns::tests
	@echo "✅ Design Patterns module ผ่านการตรวจสอบ"

# บทเรียนเฉพาะทาง (22-27)
web:
	@echo "🌐 ตรวจสอบ Web Development module..."
	cargo check
	cargo clippy
	cargo test web_development::tests
	@echo "✅ Web Development module ผ่านการตรวจสอบ"

cli:
	@echo "💻 ตรวจสอบ CLI Applications module..."
	cargo check
	cargo clippy
	cargo test cli_applications::tests
	@echo "✅ CLI Applications module ผ่านการตรวจสอบ"

devops:
	@echo "🔧 ตรวจสอบ DevOps module..."
	cargo check
	cargo clippy
	cargo test devops::tests
	@echo "✅ DevOps module ผ่านการตรวจสอบ"

gamedev:
	@echo "🎮 ตรวจสอบ Game Development module..."
	cargo check
	cargo clippy
	cargo test game_development::tests
	@echo "✅ Game Development module ผ่านการตรวจสอบ"

blockchain:
	@echo "⛓️ ตรวจสอบ Blockchain module..."
	cargo check
	cargo clippy
	cargo test blockchain::tests
	@echo "✅ Blockchain module ผ่านการตรวจสอบ"

mobile:
	@echo "📱 ตรวจสอบ Mobile Development module..."
	cargo check
	cargo clippy
	cargo test mobile_development::tests
	@echo "✅ Mobile Development module ผ่านการตรวจสอบ"

# อื่นๆ
clean:
	@echo "🧹 ลบไฟล์ที่สร้างจากการ build..."
	cargo clean

release:
	@echo "🚀 Build แบบ release mode..."
	cargo build --release

doc:
	@echo "📚 สร้าง documentation..."
	cargo doc --open

# การรัน scripts
script-basics:
	@echo "🔥 รัน script สำหรับ Basics..."
	./scripts/check_basics.sh

script-functions:
	@echo "🚀 รัน script สำหรับ Functions..."
	./scripts/check_functions.sh

script-all:
	@echo "🦀 รัน script สำหรับทั้งโปรเจกต์..."
	./scripts/check_all.sh

script-interactive:
	@echo "📚 รัน script แบบ interactive..."
	./scripts/check_by_topic.sh

# Rust Quality Check v2025 Scripts
script-quality:
	@echo "🔍 รัน Quality Check Script v2025..."
	./scripts/quality_check.sh

setup-hooks:
	@echo "🔧 ติดตั้ง Pre-commit Hooks..."
	./scripts/setup_hooks.sh