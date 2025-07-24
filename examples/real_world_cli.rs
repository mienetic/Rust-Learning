//! ตัวอย่างโปรเจกต์ CLI จริง - Task Manager
//!
//! โปรเจกต์นี้แสดงการใช้งาน Rust ในการสร้าง CLI application ที่ใช้งานจริง
//! ครอบคลุม: CLI parsing, File I/O, Error handling, Serialization, Testing

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Task Manager CLI - จัดการงานของคุณ
#[derive(Parser)]
#[command(name = "task-manager")]
#[command(about = "A simple task manager CLI application")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// เพิ่มงานใหม่
    Add {
        /// ชื่องาน
        title: String,
        /// คำอธิบายงาน
        #[arg(short, long)]
        description: Option<String>,
        /// ระดับความสำคัญ (1-5)
        #[arg(short, long, default_value = "3")]
        priority: u8,
    },
    /// แสดงรายการงานทั้งหมด
    List {
        /// แสดงเฉพาะงานที่เสร็จแล้ว
        #[arg(short, long)]
        completed: bool,
        /// แสดงเฉพาะงานที่ยังไม่เสร็จ
        #[arg(short, long)]
        pending: bool,
    },
    /// ทำเครื่องหมายงานเป็นเสร็จแล้ว
    Complete {
        /// ID ของงาน
        id: usize,
    },
    /// ลบงาน
    Remove {
        /// ID ของงาน
        id: usize,
    },
    /// แสดงสถิติ
    Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: Option<String>,
    priority: u8,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

impl Task {
    fn new(id: usize, title: String, description: Option<String>, priority: u8) -> Self {
        Self {
            id,
            title,
            description,
            priority,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }

    const fn priority_emoji(&self) -> &'static str {
        match self.priority {
            1 => "🟢",
            2 => "🟡",
            3 => "🟠",
            4 => "🔴",
            5 => "🚨",
            _ => "⚪",
        }
    }

    const fn status_emoji(&self) -> &'static str {
        if self.completed { "✅" } else { "⏳" }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    const fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, description: Option<String>, priority: u8) -> Result<()> {
        if priority > 5 {
            anyhow::bail!("Priority must be between 1 and 5");
        }

        let task = Task::new(self.next_id, title, description, priority);
        self.tasks.push(task);
        self.next_id += 1;

        println!("✅ เพิ่มงานใหม่สำเร็จ (ID: {})", self.next_id - 1);
        Ok(())
    }

    fn list_tasks(&self, show_completed: bool, show_pending: bool) {
        let tasks_to_show: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| {
                if show_completed && show_pending {
                    true
                } else if show_completed {
                    task.completed
                } else if show_pending {
                    !task.completed
                } else {
                    true
                }
            })
            .collect();

        if tasks_to_show.is_empty() {
            println!("📝 ไม่มีงานที่ตรงกับเงื่อนไข");
            return;
        }

        println!("\n📋 รายการงาน:");
        println!("{:-<80}", "");

        for task in tasks_to_show {
            println!(
                "{} {} [{}] {} - {}",
                task.status_emoji(),
                task.priority_emoji(),
                task.id,
                task.title,
                task.created_at.format("%Y-%m-%d %H:%M")
            );

            if let Some(desc) = &task.description {
                println!("    📝 {desc}");
            }

            if task.completed {
                if let Some(completed_at) = task.completed_at {
                    println!("    ✅ เสร็จเมื่อ: {}", completed_at.format("%Y-%m-%d %H:%M"));
                }
            }
            println!();
        }
    }

    fn complete_task(&mut self, id: usize) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .context(format!("ไม่พบงาน ID: {id}"))?;

        if task.completed {
            println!("⚠️ งานนี้เสร็จแล้ว");
            return Ok(());
        }

        task.complete();
        println!("✅ ทำเครื่องหมายงาน '{}' เป็นเสร็จแล้ว", task.title);
        Ok(())
    }

    fn remove_task(&mut self, id: usize) -> Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .context(format!("ไม่พบงาน ID: {id}"))?;

        let removed_task = self.tasks.remove(index);
        println!("🗑️ ลบงาน '{}' แล้ว", removed_task.title);
        Ok(())
    }

    fn show_stats(&self) {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|task| task.completed).count();
        let pending = total - completed;

        let high_priority = self
            .tasks
            .iter()
            .filter(|task| !task.completed && task.priority >= 4)
            .count();

        println!("\n📊 สถิติงาน:");
        println!("{:-<40}", "");
        println!("📝 งานทั้งหมด: {total}");
        println!(
            "✅ เสร็จแล้ว: {} ({:.1}%)",
            completed,
            if total > 0 {
                completed as f64 / total as f64 * 100.0
            } else {
                0.0
            }
        );
        println!(
            "⏳ ยังไม่เสร็จ: {} ({:.1}%)",
            pending,
            if total > 0 {
                pending as f64 / total as f64 * 100.0
            } else {
                0.0
            }
        );
        println!("🚨 ความสำคัญสูง (ยังไม่เสร็จ): {high_priority}");

        if !self.tasks.is_empty() {
            let avg_priority = self
                .tasks
                .iter()
                .filter(|task| !task.completed)
                .map(|task| f64::from(task.priority))
                .sum::<f64>()
                / pending.max(1) as f64;

            println!("📈 ความสำคัญเฉลี่ย (งานที่ยังไม่เสร็จ): {avg_priority:.1}");
        }
    }

    fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize tasks")?;

        fs::write(path, json).context("Failed to write tasks to file")?;

        Ok(())
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path).context("Failed to read tasks file")?;

        let task_manager: Self =
            serde_json::from_str(&content).context("Failed to parse tasks file")?;

        Ok(task_manager)
    }
}

fn get_data_file_path() -> Result<std::path::PathBuf> {
    let home_dir = dirs::home_dir().context("Could not find home directory")?;

    let data_dir = home_dir.join(".task-manager");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
    }

    Ok(data_dir.join("tasks.json"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_file = get_data_file_path()?;
    let mut task_manager =
        TaskManager::load_from_file(&data_file).context("Failed to load tasks")?;

    match cli.command {
        Commands::Add {
            title,
            description,
            priority,
        } => {
            task_manager.add_task(title, description, priority)?;
        }
        Commands::List { completed, pending } => {
            task_manager.list_tasks(completed, pending);
        }
        Commands::Complete { id } => {
            task_manager.complete_task(id)?;
        }
        Commands::Remove { id } => {
            task_manager.remove_task(id)?;
        }
        Commands::Stats => {
            task_manager.show_stats();
        }
    }

    task_manager
        .save_to_file(&data_file)
        .context("Failed to save tasks")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_task_creation() {
        let task = Task::new(
            1,
            "Test Task".to_string(),
            Some("Test Description".to_string()),
            3,
        );

        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test Description".to_string()));
        assert_eq!(task.priority, 3);
        assert!(!task.completed);
        assert!(task.completed_at.is_none());
    }

    #[test]
    fn test_task_completion() {
        let mut task = Task::new(1, "Test Task".to_string(), None, 3);

        assert!(!task.completed);
        assert!(task.completed_at.is_none());

        task.complete();

        assert!(task.completed);
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn test_task_manager_add_task() {
        let mut manager = TaskManager::new();

        let result = manager.add_task("Test Task".to_string(), Some("Description".to_string()), 3);

        assert!(result.is_ok());
        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.next_id, 2);
        assert_eq!(manager.tasks[0].title, "Test Task");
    }

    #[test]
    fn test_task_manager_invalid_priority() {
        let mut manager = TaskManager::new();

        let result = manager.add_task(
            "Test Task".to_string(),
            None,
            10, // Invalid priority
        );

        assert!(result.is_err());
        assert_eq!(manager.tasks.len(), 0);
    }

    #[test]
    fn test_task_manager_complete_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Test Task".to_string(), None, 3).unwrap();

        let result = manager.complete_task(1);
        assert!(result.is_ok());
        assert!(manager.tasks[0].completed);
    }

    #[test]
    fn test_task_manager_complete_nonexistent_task() {
        let mut manager = TaskManager::new();

        let result = manager.complete_task(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_task_manager_remove_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Test Task".to_string(), None, 3).unwrap();

        assert_eq!(manager.tasks.len(), 1);

        let result = manager.remove_task(1);
        assert!(result.is_ok());
        assert_eq!(manager.tasks.len(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_tasks.json");

        // Create and save task manager
        let mut manager = TaskManager::new();
        manager.add_task("Test Task".to_string(), None, 3).unwrap();
        manager.save_to_file(&file_path).unwrap();

        // Load task manager
        let loaded_manager = TaskManager::load_from_file(&file_path).unwrap();

        assert_eq!(loaded_manager.tasks.len(), 1);
        assert_eq!(loaded_manager.tasks[0].title, "Test Task");
        assert_eq!(loaded_manager.next_id, 2);
    }

    #[test]
    fn test_priority_emoji() {
        let task1 = Task::new(1, "Low".to_string(), None, 1);
        let task5 = Task::new(2, "Critical".to_string(), None, 5);

        assert_eq!(task1.priority_emoji(), "🟢");
        assert_eq!(task5.priority_emoji(), "🚨");
    }

    #[test]
    fn test_status_emoji() {
        let mut task = Task::new(1, "Test".to_string(), None, 3);

        assert_eq!(task.status_emoji(), "⏳");

        task.complete();
        assert_eq!(task.status_emoji(), "✅");
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_cli_help() {
        let output = Command::new("cargo")
            .args(["run", "--example", "real_world_cli", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Task Manager CLI"));
    }
}
