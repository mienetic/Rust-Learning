//! # Async Channels - ระบบสื่อสารแห่งอนาคต! 📡⚡
//!
//! ไฟล์นี้เป็นศูนย์กลางการสื่อสารแบบ async ที่จะทำให้คุณเป็นนักสื่อสารมืออาชีพ! 🎭✨
//!
//! ## สิ่งที่จะได้เรียนรู้: 🎯
//! - 📡 Tokio Channels - ระบบสื่อสารแห่งอนาคต! (วิทยุสื่อสาร!)
//! - 👥 MPSC Channels - หลายคนส่ง คนเดียวรับ! (ไปรษณีย์หลายสาขา!)
//! - 🎯 Oneshot Channels - ส่งครั้งเดียว จบ! (จดหมายด่วน!)
//! - 📢 Broadcast Channels - ประกาศข่าวทั่วโลก! (หอกระจายข่าว!)
//! - 🤝 Task Coordination - การประสานงานแบบเทพ! (ผู้จัดการทีม!)
//!
//! 🎪 เตรียมตัวให้พร้อม! เราจะมาเรียนรู้ศิลปะการสื่อสารแบบ async กัน! 🚀📻

use std::time::Duration;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::time::sleep;

/// ตัวอย่างการใช้ channels สำหรับ communication - ไปรษณีย์แห่งอนาคต! 📮⚡
pub async fn channels_example() {
    println!("\n📡✨ === ตัวอย่าง Async Channels: ไปรษณีย์แห่งอนาคต! === ✨📡");
    println!("🎯 เตรียมพบกับระบบส่งจดหมายที่ไม่เคยมีมาก่อน! (ไม่ต้องรอคิวนาน!) 🚀");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10); // กล่องจดหมายขนาด 10 ใบ! 📮

    // Producer task - บุรุษไปรษณีย์แห่งอนาคต! 👨‍💼📮
    let producer = tokio::spawn(async move {
        println!("👨‍💼 บุรุษไปรษณีย์เริ่มงาน! เตรียมส่งจดหมาย 5 ฉบับ! 📬✨");
        for i in 1..=5 {
            let message = format!("📜 จดหมายมหัศจรรย์ #{i} (ส่งด้วยความรัก! 💕)");
            println!("📤🚀 ส่ง: {message}");

            if tx.send(message).await.is_err() {
                println!("❌💥 โอ๊ะโอ! กล่องจดหมายเต็ม! (หรือไม่มีใครรับ!) 😱");
                break;
            }

            sleep(Duration::from_millis(300)).await; // พักเขียนจดหมายฉบับต่อไป! ✍️
        }
        println!("📤🎉 บุรุษไปรษณีย์เสร็จงาน! ส่งจดหมายครบแล้ว! (ได้เงินเดือน!) 💰");
    });

    // Consumer task - คนรับจดหมายผู้อดทน! 👩‍💼📬
    let consumer = tokio::spawn(async move {
        println!("👩‍💼 คนรับจดหมายพร้อมแล้ว! รอจดหมายมาถึง... 📬👀");
        let mut count = 0;
        while let Some(message) = rx.recv().await {
            count += 1;
            println!("📥💖 รับจดหมายที่ {count}: {message} (อ่านด้วยความตื้นตัน!) 😊");
            sleep(Duration::from_millis(100)).await; // อ่านจดหมายอย่างละเอียด! 👓
        }
        println!("📥🏆 คนรับจดหมายเสร็จงาน! อ่านจดหมายครบ {count} ฉบับแล้ว! (ความสุขล้นหัวใจ!) 💝");
    });

    // รอให้ทั้งสอง tasks เสร็จ - รอให้ไปรษณีย์ปิด! 🏢
    println!("⏳ รอให้ระบบไปรษณีย์ทำงานเสร็จ... (ไม่ต้องกังวล ไม่บล็อกใคร!) 😌");
    let _ = tokio::join!(producer, consumer);
    println!("🎊 ระบบไปรษณีย์แห่งอนาคตทำงานเสร็จสิ้น! ทุกคนมีความสุข! 🌈✨");
}

/// ตัวอย่างการใช้ multiple producers - ทีมบุรุษไปรษณีย์ซูเปอร์! 👥📮⚡
pub async fn multiple_producers_example() {
    println!("\n👥🚀 === ตัวอย่าง Multiple Producers: ทีมบุรุษไปรษณีย์ซูเปอร์! === 🚀👥");
    println!("🎪 เตรียมพบกับการแข่งขันส่งจดหมายสุดมันส์! 3 ทีม 1 เป้าหมาย! 🏆");

    let (tx, mut rx) = mpsc::channel::<String>(20); // กล่องจดหมายยักษ์ขนาด 20 ใบ! 📮🏢

    // สร้าง multiple producers - เรียกทีมบุรุษไปรษณีย์! 👥
    let mut producers = Vec::new();
    println!("🎯 เรียกทีมบุรุษไปรษณีย์ทั้ง 3 ทีม! พร้อมแข่งขัน! 🏁");

    for i in 1..=3 {
        let tx_clone = tx.clone();
        let producer = tokio::spawn(async move {
            println!("👨‍💼 [ทีม{i}] บุรุษไปรษณีย์เข้าสู่สนาม! พร้อมส่งจดหมาย 3 ฉบับ! 🏃‍♂️💨");
            for j in 1..=3 {
                let message = format!("🎁 [ทีม{i}] จดหมายพิเศษ #{j} (ส่งด้วยใจรัก! 💝)");
                println!("📤🏃‍♂️ [ทีม{i}] วิ่งส่ง: {message}");

                if tx_clone.send(message).await.is_err() {
                    println!("❌😱 [ทีม{i}] โอ๊ะโอ! ส่งไม่ได้! (อาจจะเหนื่อยเกินไป!) 💦");
                    break;
                }

                sleep(Duration::from_millis(200 + i * 100)).await; // แต่ละทีมมีจังหวะต่างกัน! 🎵
            }
            println!("📤🏆 [ทีม{i}] บุรุษไปรษณีย์เสร็จภารกิจ! (ได้โบนัส!) 💰✨");
        });
        producers.push(producer);
    }

    // ปิด original sender - ปิดสำนักงานใหญ่! 🏢🔒
    drop(tx);
    println!("🏢🔒 ปิดสำนักงานใหญ่! ให้ทีมภาคสนามทำงานต่อ!");

    // Consumer task - ผู้รับจดหมายซูเปอร์! 👩‍💼🦸‍♀️
    let consumer = tokio::spawn(async move {
        println!("👩‍💼🦸‍♀️ ผู้รับจดหมายซูเปอร์พร้อมแล้ว! รอรับจดหมายจากทุกทีม! 📬⚡");
        let mut count = 0;
        while let Some(message) = rx.recv().await {
            count += 1;
            println!("📥🎉 [รับที่ {count}] ได้จดหมาย: {message} (อ่านด้วยความปลาบปลื้ม!) 😍");
            sleep(Duration::from_millis(50)).await; // อ่านอย่างรวดเร็วแต่ละเอียด! ⚡👓
        }
        println!("📥🏅 ผู้รับจดหมายซูเปอร์เสร็จงาน! รับครบ {count} ฉบับ! (ทุกฉบับมีค่า!) 💎");
    });

    // รอให้ทุก tasks เสร็จ - รอให้การแข่งขันจบ! 🏁
    println!("⏳🏁 รอให้การแข่งขันส่งจดหมายจบ... (ใครจะเป็นแชมป์?) 🏆");
    for (i, producer) in producers.into_iter().enumerate() {
        let _ = producer.await;
        println!("✅ ทีม{} เข้าเส้นชัยแล้ว! 🏃‍♂️🏁", i + 1);
    }
    let _ = consumer.await;
    println!("🎊🏆 การแข่งขันส่งจดหมายเสร็จสิ้น! ทุกทีมเป็นแชมป์! (ไม่มีใครแพ้!) 🌈✨");
}

/// ตัวอย่างการใช้ oneshot channels - จดหมายด่วนพิเศษ! 🎯⚡
pub async fn oneshot_example() {
    println!("\n🎯⚡ === ตัวอย่าง Oneshot Channels: จดหมายด่วนพิเศษ! === ⚡🎯");
    println!("🚀 เตรียมพบกับบริการจดหมายด่วนที่เร็วที่สุดในจักรวาล! (ส่งครั้งเดียว จบ!) 💫");

    let (tx, rx) = oneshot::channel::<String>(); // ท่อส่งจดหมายด่วนพิเศษ! 🚀📮

    // Task ที่ส่งผลลัพธ์ - นักวิทยาศาสตร์บ้า! 👨‍🔬🧪
    let sender_task = tokio::spawn(async move {
        println!("👨‍🔬🧪 นักวิทยาศาสตร์เริ่มทำการทดลอง! (สูตรลับกำลังมา!) ⚗️✨");
        println!("🔄💫 กำลังประมวลผลสูตรมหัศจรรย์... (อย่าเพิ่งไปไหน!) 🎭");
        sleep(Duration::from_millis(500)).await; // ทำการทดลองอย่างระมัดระวัง! 🧬

        let result = "🎉 สูตรมหัศจรรย์สำเร็จ! ความลับของจักรวาล = 42! 🌌✨".to_string();
        println!("📤🚀 ส่งผลลัพธ์ด่วนพิเศษ: {result}");

        if tx.send(result).is_err() {
            println!("❌💥 โอ๊ะโอ! ส่งไม่ได้! (อาจจะมีคนดักฟัง!) 🕵️‍♂️");
        } else {
            println!("✅🎊 ส่งสำเร็จ! จดหมายด่วนออกเดินทางแล้ว! 🏃‍♂️💨");
        }
    });

    // Task ที่รอรับผลลัพธ์ - นักสืบรอคำตอบ! 🕵️‍♀️🔍
    let receiver_task = tokio::spawn(async move {
        println!("🕵️‍♀️🔍 นักสืบพร้อมแล้ว! รอรับข้อมูลลับสุดยอด... (ตื่นเต้นมาก!) 💓");
        println!("⏳👀 จ้องมองกล่องจดหมายด่วน... (หัวใจเต้นแรง!) 💗");

        if let Ok(result) = rx.await {
            println!("📥🎉 รับผลลัพธ์สำเร็จ! {result}");
            println!("🤯💫 ตะลึง! ความลับของจักรวาลถูกเปิดเผยแล้ว! (ชีวิตมีความหมาย!) 🌟");
        } else {
            println!("❌😱 ไม่ได้รับผลลัพธ์! (อาจจะถูกดักจับ!) 🚨");
            println!("💔 ความลับของจักรวาลยังคงเป็นปริศนา... 🌌❓");
        }
    });

    println!("⏳🎭 รอให้การทดลองและการสืบสวนเสร็จสิ้น... (ประวัติศาสตร์กำลังเกิดขึ้น!) 📚✨");
    let _ = tokio::join!(sender_task, receiver_task);
    println!("🎊🏆 ภารกิจจดหมายด่วนพิเศษเสร็จสิ้น! ความจริงถูกเปิดเผยแล้ว! 🌈🔮");
}

/// ตัวอย่างการใช้ broadcast channels - หอกระจายข่าวแห่งอนาคต! 📢⚡
pub async fn broadcast_example() {
    println!("\n📢🌟 === ตัวอย่าง Broadcast Channels: หอกระจายข่าวแห่งอนาคต! === 🌟📢");
    println!("📻 เตรียมพบกับสถานีวิทยุที่ส่งข่าวไปทั่วจักรวาล! (ทุกคนได้ฟังพร้อมกัน!) 🌌✨");

    let (tx, _) = broadcast::channel::<String>(10); // เครื่องส่งสัญญาณแรงสูง! 📡🔋

    // สร้าง multiple subscribers - ทีมผู้ฟังข่าวมืออาชีพ! 👂🎧
    let mut subscribers = Vec::new();
    println!("🎧 เรียกทีมผู้ฟังข่าวมืออาชีพ! ทั้งหมด 3 คน! 👥📻");

    for i in 1..=3 {
        let mut rx = tx.subscribe();
        let subscriber = tokio::spawn(async move {
            println!("👂🎧 [ผู้ฟัง{i}] เปิดวิทยุแล้ว! พร้อมฟังข่าวสาร! (หูแหลมมาก!) 🦻✨");

            let mut news_count = 0;
            while let Ok(message) = rx.recv().await {
                news_count += 1;
                println!("📥📻 [ผู้ฟัง{i}] ได้ยินข่าวที่ {news_count}: {message} (จดบันทึกไว้แล้ว!) 📝");
                sleep(Duration::from_millis(100)).await; // ย่อยข่าวในใจ! 🧠💭
            }

            println!("👂🔇 [ผู้ฟัง{i}] ปิดวิทยุแล้ว! ฟังข่าวครบ {news_count} ข่าว! (หูพอแล้ว!) 🎧💤");
        });
        subscribers.push(subscriber);
    }

    // Broadcaster task - ดีเจข่าวสารแห่งอนาคต! 🎙️👨‍💼
    let broadcaster = tokio::spawn(async move {
        println!("🎙️👨‍💼 ดีเจข่าวสารเข้าสู่ห้องออกอากาศ! เตรียมส่งข่าวสำคัญ 5 ข่าว! 📺✨");
        for i in 1..=5 {
            let message = format!("🌟 ข่าวด่วนที่ {i}: มีการค้นพบความลับของจักรวาล! (ตื่นเต้นมาก!) 🚀🔍");
            println!("📢🎙️ Broadcast ข่าวสำคัญ: {message}");

            if tx.send(message).is_err() {
                println!("❌📻 โอ๊ะโอ! ไม่มีใครฟัง! (อาจจะไฟฟ้าดับ!) ⚡💔");
                break;
            }

            sleep(Duration::from_millis(400)).await; // เตรียมข่าวต่อไป! 📰✍️
        }
        println!("📢🎉 ดีเจข่าวสารเสร็จงาน! ส่งข่าวครบแล้ว! (ได้เรตติ้งสูง!) 📊🏆");
    });

    // รอให้ broadcaster เสร็จก่อน - รอให้รายการจบ! 📺
    println!("⏳📺 รอให้รายการข่าวจบ... (อย่าเปลี่ยนช่อง!) 📻");
    let _ = broadcaster.await;

    // รอให้ subscribers ประมวลผลข้อความที่เหลือ - ให้เวลาย่อยข่าว! 🧠
    println!("💭 ให้เวลาผู้ฟังย่อยข่าวในใจ... (ข่าวสำคัญต้องคิดให้ดี!) 🤔");
    sleep(Duration::from_millis(500)).await;

    // รอให้ทุก subscribers เสร็จ - รอให้ทุกคนปิดวิทยุ! 📻
    println!("🔇 รอให้ทุกคนปิดวิทยุ... (จบรายการแล้ว!) 📺");
    for (i, subscriber) in subscribers.into_iter().enumerate() {
        let _ = subscriber.await;
        println!("✅ ผู้ฟัง{} ปิดวิทยุแล้ว! 📻💤", i + 1);
    }
    println!("🎊📻 สถานีวิทยุแห่งอนาคตปิดการออกอากาศ! ทุกคนได้ข่าวครบแล้ว! 🌈✨");
}

/// ตัวอย่างการใช้ channels สำหรับ coordination - ผู้จัดการทีมแห่งอนาคต! 🤝⚡
pub async fn coordination_example() {
    println!("\n🤝🎯 === ตัวอย่าง Task Coordination: ผู้จัดการทีมแห่งอนาคต! === 🎯🤝");
    println!("👔 เตรียมพบกับการจัดการทีมงานที่ลงตัวที่สุด! (ไม่มีใครงานล่าช้า!) 🏢✨");

    let (start_tx, _start_rx) = broadcast::channel::<()>(1); // นกหวีดเริ่มงาน! 📯
    let (done_tx, mut done_rx) = mpsc::channel::<String>(10); // กล่องรายงานผล! 📋

    // สร้าง worker tasks - เรียกทีมงานมืออาชีพ! 👷‍♂️👷‍♀️
    let mut workers = Vec::new();
    println!("👥 เรียกทีมงานมืออาชีพ! ทั้งหมด 3 คน! พร้อมรับมือทุกภารกิจ! 💪");

    for i in 1..=3 {
        let mut start_signal = start_tx.subscribe();
        let done_sender = done_tx.clone();

        let worker = tokio::spawn(async move {
            println!("👷‍♂️ [พนักงาน{i}] มาถึงที่ทำงานแล้ว! รอสัญญาณเริ่มงาน... (กาแฟพร้อม!) ☕👀");

            // รอสัญญาณเริ่มงาน - รอเสียงนกหวีด! 📯
            if start_signal.recv().await.is_ok() {
                println!("🚀💨 [พนักงาน{i}] ได้ยินนกหวีด! เริ่มทำงานเต็มสปีด! (โหมดเทอร์โบ!) ⚡");

                // จำลองการทำงาน - ทำงานอย่างมีความสุข! 😊
                let work_time = 200 + i * 100;
                println!("⏳🔧 [พนักงาน{i}] กำลังทำงาน... คาดว่าใช้เวลา {work_time}ms (ทำด้วยใจรัก!) 💝");
                sleep(Duration::from_millis(work_time)).await;

                let result = format!("🏆 พนักงาน{i} ทำงานเสร็จแล้ว! (ใช้เวลา {work_time}ms) ผลงานคุณภาพเยี่ยม! ✨");
                println!("✅🎉 [พนักงาน{i}] {result}");

                // ส่งสัญญาณว่าเสร็จแล้ว - รายงานผลงาน! 📊
                if done_sender.send(result).await.is_err() {
                    println!("❌📋 [พนักงาน{i}] โอ๊ะโอ! รายงานผลไม่ได้! (อาจจะผู้จัดการไปกินข้าว!) 🍽️");
                }
            }
        });

        workers.push(worker);
    }

    // ปิด done_tx ต้นฉบับ - ผู้จัดการปิดกล่องรายงาน! 📋🔒
    drop(done_tx);
    println!("📋🔒 ผู้จัดการปิดกล่องรายงานแล้ว! รอรับผลงานจากทีม!");

    // รอสักครู่แล้วส่งสัญญาณเริ่มงาน - ให้เวลาเตรียมตัว! ⏰
    println!("⏰ ให้เวลาทีมงานเตรียมตัว... (ดื่มกาแฟก่อน!) ☕");
    sleep(Duration::from_millis(500)).await;
    println!("📣📯 ผู้จัดการเป่านกหวีด! เริ่มงานได้! (ทุกคนพร้อมแล้ว!) 🎺");
    let _ = start_tx.send(());

    // รอรับผลลัพธ์จาก workers - รอรับรายงานผลงาน! 📊
    println!("📊👀 ผู้จัดการรอรับรายงานผลงาน... (ตื่นเต้นมาก!) 💓");
    let mut completed = 0;
    while let Some(result) = done_rx.recv().await {
        completed += 1;
        println!("📊🎉 [รายงานที่ {completed}] {result} (ภูมิใจมาก!) 🥳");
    }

    // รอให้ทุก workers เสร็จ - รอให้ทุกคนกลับบ้าน! 🏠
    println!("⏳🏠 รอให้ทุกคนเก็บของกลับบ้าน... (อย่าลืมของ!) 🎒");
    for (i, worker) in workers.into_iter().enumerate() {
        let _ = worker.await;
        println!("✅🚪 พนักงาน{} กลับบ้านแล้ว! (ขับรถระวังนะ!) 🚗💨", i + 1);
    }

    println!("🎉🏆 ทุกคนเสร็จงานแล้ว! ทีมงานที่ยอดเยี่ยม! (ได้โบนัสแน่นอน!) 💰✨");
}

/// ฟังก์ชันรวมตัวอย่างทั้งหมด - เปิดโรงเรียนสอนการสื่อสารแห่งอนาคต! 🎓📡
pub async fn run_channels_examples() {
    println!("\n🎓📡 === โรงเรียนสอนการสื่อสารแห่งอนาคต! === 📡🎓");
    println!("🌟 ยินดีต้อนรับสู่หลักสูตรการสื่อสารขั้นเทพ! (รับประกันเก่งแน่นอน!) 🏆✨");

    println!("\n📚 บทเรียนที่ 1: ไปรษณีย์แห่งอนาคต! 📮⚡");
    channels_example().await;
    
    println!("\n📚 บทเรียนที่ 2: ทีมบุรุษไปรษณีย์ซูเปอร์! 👥📮⚡");
    multiple_producers_example().await;
    
    println!("\n📚 บทเรียนที่ 3: จดหมายด่วนพิเศษ! 🎯⚡");
    oneshot_example().await;
    
    println!("\n📚 บทเรียนที่ 4: หอกระจายข่าวแห่งอนาคต! 📢⚡");
    broadcast_example().await;
    
    println!("\n📚 บทเรียนที่ 5: ผู้จัดการทีมแห่งอนาคต! 🤝⚡");
    coordination_example().await;

    println!("\n🎉🏆 === จบหลักสูตรแล้ว! คุณเป็นมาสเตอร์การสื่อสารแล้ว! === 🏆🎉");
    println!("\n💡🌟 สิ่งที่คุณเรียนรู้ได้ (ทักษะใหม่ที่เจ๋งมาก!):");
    println!("   📮⚡ การใช้ mpsc channels - ไปรษณีย์หลายคนส่ง คนเดียวรับ!");
    println!("   🎯⚡ การใช้ oneshot channels - จดหมายด่วนพิเศษ!");
    println!("   📢⚡ การใช้ broadcast channels - หอกระจายข่าวสุดเจ๋ง!");
    println!("   🤝⚡ การ coordinate tasks ด้วย channels - จัดการทีมอย่างมืออาชีพ!");
    println!("\n🌈✨ ขอแสดงความยินดี! คุณพร้อมสร้างแอปพลิเคชันสื่อสารแห่งอนาคตแล้ว! 🚀🎊");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_mpsc_channel() {
        let (tx, mut rx) = mpsc::channel::<i32>(5);

        // ส่งข้อมูล
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();

        // ปิด sender
        drop(tx);

        // รับข้อมูล
        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(rx.recv().await, Some(2));
        assert_eq!(rx.recv().await, Some(3));
        assert_eq!(rx.recv().await, None); // channel ปิดแล้ว
    }

    #[test]
    async fn test_oneshot_channel() {
        let (tx, rx) = oneshot::channel::<String>();

        // ส่งข้อมูล
        tx.send("test message".to_string()).unwrap();

        // รับข้อมูล
        let result = rx.await.unwrap();
        assert_eq!(result, "test message");
    }

    #[test]
    async fn test_broadcast_channel() {
        let (tx, mut rx1) = broadcast::channel::<i32>(5);
        let mut rx2 = tx.subscribe();

        // ส่งข้อมูล
        tx.send(42).unwrap();
        tx.send(84).unwrap();

        // ทั้งสอง receivers ควรได้รับข้อมูลเดียวกัน
        assert_eq!(rx1.recv().await.unwrap(), 42);
        assert_eq!(rx2.recv().await.unwrap(), 42);

        assert_eq!(rx1.recv().await.unwrap(), 84);
        assert_eq!(rx2.recv().await.unwrap(), 84);
    }

    #[test]
    async fn test_multiple_producers() {
        let (tx, mut rx) = mpsc::channel::<i32>(10);

        // สร้าง 2 producers
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        let producer1 = tokio::spawn(async move {
            tx1.send(1).await.unwrap();
            tx1.send(2).await.unwrap();
        });

        let producer2 = tokio::spawn(async move {
            tx2.send(10).await.unwrap();
            tx2.send(20).await.unwrap();
        });

        // ปิด original sender
        drop(tx);

        // รอให้ producers เสร็จ
        let _ = tokio::join!(producer1, producer2);

        // รับข้อมูลทั้งหมด
        let mut received = Vec::new();
        while let Some(value) = rx.recv().await {
            received.push(value);
        }

        // ควรได้รับข้อมูล 4 ตัว
        assert_eq!(received.len(), 4);

        // เรียงลำดับเพื่อเปรียบเทียบ
        received.sort_unstable();
        assert_eq!(received, vec![1, 2, 10, 20]);
    }
}
