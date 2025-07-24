#!/bin/bash
# 🚀 Script สำหรับตรวจสอบ Functions Module

echo "🚀 === ตรวจสอบ Functions Module ==="
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

echo "📋 กำลังตรวจสอบการคอมไพล์ functions module..."
cargo check --lib --message-format=short 2>&1 | grep -i "functions"

echo "💡 หมายเหตุ: Functions module ยังไม่มี unit tests"
echo "💡 สามารถเพิ่ม tests ได้โดยเพิ่ม #[cfg(test)] mod tests ในไฟล์ต่างๆ"

echo "🎉 Functions Module ผ่านการตรวจสอบพื้นฐาน!"
echo "📝 แนะนำ: เพิ่ม unit tests สำหรับ functions เพื่อความสมบูรณ์"