#!/bin/bash
# 🔍 Rust Project Quality Check Script v2025
# สคริปต์ตรวจสอบคุณภาพโค้ดแบบครอบคลุม

echo "🔍 === Rust Project Quality Check v2025 ==="
echo "📅 เวลา: $(date)"
echo "🎯 เป้าหมาย: Zero Unexplained Warnings"
echo ""

# ตัวแปรสำหรับนับผลลัพธ์
PASSED=0
FAILED=0
WARNINGS=0

# ฟังก์ชันสำหรับแสดงผลลัพธ์
check_result() {
    if [ $1 -eq 0 ]; then
        echo "✅ $2: ผ่าน"
        PASSED=$((PASSED + 1))
    else
        echo "❌ $2: ไม่ผ่าน"
        FAILED=$((FAILED + 1))
    fi
}

check_warning() {
    if [ $1 -eq 0 ]; then
        echo "✅ $2: ผ่าน"
        PASSED=$((PASSED + 1))
    else
        echo "⚠️ $2: มี warnings"
        WARNINGS=$((WARNINGS + 1))
    fi
}

# 1. ตรวจสอบ Rust installation
echo "📋 ขั้นตอนที่ 1: ตรวจสอบ Rust installation"
rustc --version
cargo --version
echo ""

# 2. ตรวจสอบ dependencies
echo "📋 ขั้นตอนที่ 2: ตรวจสอบ dependencies"
cargo check --all-targets
check_result $? "Dependency Check"
echo ""

# 3. ตรวจสอบ code formatting
echo "📋 ขั้นตอนที่ 3: ตรวจสอบ code formatting"
cargo fmt --check
check_result $? "Code Formatting"
echo ""

# 4. ตรวจสอบ compilation
echo "📋 ขั้นตอนที่ 4: ตรวจสอบ compilation (all targets)"
cargo check --all-targets
check_result $? "Compilation Check"
echo ""

# 5. ตรวจสอบ clippy (แสดง warnings)
echo "📋 ขั้นตอนที่ 5: ตรวจสอบ clippy warnings"
cargo clippy --all-targets
check_warning $? "Clippy Warnings"
echo ""

# 6. ตรวจสอบ clippy (strict mode)
echo "📋 ขั้นตอนที่ 6: ตรวจสอบ clippy (strict mode)"
cargo clippy --all-targets -- -D warnings
check_result $? "Clippy Strict Mode"
echo ""

# 7. รัน unit tests
echo "📋 ขั้นตอนที่ 7: รัน unit tests"
cargo test
check_result $? "Unit Tests"
echo ""

# 8. รัน integration tests
echo "📋 ขั้นตอนที่ 8: รัน integration tests"
cargo test --test '*'
check_result $? "Integration Tests"
echo ""

# 9. ตรวจสอบ documentation
echo "📋 ขั้นตอนที่ 9: ตรวจสอบ documentation"
cargo doc --no-deps
check_result $? "Documentation"
echo ""

# 10. ตรวจสอบ security audit (ถ้ามี cargo-audit)
if command -v cargo-audit &> /dev/null; then
    echo "📋 ขั้นตอนที่ 10: ตรวจสอบ security audit"
    cargo audit
    check_result $? "Security Audit"
    echo ""
else
    echo "💡 cargo-audit ไม่พบ - ข้ามการตรวจสอบ security"
    echo "   ติดตั้ง: cargo install cargo-audit"
    echo ""
fi

# สรุปผลลัพธ์
echo "📊 === สรุปผลลัพธ์ Quality Check v2025 ==="
echo "✅ ผ่าน: $PASSED ขั้นตอน"
echo "⚠️ Warnings: $WARNINGS ขั้นตอน"
echo "❌ ไม่ผ่าน: $FAILED ขั้นตอน"
echo ""

if [ $FAILED -eq 0 ]; then
    if [ $WARNINGS -eq 0 ]; then
        echo "🎯 เป้าหมาย Zero Unexplained Warnings: บรรลุ!"
        echo "🎉 โปรเจกต์ผ่านการตรวจสอบคุณภาพทั้งหมด!"
        echo "🚀 พร้อมสำหรับ production และ CI/CD"
        exit 0
    else
        echo "⚠️ มี warnings บางส่วน - แนะนำให้แก้ไข"
        echo "💡 ใช้ 'make fix' เพื่อแก้ไขอัตโนมัติ"
        echo "💡 หรือเพิ่มคำอธิบายสำหรับ warnings ที่แก้ไขไม่ได้"
        exit 0
    fi
else
    echo "❌ พบปัญหาที่ต้องแก้ไข: $FAILED ขั้นตอน"
    echo "💡 กรุณาแก้ไขปัญหาก่อนดำเนินการต่อ"
    exit 1
fi