//! Integration Tests Example
//! แสดงวิธีการเขียน integration tests ที่ทดสอบการทำงานของระบบทั้งหมด

use rust_concepts::*;

use tempfile::TempDir;

/// ทดสอบการทำงานของ Task Manager แบบ end-to-end
#[test]
fn test_task_manager_integration() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("tasks.json");

    // สร้าง TaskManager ใหม่
    let mut manager = TaskManager::new(file_path.clone());

    // เพิ่ม tasks
    let task1_id = manager.add_task("Complete project".to_string(), "High".to_string());
    let task2_id = manager.add_task("Review code".to_string(), "Medium".to_string());

    // ตรวจสอบว่ามี tasks ทั้งหมด 2 รายการ
    assert_eq!(manager.list_tasks().len(), 2);

    // บันทึกลงไฟล์
    manager.save_to_file().unwrap();

    // ตรวจสอบว่าไฟล์ถูกสร้างขึ้น
    assert!(file_path.exists());

    // สร้าง TaskManager ใหม่และโหลดจากไฟล์
    let mut new_manager = TaskManager::new(file_path);
    new_manager.load_from_file().unwrap();

    // ตรวจสอบว่าข้อมูลถูกโหลดมาถูกต้อง
    assert_eq!(new_manager.list_tasks().len(), 2);

    // ทำเครื่องหมายว่าเสร็จแล้ว
    new_manager.complete_task(&task1_id).unwrap();

    // ตรวจสอบสถานะ
    let tasks = new_manager.list_tasks();
    let completed_task = tasks.iter().find(|t| t.id == task1_id).unwrap();
    assert!(completed_task.completed);

    // ลบ task
    new_manager.remove_task(&task2_id).unwrap();
    assert_eq!(new_manager.list_tasks().len(), 1);
}

/// ทดสอบการจัดการ error ในสถานการณ์ต่างๆ
#[test]
fn test_error_handling_integration() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent_dir").join("tasks.json");

    let mut manager = TaskManager::new(file_path);

    // ทดสอบการบันทึกไฟล์ในโฟลเดอร์ที่ไม่มีอยู่
    let result = manager.save_to_file();
    assert!(result.is_err());

    // ทดสอบการลบ task ที่ไม่มีอยู่
    let fake_id = uuid::Uuid::new_v4();
    let result = manager.remove_task(&fake_id);
    assert!(result.is_err());

    // ทดสอบการทำเครื่องหมายเสร็จสำหรับ task ที่ไม่มีอยู่
    let result = manager.complete_task(&fake_id);
    assert!(result.is_err());
}

/// ทดสอบ performance ของ collections
#[test]
fn test_collections_performance() {
    use std::collections::HashMap;
    use std::time::Instant;

    let start = Instant::now();

    // ทดสอบ Vec operations
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }

    let vec_time = start.elapsed();

    let start = Instant::now();

    // ทดสอบ HashMap operations
    let mut map = HashMap::new();
    for i in 0..10000 {
        map.insert(i, i.to_string());
    }

    let map_time = start.elapsed();

    // ตรวจสอบว่าการดำเนินการเสร็จสิ้นภายในเวลาที่กำหนด (1 วินาที)
    assert!(vec_time.as_secs() < 1);
    assert!(map_time.as_secs() < 1);

    println!("Vec operations took: {vec_time:?}");
    println!("HashMap operations took: {map_time:?}");
}

/// ทดสอบ concurrent operations
#[tokio::test]
async fn test_concurrent_operations() {
    use std::sync::{Arc, Mutex};
    use tokio::task;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // สร้าง 10 tasks ที่ทำงานพร้อมกัน
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = task::spawn(async move {
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // รอให้ทุก tasks เสร็จสิ้น
    for handle in handles {
        handle.await.unwrap();
    }

    // ตรวจสอบผลลัพธ์
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 1000);
}

/// ทดสอบ memory usage และ cleanup
#[test]
fn test_memory_management() {
    use std::mem;

    // ทดสอบ memory usage ของ structures ต่างๆ
    let task = Task {
        id: uuid::Uuid::new_v4(),
        title: "Test task".to_string(),
        priority: "High".to_string(),
        completed: false,
        created_at: chrono::Utc::now(),
    };

    // ตรวจสอบขนาดของ struct
    let task_size = mem::size_of_val(&task);
    println!("Task size: {task_size} bytes");

    // ทดสอบการใช้ memory กับ Vec ขนาดใหญ่
    let large_vec: Vec<i32> = (0..1_000_000).collect();
    let vec_size = mem::size_of_val(&large_vec) + large_vec.capacity() * mem::size_of::<i32>();
    println!("Large Vec size: {vec_size} bytes");

    // ตรวจสอบว่า memory ไม่เกิน 100MB
    assert!(vec_size < 100_000_000);

    // ทดสอบการ drop และ cleanup
    drop(large_vec);
    // หลังจาก drop แล้ว memory ควรถูกคืนให้ระบบ
}

/// ทดสอบ serialization และ deserialization
#[test]
fn test_serialization_integration() {
    let original_task = Task {
        id: uuid::Uuid::new_v4(),
        title: "Test serialization".to_string(),
        priority: "Medium".to_string(),
        completed: true,
        created_at: chrono::Utc::now(),
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&original_task).unwrap();
    println!("Serialized JSON: {json_str}");

    // Deserialize from JSON
    let deserialized_task: Task = serde_json::from_str(&json_str).unwrap();

    // ตรวจสอบว่าข้อมูลถูกต้อง
    assert_eq!(original_task.id, deserialized_task.id);
    assert_eq!(original_task.title, deserialized_task.title);
    assert_eq!(original_task.priority, deserialized_task.priority);
    assert_eq!(original_task.completed, deserialized_task.completed);

    // ทดสอบ pretty printing
    let pretty_json = serde_json::to_string_pretty(&original_task).unwrap();
    println!("Pretty JSON:\n{pretty_json}");
}

/// ทดสอบ configuration และ environment variables
#[test]
fn test_configuration() {
    use std::env;

    // ตั้งค่า environment variable
    unsafe {
        env::set_var("RUST_CONCEPTS_DEBUG", "true");
        env::set_var("RUST_CONCEPTS_MAX_TASKS", "1000");
    }

    // อ่านค่า configuration
    let debug_mode = env::var("RUST_CONCEPTS_DEBUG").unwrap_or_else(|_| "false".to_string());
    let max_tasks = env::var("RUST_CONCEPTS_MAX_TASKS")
        .unwrap_or_else(|_| "100".to_string())
        .parse::<usize>()
        .unwrap_or(100);

    assert_eq!(debug_mode, "true");
    assert_eq!(max_tasks, 1000);

    // ทำความสะอาด environment variables
    unsafe {
        env::remove_var("RUST_CONCEPTS_DEBUG");
        env::remove_var("RUST_CONCEPTS_MAX_TASKS");
    }
}

/// Helper function สำหรับการทดสอบ
fn create_sample_tasks() -> Vec<Task> {
    vec![
        Task {
            id: uuid::Uuid::new_v4(),
            title: "Task 1".to_string(),
            priority: "High".to_string(),
            completed: false,
            created_at: chrono::Utc::now(),
        },
        Task {
            id: uuid::Uuid::new_v4(),
            title: "Task 2".to_string(),
            priority: "Medium".to_string(),
            completed: true,
            created_at: chrono::Utc::now(),
        },
    ]
}

/// ทดสอบ helper functions
#[test]
fn test_helper_functions() {
    let tasks = create_sample_tasks();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].title, "Task 1");
    assert!(tasks[1].completed);
}
