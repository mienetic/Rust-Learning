//! แบบฝึกหัด Advanced Patterns - ห้องปฏิบัติการรูปแบบขั้นสูง! 🎯🔧
//! ที่นี่เราจะฝึกใช้ design patterns และเทคนิคขั้นสูงของ Rust!

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

/// ระบบจัดการงาน (Task Management System) - โปรเจกต์ขั้นสูง! 📋🚀
/// ใช้หลาย patterns รวมกัน: Builder, Observer, State, Strategy

// ===== Builder Pattern สำหรับ Task ===== 🏗️

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "ต่ำ 🟢"),
            Self::Medium => write!(f, "ปานกลาง 🟡"),
            Self::High => write!(f, "สูง 🟠"),
            Self::Critical => write!(f, "วิกฤต 🔴"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
    Cancelled,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo => write!(f, "รอดำเนินการ 📝"),
            Self::InProgress => write!(f, "กำลังทำ ⚡"),
            Self::Review => write!(f, "รอตรวจสอบ 👀"),
            Self::Done => write!(f, "เสร็จแล้ว ✅"),
            Self::Cancelled => write!(f, "ยกเลิก ❌"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub assignee: Option<String>,
    pub estimated_hours: Option<u32>,
    pub tags: Vec<String>,
}

/// Builder Pattern สำหรับสร้าง Task - สร้างงานแบบขั้นตอน! 🏗️✨
#[derive(Debug, Clone)]
pub struct TaskBuilder {
    id: Option<u32>,
    title: Option<String>,
    description: Option<String>,
    priority: TaskPriority,
    status: TaskStatus,
    assignee: Option<String>,
    estimated_hours: Option<u32>,
    tags: Vec<String>,
}

impl Default for TaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskBuilder {
    #[must_use] pub const fn new() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            priority: TaskPriority::Medium,
            status: TaskStatus::Todo,
            assignee: None,
            estimated_hours: None,
            tags: Vec::new(),
        }
    }
    
    #[must_use] pub const fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }
    
    #[must_use] pub const fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
    
    #[must_use] pub const fn status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }
    
    pub fn assignee<S: Into<String>>(mut self, assignee: S) -> Self {
        self.assignee = Some(assignee.into());
        self
    }
    
    #[must_use] pub const fn estimated_hours(mut self, hours: u32) -> Self {
        self.estimated_hours = Some(hours);
        self
    }
    
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    pub fn tags<I, S>(mut self, tags: I) -> Self 
    where 
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags.extend(tags.into_iter().map(std::convert::Into::into));
        self
    }
    
    pub fn build(self) -> Result<Task, String> {
        let id = self.id.ok_or("ID is required")?;
        let title = self.title.ok_or("Title is required")?;
        let description = self.description.unwrap_or_default();
        
        Ok(Task {
            id,
            title,
            description,
            priority: self.priority,
            status: self.status,
            assignee: self.assignee,
            estimated_hours: self.estimated_hours,
            tags: self.tags,
        })
    }
}

// ===== Observer Pattern สำหรับ Task Events ===== 👀📢

pub trait TaskObserver {
    fn on_task_created(&self, task: &Task);
    fn on_task_updated(&self, old_task: &Task, new_task: &Task);
    fn on_task_deleted(&self, task: &Task);
}

/// Logger Observer - บันทึกเหตุการณ์! 📝
pub struct TaskLogger;

impl TaskObserver for TaskLogger {
    fn on_task_created(&self, task: &Task) {
        println!("📝 [LOG] งานใหม่ถูกสร้าง: {} (ID: {}) - {}", 
            task.title, task.id, task.priority);
    }
    
    fn on_task_updated(&self, old_task: &Task, new_task: &Task) {
        println!("📝 [LOG] งานถูกอัปเดต: {} (ID: {}) - {} -> {}", 
            new_task.title, new_task.id, old_task.status, new_task.status);
    }
    
    fn on_task_deleted(&self, task: &Task) {
        println!("📝 [LOG] งานถูกลบ: {} (ID: {})", task.title, task.id);
    }
}

/// Notification Observer - ส่งการแจ้งเตือน! 🔔
pub struct TaskNotifier;

impl TaskObserver for TaskNotifier {
    fn on_task_created(&self, task: &Task) {
        if task.priority == TaskPriority::Critical {
            println!("🚨 [ALERT] งานวิกฤตถูกสร้าง: {}! ต้องดำเนินการทันที!", task.title);
        }
    }
    
    fn on_task_updated(&self, _old_task: &Task, new_task: &Task) {
        if new_task.status == TaskStatus::Done {
            println!("🎉 [NOTIFICATION] ยินดีด้วย! งาน '{}' เสร็จสิ้นแล้ว!", new_task.title);
        }
    }
    
    fn on_task_deleted(&self, task: &Task) {
        println!("🗑️ [NOTIFICATION] งาน '{}' ถูกลบออกจากระบบ", task.title);
    }
}

// ===== Strategy Pattern สำหรับ Task Sorting ===== 🎯📊

pub trait SortStrategy {
    fn sort(&self, tasks: &mut Vec<Task>);
    fn name(&self) -> &'static str;
}

/// เรียงตาม Priority - งานสำคัญก่อน! 🔥
pub struct PrioritySorter;

impl SortStrategy for PrioritySorter {
    fn sort(&self, tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| {
            use TaskPriority::{Critical, High, Medium, Low};
            let priority_order = |p: &TaskPriority| match p {
                Critical => 0,
                High => 1,
                Medium => 2,
                Low => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });
    }
    
    fn name(&self) -> &'static str {
        "เรียงตามความสำคัญ"
    }
}

/// เรียงตาม Status - งานที่กำลังทำก่อน! ⚡
pub struct StatusSorter;

impl SortStrategy for StatusSorter {
    fn sort(&self, tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| {
            use TaskStatus::{InProgress, Review, Todo, Done, Cancelled};
            let status_order = |s: &TaskStatus| match s {
                InProgress => 0,
                Review => 1,
                Todo => 2,
                Done => 3,
                Cancelled => 4,
            };
            status_order(&a.status).cmp(&status_order(&b.status))
        });
    }
    
    fn name(&self) -> &'static str {
        "เรียงตามสถานะ"
    }
}

/// เรียงตาม Estimated Hours - งานเร็วก่อน! ⏱️
pub struct DurationSorter;

impl SortStrategy for DurationSorter {
    fn sort(&self, tasks: &mut Vec<Task>) {
        tasks.sort_by(|a, b| {
            let hours_a = a.estimated_hours.unwrap_or(u32::MAX);
            let hours_b = b.estimated_hours.unwrap_or(u32::MAX);
            hours_a.cmp(&hours_b)
        });
    }
    
    fn name(&self) -> &'static str {
        "เรียงตามระยะเวลา"
    }
}

// ===== State Pattern สำหรับ Task Workflow ===== 🔄⚙️

pub trait TaskState {
    fn can_transition_to(&self, new_status: &TaskStatus) -> bool;
    fn get_allowed_transitions(&self) -> Vec<TaskStatus>;
    fn get_current_status(&self) -> TaskStatus;
}

pub struct TodoState;
pub struct InProgressState;
pub struct ReviewState;
pub struct DoneState;
pub struct CancelledState;

impl TaskState for TodoState {
    fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        matches!(new_status, TaskStatus::InProgress | TaskStatus::Cancelled)
    }
    
    fn get_allowed_transitions(&self) -> Vec<TaskStatus> {
        vec![TaskStatus::InProgress, TaskStatus::Cancelled]
    }
    
    fn get_current_status(&self) -> TaskStatus {
        TaskStatus::Todo
    }
}

impl TaskState for InProgressState {
    fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        matches!(new_status, TaskStatus::Review | TaskStatus::Done | TaskStatus::Cancelled)
    }
    
    fn get_allowed_transitions(&self) -> Vec<TaskStatus> {
        vec![TaskStatus::Review, TaskStatus::Done, TaskStatus::Cancelled]
    }
    
    fn get_current_status(&self) -> TaskStatus {
        TaskStatus::InProgress
    }
}

impl TaskState for ReviewState {
    fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        matches!(new_status, TaskStatus::Done | TaskStatus::InProgress)
    }
    
    fn get_allowed_transitions(&self) -> Vec<TaskStatus> {
        vec![TaskStatus::Done, TaskStatus::InProgress]
    }
    
    fn get_current_status(&self) -> TaskStatus {
        TaskStatus::Review
    }
}

impl TaskState for DoneState {
    fn can_transition_to(&self, _new_status: &TaskStatus) -> bool {
        false // งานเสร็จแล้วไม่สามารถเปลี่ยนสถานะได้!
    }
    
    fn get_allowed_transitions(&self) -> Vec<TaskStatus> {
        vec![] // ไม่มีการเปลี่ยนแปลงได้
    }
    
    fn get_current_status(&self) -> TaskStatus {
        TaskStatus::Done
    }
}

impl TaskState for CancelledState {
    fn can_transition_to(&self, _new_status: &TaskStatus) -> bool {
        false // งานที่ยกเลิกแล้วไม่สามารถเปลี่ยนสถานะได้!
    }
    
    fn get_allowed_transitions(&self) -> Vec<TaskStatus> {
        vec![] // ไม่มีการเปลี่ยนแปลงได้
    }
    
    fn get_current_status(&self) -> TaskStatus {
        TaskStatus::Cancelled
    }
}

// ===== Task Manager ที่ใช้ทุก Patterns ===== 🎯🚀

pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    observers: Vec<Box<dyn TaskObserver>>,
    next_id: u32,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    #[must_use] pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            observers: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_observer(&mut self, observer: Box<dyn TaskObserver>) {
        self.observers.push(observer);
    }
    
    pub fn create_task(&mut self, builder: TaskBuilder) -> Result<u32, String> {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = builder.id(id).build()?;
        
        // แจ้งเตือน observers
        for observer in &self.observers {
            observer.on_task_created(&task);
        }
        
        self.tasks.insert(id, task);
        Ok(id)
    }
    
    pub fn update_task_status(&mut self, id: u32, new_status: TaskStatus) -> Result<(), String> {
        let task = self.tasks.get_mut(&id).ok_or("Task not found")?;
        
        // ตรวจสอบว่าสามารถเปลี่ยนสถานะได้หรือไม่
        let can_transition = match &task.status {
            TaskStatus::Todo => TodoState.can_transition_to(&new_status),
            TaskStatus::InProgress => InProgressState.can_transition_to(&new_status),
            TaskStatus::Review => ReviewState.can_transition_to(&new_status),
            TaskStatus::Done => DoneState.can_transition_to(&new_status),
            TaskStatus::Cancelled => CancelledState.can_transition_to(&new_status),
        };
        
        if !can_transition {
            return Err(format!("Cannot transition from {} to {}", task.status, new_status));
        }
        
        let old_task = task.clone();
        task.status = new_status;
        
        // แจ้งเตือน observers
        for observer in &self.observers {
            observer.on_task_updated(&old_task, task);
        }
        
        Ok(())
    }
    
    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        let task = self.tasks.remove(&id).ok_or("Task not found")?;
        
        // แจ้งเตือน observers
        for observer in &self.observers {
            observer.on_task_deleted(&task);
        }
        
        Ok(())
    }
    
    pub fn get_tasks_sorted(&self, strategy: &dyn SortStrategy) -> Vec<Task> {
        let mut tasks: Vec<Task> = self.tasks.values().cloned().collect();
        strategy.sort(&mut tasks);
        tasks
    }
    
    #[must_use] pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }
    
    #[must_use] pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    #[must_use] pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.values().filter(|task| task.status == status).collect()
    }
    
    #[must_use] pub fn get_tasks_by_assignee(&self, assignee: &str) -> Vec<&Task> {
        self.tasks.values()
            .filter(|task| task.assignee.as_ref().is_some_and(|a| a == assignee))
            .collect()
    }
}

// ===== Phantom Types สำหรับ Type Safety ===== 👻🔒

/// Phantom Types สำหรับระบุประเภทของ Task ในระดับ Type System
pub struct Validated;
pub struct Unvalidated;

/// Task ที่มี Type State - ความปลอดภัยในระดับ Type! 👻✨
pub struct TypedTask<T> {
    task: Task,
    _phantom: PhantomData<T>,
}

impl TypedTask<Unvalidated> {
    #[must_use] pub const fn new(task: Task) -> Self {
        Self {
            task,
            _phantom: PhantomData,
        }
    }
    
    pub fn validate(self) -> Result<TypedTask<Validated>, String> {
        if self.task.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        
        if self.task.assignee.is_none() && self.task.priority == TaskPriority::Critical {
            return Err("Critical tasks must have an assignee".to_string());
        }
        
        Ok(TypedTask {
            task: self.task,
            _phantom: PhantomData,
        })
    }
}

impl TypedTask<Validated> {
    #[must_use] pub fn execute(&self) -> String {
        format!("Executing validated task: {}", self.task.title)
    }
    
    #[must_use] pub const fn get_task(&self) -> &Task {
        &self.task
    }
}

// ===== Zero-Cost Abstractions Example ===== 💨⚡

/// Iterator ที่มีประสิทธิภาพสูง - Zero-Cost Abstraction! 💨
pub struct TaskFilter<I> {
    iter: I,
    priority_filter: Option<TaskPriority>,
    status_filter: Option<TaskStatus>,
}

impl<I> TaskFilter<I>
where
    I: Iterator<Item = Task>,
{
    pub const fn new(iter: I) -> Self {
        Self {
            iter,
            priority_filter: None,
            status_filter: None,
        }
    }
    
    pub const fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority_filter = Some(priority);
        self
    }
    
    pub const fn with_status(mut self, status: TaskStatus) -> Self {
        self.status_filter = Some(status);
        self
    }
}

impl<I> Iterator for TaskFilter<I>
where
    I: Iterator<Item = Task>,
{
    type Item = Task;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let task = self.iter.next()?;
            
            if let Some(ref priority) = self.priority_filter {
                if task.priority != *priority {
                    continue;
                }
            }
            
            if let Some(ref status) = self.status_filter {
                if task.status != *status {
                    continue;
                }
            }
            
            return Some(task);
        }
    }
}

// ===== Command Pattern สำหรับ Undo/Redo ===== ↩️↪️

pub trait Command {
    fn execute(&mut self, manager: &mut TaskManager) -> Result<(), String>;
    fn undo(&mut self, manager: &mut TaskManager) -> Result<(), String>;
    fn description(&self) -> String;
}

pub struct CreateTaskCommand {
    task_builder: TaskBuilder,
    created_id: Option<u32>,
    executed: bool,
}

impl CreateTaskCommand {
    #[must_use] pub const fn new(builder: TaskBuilder) -> Self {
        Self {
            task_builder: builder,
            created_id: None,
            executed: false,
        }
    }
}

impl Command for CreateTaskCommand {
    fn execute(&mut self, manager: &mut TaskManager) -> Result<(), String> {
        // Always create a new task when execute is called
        let id = manager.create_task(self.task_builder.clone())?;
        self.created_id = Some(id);
        self.executed = true;
        Ok(())
    }
    
    fn undo(&mut self, manager: &mut TaskManager) -> Result<(), String> {
        if let Some(id) = self.created_id {
            manager.delete_task(id)?;
            self.created_id = None;
            Ok(())
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    
    fn description(&self) -> String {
        "Create Task".to_string()
    }
}

pub struct UpdateTaskStatusCommand {
    task_id: u32,
    new_status: TaskStatus,
    old_status: Option<TaskStatus>,
}

impl UpdateTaskStatusCommand {
    #[must_use] pub const fn new(task_id: u32, new_status: TaskStatus) -> Self {
        Self {
            task_id,
            new_status,
            old_status: None,
        }
    }
}

impl Command for UpdateTaskStatusCommand {
    fn execute(&mut self, manager: &mut TaskManager) -> Result<(), String> {
        if let Some(task) = manager.get_task(self.task_id) {
            self.old_status = Some(task.status.clone());
        }
        manager.update_task_status(self.task_id, self.new_status.clone())
    }
    
    fn undo(&mut self, manager: &mut TaskManager) -> Result<(), String> {
        if let Some(old_status) = self.old_status.take() {
            manager.update_task_status(self.task_id, old_status)
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    
    fn description(&self) -> String {
        format!("Update Task {} Status", self.task_id)
    }
}

/// Command History สำหรับ Undo/Redo - เครื่องเวลา! ⏰🔄
pub struct CommandHistory {
    commands: Vec<Box<dyn Command>>,
    current_index: Option<usize>,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHistory {
    #[must_use] pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_index: None,
        }
    }
    
    pub fn execute_command(&mut self, mut command: Box<dyn Command>, manager: &mut TaskManager) -> Result<(), String> {
        command.execute(manager)?;
        
        // ลบ commands ที่อยู่หลัง current index (สำหรับ branching)
        if let Some(index) = self.current_index {
            self.commands.truncate(index + 1);
        }
        
        self.commands.push(command);
        self.current_index = Some(self.commands.len() - 1);
        
        Ok(())
    }
    
    pub fn undo(&mut self, manager: &mut TaskManager) -> Result<String, String> {
        if let Some(index) = self.current_index {
            let command = &mut self.commands[index];
            let description = command.description();
            command.undo(manager)?;
            
            self.current_index = if index > 0 { Some(index - 1) } else { None };
            
            Ok(format!("Undid: {description}"))
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    
    pub fn redo(&mut self, manager: &mut TaskManager) -> Result<String, String> {
        let next_index = match self.current_index {
            Some(index) => index + 1,
            None => 0,
        };
        
        if next_index < self.commands.len() {
            let command = &mut self.commands[next_index];
            let description = command.description();
            command.execute(manager)?;
            
            self.current_index = Some(next_index);
            
            Ok(format!("Redid: {description}"))
        } else {
            Err("Nothing to redo".to_string())
        }
    }
    
    #[must_use] pub fn get_history(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| cmd.description()).collect()
    }
}

// ===== ฟังก์ชันสำหรับทดสอบทุก Patterns ===== 🧪🎯

/// ฟังก์ชันหลักสำหรับทดสอบ Advanced Patterns - โชว์ทุกเทคนิค! 🎭✨
pub fn practice_advanced_patterns() {
    println!("🎯 === แบบฝึกหัด Advanced Patterns: ห้องปฏิบัติการรูปแบบขั้นสูง! === 🎯");
    
    // สร้าง Task Manager และเพิ่ม Observers
    let mut manager = TaskManager::new();
    manager.add_observer(Box::new(TaskLogger));
    manager.add_observer(Box::new(TaskNotifier));
    
    println!("\n🏗️ === Builder Pattern: สร้างงานแบบขั้นตอน! === 🏗️");
    
    // ใช้ Builder Pattern สร้าง Tasks
    let task1_builder = TaskBuilder::new()
        .title("พัฒนา API สำหรับระบบจัดการผู้ใช้")
        .description("สร้าง REST API สำหรับ CRUD operations ของผู้ใช้")
        .priority(TaskPriority::High)
        .assignee("Alice")
        .estimated_hours(8)
        .tags(["backend", "api", "user-management"]);
    
    let task2_builder = TaskBuilder::new()
        .title("แก้ไขบัค UI ในหน้า Dashboard")
        .description("แก้ไขปัญหาการแสดงผลกราฟที่ไม่ถูกต้อง")
        .priority(TaskPriority::Medium)
        .assignee("Bob")
        .estimated_hours(4)
        .tag("frontend")
        .tag("bugfix");
    
    let task3_builder = TaskBuilder::new()
        .title("ระบบรักษาความปลอดภัยล่ม!")
        .description("เซิร์ฟเวอร์ถูกโจมตี ต้องแก้ไขทันที!")
        .priority(TaskPriority::Critical)
        .assignee("Charlie")
        .estimated_hours(2)
        .tags(["security", "urgent", "hotfix"]);
    
    // สร้าง Command History สำหรับ Undo/Redo
    let mut history = CommandHistory::new();
    
    // สร้างงานผ่าน Command Pattern
    println!("\n↩️ === Command Pattern: เครื่องเวลาสำหรับ Undo/Redo! === ↩️");
    
    let create_cmd1 = Box::new(CreateTaskCommand::new(task1_builder));
    let create_cmd2 = Box::new(CreateTaskCommand::new(task2_builder));
    let create_cmd3 = Box::new(CreateTaskCommand::new(task3_builder));
    
    history.execute_command(create_cmd1, &mut manager).unwrap();
    history.execute_command(create_cmd2, &mut manager).unwrap();
    history.execute_command(create_cmd3, &mut manager).unwrap();
    
    println!("\n📊 === Strategy Pattern: เรียงงานแบบต่างๆ! === 📊");
    
    // ทดสอบ Strategy Pattern
    let priority_sorter = PrioritySorter;
    let status_sorter = StatusSorter;
    let duration_sorter = DurationSorter;
    
    println!("\n🔥 เรียงตามความสำคัญ:");
    let tasks_by_priority = manager.get_tasks_sorted(&priority_sorter);
    for task in &tasks_by_priority {
        println!("  📋 {} - {} ({}ชม.)", 
            task.title, task.priority, 
            task.estimated_hours.map_or("?".to_string(), |h| h.to_string()));
    }
    
    println!("\n⚡ เรียงตามสถานะ:");
    let tasks_by_status = manager.get_tasks_sorted(&status_sorter);
    for task in &tasks_by_status {
        println!("  📋 {} - {}", task.title, task.status);
    }
    
    println!("\n⏱️ เรียงตามระยะเวลา:");
    let tasks_by_duration = manager.get_tasks_sorted(&duration_sorter);
    for task in &tasks_by_duration {
        println!("  📋 {} - {}ชม.", 
            task.title, 
            task.estimated_hours.map_or("ไม่ระบุ".to_string(), |h| h.to_string()));
    }
    
    println!("\n🔄 === State Pattern: การเปลี่ยนสถานะงาน! === 🔄");
    
    // ทดสอบ State Pattern
    let update_cmd1 = Box::new(UpdateTaskStatusCommand::new(1, TaskStatus::InProgress));
    let update_cmd2 = Box::new(UpdateTaskStatusCommand::new(2, TaskStatus::InProgress));
    let update_cmd3 = Box::new(UpdateTaskStatusCommand::new(1, TaskStatus::Review));
    let update_cmd4 = Box::new(UpdateTaskStatusCommand::new(1, TaskStatus::Done));
    
    history.execute_command(update_cmd1, &mut manager).unwrap();
    history.execute_command(update_cmd2, &mut manager).unwrap();
    history.execute_command(update_cmd3, &mut manager).unwrap();
    history.execute_command(update_cmd4, &mut manager).unwrap();
    
    // ทดสอบ Undo/Redo
    println!("\n⏰ === Undo/Redo: เครื่องเวลา! === ⏰");
    
    // ทดสอบ Undo อย่างปลอดภัย
    match history.undo(&mut manager) {
        Ok(msg) => println!("🔙 Undo: {}", msg),
        Err(e) => println!("❌ Undo ล้มเหลว: {}", e),
    }
    
    match history.undo(&mut manager) {
        Ok(msg) => println!("🔙 Undo: {}", msg),
        Err(e) => println!("❌ Undo ล้มเหลว: {}", e),
    }
    
    // ทดสอบ Redo อย่างปลอดภัย
    match history.redo(&mut manager) {
        Ok(msg) => println!("🔄 Redo: {}", msg),
        Err(e) => println!("❌ Redo ล้มเหลว: {}", e),
    }
    
    println!("\n👻 === Phantom Types: ความปลอดภัยในระดับ Type! === 👻");
    
    // ทดสอบ Phantom Types
    let unvalidated_task = TypedTask::<Unvalidated>::new(
        TaskBuilder::new()
            .id(999)
            .title("งานทดสอบ Phantom Types")
            .description("ทดสอบระบบ Type Safety")
            .priority(TaskPriority::Low)
            .assignee("Tester")
            .build()
            .unwrap()
    );
    
    match unvalidated_task.validate() {
        Ok(validated_task) => {
            println!("✅ งานผ่านการตรวจสอบ: {}", validated_task.execute());
        }
        Err(e) => {
            println!("❌ งานไม่ผ่านการตรวจสอบ: {e}");
        }
    }
    
    println!("\n💨 === Zero-Cost Abstractions: Iterator ที่มีประสิทธิภาพ! === 💨");
    
    // ทดสอบ Zero-Cost Abstractions
    let all_tasks: Vec<Task> = manager.get_all_tasks().into_iter().cloned().collect();
    let high_priority_tasks: Vec<Task> = TaskFilter::new(all_tasks.into_iter())
        .with_priority(TaskPriority::High)
        .collect();
    
    println!("🔥 งานความสำคัญสูง:");
    for task in high_priority_tasks {
        println!("  📋 {} - {}", task.title, task.priority);
    }
    
    // สรุปสถิติ
    println!("\n📈 === สรุปสถิติระบบจัดการงาน === 📈");
    let all_tasks = manager.get_all_tasks();
    let total_tasks = all_tasks.len();
    let done_tasks = manager.get_tasks_by_status(TaskStatus::Done).len();
    let in_progress_tasks = manager.get_tasks_by_status(TaskStatus::InProgress).len();
    
    println!("📊 งานทั้งหมด: {total_tasks} งาน");
    println!("✅ งานเสร็จแล้ว: {done_tasks} งาน");
    println!("⚡ งานที่กำลังทำ: {in_progress_tasks} งาน");
    println!("📈 ความคืบหน้า: {:.1}%", 
        if total_tasks > 0 { (done_tasks as f64 / total_tasks as f64) * 100.0 } else { 0.0 });
    
    println!("\n🎉 จบแบบฝึกหัด Advanced Patterns! (เป็นสถาปนิกซอฟต์แวร์แล้ว! 🏗️👨‍💻)");
}

/// ตัวอย่างการใช้ Newtype Pattern - ห่อหุ้มเพื่อความปลอดภัย! 📦🔒
pub fn newtype_pattern_example() {
    println!("\n📦 === Newtype Pattern: ห่อหุ้มเพื่อความปลอดภัย! === 📦");
    
    // Newtype สำหรับ Email - ป้องกันการใช้ String ผิด!
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Email(String);
    
    impl Email {
        pub fn new(email: String) -> Result<Self, String> {
            if email.contains('@') && email.contains('.') {
                Ok(Self(email))
            } else {
                Err("Invalid email format".to_string())
            }
        }
        
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
    
    // Newtype สำหรับ UserId - ป้องกันการสับสน ID!
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserId(u32);
    
    impl UserId {
        pub const fn new(id: u32) -> Self {
            Self(id)
        }
        
        pub const fn value(&self) -> u32 {
            self.0
        }
    }
    
    // ทดสอบ Newtype Pattern
    match Email::new("user@example.com".to_string()) {
        Ok(email) => println!("✅ Email ถูกต้อง: {}", email.as_str()),
        Err(e) => println!("❌ Email ไม่ถูกต้อง: {e}"),
    }
    
    match Email::new("invalid-email".to_string()) {
        Ok(email) => println!("✅ Email ถูกต้อง: {}", email.as_str()),
        Err(e) => println!("❌ Email ไม่ถูกต้อง: {e}"),
    }
    
    let user_id = UserId::new(12345);
    println!("👤 User ID: {} (ปลอดภัยจากการสับสน!)", user_id.value());
}

/// ตัวอย่างการใช้ Type State Pattern - สถานะในระดับ Type! 🎭🔒
pub fn type_state_pattern_example() {
    println!("\n🎭 === Type State Pattern: สถานะในระดับ Type! === 🎭");
    
    // States
    pub struct Draft;
    pub struct Published;
    pub struct Archived;
    
    // Document ที่มี Type State
    pub struct Document<State> {
        title: String,
        content: String,
        _state: PhantomData<State>,
    }
    
    impl Document<Draft> {
        pub const fn new(title: String) -> Self {
            Self {
                title,
                content: String::new(),
                _state: PhantomData,
            }
        }
        
        pub fn add_content(&mut self, content: &str) {
            self.content.push_str(content);
        }
        
        pub fn publish(self) -> Document<Published> {
            Document {
                title: self.title,
                content: self.content,
                _state: PhantomData,
            }
        }
    }
    
    impl Document<Published> {
        pub fn view(&self) -> String {
            format!("📄 {}: {}", self.title, self.content)
        }
        
        pub fn archive(self) -> Document<Archived> {
            Document {
                title: self.title,
                content: self.content,
                _state: PhantomData,
            }
        }
    }
    
    impl Document<Archived> {
        pub fn get_title(&self) -> &str {
            &self.title
        }
    }
    
    // ทดสอบ Type State Pattern
    let mut draft = Document::<Draft>::new("บทความเกี่ยวกับ Rust".to_string());
    draft.add_content("Rust เป็นภาษาโปรแกรมที่ปลอดภัยและรวดเร็ว...");
    
    let published = draft.publish();
    println!("📖 เอกสารที่เผยแพร่: {}", published.view());
    
    let archived = published.archive();
    println!("📚 เอกสารที่เก็บถาวร: {}", archived.get_title());
    
    println!("🎭 Type State Pattern ช่วยป้องกันการใช้งานผิดในระดับ compile time!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_builder() {
        let task = TaskBuilder::new()
            .id(1)
            .title("Test Task")
            .description("Test Description")
            .priority(TaskPriority::High)
            .assignee("Tester")
            .estimated_hours(5)
            .tag("test")
            .build()
            .unwrap();
        
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.priority, TaskPriority::High);
        assert_eq!(task.assignee, Some("Tester".to_string()));
        assert_eq!(task.estimated_hours, Some(5));
        assert!(task.tags.contains(&"test".to_string()));
    }
    
    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        
        let builder = TaskBuilder::new()
            .title("Test Task")
            .description("Test Description");
        
        let id = manager.create_task(builder).unwrap();
        assert_eq!(id, 1);
        
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::Todo);
    }
    
    #[test]
    fn test_state_transitions() {
        let mut manager = TaskManager::new();
        
        let builder = TaskBuilder::new()
            .title("Test Task")
            .description("Test Description");
        
        let id = manager.create_task(builder).unwrap();
        
        // Valid transition: Todo -> InProgress
        assert!(manager.update_task_status(id, TaskStatus::InProgress).is_ok());
        
        // Valid transition: InProgress -> Done
        assert!(manager.update_task_status(id, TaskStatus::Done).is_ok());
        
        // Invalid transition: Done -> InProgress
        assert!(manager.update_task_status(id, TaskStatus::InProgress).is_err());
    }
    
    #[test]
    fn test_sorting_strategies() {
        let tasks = vec![
            TaskBuilder::new().id(1).title("Low Priority").priority(TaskPriority::Low).build().unwrap(),
            TaskBuilder::new().id(2).title("High Priority").priority(TaskPriority::High).build().unwrap(),
            TaskBuilder::new().id(3).title("Critical Priority").priority(TaskPriority::Critical).build().unwrap(),
        ];
        
        let mut tasks_copy = tasks;
        let priority_sorter = PrioritySorter;
        priority_sorter.sort(&mut tasks_copy);
        
        // Critical should be first, Low should be last
        assert_eq!(tasks_copy[0].priority, TaskPriority::Critical);
        assert_eq!(tasks_copy[2].priority, TaskPriority::Low);
    }
    
    #[test]
    fn test_phantom_types() {
        let task = TaskBuilder::new()
            .id(1)
            .title("Test Task")
            .assignee("Tester")
            .build()
            .unwrap();
        
        let unvalidated = TypedTask::<Unvalidated>::new(task);
        let validated = unvalidated.validate().unwrap();
        
        assert!(validated.execute().contains("Test Task"));
    }
    
    #[test]
    fn test_command_pattern() {
        let mut manager = TaskManager::new();
        let mut history = CommandHistory::new();
        
        let builder = TaskBuilder::new()
            .title("Test Task")
            .description("Test Description");
        
        let create_cmd = Box::new(CreateTaskCommand::new(builder));
        history.execute_command(create_cmd, &mut manager).unwrap();
        
        assert_eq!(manager.get_all_tasks().len(), 1);
        
        // Test undo
        history.undo(&mut manager).unwrap();
        assert_eq!(manager.get_all_tasks().len(), 0);
        
        // Test redo
        history.redo(&mut manager).unwrap();
        assert_eq!(manager.get_all_tasks().len(), 1);
    }
}