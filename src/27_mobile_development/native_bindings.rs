//! 🔗 Native Platform Bindings
//! 
//! สาธิตการสร้าง bindings ระหว่าง Rust และ native platforms
//! รวมถึง iOS (Swift/Objective-C), Android (Java/Kotlin), และ FFI

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{Arc, Mutex};

/// 📱 Platform Types
#[derive(Debug, Clone, PartialEq)]
pub enum NativePlatform {
    iOS,
    Android,
    Windows,
    MacOS,
    Linux,
}

/// 🔗 Binding Type
#[derive(Debug, Clone, PartialEq)]
pub enum BindingType {
    FFI,        // Foreign Function Interface
    JNI,        // Java Native Interface (Android)
    Swift,      // Swift bindings (iOS)
    ObjectiveC, // Objective-C bindings (iOS)
    COM,        // Component Object Model (Windows)
}

/// 📋 Function Signature
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<(String, String)>, // (name, type)
    pub is_async: bool,
    pub is_safe: bool,
}

impl FunctionSignature {
    pub fn new(name: &str, return_type: &str) -> Self {
        Self {
            name: name.to_string(),
            return_type: return_type.to_string(),
            parameters: Vec::new(),
            is_async: false,
            is_safe: true,
        }
    }
    
    pub fn add_parameter(&mut self, name: &str, param_type: &str) {
        self.parameters.push((name.to_string(), param_type.to_string()));
    }
    
    pub fn set_async(&mut self, is_async: bool) {
        self.is_async = is_async;
    }
    
    pub fn set_unsafe(&mut self, is_unsafe: bool) {
        self.is_safe = !is_unsafe;
    }
}

/// 🔧 Binding Generator
pub struct BindingGenerator {
    platform: NativePlatform,
    binding_type: BindingType,
    functions: Vec<FunctionSignature>,
    types: HashMap<String, String>,
    includes: Vec<String>,
}

impl BindingGenerator {
    pub fn new(platform: NativePlatform, binding_type: BindingType) -> Self {
        Self {
            platform,
            binding_type,
            functions: Vec::new(),
            types: HashMap::new(),
            includes: Vec::new(),
        }
    }
    
    pub fn add_function(&mut self, function: FunctionSignature) {
        self.functions.push(function);
    }
    
    pub fn add_type_mapping(&mut self, rust_type: &str, native_type: &str) {
        self.types.insert(rust_type.to_string(), native_type.to_string());
    }
    
    pub fn add_include(&mut self, include: &str) {
        self.includes.push(include.to_string());
    }
    
    pub fn generate_header(&self) -> String {
        match self.binding_type {
            BindingType::FFI => self.generate_c_header(),
            BindingType::Swift => self.generate_swift_header(),
            BindingType::ObjectiveC => self.generate_objc_header(),
            BindingType::JNI => self.generate_jni_header(),
            BindingType::COM => self.generate_com_header(),
        }
    }
    
    pub fn generate_implementation(&self) -> String {
        match self.binding_type {
            BindingType::FFI => self.generate_c_implementation(),
            BindingType::Swift => self.generate_swift_implementation(),
            BindingType::ObjectiveC => self.generate_objc_implementation(),
            BindingType::JNI => self.generate_jni_implementation(),
            BindingType::COM => self.generate_com_implementation(),
        }
    }
    
    fn generate_c_header(&self) -> String {
        let mut header = String::new();
        
        header.push_str("#ifndef RUST_BINDINGS_H\n");
        header.push_str("#define RUST_BINDINGS_H\n\n");
        
        header.push_str("#ifdef __cplusplus\n");
        header.push_str("extern \"C\" {\n");
        header.push_str("#endif\n\n");
        
        for include in &self.includes {
            header.push_str(&format!("#include <{}>\n", include));
        }
        header.push_str("\n");
        
        for function in &self.functions {
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let c_type = self.rust_to_c_type(param_type);
                    format!("{} {}", c_type, name)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_c_type(&function.return_type);
            header.push_str(&format!("{} {}({});\n", return_type, function.name, params));
        }
        
        header.push_str("\n#ifdef __cplusplus\n");
        header.push_str("}\n");
        header.push_str("#endif\n\n");
        header.push_str("#endif // RUST_BINDINGS_H\n");
        
        header
    }
    
    fn generate_c_implementation(&self) -> String {
        let mut impl_code = String::new();
        
        impl_code.push_str("// Rust FFI Implementation\n");
        impl_code.push_str("use std::ffi::{CStr, CString};\n");
        impl_code.push_str("use std::os::raw::{c_char, c_int, c_void};\n\n");
        
        for function in &self.functions {
            let safety = if function.is_safe { "" } else { "unsafe " };
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let c_type = self.rust_to_c_type(param_type);
                    format!("{}: {}", name, c_type)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_c_type(&function.return_type);
            
            impl_code.push_str(&format!(
                "#[no_mangle]\npub {}extern \"C\" fn {}({}) -> {} {{\n",
                safety, function.name, params, return_type
            ));
            
            impl_code.push_str(&format!(
                "    // TODO: Implement {}\n",
                function.name
            ));
            
            if function.return_type != "void" {
                impl_code.push_str(&format!(
                    "    {} // Default return value\n",
                    self.get_default_value(&return_type)
                ));
            }
            
            impl_code.push_str("}\n\n");
        }
        
        impl_code
    }
    
    fn generate_swift_header(&self) -> String {
        let mut header = String::new();
        
        header.push_str("// Swift Bindings for Rust Library\n");
        header.push_str("import Foundation\n\n");
        
        header.push_str("@objc public class RustLibrary: NSObject {\n\n");
        
        for function in &self.functions {
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let swift_type = self.rust_to_swift_type(param_type);
                    format!("{}: {}", name, swift_type)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_swift_type(&function.return_type);
            let async_keyword = if function.is_async { "async " } else { "" };
            
            header.push_str(&format!(
                "    @objc public {}func {}({}) -> {} {{\n",
                async_keyword, function.name, params, return_type
            ));
            
            header.push_str(&format!(
                "        // Call Rust function {}\n",
                function.name
            ));
            
            if function.return_type != "void" {
                header.push_str(&format!(
                    "        return {} // Default return\n",
                    self.get_swift_default_value(&return_type)
                ));
            }
            
            header.push_str("    }\n\n");
        }
        
        header.push_str("}\n");
        header
    }
    
    fn generate_swift_implementation(&self) -> String {
        let mut impl_code = String::new();
        
        impl_code.push_str("// Swift Implementation with Rust FFI\n");
        impl_code.push_str("import Foundation\n\n");
        
        // External function declarations
        for function in &self.functions {
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let c_type = self.rust_to_c_type(param_type);
                    format!("{}: {}", name, c_type)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_c_type(&function.return_type);
            
            impl_code.push_str(&format!(
                "@_silgen_name(\"{}\")\nfunc {}({}) -> {}\n\n",
                function.name, function.name, params, return_type
            ));
        }
        
        impl_code
    }
    
    fn generate_objc_header(&self) -> String {
        let mut header = String::new();
        
        header.push_str("// Objective-C Bindings for Rust Library\n");
        header.push_str("#import <Foundation/Foundation.h>\n\n");
        
        header.push_str("@interface RustLibrary : NSObject\n\n");
        
        for function in &self.functions {
            let params = function.parameters.iter()
                .enumerate()
                .map(|(i, (name, param_type))| {
                    let objc_type = self.rust_to_objc_type(param_type);
                    if i == 0 {
                        format!(":({}){}", objc_type, name)
                    } else {
                        format!(" {}:({}){}", name, objc_type, name)
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            
            let return_type = self.rust_to_objc_type(&function.return_type);
            
            header.push_str(&format!(
                "- ({}){}{}\n",
                return_type, function.name, params
            ));
        }
        
        header.push_str("\n@end\n");
        header
    }
    
    fn generate_objc_implementation(&self) -> String {
        let mut impl_code = String::new();
        
        impl_code.push_str("// Objective-C Implementation\n");
        impl_code.push_str("#import \"RustLibrary.h\"\n\n");
        
        // External C function declarations
        for function in &self.functions {
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let c_type = self.rust_to_c_type(param_type);
                    format!("{} {}", c_type, name)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_c_type(&function.return_type);
            
            impl_code.push_str(&format!(
                "extern {} {}({});\n",
                return_type, function.name, params
            ));
        }
        
        impl_code.push_str("\n@implementation RustLibrary\n\n");
        
        for function in &self.functions {
            let params = function.parameters.iter()
                .enumerate()
                .map(|(i, (name, param_type))| {
                    let objc_type = self.rust_to_objc_type(param_type);
                    if i == 0 {
                        format!(":({}){}", objc_type, name)
                    } else {
                        format!(" {}:({}){}", name, objc_type, name)
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            
            let return_type = self.rust_to_objc_type(&function.return_type);
            
            impl_code.push_str(&format!(
                "- ({}){}{}{{\n",
                return_type, function.name, params
            ));
            
            let call_params = function.parameters.iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            
            if function.return_type != "void" {
                impl_code.push_str(&format!(
                    "    return {}({});\n",
                    function.name, call_params
                ));
            } else {
                impl_code.push_str(&format!(
                    "    {}({});\n",
                    function.name, call_params
                ));
            }
            
            impl_code.push_str("}\n\n");
        }
        
        impl_code.push_str("@end\n");
        impl_code
    }
    
    fn generate_jni_header(&self) -> String {
        let mut header = String::new();
        
        header.push_str("// JNI Bindings for Android\n");
        header.push_str("package com.example.rustlibrary;\n\n");
        
        header.push_str("public class RustLibrary {\n");
        header.push_str("    static {\n");
        header.push_str("        System.loadLibrary(\"rustlibrary\");\n");
        header.push_str("    }\n\n");
        
        for function in &self.functions {
            let params = function.parameters.iter()
                .map(|(name, param_type)| {
                    let java_type = self.rust_to_java_type(param_type);
                    format!("{} {}", java_type, name)
                })
                .collect::<Vec<_>>()
                .join(", ");
            
            let return_type = self.rust_to_java_type(&function.return_type);
            
            header.push_str(&format!(
                "    public static native {} {}({});\n",
                return_type, function.name, params
            ));
        }
        
        header.push_str("}\n");
        header
    }
    
    fn generate_jni_implementation(&self) -> String {
        let mut impl_code = String::new();
        
        impl_code.push_str("// JNI Implementation in Rust\n");
        impl_code.push_str("use jni::JNIEnv;\n");
        impl_code.push_str("use jni::objects::{JClass, JString};\n");
        impl_code.push_str("use jni::sys::{jstring, jint, jlong};\n\n");
        
        for function in &self.functions {
            let jni_name = format!("Java_com_example_rustlibrary_RustLibrary_{}", function.name);
            
            let mut params = vec![
                "env: JNIEnv".to_string(),
                "_class: JClass".to_string(),
            ];
            
            for (name, param_type) in &function.parameters {
                let jni_type = self.rust_to_jni_type(param_type);
                params.push(format!("{}: {}", name, jni_type));
            }
            
            let return_type = self.rust_to_jni_type(&function.return_type);
            
            impl_code.push_str(&format!(
                "#[no_mangle]\npub extern \"system\" fn {}({}) -> {} {{\n",
                jni_name, params.join(", "), return_type
            ));
            
            impl_code.push_str(&format!(
                "    // TODO: Implement {}\n",
                function.name
            ));
            
            if function.return_type != "void" {
                impl_code.push_str(&format!(
                    "    {} // Default return value\n",
                    self.get_jni_default_value(&return_type)
                ));
            }
            
            impl_code.push_str("}\n\n");
        }
        
        impl_code
    }
    
    fn generate_com_header(&self) -> String {
        "// COM bindings not implemented in this example".to_string()
    }
    
    fn generate_com_implementation(&self) -> String {
        "// COM implementation not implemented in this example".to_string()
    }
    
    fn rust_to_c_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" => "int32_t".to_string(),
            "i64" => "int64_t".to_string(),
            "u32" => "uint32_t".to_string(),
            "u64" => "uint64_t".to_string(),
            "f32" => "float".to_string(),
            "f64" => "double".to_string(),
            "bool" => "bool".to_string(),
            "String" => "const char*".to_string(),
            "&str" => "const char*".to_string(),
            "()" | "void" => "void".to_string(),
            _ => self.types.get(rust_type).cloned().unwrap_or_else(|| "void*".to_string()),
        }
    }
    
    fn rust_to_swift_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" => "Int32".to_string(),
            "i64" => "Int64".to_string(),
            "u32" => "UInt32".to_string(),
            "u64" => "UInt64".to_string(),
            "f32" => "Float".to_string(),
            "f64" => "Double".to_string(),
            "bool" => "Bool".to_string(),
            "String" | "&str" => "String".to_string(),
            "()" | "void" => "Void".to_string(),
            _ => "Any".to_string(),
        }
    }
    
    fn rust_to_objc_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" => "NSInteger".to_string(),
            "i64" => "long long".to_string(),
            "u32" => "NSUInteger".to_string(),
            "u64" => "unsigned long long".to_string(),
            "f32" => "float".to_string(),
            "f64" => "double".to_string(),
            "bool" => "BOOL".to_string(),
            "String" | "&str" => "NSString*".to_string(),
            "()" | "void" => "void".to_string(),
            _ => "id".to_string(),
        }
    }
    
    fn rust_to_java_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" => "int".to_string(),
            "i64" => "long".to_string(),
            "u32" => "int".to_string(),
            "u64" => "long".to_string(),
            "f32" => "float".to_string(),
            "f64" => "double".to_string(),
            "bool" => "boolean".to_string(),
            "String" | "&str" => "String".to_string(),
            "()" | "void" => "void".to_string(),
            _ => "Object".to_string(),
        }
    }
    
    fn rust_to_jni_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" => "jint".to_string(),
            "i64" => "jlong".to_string(),
            "u32" => "jint".to_string(),
            "u64" => "jlong".to_string(),
            "f32" => "jfloat".to_string(),
            "f64" => "jdouble".to_string(),
            "bool" => "jboolean".to_string(),
            "String" | "&str" => "jstring".to_string(),
            "()" | "void" => "()".to_string(),
            _ => "jobject".to_string(),
        }
    }
    
    fn get_default_value(&self, c_type: &str) -> String {
        match c_type {
            "int32_t" | "uint32_t" | "int64_t" | "uint64_t" => "0".to_string(),
            "float" | "double" => "0.0".to_string(),
            "bool" => "false".to_string(),
            "const char*" => "NULL".to_string(),
            _ => "NULL".to_string(),
        }
    }
    
    fn get_swift_default_value(&self, swift_type: &str) -> String {
        match swift_type {
            "Int32" | "Int64" | "UInt32" | "UInt64" => "0".to_string(),
            "Float" | "Double" => "0.0".to_string(),
            "Bool" => "false".to_string(),
            "String" => "\"\"".to_string(),
            _ => "nil".to_string(),
        }
    }
    
    fn get_jni_default_value(&self, jni_type: &str) -> String {
        match jni_type {
            "jint" | "jlong" => "0".to_string(),
            "jfloat" | "jdouble" => "0.0".to_string(),
            "jboolean" => "false as jboolean".to_string(),
            "jstring" => "std::ptr::null_mut()".to_string(),
            "()" => "".to_string(),
            _ => "std::ptr::null_mut()".to_string(),
        }
    }
}

/// 📱 Mobile FFI Helper
pub struct MobileFFI {
    ios_bindings: Option<BindingGenerator>,
    android_bindings: Option<BindingGenerator>,
    shared_functions: Vec<FunctionSignature>,
}

impl MobileFFI {
    pub fn new() -> Self {
        Self {
            ios_bindings: None,
            android_bindings: None,
            shared_functions: Vec::new(),
        }
    }
    
    pub fn setup_ios_bindings(&mut self) {
        let mut generator = BindingGenerator::new(NativePlatform::iOS, BindingType::Swift);
        generator.add_include("Foundation/Foundation.h");
        self.ios_bindings = Some(generator);
    }
    
    pub fn setup_android_bindings(&mut self) {
        let mut generator = BindingGenerator::new(NativePlatform::Android, BindingType::JNI);
        self.android_bindings = Some(generator);
    }
    
    pub fn add_shared_function(&mut self, function: FunctionSignature) {
        self.shared_functions.push(function.clone());
        
        if let Some(ref mut ios) = self.ios_bindings {
            ios.add_function(function.clone());
        }
        
        if let Some(ref mut android) = self.android_bindings {
            android.add_function(function);
        }
    }
    
    pub fn generate_all_bindings(&self) -> HashMap<String, String> {
        let mut bindings = HashMap::new();
        
        if let Some(ref ios) = self.ios_bindings {
            bindings.insert("ios_header".to_string(), ios.generate_header());
            bindings.insert("ios_implementation".to_string(), ios.generate_implementation());
        }
        
        if let Some(ref android) = self.android_bindings {
            bindings.insert("android_header".to_string(), android.generate_header());
            bindings.insert("android_implementation".to_string(), android.generate_implementation());
        }
        
        bindings
    }
}

/// 🔧 C FFI Examples

// Example C-compatible functions
#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn create_greeting(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let c_str = CStr::from_ptr(name);
        if let Ok(name_str) = c_str.to_str() {
            let greeting = format!("Hello, {}!", name_str);
            if let Ok(c_string) = CString::new(greeting) {
                return c_string.into_raw();
            }
        }
    }
    
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn process_array(arr: *const c_int, len: c_int) -> c_int {
    if arr.is_null() || len <= 0 {
        return 0;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(arr, len as usize);
        slice.iter().sum()
    }
}

/// 🔗 สาธิตการใช้งาน Native Bindings
pub fn demonstrate_native_bindings() {
    println!("🔗 === Native Platform Bindings Demo ===");
    
    // สร้าง Mobile FFI Helper
    let mut mobile_ffi = MobileFFI::new();
    mobile_ffi.setup_ios_bindings();
    mobile_ffi.setup_android_bindings();
    
    // เพิ่มฟังก์ชันที่ใช้ร่วมกัน
    let mut add_func = FunctionSignature::new("add_numbers", "i32");
    add_func.add_parameter("a", "i32");
    add_func.add_parameter("b", "i32");
    mobile_ffi.add_shared_function(add_func);
    
    let mut greeting_func = FunctionSignature::new("create_greeting", "String");
    greeting_func.add_parameter("name", "&str");
    mobile_ffi.add_shared_function(greeting_func);
    
    let mut process_func = FunctionSignature::new("process_array", "i32");
    process_func.add_parameter("arr", "*const i32");
    process_func.add_parameter("len", "i32");
    process_func.set_unsafe(true);
    mobile_ffi.add_shared_function(process_func);
    
    // สร้าง bindings สำหรับทุก platform
    let bindings = mobile_ffi.generate_all_bindings();
    
    println!("\n📱 Generated Bindings:");
    
    if let Some(ios_header) = bindings.get("ios_header") {
        println!("\n🍎 iOS Swift Header:");
        println!("{}", ios_header);
    }
    
    if let Some(android_header) = bindings.get("android_header") {
        println!("\n🤖 Android JNI Header:");
        println!("{}", android_header);
    }
    
    // สาธิต C FFI functions
    println!("\n🔧 Testing C FFI Functions:");
    
    // Test add_numbers
    let result = add_numbers(10, 20);
    println!("   add_numbers(10, 20) = {}", result);
    
    // Test create_greeting
    let name = CString::new("Rust Developer").unwrap();
    let greeting_ptr = create_greeting(name.as_ptr());
    if !greeting_ptr.is_null() {
        unsafe {
            let greeting_cstr = CStr::from_ptr(greeting_ptr);
            if let Ok(greeting_str) = greeting_cstr.to_str() {
                println!("   create_greeting(\"Rust Developer\") = \"{}\"", greeting_str);
            }
            free_string(greeting_ptr);
        }
    }
    
    // Test process_array
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = process_array(numbers.as_ptr(), numbers.len() as c_int);
    println!("   process_array([1,2,3,4,5]) = {}", sum);
    
    // Platform-specific examples
    println!("\n📱 Platform-specific Binding Examples:");
    show_ios_binding_example();
    show_android_binding_example();
    
    // Best practices
    println!("\n💡 Native Binding Best Practices:");
    show_binding_best_practices();
}

/// 🍎 iOS Binding Example
fn show_ios_binding_example() {
    println!("\n🍎 iOS Swift Integration Example:");
    
    let swift_code = r#"
// Swift code calling Rust
import Foundation

class RustBridge {
    // Load the Rust library
    private let rustLib = dlopen("librust_mobile.dylib", RTLD_LAZY)
    
    func addNumbers(_ a: Int32, _ b: Int32) -> Int32 {
        typealias AddFunction = @convention(c) (Int32, Int32) -> Int32
        
        guard let rustLib = rustLib,
              let addPtr = dlsym(rustLib, "add_numbers") else {
            return 0
        }
        
        let addFunc = unsafeBitCast(addPtr, to: AddFunction.self)
        return addFunc(a, b)
    }
    
    deinit {
        if let rustLib = rustLib {
            dlclose(rustLib)
        }
    }
}

// Usage
let bridge = RustBridge()
let result = bridge.addNumbers(10, 20)
print("Result: \(result)")
"#;
    
    println!("{}", swift_code);
}

/// 🤖 Android Binding Example
fn show_android_binding_example() {
    println!("\n🤖 Android JNI Integration Example:");
    
    let java_code = r#"
// Java code calling Rust via JNI
package com.example.rustmobile;

public class RustBridge {
    static {
        System.loadLibrary("rust_mobile");
    }
    
    // Native method declarations
    public static native int addNumbers(int a, int b);
    public static native String createGreeting(String name);
    public static native int processArray(int[] array);
    
    // Wrapper methods with error handling
    public static int safeAddNumbers(int a, int b) {
        try {
            return addNumbers(a, b);
        } catch (Exception e) {
            Log.e("RustBridge", "Error in addNumbers", e);
            return 0;
        }
    }
    
    public static String safeCreateGreeting(String name) {
        try {
            return createGreeting(name != null ? name : "");
        } catch (Exception e) {
            Log.e("RustBridge", "Error in createGreeting", e);
            return "Hello!";
        }
    }
}

// Usage in Activity
public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        int result = RustBridge.safeAddNumbers(10, 20);
        String greeting = RustBridge.safeCreateGreeting("Android User");
        
        Log.d("MainActivity", "Result: " + result);
        Log.d("MainActivity", "Greeting: " + greeting);
    }
}
"#;
    
    println!("{}", java_code);
}

/// 💡 Native Binding Best Practices
fn show_binding_best_practices() {
    let practices = vec![
        "🛡️ Always validate input parameters from native code",
        "🔒 Use safe Rust patterns even in FFI functions",
        "📝 Document memory ownership and lifetime rules",
        "🧪 Test bindings thoroughly on target devices",
        "⚡ Minimize data copying between Rust and native code",
        "🔄 Use consistent error handling patterns",
        "📦 Keep FFI surface area as small as possible",
        "🎯 Use code generation tools when possible",
        "🔍 Profile performance of cross-language calls",
        "📱 Consider platform-specific optimizations",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🛠️ Recommended Tools:");
    println!("   • cbindgen - Generate C headers from Rust");
    println!("   • bindgen - Generate Rust bindings from C headers");
    println!("   • cxx - Safe C++ bindings");
    println!("   • jni - Java Native Interface for Rust");
    println!("   • swift-bridge - Swift bindings generator");
    println!("   • safer_ffi - Safer FFI patterns");
    
    println!("\n📋 Build Configuration:");
    println!("   • Use cargo-ndk for Android builds");
    println!("   • Use cargo-lipo for iOS universal binaries");
    println!("   • Configure proper target architectures");
    println!("   • Set up cross-compilation toolchains");
    println!("   • Use conditional compilation for platform-specific code");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_signature() {
        let mut func = FunctionSignature::new("test_func", "i32");
        func.add_parameter("param1", "i32");
        func.add_parameter("param2", "&str");
        func.set_async(true);
        
        assert_eq!(func.name, "test_func");
        assert_eq!(func.return_type, "i32");
        assert_eq!(func.parameters.len(), 2);
        assert!(func.is_async);
    }
    
    #[test]
    fn test_binding_generator() {
        let mut generator = BindingGenerator::new(NativePlatform::iOS, BindingType::Swift);
        
        let mut func = FunctionSignature::new("add", "i32");
        func.add_parameter("a", "i32");
        func.add_parameter("b", "i32");
        
        generator.add_function(func);
        generator.add_include("Foundation/Foundation.h");
        
        let header = generator.generate_header();
        assert!(header.contains("add"));
        assert!(header.contains("Int32"));
    }
    
    #[test]
    fn test_c_ffi_functions() {
        // Test add_numbers
        let result = add_numbers(5, 3);
        assert_eq!(result, 8);
        
        // Test create_greeting
        let name = CString::new("Test").unwrap();
        let greeting_ptr = create_greeting(name.as_ptr());
        assert!(!greeting_ptr.is_null());
        
        unsafe {
            let greeting_cstr = CStr::from_ptr(greeting_ptr);
            let greeting_str = greeting_cstr.to_str().unwrap();
            assert!(greeting_str.contains("Hello, Test!"));
            free_string(greeting_ptr);
        }
        
        // Test process_array
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = process_array(numbers.as_ptr(), numbers.len() as c_int);
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_mobile_ffi() {
        let mut mobile_ffi = MobileFFI::new();
        mobile_ffi.setup_ios_bindings();
        mobile_ffi.setup_android_bindings();
        
        let mut func = FunctionSignature::new("test", "i32");
        func.add_parameter("x", "i32");
        mobile_ffi.add_shared_function(func);
        
        let bindings = mobile_ffi.generate_all_bindings();
        assert!(bindings.contains_key("ios_header"));
        assert!(bindings.contains_key("android_header"));
    }
}