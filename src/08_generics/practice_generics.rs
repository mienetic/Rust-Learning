//! Practice Generics - ฝึกฝนการใช้ Generics แบบมือโปร! 🎯✨
//!
//! เหมือนเล่นเลโก้ที่ต่อได้ทุกชิ้น หรือเป็นช่างที่ใช้เครื่องมือได้ทุกอย่าง!
//! ฝึกฝนจนเป็นมืออาชีพ - นี่คือการเรียนรู้แบบ hands-on! 🔧🎪
//! เหมือนเรียนขับรถที่ขับได้ทุกคัน หรือเรียนทำอาหารที่ทำได้ทุกเมนู! 🚗👨‍🍳

/// ฟังก์ชันสำหรับฝึกฝน Generics
/// มาฝึกใช้ Generics กับตัวอย่างจริงกันเถอะ! เหมือนเล่นเลโก้ที่ต่อได้ทุกชิ้น! 🎯
#[allow(clippy::too_many_lines)]
pub fn practice_generics() {
    println!("🎯 === ฝึกฝน Generics: เวลาฝึกหัดใช้ของเล่นใหม่! === 🎯");
    println!("🎪 เหมือนเข้าค่ายฝึกซ้อมกีฬา หรือเรียนเต้นรำที่ต้องฝึกจนชำนาญ! 💃🕺");

    // 1. Generic Stack - กองซ้อนที่ใส่อะไรก็ได้!
    // เหมือนกองหนังสือที่เรียงซ้อนกัน หรือจานที่ล้างแล้วเรียงไว้! 📚🍽️
    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,  // กองของที่เรียงซ้อนกัน
    }

    impl<T> Stack<T> {
        const fn new() -> Self {
            Self { items: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.items.push(item);  // ใส่ของเข้าไปในกอง
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()  // เอาของออกจากด้านบน
        }

        fn peek(&self) -> Option<&T> {
            self.items.last()  // แอบดูของด้านบนโดยไม่เอาออก
        }

        const fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        const fn size(&self) -> usize {
            self.items.len()
        }
    }

    println!("\n📚 === Generic Stack: กองซ้อนที่ไม่เลือกของ! === 📚");
    println!("🥞 เหมือนแพนเค้กที่ซ้อนกันสูง หรือกองเหรียญที่เก็บไว้ในกระปุก! 🪙💰");

    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    println!("🔢 Integer Stack: {int_stack:?} (กองตัวเลขที่เรียงตัวดี!)");
    println!("👀 Peek: {:?} (แอบดูของด้านบน!)", int_stack.peek());
    println!("📤 Pop: {:?} (เอาของออกจากด้านบน!)", int_stack.pop());
    println!("📊 Size: {} (มีของเหลือ {} ชิ้น!)", int_stack.size(), int_stack.size());

    let mut string_stack = Stack::new();
    string_stack.push(String::from("first"));
    string_stack.push(String::from("second"));
    string_stack.push(String::from("third"));

    println!("\n📝 String Stack: {string_stack:?} (กองข้อความที่น่ารัก!)");
    while !string_stack.is_empty() {
        println!("📤 Pop: {:?} (เอาข้อความออกทีละตัว!)", string_stack.pop());
    }

    // 2. Generic Pair with Operations - คู่ที่ทำงานร่วมกันได้!
    // เหมือนคู่หูนักสืบ หรือคู่รักที่เข้าใจกันดี! 🕵️‍♂️🕵️‍♀️💕
    #[derive(Debug, Clone)]
    struct Pair<T> {
        first: T,   // คนแรก
        second: T,  // คนที่สอง
    }

    impl<T> Pair<T> {
        const fn new(first: T, second: T) -> Self {
            Self { first, second }
        }

        // Methods เหล่านี้ใช้เพื่อแสดงตัวอย่างการเข้าถึงข้อมูลใน Pair
        // Warning: methods ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        const fn first(&self) -> &T {
            &self.first  // ดูคนแรก
        }

        #[allow(dead_code)]
        const fn second(&self) -> &T {
            &self.second  // ดูคนที่สอง
        }

        fn swap(self) -> Self {
            // สลับที่กัน
            Self {
                first: self.second,
                second: self.first,
            }
        }
    }

    impl<T: PartialOrd> Pair<T> {
        fn max(&self) -> &T {
            // หาคนที่ใหญ่กว่า (หรือมากกว่า) แบบ bigger winner
            if self.first >= self.second {
                &self.first
            } else {
                &self.second
            }
        }

        fn min(&self) -> &T {
            // หาคนที่เล็กกว่า (หรือน้อยกว่า) แบบ smaller one
            if self.first <= self.second {
                &self.first
            } else {
                &self.second
            }
        }
    }

    impl<T: std::ops::Add<Output = T> + Copy> Pair<T> {
        fn sum(&self) -> T {
            self.first + self.second  // บวกกันเหมือนคู่รักที่ช่วยเหลือกันแบบ teamwork!
        }
    }

    println!("\n👫 === Generic Pair: คู่ที่ลงตัว! === 👫");
    println!("💑 เหมือนคู่รักที่เข้ากันได้ดี หรือคู่หูที่ทำงานร่วมกันเป็น! 👨‍💼👩‍💼");

    let number_pair = Pair::new(10, 20);
    println!("🔢 Number Pair: {number_pair:?} (คู่ตัวเลขที่น่ารัก!)");
    println!("🔺 Max: {} (คนที่ใหญ่กว่าแบบ bigger one!)", number_pair.max());
    println!("🔻 Min: {} (คนที่เล็กกว่าแบบ smaller one!)", number_pair.min());
    println!("➕ Sum: {} (รวมกันแล้วได้เท่านี้แบบ total result!)", number_pair.sum());

    let swapped = number_pair.swap();
    println!("🔄 Swapped: {swapped:?} (สลับที่กันแล้ว!)");

    let char_pair = Pair::new('z', 'a');
    println!("\n🔤 Char Pair: {char_pair:?} (คู่ตัวอักษรที่แปลกแบบ odd character duo!)");
    println!("🔺 Max: {} (ตัวอักษรที่ใหญ่กว่าใน ASCII แบบ ASCII winner!)", char_pair.max());
    println!("🔻 Min: {} (ตัวอักษรที่เล็กกว่าใน ASCII แบบ ASCII smaller!)", char_pair.min());

    // 3. Generic Result Wrapper - ห่อหุ้มผลลัพธ์ให้ปลอดภัย!
    // เหมือนถุงมือที่ป้องกันมือ หรือหมวกกันน็อคที่ป้องกันหัว! 🧤⛑️
    #[derive(Debug)]
    struct SafeOperation<T, E> {
        result: Result<T, E>,  // ผลลัพธ์ที่อาจสำเร็จหรือล้มเหลวแบบ success or failure
    }

    impl<T, E> SafeOperation<T, E> {
        const fn new(result: Result<T, E>) -> Self {
            Self { result }
        }

        const fn is_ok(&self) -> bool {
            self.result.is_ok()
        }

        const fn is_err(&self) -> bool {
            self.result.is_err()
        }

        // Method นี้ใช้เพื่อแสดงตัวอย่างการจัดการ Result
        // Warning: method ไม่ถูกใช้เพราะเป็นตัวอย่างการสาธิต
        #[allow(dead_code)]
        fn unwrap_or(self, default: T) -> T {
            self.result.unwrap_or(default)  // ถ้าผิดพลาดให้ใช้ค่าเริ่มต้น
        }
    }

    impl<T: Clone, E> SafeOperation<T, E> {
        fn get_or_clone(&self, default: T) -> T {
            match &self.result {
                Ok(value) => value.clone(),  // ถ้าสำเร็จให้คัดลอกค่า
                Err(_) => default,           // ถ้าผิดพลาดให้ใช้ค่าเริ่มต้น
            }
        }
    }

    println!("\n🛡️ === Safe Operation Wrapper: ห่อหุ้มความปลอดภัย! === 🛡️");
    println!("🦺 เหมือนเสื้อกั๊กนิรภัยที่ป้องกันอันตราย หรือประกันภัยที่คุ้มครองเรา! 🏥💼");

    let good_op: SafeOperation<i32, String> = SafeOperation::new(Ok(42));
    let bad_op: SafeOperation<i32, String> = SafeOperation::new(Err(String::from("Error")));

    println!("✅ Good operation is ok: {} (ทุกอย่างเรียบร้อย!)", good_op.is_ok());
    println!("❌ Bad operation is err: {} (มีปัญหาเกิดขึ้น!)", bad_op.is_err());

    println!("🔢 Good result or default: {} (ได้ผลลัพธ์ที่ต้องการ!)", good_op.get_or_clone(0));
    println!("🔢 Bad result or default: {} (ใช้ค่าเริ่มต้นแทน!)", bad_op.get_or_clone(0));

    println!("\n🎉 จบแบบฝึกหัด Generics! เก่งมากเลย! 🎉");
    println!("🏆 ยินดีด้วย! คุณเป็นนักเขียนโค้ด Generic ที่เก่งแล้ว! 🥇✨");
    println!("🎓 จากนี้ไปคุณสามารถสร้างโค้ดที่ยืดหยุ่นและใช้ซ้ำได้แล้ว! 🚀💪");
}
