#!/bin/bash
# 🔥 Script สำหรับตรวจสอบ Basics Module

echo "🔥 === ตรวจสอบ Basics Module ==="
echo "📋 กำลังตรวจสอบ syntax และ type errors..."
cargo check --lib

if [ $? -eq 0 ]; then
    echo "✅ cargo check ผ่าน"
else
    echo "❌ cargo check ไม่ผ่าน"
    exit 1
fi

echo "📋 กำลังตรวจสอบ code quality..."
cargo clippy --lib

if [ $? -eq 0 ]; then
    echo "✅ cargo clippy ผ่าน"
else
    echo "❌ cargo clippy ไม่ผ่าน"
    exit 1
fi

echo "📋 กำลังรัน tests สำหรับ basics..."
cargo test basics::tests

if [ $? -eq 0 ]; then
    echo "✅ cargo test ผ่าน"
else
    echo "❌ cargo test ไม่ผ่าน"
    exit 1
fi

echo "🎉 Basics Module ผ่านการตรวจสอบทั้งหมด!"