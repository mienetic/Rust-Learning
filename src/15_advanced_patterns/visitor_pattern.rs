//! Visitor Pattern Implementation
//!
//! การใช้งาน Visitor Pattern ใน Rust
//! ครอบคลุม AST Processing, File System Operations, และ Data Structure Traversal

use std::collections::HashMap;
// use std::fmt;

/// Visitor trait สำหรับ AST nodes
pub trait AstVisitor {
    fn visit_number(&mut self, value: f64);
    fn visit_string(&mut self, value: &str);
    fn visit_binary_op(&mut self, left: &AstNode, op: &str, right: &AstNode);
    fn visit_unary_op(&mut self, op: &str, operand: &AstNode);
    fn visit_variable(&mut self, name: &str);
    fn visit_function_call(&mut self, name: &str, args: &[AstNode]);
}

/// AST Node types
#[derive(Debug, Clone)]
pub enum AstNode {
    Number(f64),
    String(String),
    Variable(String),
    BinaryOp {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },
    UnaryOp {
        operator: String,
        operand: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<AstNode>,
    },
}

impl AstNode {
    pub fn accept(&self, visitor: &mut dyn AstVisitor) {
        match self {
            Self::Number(value) => visitor.visit_number(*value),
            Self::String(value) => visitor.visit_string(value),
            Self::Variable(name) => visitor.visit_variable(name),
            Self::BinaryOp { left, operator, right } => {
                visitor.visit_binary_op(left, operator, right);
            }
            Self::UnaryOp { operator, operand } => {
                visitor.visit_unary_op(operator, operand);
            }
            Self::FunctionCall { name, arguments } => {
                visitor.visit_function_call(name, arguments);
            }
        }
    }
}

/// Pretty printer visitor
#[derive(Debug)]
pub struct PrettyPrinter {
    output: String,
    indent_level: usize,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyPrinter {
    #[must_use] pub const fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    #[must_use] pub fn get_output(&self) -> &str {
        &self.output
    }

    fn add_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }

    const fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    const fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
}

impl AstVisitor for PrettyPrinter {
    fn visit_number(&mut self, value: f64) {
        self.output.push_str(&format!("{value}"));
    }

    fn visit_string(&mut self, value: &str) {
        self.output.push_str(&format!("\"{value}\""));
    }

    fn visit_variable(&mut self, name: &str) {
        self.output.push_str(name);
    }

    fn visit_binary_op(&mut self, left: &AstNode, op: &str, right: &AstNode) {
        self.output.push('(');
        left.accept(self);
        self.output.push_str(&format!(" {op} "));
        right.accept(self);
        self.output.push(')');
    }

    fn visit_unary_op(&mut self, op: &str, operand: &AstNode) {
        self.output.push_str(op);
        self.output.push('(');
        operand.accept(self);
        self.output.push(')');
    }

    fn visit_function_call(&mut self, name: &str, args: &[AstNode]) {
        self.output.push_str(name);
        self.output.push('(');
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            arg.accept(self);
        }
        self.output.push(')');
    }
}

/// Evaluator visitor
#[derive(Debug)]
pub struct Evaluator {
    variables: HashMap<String, f64>,
    result: f64,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    #[must_use] pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            result: 0.0,
        }
    }

    pub fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    #[must_use] pub const fn get_result(&self) -> f64 {
        self.result
    }

    pub fn evaluate(&mut self, node: &AstNode) -> f64 {
        node.accept(self);
        self.result
    }
}

impl AstVisitor for Evaluator {
    fn visit_number(&mut self, value: f64) {
        self.result = value;
    }

    fn visit_string(&mut self, _value: &str) {
        // Strings evaluate to 0 in this simple evaluator
        self.result = 0.0;
    }

    fn visit_variable(&mut self, name: &str) {
        self.result = self.variables.get(name).copied().unwrap_or(0.0);
    }

    fn visit_binary_op(&mut self, left: &AstNode, op: &str, right: &AstNode) {
        let left_val = {
            left.accept(self);
            self.result
        };
        
        let right_val = {
            right.accept(self);
            self.result
        };
        
        self.result = match op {
            "+" => left_val + right_val,
            "-" => left_val - right_val,
            "*" => left_val * right_val,
            "/" => if right_val == 0.0 { f64::INFINITY } else { left_val / right_val },
            "^" => left_val.powf(right_val),
            _ => 0.0,
        };
    }

    fn visit_unary_op(&mut self, op: &str, operand: &AstNode) {
        operand.accept(self);
        
        self.result = match op {
            "-" => -self.result,
            "+" => self.result,
            "!" => if self.result == 0.0 { 1.0 } else { 0.0 },
            _ => self.result,
        };
    }

    fn visit_function_call(&mut self, name: &str, args: &[AstNode]) {
        match name {
            "sin" if args.len() == 1 => {
                args[0].accept(self);
                self.result = self.result.sin();
            }
            "cos" if args.len() == 1 => {
                args[0].accept(self);
                self.result = self.result.cos();
            }
            "sqrt" if args.len() == 1 => {
                args[0].accept(self);
                self.result = self.result.sqrt();
            }
            "max" if args.len() == 2 => {
                args[0].accept(self);
                let val1 = self.result;
                args[1].accept(self);
                let val2 = self.result;
                self.result = val1.max(val2);
            }
            "min" if args.len() == 2 => {
                args[0].accept(self);
                let val1 = self.result;
                args[1].accept(self);
                let val2 = self.result;
                self.result = val1.min(val2);
            }
            _ => self.result = 0.0, // Unknown function
        }
    }
}

/// Variable collector visitor
#[derive(Debug)]
pub struct VariableCollector {
    variables: Vec<String>,
}

impl Default for VariableCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableCollector {
    #[must_use] pub const fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    #[must_use] pub fn get_variables(&self) -> &[String] {
        &self.variables
    }

    pub fn collect(&mut self, node: &AstNode) -> Vec<String> {
        self.variables.clear();
        node.accept(self);
        self.variables.sort();
        self.variables.dedup();
        self.variables.clone()
    }
}

impl AstVisitor for VariableCollector {
    fn visit_number(&mut self, _value: f64) {
        // Numbers don't contain variables
    }

    fn visit_string(&mut self, _value: &str) {
        // Strings don't contain variables
    }

    fn visit_variable(&mut self, name: &str) {
        self.variables.push(name.to_string());
    }

    fn visit_binary_op(&mut self, left: &AstNode, _op: &str, right: &AstNode) {
        left.accept(self);
        right.accept(self);
    }

    fn visit_unary_op(&mut self, _op: &str, operand: &AstNode) {
        operand.accept(self);
    }

    fn visit_function_call(&mut self, _name: &str, args: &[AstNode]) {
        for arg in args {
            arg.accept(self);
        }
    }
}

/// File system visitor pattern
pub trait FileSystemVisitor {
    fn visit_file(&mut self, path: &str, size: u64, extension: &str);
    fn visit_directory(&mut self, path: &str, children: &[FileSystemNode]);
    fn visit_symlink(&mut self, path: &str, target: &str);
}

/// File system node types
#[derive(Debug, Clone)]
pub enum FileSystemNode {
    File {
        path: String,
        size: u64,
        extension: String,
    },
    Directory {
        path: String,
        children: Vec<FileSystemNode>,
    },
    Symlink {
        path: String,
        target: String,
    },
}

impl FileSystemNode {
    pub fn accept(&self, visitor: &mut dyn FileSystemVisitor) {
        match self {
            Self::File { path, size, extension } => {
                visitor.visit_file(path, *size, extension);
            }
            Self::Directory { path, children } => {
                visitor.visit_directory(path, children);
            }
            Self::Symlink { path, target } => {
                visitor.visit_symlink(path, target);
            }
        }
    }

    #[must_use] pub fn get_path(&self) -> &str {
        match self {
            Self::File { path, .. } => path,
            Self::Directory { path, .. } => path,
            Self::Symlink { path, .. } => path,
        }
    }
}

/// File size calculator visitor
#[derive(Debug)]
pub struct FileSizeCalculator {
    total_size: u64,
    file_count: usize,
    directory_count: usize,
}

impl Default for FileSizeCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSizeCalculator {
    #[must_use] pub const fn new() -> Self {
        Self {
            total_size: 0,
            file_count: 0,
            directory_count: 0,
        }
    }

    #[must_use] pub const fn get_stats(&self) -> (u64, usize, usize) {
        (self.total_size, self.file_count, self.directory_count)
    }

    pub fn calculate(&mut self, node: &FileSystemNode) -> u64 {
        self.total_size = 0;
        self.file_count = 0;
        self.directory_count = 0;
        node.accept(self);
        self.total_size
    }
}

impl FileSystemVisitor for FileSizeCalculator {
    fn visit_file(&mut self, _path: &str, size: u64, _extension: &str) {
        self.total_size += size;
        self.file_count += 1;
    }

    fn visit_directory(&mut self, _path: &str, children: &[FileSystemNode]) {
        self.directory_count += 1;
        for child in children {
            child.accept(self);
        }
    }

    fn visit_symlink(&mut self, _path: &str, _target: &str) {
        // Symlinks don't contribute to size
    }
}

/// File finder visitor
#[derive(Debug)]
pub struct FileFinder {
    pattern: String,
    matches: Vec<String>,
}

impl FileFinder {
    #[must_use] pub const fn new(pattern: String) -> Self {
        Self {
            pattern,
            matches: Vec::new(),
        }
    }

    #[must_use] pub fn get_matches(&self) -> &[String] {
        &self.matches
    }

    pub fn find(&mut self, node: &FileSystemNode) -> Vec<String> {
        self.matches.clear();
        node.accept(self);
        self.matches.clone()
    }

    fn matches_pattern(&self, path: &str) -> bool {
        path.contains(&self.pattern)
    }
}

impl FileSystemVisitor for FileFinder {
    fn visit_file(&mut self, path: &str, _size: u64, _extension: &str) {
        if self.matches_pattern(path) {
            self.matches.push(path.to_string());
        }
    }

    fn visit_directory(&mut self, path: &str, children: &[FileSystemNode]) {
        if self.matches_pattern(path) {
            self.matches.push(path.to_string());
        }
        for child in children {
            child.accept(self);
        }
    }

    fn visit_symlink(&mut self, path: &str, _target: &str) {
        if self.matches_pattern(path) {
            self.matches.push(path.to_string());
        }
    }
}

/// File tree printer visitor
#[derive(Debug)]
pub struct FileTreePrinter {
    output: String,
    indent_level: usize,
}

impl Default for FileTreePrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl FileTreePrinter {
    #[must_use] pub const fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }

    #[must_use] pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn print_tree(&mut self, node: &FileSystemNode) -> String {
        self.output.clear();
        self.indent_level = 0;
        node.accept(self);
        self.output.clone()
    }

    fn add_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }

    fn format_size(size: u64) -> String {
        if size < 1024 {
            format!("{size} B")
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

impl FileSystemVisitor for FileTreePrinter {
    fn visit_file(&mut self, path: &str, size: u64, extension: &str) {
        self.add_indent();
        let filename = path.split('/').next_back().unwrap_or(path);
        self.output.push_str(&format!(
            "📄 {} ({}) [{}]\n",
            filename,
            Self::format_size(size),
            extension
        ));
    }

    fn visit_directory(&mut self, path: &str, children: &[FileSystemNode]) {
        self.add_indent();
        let dirname = path.split('/').next_back().unwrap_or(path);
        self.output.push_str(&format!("📁 {dirname}/\n"));
        
        self.indent_level += 1;
        for child in children {
            child.accept(self);
        }
        self.indent_level -= 1;
    }

    fn visit_symlink(&mut self, path: &str, target: &str) {
        self.add_indent();
        let linkname = path.split('/').next_back().unwrap_or(path);
        self.output.push_str(&format!("🔗 {linkname} -> {target}\n"));
    }
}

/// สาธิตการใช้งาน Visitor Pattern
pub fn demonstrate_visitor() {
    println!("👁️  Visitor Pattern Examples:");
    
    // AST Visitor Example
    println!("\n🌳 AST Processing:");
    
    // Create a complex expression: (x + 5) * sin(y) - sqrt(z)
    let ast = AstNode::BinaryOp {
        left: Box::new(AstNode::BinaryOp {
            left: Box::new(AstNode::BinaryOp {
                left: Box::new(AstNode::Variable("x".to_string())),
                operator: "+".to_string(),
                right: Box::new(AstNode::Number(5.0)),
            }),
            operator: "*".to_string(),
            right: Box::new(AstNode::FunctionCall {
                name: "sin".to_string(),
                arguments: vec![AstNode::Variable("y".to_string())],
            }),
        }),
        operator: "-".to_string(),
        right: Box::new(AstNode::FunctionCall {
            name: "sqrt".to_string(),
            arguments: vec![AstNode::Variable("z".to_string())],
        }),
    };
    
    // Pretty print the AST
    let mut printer = PrettyPrinter::new();
    ast.accept(&mut printer);
    println!("🖨️  Pretty printed: {}", printer.get_output());
    
    // Collect variables
    let mut collector = VariableCollector::new();
    let variables = collector.collect(&ast);
    println!("📝 Variables found: {variables:?}");
    
    // Evaluate the expression
    let mut evaluator = Evaluator::new();
    evaluator.set_variable("x".to_string(), 3.0);
    evaluator.set_variable("y".to_string(), std::f64::consts::PI / 2.0); // sin(π/2) = 1
    evaluator.set_variable("z".to_string(), 16.0); // sqrt(16) = 4
    
    let result = evaluator.evaluate(&ast);
    println!("🧮 Evaluation result: {result}");
    
    // Create another expression: max(10, min(x, y))
    let ast2 = AstNode::FunctionCall {
        name: "max".to_string(),
        arguments: vec![
            AstNode::Number(10.0),
            AstNode::FunctionCall {
                name: "min".to_string(),
                arguments: vec![
                    AstNode::Variable("x".to_string()),
                    AstNode::Variable("y".to_string()),
                ],
            },
        ],
    };
    
    let mut printer2 = PrettyPrinter::new();
    ast2.accept(&mut printer2);
    println!("🖨️  Expression 2: {}", printer2.get_output());
    
    let result2 = evaluator.evaluate(&ast2);
    println!("🧮 Evaluation result 2: {result2}");
    
    // File System Visitor Example
    println!("\n📁 File System Processing:");
    
    // Create a sample file system structure
    let filesystem = FileSystemNode::Directory {
        path: "/home/user".to_string(),
        children: vec![
            FileSystemNode::Directory {
                path: "/home/user/documents".to_string(),
                children: vec![
                    FileSystemNode::File {
                        path: "/home/user/documents/report.pdf".to_string(),
                        size: 2048576, // 2MB
                        extension: "pdf".to_string(),
                    },
                    FileSystemNode::File {
                        path: "/home/user/documents/notes.txt".to_string(),
                        size: 1024, // 1KB
                        extension: "txt".to_string(),
                    },
                ],
            },
            FileSystemNode::Directory {
                path: "/home/user/pictures".to_string(),
                children: vec![
                    FileSystemNode::File {
                        path: "/home/user/pictures/vacation.jpg".to_string(),
                        size: 5242880, // 5MB
                        extension: "jpg".to_string(),
                    },
                    FileSystemNode::File {
                        path: "/home/user/pictures/profile.png".to_string(),
                        size: 512000, // 500KB
                        extension: "png".to_string(),
                    },
                ],
            },
            FileSystemNode::File {
                path: "/home/user/config.json".to_string(),
                size: 256, // 256B
                extension: "json".to_string(),
            },
            FileSystemNode::Symlink {
                path: "/home/user/desktop".to_string(),
                target: "/home/user/Desktop".to_string(),
            },
        ],
    };
    
    // Print file tree
    let mut tree_printer = FileTreePrinter::new();
    let tree_output = tree_printer.print_tree(&filesystem);
    println!("🌳 File tree:");
    print!("{tree_output}");
    
    // Calculate total size
    let mut size_calculator = FileSizeCalculator::new();
    let total_size = size_calculator.calculate(&filesystem);
    let (_, file_count, dir_count) = size_calculator.get_stats();
    println!("📊 Statistics:");
    println!("   Total size: {} bytes ({:.2} MB)", total_size, total_size as f64 / (1024.0 * 1024.0));
    println!("   Files: {file_count}");
    println!("   Directories: {dir_count}");
    
    // Find files with specific patterns
    let mut finder = FileFinder::new("picture".to_string());
    let matches = finder.find(&filesystem);
    println!("🔍 Files/directories containing 'picture': {matches:?}");
    
    let mut finder2 = FileFinder::new(".jpg".to_string());
    let matches2 = finder2.find(&filesystem);
    println!("🔍 JPG files: {matches2:?}");
    
    println!("\n✅ Visitor pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_pretty_printer() {
        let ast = AstNode::BinaryOp {
            left: Box::new(AstNode::Number(5.0)),
            operator: "+".to_string(),
            right: Box::new(AstNode::Number(3.0)),
        };
        
        let mut printer = PrettyPrinter::new();
        ast.accept(&mut printer);
        assert_eq!(printer.get_output(), "(5 + 3)");
    }

    #[test]
    fn test_ast_evaluator() {
        let ast = AstNode::BinaryOp {
            left: Box::new(AstNode::Number(10.0)),
            operator: "*".to_string(),
            right: Box::new(AstNode::Number(2.0)),
        };
        
        let mut evaluator = Evaluator::new();
        let result = evaluator.evaluate(&ast);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_variable_collector() {
        let ast = AstNode::BinaryOp {
            left: Box::new(AstNode::Variable("x".to_string())),
            operator: "+".to_string(),
            right: Box::new(AstNode::Variable("y".to_string())),
        };
        
        let mut collector = VariableCollector::new();
        let variables = collector.collect(&ast);
        assert_eq!(variables, vec!["x".to_string(), "y".to_string()]);
    }

    #[test]
    fn test_file_size_calculator() {
        let filesystem = FileSystemNode::Directory {
            path: "/test".to_string(),
            children: vec![
                FileSystemNode::File {
                    path: "/test/file1.txt".to_string(),
                    size: 100,
                    extension: "txt".to_string(),
                },
                FileSystemNode::File {
                    path: "/test/file2.txt".to_string(),
                    size: 200,
                    extension: "txt".to_string(),
                },
            ],
        };
        
        let mut calculator = FileSizeCalculator::new();
        let total_size = calculator.calculate(&filesystem);
        assert_eq!(total_size, 300);
        
        let (_, file_count, dir_count) = calculator.get_stats();
        assert_eq!(file_count, 2);
        assert_eq!(dir_count, 1);
    }

    #[test]
    fn test_file_finder() {
        let filesystem = FileSystemNode::Directory {
            path: "/test".to_string(),
            children: vec![
                FileSystemNode::File {
                    path: "/test/document.pdf".to_string(),
                    size: 100,
                    extension: "pdf".to_string(),
                },
                FileSystemNode::File {
                    path: "/test/image.jpg".to_string(),
                    size: 200,
                    extension: "jpg".to_string(),
                },
            ],
        };
        
        let mut finder = FileFinder::new("pdf".to_string());
        let matches = finder.find(&filesystem);
        assert_eq!(matches, vec!["/test/document.pdf".to_string()]);
    }

    #[test]
    fn test_function_evaluation() {
        let ast = AstNode::FunctionCall {
            name: "max".to_string(),
            arguments: vec![
                AstNode::Number(5.0),
                AstNode::Number(10.0),
            ],
        };
        
        let mut evaluator = Evaluator::new();
        let result = evaluator.evaluate(&ast);
        assert_eq!(result, 10.0);
    }
}