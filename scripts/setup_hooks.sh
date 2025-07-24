#!/bin/bash
# 🔧 Script สำหรับติดตั้ง Pre-commit Hooks
# Rust Project Quality Check v2025

echo "🔧 === ติดตั้ง Pre-commit Hooks สำหรับ Rust Quality Check v2025 ==="
echo ""

# ตรวจสอบว่ามี Python และ pip หรือไม่
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 ไม่พบ - กรุณาติดตั้ง Python3 ก่อน"
    exit 1
fi

if ! command -v pip3 &> /dev/null; then
    echo "❌ pip3 ไม่พบ - กรุณาติดตั้ง pip3 ก่อน"
    exit 1
fi

echo "✅ Python3 และ pip3 พร้อมใช้งาน"
echo ""

# ติดตั้ง pre-commit
echo "📦 ติดตั้ง pre-commit..."
pip3 install pre-commit

if [ $? -eq 0 ]; then
    echo "✅ ติดตั้ง pre-commit สำเร็จ"
else
    echo "❌ ติดตั้ง pre-commit ไม่สำเร็จ"
    exit 1
fi

echo ""

# ติดตั้ง hooks
echo "🔗 ติดตั้ง pre-commit hooks..."
pre-commit install

if [ $? -eq 0 ]; then
    echo "✅ ติดตั้ง pre-commit hooks สำเร็จ"
else
    echo "❌ ติดตั้ง pre-commit hooks ไม่สำเร็จ"
    exit 1
fi

echo ""

# ทดสอบ hooks
echo "🧪 ทดสอบ pre-commit hooks..."
pre-commit run --all-files

echo ""
echo "🎉 === การติดตั้ง Pre-commit Hooks เสร็จสิ้น! ==="
echo ""
echo "📋 ตอนนี้ทุกครั้งที่คุณ commit โค้ด:"
echo "   1. ✅ Rust formatting จะถูกตรวจสอบ"
echo "   2. ✅ Rust compilation จะถูกตรวจสอบ"
echo "   3. ✅ Rust clippy (strict mode) จะถูกตรวจสอบ"
echo "   4. ✅ Rust tests จะถูกรัน"
echo "   5. ✅ ไฟล์ทั่วไปจะถูกตรวจสอบ"
echo ""
echo "💡 หากต้องการข้าม hooks: git commit --no-verify"
echo "💡 หากต้องการรัน hooks ด้วยตนเอง: pre-commit run --all-files"
echo "💡 หากต้องการอัพเดท hooks: pre-commit autoupdate"