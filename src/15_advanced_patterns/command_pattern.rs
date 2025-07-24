//! Command Pattern Implementation
//!
//! การใช้งาน Command Pattern ใน Rust

use std::collections::VecDeque;

/// Command trait
trait Command {
    fn execute(&self) -> Result<String, String>;
    fn undo(&self) -> Result<String, String>;
    fn description(&self) -> &str;
}

/// Text Editor Commands
struct InsertTextCommand {
    text: String,
    position: usize,
}

impl InsertTextCommand {
    const fn new(text: String, position: usize) -> Self {
        Self { text, position }
    }
}

impl Command for InsertTextCommand {
    fn execute(&self) -> Result<String, String> {
        Ok(format!("Inserted '{}' at position {}", self.text, self.position))
    }
    
    fn undo(&self) -> Result<String, String> {
        Ok(format!("Removed '{}' from position {}", self.text, self.position))
    }
    
    fn description(&self) -> &'static str {
        "Insert Text"
    }
}

struct DeleteTextCommand {
    length: usize,
    position: usize,
    deleted_text: String,
}

impl DeleteTextCommand {
    fn new(length: usize, position: usize) -> Self {
        Self {
            length,
            position,
            deleted_text: format!("text_{position}"), // Simulated
        }
    }
}

impl Command for DeleteTextCommand {
    fn execute(&self) -> Result<String, String> {
        Ok(format!("Deleted {} characters from position {}", self.length, self.position))
    }
    
    fn undo(&self) -> Result<String, String> {
        Ok(format!("Restored '{}' at position {}", self.deleted_text, self.position))
    }
    
    fn description(&self) -> &'static str {
        "Delete Text"
    }
}

/// Command Invoker (Text Editor)
struct TextEditor {
    history: VecDeque<Box<dyn Command>>,
    undo_stack: VecDeque<Box<dyn Command>>,
    max_history: usize,
}

impl TextEditor {
    fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            undo_stack: VecDeque::new(),
            max_history,
        }
    }
    
    fn execute_command(&mut self, command: Box<dyn Command>) -> Result<String, String> {
        let result = command.execute();
        
        if result.is_ok() {
            // Clear redo stack when new command is executed
            self.undo_stack.clear();
            
            // Add to history
            self.history.push_back(command);
            
            // Maintain max history size
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }
        }
        
        result
    }
    
    fn undo(&mut self) -> Result<String, String> {
        if let Some(command) = self.history.pop_back() {
            let result = command.undo();
            if result.is_ok() {
                self.undo_stack.push_back(command);
            }
            result
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    
    fn redo(&mut self) -> Result<String, String> {
        if let Some(command) = self.undo_stack.pop_back() {
            let result = command.execute();
            if result.is_ok() {
                self.history.push_back(command);
            }
            result
        } else {
            Err("Nothing to redo".to_string())
        }
    }
    
    fn get_history(&self) -> Vec<&str> {
        self.history.iter().map(|cmd| cmd.description()).collect()
    }
}

/// สาธิตการใช้งาน Command Pattern
pub fn demonstrate_command() {
    println!("⚡ Command Pattern Examples:");
    
    let mut editor = TextEditor::new(10);
    
    // Execute commands
    println!("\n📝 Executing Commands:");
    
    let insert_cmd = Box::new(InsertTextCommand::new("Hello".to_string(), 0));
    match editor.execute_command(insert_cmd) {
        Ok(result) => println!("✅ {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    let insert_cmd2 = Box::new(InsertTextCommand::new(" World".to_string(), 5));
    match editor.execute_command(insert_cmd2) {
        Ok(result) => println!("✅ {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    let delete_cmd = Box::new(DeleteTextCommand::new(3, 8));
    match editor.execute_command(delete_cmd) {
        Ok(result) => println!("✅ {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    // Show history
    println!("\n📚 Command History:");
    for (i, desc) in editor.get_history().iter().enumerate() {
        println!("  {}. {}", i + 1, desc);
    }
    
    // Undo operations
    println!("\n↩️ Undo Operations:");
    
    match editor.undo() {
        Ok(result) => println!("✅ Undo: {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    match editor.undo() {
        Ok(result) => println!("✅ Undo: {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    // Redo operations
    println!("\n↪️ Redo Operations:");
    
    match editor.redo() {
        Ok(result) => println!("✅ Redo: {result}"),
        Err(e) => println!("❌ {e}"),
    }
    
    println!("\n✅ Command Pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_execution() {
        let mut editor = TextEditor::new(5);
        let cmd = Box::new(InsertTextCommand::new("test".to_string(), 0));
        
        let result = editor.execute_command(cmd);
        assert!(result.is_ok());
        assert_eq!(editor.get_history().len(), 1);
    }
    
    #[test]
    fn test_undo_redo() {
        let mut editor = TextEditor::new(5);
        let cmd = Box::new(InsertTextCommand::new("test".to_string(), 0));
        
        editor.execute_command(cmd).unwrap();
        assert_eq!(editor.get_history().len(), 1);
        
        editor.undo().unwrap();
        assert_eq!(editor.get_history().len(), 0);
        
        editor.redo().unwrap();
        assert_eq!(editor.get_history().len(), 1);
    }
}