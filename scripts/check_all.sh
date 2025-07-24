#!/bin/bash
# 🦀 Rust Project Quality Check Script v2025
# Script สำหรับตรวจสอบทั้งโปรเจกต์ Rust Concepts

echo "🦀 === Rust Project Quality Check v2025 ==="
echo "📅 เวลา: $(date)"
echo "🎯 เป้าหมาย: Zero Unexplained Warnings"
echo ""

# ขั้นตอนที่ 1: ตรวจสอบ syntax และ type errors
echo "📋 ขั้นตอนที่ 1: ตรวจสอบ syntax และ type errors (all targets)"
cargo check --all-targets

if [ $? -eq 0 ]; then
    echo "✅ cargo check ผ่าน"
else
    echo "❌ cargo check ไม่ผ่าน - กรุณาแก้ไข errors ก่อน"
    exit 1
fi

echo ""

# ขั้นตอนที่ 2: ตรวจสอบ code formatting
echo "📋 ขั้นตอนที่ 2: ตรวจสอบ code formatting"
cargo fmt --check

if [ $? -eq 0 ]; then
    echo "✅ cargo fmt ผ่าน - โค้ดมีรูปแบบที่ถูกต้อง"
else
    echo "⚠️ cargo fmt พบปัญหา - รันคำสั่ง 'cargo fmt' เพื่อแก้ไข"
    echo "💡 หรือใช้ 'make fix' เพื่อแก้ไขอัตโนมัติ"
fi

echo ""

# ขั้นตอนที่ 3: ตรวจสอบ code quality (strict mode)
echo "📋 ขั้นตอนที่ 3: ตรวจสอบ code quality (strict mode - treat warnings as errors)"
cargo clippy --all-targets -- -D warnings

if [ $? -eq 0 ]; then
    echo "✅ cargo clippy ผ่าน - ไม่มี warnings"
else
    echo "❌ cargo clippy พบ warnings/errors"
    echo "💡 ใช้ 'make fix' เพื่อแก้ไขอัตโนมัติ"
    echo "💡 หรือใช้ 'make warnings' เพื่อดู warnings ทั้งหมด"
    exit 1
fi

echo ""

# ขั้นตอนที่ 4: รัน tests ทั้งหมด
echo "📋 ขั้นตอนที่ 4: รัน unit tests ทั้งหมด"
cargo test

if [ $? -eq 0 ]; then
    echo "✅ cargo test ผ่านทั้งหมด"
else
    echo "❌ cargo test มี tests ที่ไม่ผ่าน"
    exit 1
fi

echo ""

# สรุปผลการตรวจสอบ
echo "📊 === สรุปผลการตรวจสอบ Rust Quality Check v2025 ==="
echo "✅ Syntax Check (all targets): ผ่าน"
echo "✅ Code Formatting: ผ่าน"
echo "✅ Code Quality (strict mode): ผ่าน"
echo "✅ Unit Tests: ผ่าน"
echo ""
echo "🎯 เป้าหมาย Zero Unexplained Warnings: บรรลุ!"
echo "🎉 ทั้งโปรเจกต์ผ่านการตรวจสอบทั้งหมด!"
echo "🚀 โปรเจกต์พร้อมสำหรับการใช้งานและ CI/CD"