//! 🎨 Graphics Rendering - ระบบการเรนเดอร์กราฟิก
//! 
//! โมดูลนี้สาธิตการสร้างระบบเรนเดอร์กราฟิกสำหรับเกม
//! รวมถึง 2D/3D rendering, shaders, textures, และ animations
//! 
//! 🎭 "การเรนเดอร์เหมือนการวาดภาพ แต่คอมพิวเตอร์เป็นคนวาด!"

use std::collections::HashMap;
use std::fmt;

/// 🎨 ประเภทของ Renderer
#[derive(Debug, Clone, PartialEq)]
pub enum RendererType {
    Software,    // 🐌 ช้าแต่เข้าใจง่าย
    OpenGL,      // 🔥 เร็วและเก่าแก่
    Vulkan,      // ⚡ เร็วมากแต่ซับซ้อน
    DirectX,     // 🪟 สำหรับ Windows
    Metal,       // 🍎 สำหรับ Apple
    WebGL,       // 🌐 สำหรับเว็บ
}

impl fmt::Display for RendererType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RendererType::Software => write!(f, "Software (CPU-based)"),
            RendererType::OpenGL => write!(f, "OpenGL (Cross-platform)"),
            RendererType::Vulkan => write!(f, "Vulkan (High-performance)"),
            RendererType::DirectX => write!(f, "DirectX (Windows)"),
            RendererType::Metal => write!(f, "Metal (Apple)"),
            RendererType::WebGL => write!(f, "WebGL (Browser)"),
        }
    }
}

/// 🎯 จุดในพื้นที่ 2D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// 📏 คำนวณระยะห่างจากจุดอื่น
    pub fn distance_to(&self, other: &Point2D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 🎯 จุดในพื้นที่ 3D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// 🌈 สี RGBA
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32, // 0.0 - 1.0
    pub g: f32, // 0.0 - 1.0
    pub b: f32, // 0.0 - 1.0
    pub a: f32, // 0.0 - 1.0 (alpha/transparency)
}

impl Color {
    /// สร้างสีใหม่
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// สร้างสีจากค่า RGB (alpha = 1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
    
    /// สีพื้นฐาน
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    
    /// ผสมสีกับสีอื่น
    pub fn blend(&self, other: &Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let inv_factor = 1.0 - factor;
        
        Color::new(
            self.r * inv_factor + other.r * factor,
            self.g * inv_factor + other.g * factor,
            self.b * inv_factor + other.b * factor,
            self.a * inv_factor + other.a * factor,
        )
    }
}

/// 🖼️ Texture (พื้นผิว)
#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextureFormat {
    RGB,   // 3 bytes per pixel
    RGBA,  // 4 bytes per pixel
    Grayscale, // 1 byte per pixel
}

impl Texture {
    /// สร้าง texture ใหม่
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        let bytes_per_pixel = match format {
            TextureFormat::RGB => 3,
            TextureFormat::RGBA => 4,
            TextureFormat::Grayscale => 1,
        };
        
        let data_size = (width * height * bytes_per_pixel) as usize;
        
        Self {
            id: rand::random(), // ในการใช้งานจริงจะได้จาก GPU
            width,
            height,
            format,
            data: vec![0; data_size],
        }
    }
    
    /// สร้าง texture สีเดียว
    pub fn solid_color(width: u32, height: u32, color: Color) -> Self {
        let mut texture = Self::new(width, height, TextureFormat::RGBA);
        
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;
        
        for i in (0..texture.data.len()).step_by(4) {
            texture.data[i] = r;
            texture.data[i + 1] = g;
            texture.data[i + 2] = b;
            texture.data[i + 3] = a;
        }
        
        texture
    }
    
    /// สร้าง checkerboard pattern
    pub fn checkerboard(width: u32, height: u32, size: u32) -> Self {
        let mut texture = Self::new(width, height, TextureFormat::RGBA);
        
        for y in 0..height {
            for x in 0..width {
                let checker_x = (x / size) % 2;
                let checker_y = (y / size) % 2;
                let is_white = (checker_x + checker_y) % 2 == 0;
                
                let color = if is_white { 255 } else { 0 };
                
                let index = ((y * width + x) * 4) as usize;
                texture.data[index] = color;     // R
                texture.data[index + 1] = color; // G
                texture.data[index + 2] = color; // B
                texture.data[index + 3] = 255;   // A
            }
        }
        
        texture
    }
}

/// 🎭 Shader (โปรแกรมที่รันบน GPU)
#[derive(Debug, Clone)]
pub struct Shader {
    pub id: u32,
    pub vertex_source: String,
    pub fragment_source: String,
    pub uniforms: HashMap<String, ShaderUniform>,
}

#[derive(Debug, Clone)]
pub enum ShaderUniform {
    Float(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    Matrix4([f32; 16]),
    Texture(u32),
}

impl Shader {
    /// สร้าง shader ใหม่
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        Self {
            id: rand::random(),
            vertex_source: vertex_source.to_string(),
            fragment_source: fragment_source.to_string(),
            uniforms: HashMap::new(),
        }
    }
    
    /// ตั้งค่า uniform
    pub fn set_uniform(&mut self, name: &str, value: ShaderUniform) {
        self.uniforms.insert(name.to_string(), value);
    }
    
    /// สร้าง basic vertex shader
    pub fn basic_vertex_shader() -> String {
        r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        
        out vec2 TexCoord;
        
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;
        
        void main() {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }
        "#.to_string()
    }
    
    /// สร้าง basic fragment shader
    pub fn basic_fragment_shader() -> String {
        r#"
        #version 330 core
        out vec4 FragColor;
        
        in vec2 TexCoord;
        
        uniform sampler2D ourTexture;
        uniform vec4 color;
        
        void main() {
            FragColor = texture(ourTexture, TexCoord) * color;
        }
        "#.to_string()
    }
}

/// 🎬 Animation Frame
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub texture_id: u32,
    pub duration: f32, // seconds
    pub offset_x: f32,
    pub offset_y: f32,
}

/// 🎬 Animation
#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<AnimationFrame>,
    pub current_frame: usize,
    pub elapsed_time: f32,
    pub is_looping: bool,
    pub is_playing: bool,
}

impl Animation {
    /// สร้าง animation ใหม่
    pub fn new(name: &str, frames: Vec<AnimationFrame>) -> Self {
        Self {
            name: name.to_string(),
            frames,
            current_frame: 0,
            elapsed_time: 0.0,
            is_looping: true,
            is_playing: false,
        }
    }
    
    /// เริ่มเล่น animation
    pub fn play(&mut self) {
        self.is_playing = true;
        self.current_frame = 0;
        self.elapsed_time = 0.0;
    }
    
    /// หยุด animation
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
        self.elapsed_time = 0.0;
    }
    
    /// อัปเดต animation
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_playing || self.frames.is_empty() {
            return;
        }
        
        self.elapsed_time += delta_time;
        
        let current_frame_duration = self.frames[self.current_frame].duration;
        
        if self.elapsed_time >= current_frame_duration {
            self.elapsed_time -= current_frame_duration;
            self.current_frame += 1;
            
            if self.current_frame >= self.frames.len() {
                if self.is_looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    self.is_playing = false;
                }
            }
        }
    }
    
    /// ดึง frame ปัจจุบัน
    pub fn current_frame(&self) -> Option<&AnimationFrame> {
        self.frames.get(self.current_frame)
    }
}

/// 🎨 Graphics Renderer
#[derive(Debug)]
pub struct GraphicsRenderer {
    pub renderer_type: RendererType,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub clear_color: Color,
    pub textures: HashMap<u32, Texture>,
    pub shaders: HashMap<u32, Shader>,
    pub draw_calls: u32,
    pub triangles_rendered: u32,
}

impl GraphicsRenderer {
    /// สร้าง renderer ใหม่
    pub fn new(renderer_type: RendererType, width: u32, height: u32) -> Self {
        Self {
            renderer_type,
            viewport_width: width,
            viewport_height: height,
            clear_color: Color::BLACK,
            textures: HashMap::new(),
            shaders: HashMap::new(),
            draw_calls: 0,
            triangles_rendered: 0,
        }
    }
    
    /// ตั้งค่าสีพื้นหลัง
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
        println!("🎨 Set clear color to RGBA({:.2}, {:.2}, {:.2}, {:.2})", 
                color.r, color.g, color.b, color.a);
    }
    
    /// เคลียร์หน้าจอ
    pub fn clear(&mut self) {
        println!("🧹 Clearing screen with color {:?}", self.clear_color);
        // ในการใช้งานจริงจะเคลียร์ framebuffer
    }
    
    /// โหลด texture
    pub fn load_texture(&mut self, texture: Texture) -> u32 {
        let id = texture.id;
        println!("📷 Loading texture {} ({}x{}, {:?})", 
                id, texture.width, texture.height, texture.format);
        self.textures.insert(id, texture);
        id
    }
    
    /// โหลด shader
    pub fn load_shader(&mut self, shader: Shader) -> u32 {
        let id = shader.id;
        println!("🎭 Loading shader {}", id);
        self.shaders.insert(id, shader);
        id
    }
    
    /// วาดสี่เหลี่ยม
    pub fn draw_quad(&mut self, position: Point2D, size: Point2D, color: Color) {
        println!("🔲 Drawing quad at ({:.1}, {:.1}) size ({:.1}, {:.1}) color {:?}", 
                position.x, position.y, size.x, size.y, color);
        
        self.draw_calls += 1;
        self.triangles_rendered += 2; // quad = 2 triangles
    }
    
    /// วาดสี่เหลี่ยมพร้อม texture
    pub fn draw_textured_quad(&mut self, position: Point2D, size: Point2D, texture_id: u32) {
        if let Some(texture) = self.textures.get(&texture_id) {
            println!("🖼️ Drawing textured quad at ({:.1}, {:.1}) size ({:.1}, {:.1}) with texture {}", 
                    position.x, position.y, size.x, size.y, texture_id);
        } else {
            println!("❌ Texture {} not found!", texture_id);
            return;
        }
        
        self.draw_calls += 1;
        self.triangles_rendered += 2;
    }
    
    /// วาดวงกลม
    pub fn draw_circle(&mut self, center: Point2D, radius: f32, color: Color, segments: u32) {
        println!("⭕ Drawing circle at ({:.1}, {:.1}) radius {:.1} with {} segments", 
                center.x, center.y, radius, segments);
        
        self.draw_calls += 1;
        self.triangles_rendered += segments; // circle = fan of triangles
    }
    
    /// วาดเส้น
    pub fn draw_line(&mut self, start: Point2D, end: Point2D, color: Color, thickness: f32) {
        println!("📏 Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1}) thickness {:.1}", 
                start.x, start.y, end.x, end.y, thickness);
        
        self.draw_calls += 1;
        self.triangles_rendered += 2; // line = thick quad = 2 triangles
    }
    
    /// วาดข้อความ (จำลอง)
    pub fn draw_text(&mut self, text: &str, position: Point2D, size: f32, color: Color) {
        println!("📝 Drawing text '{}' at ({:.1}, {:.1}) size {:.1}", 
                text, position.x, position.y, size);
        
        self.draw_calls += 1;
        self.triangles_rendered += text.len() as u32 * 2; // each char = 2 triangles
    }
    
    /// เริ่มต้น frame ใหม่
    pub fn begin_frame(&mut self) {
        self.draw_calls = 0;
        self.triangles_rendered = 0;
        self.clear();
    }
    
    /// จบ frame
    pub fn end_frame(&self) {
        println!("🎬 Frame complete: {} draw calls, {} triangles", 
                self.draw_calls, self.triangles_rendered);
    }
    
    /// ดึงสถิติการเรนเดอร์
    pub fn get_render_stats(&self) -> RenderStats {
        RenderStats {
            draw_calls: self.draw_calls,
            triangles_rendered: self.triangles_rendered,
            textures_loaded: self.textures.len() as u32,
            shaders_loaded: self.shaders.len() as u32,
        }
    }
}

/// 📊 สถิติการเรนเดอร์
#[derive(Debug, Clone)]
pub struct RenderStats {
    pub draw_calls: u32,
    pub triangles_rendered: u32,
    pub textures_loaded: u32,
    pub shaders_loaded: u32,
}

/// 🎨 สาธิตการใช้งาน Graphics Rendering
pub fn demonstrate_graphics_rendering() {
    println!("🎨 === Graphics Rendering Demo ===");
    
    // สร้าง renderer
    let mut renderer = GraphicsRenderer::new(RendererType::OpenGL, 1920, 1080);
    println!("🖥️ Created {} renderer ({}x{})", 
            renderer.renderer_type, renderer.viewport_width, renderer.viewport_height);
    
    // ตั้งค่าสีพื้นหลัง
    renderer.set_clear_color(Color::new(0.2, 0.3, 0.4, 1.0));
    
    // สร้าง textures
    println!("\n📷 Creating textures:");
    let red_texture = Texture::solid_color(64, 64, Color::RED);
    let checker_texture = Texture::checkerboard(128, 128, 16);
    
    let red_id = renderer.load_texture(red_texture);
    let checker_id = renderer.load_texture(checker_texture);
    
    // สร้าง shader
    println!("\n🎭 Creating shaders:");
    let basic_shader = Shader::new(
        &Shader::basic_vertex_shader(),
        &Shader::basic_fragment_shader()
    );
    let shader_id = renderer.load_shader(basic_shader);
    
    // สร้าง animation
    println!("\n🎬 Creating animation:");
    let frames = vec![
        AnimationFrame { texture_id: red_id, duration: 0.5, offset_x: 0.0, offset_y: 0.0 },
        AnimationFrame { texture_id: checker_id, duration: 0.5, offset_x: 10.0, offset_y: 0.0 },
    ];
    let mut animation = Animation::new("test_anim", frames);
    animation.play();
    
    // จำลองการเรนเดอร์หลาย frame
    println!("\n🎬 Rendering frames:");
    for frame in 0..5 {
        println!("\n--- Frame {} ---", frame + 1);
        renderer.begin_frame();
        
        // อัปเดต animation
        animation.update(0.016); // 60 FPS = 16ms per frame
        
        // วาดพื้นหลัง
        renderer.draw_textured_quad(
            Point2D::new(0.0, 0.0), 
            Point2D::new(1920.0, 1080.0), 
            checker_id
        );
        
        // วาด sprite ที่เคลื่อนไหว
        if let Some(current_frame) = animation.current_frame() {
            renderer.draw_textured_quad(
                Point2D::new(100.0 + current_frame.offset_x, 100.0 + current_frame.offset_y),
                Point2D::new(64.0, 64.0),
                current_frame.texture_id
            );
        }
        
        // วาดรูปทรงต่างๆ
        renderer.draw_quad(
            Point2D::new(200.0, 200.0), 
            Point2D::new(100.0, 100.0), 
            Color::GREEN
        );
        
        renderer.draw_circle(
            Point2D::new(400.0, 300.0), 
            50.0, 
            Color::BLUE, 
            32
        );
        
        renderer.draw_line(
            Point2D::new(500.0, 100.0), 
            Point2D::new(600.0, 200.0), 
            Color::YELLOW, 
            5.0
        );
        
        renderer.draw_text(
            "Hello, Graphics!", 
            Point2D::new(50.0, 50.0), 
            24.0, 
            Color::WHITE
        );
        
        renderer.end_frame();
        
        // แสดงสถิติ
        let stats = renderer.get_render_stats();
        println!("📊 Stats: {} draw calls, {} triangles, {} textures, {} shaders",
                stats.draw_calls, stats.triangles_rendered, 
                stats.textures_loaded, stats.shaders_loaded);
    }
    
    // แสดง best practices
    println!("\n💡 Graphics Rendering Best Practices:");
    show_graphics_best_practices();
}

/// 💡 Graphics Best Practices
fn show_graphics_best_practices() {
    let practices = vec![
        "🎯 Batch similar draw calls together",
        "📦 Use texture atlases to reduce texture switches",
        "🔄 Implement object pooling for temporary objects",
        "📊 Profile GPU performance regularly",
        "🎨 Use appropriate texture formats and sizes",
        "⚡ Minimize state changes between draw calls",
        "🗜️ Compress textures when possible",
        "🎭 Reuse shaders across similar objects",
        "📱 Consider mobile GPU limitations",
        "🔧 Use GPU debugging tools",
        "💾 Manage VRAM usage carefully",
        "🎬 Implement frustum culling",
        "📐 Use level-of-detail (LOD) systems",
        "🌟 Implement efficient particle systems",
        "🎪 Use instanced rendering for repeated objects",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Graphics Libraries:");
    println!("   • wgpu - Modern cross-platform graphics");
    println!("   • glium - Safe OpenGL wrapper");
    println!("   • vulkano - Safe Vulkan wrapper");
    println!("   • gfx-hal - Hardware abstraction layer");
    println!("   • pixels - Simple pixel buffer");
    println!("   • miniquad - Lightweight graphics");
    println!("   • macroquad - Simple game graphics");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point2d() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        
        assert_eq!(p1.distance_to(&p2), 5.0); // 3-4-5 triangle
    }
    
    #[test]
    fn test_color_blend() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let purple = red.blend(&blue, 0.5);
        
        assert_eq!(purple.r, 0.5);
        assert_eq!(purple.g, 0.0);
        assert_eq!(purple.b, 0.5);
    }
    
    #[test]
    fn test_animation() {
        let frames = vec![
            AnimationFrame { texture_id: 1, duration: 0.5, offset_x: 0.0, offset_y: 0.0 },
            AnimationFrame { texture_id: 2, duration: 0.5, offset_x: 10.0, offset_y: 0.0 },
        ];
        
        let mut animation = Animation::new("test", frames);
        animation.play();
        
        assert_eq!(animation.current_frame, 0);
        
        animation.update(0.6); // Should advance to frame 1
        assert_eq!(animation.current_frame, 1);
    }
    
    #[test]
    fn test_texture_creation() {
        let texture = Texture::solid_color(32, 32, Color::RED);
        
        assert_eq!(texture.width, 32);
        assert_eq!(texture.height, 32);
        assert_eq!(texture.format, TextureFormat::RGBA);
        assert_eq!(texture.data.len(), 32 * 32 * 4); // RGBA = 4 bytes per pixel
    }
    
    #[test]
    fn test_renderer() {
        let mut renderer = GraphicsRenderer::new(RendererType::OpenGL, 800, 600);
        
        assert_eq!(renderer.viewport_width, 800);
        assert_eq!(renderer.viewport_height, 600);
        assert_eq!(renderer.draw_calls, 0);
        
        renderer.begin_frame();
        renderer.draw_quad(Point2D::new(0.0, 0.0), Point2D::new(100.0, 100.0), Color::RED);
        
        assert_eq!(renderer.draw_calls, 1);
        assert_eq!(renderer.triangles_rendered, 2);
    }
}

// 🎭 "การเรนเดอร์กราฟิกเหมือนการวาดภาพดิจิทัล
//     แต่แทนที่จะใช้พู่กัน เราใช้คณิตศาสตร์!
//     และแทนที่จะวาดทีละจุด เราวาดทีละล้านจุด!
//     GPU คือศิลปินที่วาดเร็วที่สุดในโลก! 🎨⚡"