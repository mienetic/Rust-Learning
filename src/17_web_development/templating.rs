//! 📄 Templating System Implementation - ระบบเทมเพลต
//! 
//! 🎨 ตัวอย่างการสร้าง template engine สำหรับ web applications ในเวิร์คช็อปพัฒนาเว็บ

use std::collections::HashMap;
use super::{HttpResponse, HttpStatus};

/// 🎨 Template Engine - เครื่องมือเทมเพลต
pub struct TemplateEngine {
    templates: HashMap<String, String>,
    partials: HashMap<String, String>,
    helpers: HashMap<String, Box<dyn Fn(&[String]) -> String + Send + Sync>>,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    #[must_use] pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
            partials: HashMap::new(),
            helpers: HashMap::new(),
        };
        
        // Add built-in helpers
        engine.add_helper("upper", |args| {
            args.first().map(|s| s.to_uppercase()).unwrap_or_default()
        });
        
        engine.add_helper("lower", |args| {
            args.first().map(|s| s.to_lowercase()).unwrap_or_default()
        });
        
        engine.add_helper("len", |args| {
            args.first().map(|s| s.len().to_string()).unwrap_or_default()
        });
        
        engine.add_helper("default", |args| {
            if args.is_empty() || args[0].is_empty() {
                args.get(1).cloned().unwrap_or_default()
            } else {
                args[0].clone()
            }
        });
        
        engine
    }
    
    /// Add a template
    pub fn add_template(&mut self, name: &str, content: &str) {
        self.templates.insert(name.to_string(), content.to_string());
    }
    
    /// Add a partial template
    pub fn add_partial(&mut self, name: &str, content: &str) {
        self.partials.insert(name.to_string(), content.to_string());
    }
    
    /// Add a helper function
    pub fn add_helper<F>(&mut self, name: &str, helper: F)
    where
        F: Fn(&[String]) -> String + Send + Sync + 'static,
    {
        self.helpers.insert(name.to_string(), Box::new(helper));
    }
    
    /// Render a template with context
    pub fn render(&self, template_name: &str, context: &TemplateContext) -> Result<String, String> {
        if let Some(template) = self.templates.get(template_name) {
            self.render_string(template, context)
        } else {
            Err(format!("Template '{template_name}' not found"))
        }
    }
    
    /// Render a template string with context
    pub fn render_string(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        // Process conditionals: {{#if condition}}...{{/if}}
        result = self.process_conditionals(&result, context)?;
        
        // Process loops: {{#each items}}...{{/each}}
        result = self.process_loops(&result, context)?;
        
        // Process partials: {{>partial_name}}
        result = self.process_partials(&result, context)?;
        
        // Process helpers: {{helper_name arg1 arg2}}
        result = self.process_helpers(&result, context)?;
        
        // Process variables: {{variable}} (last to handle loop variables like {{this}})
        result = self.process_variables(&result, context)?;
        
        Ok(result)
    }
    
    fn process_variables(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        // Simple variable replacement: {{variable}}
        while let Some(start) = result.find("{{") {
            if let Some(end) = result[start..].find("}}") {
                let end = start + end;
                let var_expr = &result[start + 2..end].trim();
                
                // Skip if it's a helper, conditional, loop, or contains spaces (helper call)
                if var_expr.starts_with('#') || var_expr.starts_with('/') || var_expr.starts_with('>') || var_expr.contains(' ') {
                    // Find the next occurrence
                    if let Some(next_start) = result[end + 2..].find("{{") {
                        result = result[..end + 2].to_string() + &self.process_variables(&result[end + 2..], context)?;
                        break;
                    }
                    break;
                }
                
                let value = context.get(var_expr).unwrap_or_default();
                result = result[..start].to_string() + &value + &result[end + 2..];
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn process_conditionals(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        while let Some(if_start) = result.find("{{#if ") {
            if let Some(if_end) = result[if_start..].find("}}") {
                let condition_end = if_start + if_end;
                let condition = result[if_start + 6..condition_end].trim();
                
                // Find the corresponding {{/if}}
                if let Some(endif_start) = result[condition_end..].find("{{/if}}") {
                    let endif_start = condition_end + endif_start;
                    let content = &result[condition_end + 2..endif_start];
                    
                    let replacement = if self.evaluate_condition(condition, context) {
                        self.render_string(content, context)?
                    } else {
                        String::new()
                    };
                    
                    result = result[..if_start].to_string() + &replacement + &result[endif_start + 7..];
                } else {
                    return Err("Missing {{/if}} for conditional".to_string());
                }
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn process_loops(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        while let Some(each_start) = result.find("{{#each ") {
            if let Some(each_end) = result[each_start..].find("}}") {
                let var_end = each_start + each_end + 2; // +2 for }}
                let var_name = result[each_start + 8..each_start + each_end].trim();
                
                // Find the corresponding {{/each}}
                if let Some(endeach_offset) = result[var_end..].find("{{/each}}") {
                    let endeach_start = var_end + endeach_offset;
                    let loop_content = &result[var_end..endeach_start];
                    
                    let replacement = if let Some(items) = context.get_array(var_name) {
                        let mut loop_result = String::new();
                        for (index, item) in items.iter().enumerate() {
                            let mut loop_context = context.clone();
                            loop_context.set("this", item);
                            loop_context.set("@index", &index.to_string());
                            loop_context.set("@first", &(index == 0).to_string());
                            loop_context.set("@last", &(index == items.len() - 1).to_string());
                            
                            loop_result.push_str(&self.render_string(loop_content, &loop_context)?);
                        }
                        loop_result
                    } else {
                        String::new()
                    };
                    
                    result = result[..each_start].to_string() + &replacement + &result[endeach_start + 9..];
                } else {
                    return Err("Missing {{/each}} for loop".to_string());
                }
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn process_partials(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        while let Some(partial_start) = result.find("{{>") {
            if let Some(partial_end) = result[partial_start..].find("}}") {
                let name_end = partial_start + partial_end;
                let partial_name = result[partial_start + 3..name_end].trim();
                
                let replacement = if let Some(partial_template) = self.partials.get(partial_name) {
                    self.render_string(partial_template, context)?
                } else {
                    format!("<!-- Partial '{partial_name}' not found -->")
                };
                
                result = result[..partial_start].to_string() + &replacement + &result[name_end + 2..];
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn process_helpers(&self, template: &str, context: &TemplateContext) -> Result<String, String> {
        let mut result = template.to_string();
        
        while let Some(start) = result.find("{{") {
            if let Some(end_pos) = result[start..].find("}}") {
                let end = start + end_pos;
                let expr = result[start + 2..end].trim();
                
                // Skip if it's a conditional, loop, partial, or simple variable
                if expr.starts_with('#') || expr.starts_with('/') || expr.starts_with('>') || !expr.contains(' ') {
                    // Move past this expression
                    let remaining = &result[end + 2..];
                    if remaining.contains("{{") {
                        let processed = self.process_helpers(remaining, context)?;
                        result = result[..end + 2].to_string() + &processed;
                    }
                    break;
                }
                
                // Parse helper call
                let parts: Vec<&str> = expr.split_whitespace().collect();
                if let Some(helper_name) = parts.first() {
                    if let Some(helper) = self.helpers.get(*helper_name) {
                        let args: Vec<String> = parts[1..].iter()
                            .map(|arg| context.get(arg).unwrap_or_else(|| (*arg).to_string()))
                            .collect();
                        
                        let replacement = helper(&args);
                        result = result[..start].to_string() + &replacement + &result[end + 2..];
                        // Continue processing from the beginning to handle nested helpers
                        continue;
                    } else {
                        // Unknown helper, move past it
                        let remaining = &result[end + 2..];
                        if remaining.contains("{{") {
                            let processed = self.process_helpers(remaining, context)?;
                            result = result[..end + 2].to_string() + &processed;
                        }
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(result)
    }
    
    fn evaluate_condition(&self, condition: &str, context: &TemplateContext) -> bool {
        // Simple condition evaluation
        if let Some(value) = context.get(condition) {
            !value.is_empty() && value != "false" && value != "0"
        } else {
            false
        }
    }
}

/// 📋 Template Context - บริบทเทมเพลตสำหรับส่งข้อมูล
#[derive(Debug, Clone)]
pub struct TemplateContext {
    variables: HashMap<String, String>,
    arrays: HashMap<String, Vec<String>>,
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateContext {
    #[must_use] pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            arrays: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
    
    pub fn set_array(&mut self, key: &str, values: Vec<String>) {
        self.arrays.insert(key.to_string(), values);
    }
    
    #[must_use] pub fn get(&self, key: &str) -> Option<String> {
        self.variables.get(key).cloned()
    }
    
    #[must_use] pub fn get_array(&self, key: &str) -> Option<&Vec<String>> {
        self.arrays.get(key)
    }
}

/// 🏗️ Layout System - ระบบเลย์เอาต์
pub struct LayoutManager {
    layouts: HashMap<String, String>,
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutManager {
    #[must_use] pub fn new() -> Self {
        Self {
            layouts: HashMap::new(),
        }
    }
    
    pub fn add_layout(&mut self, name: &str, content: &str) {
        self.layouts.insert(name.to_string(), content.to_string());
    }
    
    pub fn render_with_layout(
        &self,
        layout_name: &str,
        content: &str,
        context: &TemplateContext,
        engine: &TemplateEngine,
    ) -> Result<String, String> {
        if let Some(layout) = self.layouts.get(layout_name) {
            let mut layout_context = context.clone();
            layout_context.set("content", content);
            engine.render_string(layout, &layout_context)
        } else {
            Err(format!("Layout '{layout_name}' not found"))
        }
    }
}

/// 🔧 Template Response Builder - ตัวสร้างการตอบสนองเทมเพลต
pub struct TemplateResponse {
    engine: TemplateEngine,
    layout_manager: LayoutManager,
}

impl Default for TemplateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateResponse {
    #[must_use] pub fn new() -> Self {
        let mut engine = TemplateEngine::new();
        let mut layout_manager = LayoutManager::new();
        
        // 🏠 เพิ่มเทมเพลตเริ่มต้นสำหรับเวิร์คช็อป
        engine.add_template("home", r#"
<!DOCTYPE html>
<html>
<head>
    <title>🚀 {{title}} - Web Development Workshop</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <header>
        <h1>🎯 {{title}}</h1>
        {{#if user}}
            <p>🙋‍♂️ ยินดีต้อนรับ, {{user}}!</p>
        {{/if}}
    </header>
    
    <main>
        <h2>📚 {{subtitle}}</h2>
        <p>{{description}}</p>
        
        {{#if features}}
            <h3>✨ คุณสมบัติของเวิร์คช็อป:</h3>
            <ul>
            {{#each features}}
                <li>🔧 {{this}} (ลำดับที่ {{@index}})</li>
            {{/each}}
            </ul>
        {{/if}}
        
        {{>footer}}
    </main>
</body>
</html>
        "#);
        
        engine.add_template("user_profile", r#"
<div class="user-profile">
    <h2>👤 โปรไฟล์ของ {{upper name}}</h2>
    <p>📧 อีเมล: {{email}}</p>
    <p>🎂 อายุ: {{age}} ปี</p>
    <p>📝 ประวัติ: {{default bio "ไม่มีข้อมูลประวัติ"}}</p>
    
    {{#if posts}}
        <h3>📰 โพสต์ล่าสุด ({{len posts}} โพสต์):</h3>
        <div class="posts">
        {{#each posts}}
            <article class="post {{#if @first}}first{{/if}} {{#if @last}}last{{/if}}">
                <h4>📄 {{this}}</h4>
                <small>โพสต์ที่ #{{@index}}</small>
            </article>
        {{/each}}
        </div>
    {{/if}}
</div>
        "#);
        
        engine.add_partial("footer", r"
<footer>
    <p>© 2024 🦀 Rust Web Development Workshop. สงวนลิขสิทธิ์</p>
    <p>🕐 สร้างเมื่อ: {{timestamp}}</p>
</footer>
        ");
        
        // 🎨 เพิ่มเลย์เอาต์เริ่มต้นสำหรับเวิร์คช็อป
        layout_manager.add_layout("main", r#"
<!DOCTYPE html>
<html>
<head>
    <title>🚀 {{page_title}} - {{site_name}}</title>
    <meta charset="utf-8">
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        header { border-bottom: 2px solid #007acc; padding-bottom: 20px; margin-bottom: 20px; }
        footer { border-top: 2px solid #007acc; padding-top: 20px; margin-top: 20px; }
        nav a { color: #007acc; text-decoration: none; margin: 0 10px; }
        nav a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🎯 {{site_name}}</h1>
            <nav>
                <a href="/">🏠 หน้าแรก</a> |
                <a href="/about">📖 เกี่ยวกับ</a> |
                <a href="/contact">📞 ติดต่อ</a>
            </nav>
        </header>
        
        <main>
            {{content}}
        </main>
        
        <footer>
            <p>© 2024 🦀 {{site_name}}. ขับเคลื่อนด้วย Rust ในเวิร์คช็อปพัฒนาเว็บ</p>
        </footer>
    </div>
</body>
</html>
        "#);
        
        Self {
            engine,
            layout_manager,
        }
    }
    
    pub fn render_template(&self, template_name: &str, context: &TemplateContext) -> Result<HttpResponse, String> {
        match self.engine.render(template_name, context) {
            Ok(html) => Ok(HttpResponse::html(HttpStatus::Ok, &html)),
            Err(error) => Ok(HttpResponse::html(HttpStatus::InternalServerError, &format!("Template Error: {error}"))),
        }
    }
    
    pub fn render_with_layout(
        &self,
        template_name: &str,
        layout_name: &str,
        context: &TemplateContext,
    ) -> Result<HttpResponse, String> {
        match self.engine.render(template_name, context) {
            Ok(content) => {
                match self.layout_manager.render_with_layout(layout_name, &content, context, &self.engine) {
                    Ok(html) => Ok(HttpResponse::html(HttpStatus::Ok, &html)),
                    Err(error) => Ok(HttpResponse::html(HttpStatus::InternalServerError, &format!("Layout Error: {error}"))),
                }
            }
            Err(error) => Ok(HttpResponse::html(HttpStatus::InternalServerError, &format!("Template Error: {error}"))),
        }
    }
}

/// 🎭 ฟังก์ชันสำหรับแสดงตัวอย่างการใช้งานในเวิร์คช็อปพัฒนาเว็บ
pub fn demonstrate_templating() {
    println!("📄 Web Development Workshop - Templating System Example");
    
    let template_response = TemplateResponse::new();
    
    // Example 1: 🏠 หน้าแรกของเวิร์คช็อป
    println!("\n--- 🏠 Home Page Template ---");
    let mut home_context = TemplateContext::new();
    home_context.set("title", "ยินดีต้อนรับสู่เวิร์คช็อปพัฒนาเว็บด้วย Rust");
    home_context.set("subtitle", "สร้างเว็บแอปพลิเคชันที่รวดเร็วและปลอดภัย");
    home_context.set("description", "เวิร์คช็อปนี้แสดงให้เห็นการพัฒนาเว็บสมัยใหม่ด้วย Rust");
    home_context.set("user", "นักพัฒนาเวิร์คช็อป");
    home_context.set("timestamp", "2024-01-15 10:30:00");
    home_context.set_array("features", vec![
        "ความปลอดภัยของหน่วยความจำ".to_string(),
        "การแยกส่วนที่ไม่มีต้นทุน".to_string(),
        "การทำงานพร้อมกันอย่างไร้กังวล".to_string(),
        "ประสิทธิภาพที่รวดเร็ว".to_string(),
    ]);
    
    match template_response.render_template("home", &home_context) {
        Ok(response) => {
            println!("Status: {}", response.status.as_str());
            println!("Content-Type: {}", response.headers.get("Content-Type").unwrap_or(&"unknown".to_string()));
            println!("HTML Length: {} characters", response.body.len());
            println!("HTML Preview: {}...", &response.body[..200.min(response.body.len())]);
        }
        Err(error) => println!("Error: {error}"),
    }
    
    // Example 2: 👤 โปรไฟล์ผู้ใช้เวิร์คช็อป
    println!("\n--- 👤 User Profile Template ---");
    let mut profile_context = TemplateContext::new();
    profile_context.set("name", "อลิซ สมิธ");
    profile_context.set("email", "alice@workshop.example.com");
    profile_context.set("age", "28");
    profile_context.set("bio", "นักพัฒนาซอฟต์แวร์ที่หลงใหลใน Rust และเวิร์คช็อปพัฒนาเว็บ");
    profile_context.set_array("posts", vec![
        "เริ่มต้นกับ Rust ในเวิร์คช็อป".to_string(),
        "สร้าง Web APIs ด้วย Rust".to_string(),
        "เทคนิค Async Programming".to_string(),
    ]);
    
    match template_response.render_template("user_profile", &profile_context) {
        Ok(response) => {
            println!("Status: {}", response.status.as_str());
            println!("HTML Length: {} characters", response.body.len());
            println!("HTML Content:\n{}", response.body);
        }
        Err(error) => println!("Error: {error}"),
    }
    
    // Example 3: 🎨 เทมเพลตพร้อมเลย์เอาต์
    println!("\n--- 🎨 Template with Layout ---");
    let mut layout_context = TemplateContext::new();
    layout_context.set("page_title", "โปรไฟล์ผู้ใช้เวิร์คช็อป");
    layout_context.set("site_name", "Rust Web Development Workshop");
    layout_context.set("name", "บ็อบ จอห์นสัน");
    layout_context.set("email", "bob@workshop.example.com");
    layout_context.set("age", "35");
    layout_context.set_array("posts", vec![
        "การปรับปรุงประสิทธิภาพ Rust".to_string(),
        "แนวทางปฏิบัติที่ดีในการจัดการหน่วยความจำ".to_string(),
    ]);
    
    match template_response.render_with_layout("user_profile", "main", &layout_context) {
        Ok(response) => {
            println!("Status: {}", response.status.as_str());
            println!("Full HTML Length: {} characters", response.body.len());
            println!("HTML Preview: {}...", &response.body[..300.min(response.body.len())]);
        }
        Err(error) => println!("Error: {error}"),
    }
    
    // Example 4: 🔧 การใช้งาน Template Engine แบบกำหนดเอง
    println!("\n--- 🔧 Custom Template Engine ---");
    let mut engine = TemplateEngine::new();
    
    // เพิ่ม helper แบบกำหนดเอง
    engine.add_helper("format_date", |args| {
        if let Some(date) = args.first() {
            format!("📅 {date}")
        } else {
            "📅 ไม่ทราบวันที่".to_string()
        }
    });
    
    engine.add_template("custom", r#"
<div class="custom-template">
    <h1>🎯 {{title}}</h1>
    <p>สร้างเมื่อ: {{format_date created_date}}</p>
    
    {{#if items}}
        <h2>📋 รายการ ({{len items}} รายการ):</h2>
        <ol>
        {{#each items}}
            <li>🔧 {{upper this}} {{#if @first}}(แรก){{/if}} {{#if @last}}(สุดท้าย){{/if}}</li>
        {{/each}}
        </ol>
    {{/if}}
    
    {{#if show_footer}}
        <footer>🙏 ขอบคุณที่ใช้บริการเวิร์คช็อปของเรา!</footer>
    {{/if}}
</div>
    "#);
    
    let mut custom_context = TemplateContext::new();
    custom_context.set("title", "การสาธิต Custom Template ในเวิร์คช็อป");
    custom_context.set("created_date", "2024-01-15");
    custom_context.set("show_footer", "true");
    custom_context.set_array("items", vec![
        "การเขียนโปรแกรม rust".to_string(),
        "การพัฒนาเว็บ".to_string(),
        "เครื่องมือเทมเพลต".to_string(),
    ]);
    
    match engine.render("custom", &custom_context) {
        Ok(html) => {
            println!("Custom Template Result:");
            println!("{html}");
        }
        Err(error) => println!("Error: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_engine() {
        let mut engine = TemplateEngine::new();
        engine.add_template("test", "Hello {{name}}!");
        
        let mut context = TemplateContext::new();
        context.set("name", "World");
        
        let result = engine.render("test", &context).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_template_conditionals() {
        let mut engine = TemplateEngine::new();
        engine.add_template("test", "{{#if show}}Visible{{/if}}");
        
        let mut context = TemplateContext::new();
        context.set("show", "true");
        
        let result = engine.render("test", &context).unwrap();
        assert_eq!(result, "Visible");
    }

    #[test]
    fn test_template_loops() {
        let mut engine = TemplateEngine::new();
        engine.add_template("test", "{{#each items}}{{this}} {{/each}}");
        
        let mut context = TemplateContext::new();
        context.set_array("items", vec!["A".to_string(), "B".to_string()]);
        
        let result = engine.render("test", &context).unwrap();
        assert_eq!(result, "A B ");
    }

    #[test]
    fn test_template_helpers() {
        let mut engine = TemplateEngine::new();
        engine.add_template("test", "{{upper name}}");
        
        let mut context = TemplateContext::new();
        context.set("name", "hello");
        
        let result = engine.render("test", &context).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_template_response() {
        let template_response = TemplateResponse::new();
        let mut context = TemplateContext::new();
        context.set("title", "Test");
        context.set("subtitle", "Testing");
        context.set("description", "Test description");
        
        let response = template_response.render_template("home", &context).unwrap();
        assert_eq!(response.status, HttpStatus::Ok);
        assert!(response.body.contains("Test"));
    }

    #[test]
    fn test_demonstrate_templating() {
        // Test that the function runs without panicking
        demonstrate_templating();
    }
}