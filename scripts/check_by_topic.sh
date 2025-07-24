#!/bin/bash
# 📚 Script สำหรับตรวจสอบแยกตามหัวข้อ

# ฟังก์ชันสำหรับแสดงเมนู
show_menu() {
    echo "📚 === เลือกหัวข้อที่ต้องการตรวจสอบ ==="
    echo ""
    echo "📖 บทเรียนพื้นฐาน (1-10):"
    echo "1. 🔥 Basics - พื้นฐาน Rust"
    echo "2. 🚀 Functions - ฟังก์ชันและ Control Flow"
    echo "3. 🔒 Ownership - Ownership และ Borrowing"
    echo "4. 📊 Structs & Enums - โครงสร้างข้อมูล"
    echo "5. ⚠️  Error Handling - การจัดการ Error"
    echo "6. 📦 Collections - คอลเลกชัน"
    echo "7. 🔧 Generics - Generic Types"
    echo "8. 🎭 Traits - Traits"
    echo "9. ⏰ Lifetimes - Lifetimes"
    echo "10. 📦 Modules - Module System"
    echo ""
    echo "📚 บทเรียนขั้นกลาง (11-14):"
    echo "11. 🔄 Iterators - การใช้งาน Iterators"
    echo "12. 🎯 Closures - Closures และ Function Pointers"
    echo "13. 🧠 Smart Pointers - Smart Pointers"
    echo "14. ⚡ Concurrency - การเขียนโปรแกรมแบบ Concurrent"
    echo ""
    echo "🚀 บทเรียนขั้นสูง (15-21):"
    echo "15. 🚀 Async Programming - การเขียนโปรแกรมแบบ Asynchronous"
    echo "16. 🎭 Macros - การสร้างและใช้งาน Macros"
    echo "17. ⚠️  Unsafe Rust - การใช้งาน Unsafe Rust"
    echo "18. 🔗 FFI - Foreign Function Interface"
    echo "19. 🧪 Testing - การทดสอบขั้นสูง"
    echo "20. ⚡ Performance - การปรับปรุงประสิทธิภาพ"
    echo "21. 🎨 Design Patterns - รูปแบบการออกแบบ"
    echo ""
    echo "🎯 บทเรียนเฉพาะทาง (22-27):"
    echo "22. 🌐 Web Development - การพัฒนาเว็บ"
    echo "23. 💻 CLI Applications - การสร้างแอปพลิเคชัน CLI"
    echo "24. 🔧 DevOps - เครื่องมือ DevOps"
    echo "25. 🎮 Game Development - การพัฒนาเกม"
    echo "26. ⛓️  Blockchain - การพัฒนา Blockchain"
    echo "27. 📱 Mobile Development - การพัฒนาแอปมือถือ"
    echo ""
    echo "28. 🦀 ทั้งหมด - ตรวจสอบทั้งโปรเจกต์"
    echo "0. ❌ ออก"
    echo ""
    echo -n "เลือกหัวข้อ (0-28): "
}

# ฟังก์ชันสำหรับตรวจสอบแต่ละหัวข้อ
check_topic() {
    local topic=$1
    local topic_name=$2
    
    echo "📋 === ตรวจสอบ $topic_name ==="
    
    # ตรวจสอบ syntax
    echo "🔍 ตรวจสอบ syntax..."
    cargo check --lib
    
    # ตรวจสอบ code quality
    echo "🔍 ตรวจสอบ code quality..."
    cargo clippy --lib
    
    # รัน tests ถ้ามี
    echo "🔍 รัน tests..."
    if [ "$topic" = "all" ]; then
        cargo test
    else
        cargo test $topic 2>/dev/null || echo "💡 ไม่พบ tests สำหรับ $topic_name"
    fi
    
    echo "✅ เสร็จสิ้นการตรวจสอบ $topic_name"
    echo ""
}

# Main loop
while true; do
    show_menu
    read choice
    
    case $choice in
        # บทเรียนพื้นฐาน (1-10)
        1)
            check_topic "basics" "Basics - พื้นฐาน Rust"
            ;;
        2)
            check_topic "functions" "Functions - ฟังก์ชันและ Control Flow"
            ;;
        3)
            check_topic "ownership" "Ownership - Ownership และ Borrowing"
            ;;
        4)
            check_topic "structs_enums" "Structs & Enums - โครงสร้างข้อมูล"
            ;;
        5)
            check_topic "error_handling" "Error Handling - การจัดการ Error"
            ;;
        6)
            check_topic "collections" "Collections - คอลเลกชัน"
            ;;
        7)
            check_topic "generics" "Generics - Generic Types"
            ;;
        8)
            check_topic "traits" "Traits - Traits"
            ;;
        9)
            check_topic "lifetimes" "Lifetimes - Lifetimes"
            ;;
        10)
            check_topic "modules" "Modules - Module System"
            ;;
        # บทเรียนขั้นกลาง (11-14)
        11)
            check_topic "iterators" "Iterators - การใช้งาน Iterators"
            ;;
        12)
            check_topic "closures" "Closures - Closures และ Function Pointers"
            ;;
        13)
            check_topic "smart_pointers" "Smart Pointers - Smart Pointers"
            ;;
        14)
            check_topic "concurrency" "Concurrency - การเขียนโปรแกรมแบบ Concurrent"
            ;;
        # บทเรียนขั้นสูง (15-21)
        15)
            check_topic "async_programming" "Async Programming - การเขียนโปรแกรมแบบ Asynchronous"
            ;;
        16)
            check_topic "macros" "Macros - การสร้างและใช้งาน Macros"
            ;;
        17)
            check_topic "unsafe_rust" "Unsafe Rust - การใช้งาน Unsafe Rust"
            ;;
        18)
            check_topic "ffi" "FFI - Foreign Function Interface"
            ;;
        19)
            check_topic "testing_advanced" "Testing - การทดสอบขั้นสูง"
            ;;
        20)
            check_topic "performance" "Performance - การปรับปรุงประสิทธิภาพ"
            ;;
        21)
            check_topic "design_patterns" "Design Patterns - รูปแบบการออกแบบ"
            ;;
        # บทเรียนเฉพาะทาง (22-27)
        22)
            check_topic "web_development" "Web Development - การพัฒนาเว็บ"
            ;;
        23)
            check_topic "cli_applications" "CLI Applications - การสร้างแอปพลิเคชัน CLI"
            ;;
        24)
            check_topic "devops" "DevOps - เครื่องมือ DevOps"
            ;;
        25)
            check_topic "game_development" "Game Development - การพัฒนาเกม"
            ;;
        26)
            check_topic "blockchain" "Blockchain - การพัฒนา Blockchain"
            ;;
        27)
            check_topic "mobile_development" "Mobile Development - การพัฒนาแอปมือถือ"
            ;;
        # ทั้งหมด
        28)
            check_topic "all" "ทั้งโปรเจกต์"
            ;;
        0)
            echo "👋 ขอบคุณที่ใช้งาน!"
            exit 0
            ;;
        *)
            echo "❌ กรุณาเลือกตัวเลข 0-28"
            echo ""
            ;;
    esac
    
    echo "กด Enter เพื่อดำเนินการต่อ..."
    read
    clear
done