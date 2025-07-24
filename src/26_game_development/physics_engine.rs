//! ⚡ Physics Engine - เครื่องยนต์ฟิสิกส์
//! 
//! โมดูลนี้สาธิตการสร้างระบบฟิสิกส์สำหรับเกม
//! รวมถึง collision detection, rigid body dynamics, และ particle systems
//! 
//! 🌍 "ฟิสิกส์ในเกมไม่จำเป็นต้องเหมือนจริง แค่ให้สนุก!"

use std::collections::HashMap;
use std::fmt;

/// 🎯 Vector 2D สำหรับตำแหน่ง, ความเร็ว, แรง
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// สร้าง vector ใหม่
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    /// Vector ศูนย์
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    
    /// คำนวณความยาวของ vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// คำนวณความยาวกำลังสอง (เร็วกว่า magnitude)
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    /// ทำให้ vector มีความยาว 1
    pub fn normalize(&self) -> Vec2 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vec2::new(self.x / mag, self.y / mag)
        } else {
            Vec2::ZERO
        }
    }
    
    /// คำนวณ dot product
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    /// คำนวณระยะห่างจาก vector อื่น
    pub fn distance_to(&self, other: &Vec2) -> f32 {
        (*self - *other).magnitude()
    }
    
    /// หมุน vector ตามมุม (radians)
    pub fn rotate(&self, angle: f32) -> Vec2 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Vec2::new(
            self.x * cos_a - self.y * sin_a,
            self.x * sin_a + self.y * cos_a
        )
    }
    
    /// จำกัดความยาวของ vector
    pub fn clamp_magnitude(&self, max_magnitude: f32) -> Vec2 {
        let mag = self.magnitude();
        if mag > max_magnitude {
            self.normalize() * max_magnitude
        } else {
            *self
        }
    }
}

// การดำเนินการทางคณิตศาสตร์สำหรับ Vec2
impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

/// 📦 Bounding Box สำหรับ collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox {
    /// สร้าง bounding box ใหม่
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    
    /// สร้างจากตำแหน่งกลางและขนาด
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = size / 2.0;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }
    
    /// ตรวจสอบว่าจุดอยู่ใน bounding box หรือไม่
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
    
    /// ตรวจสอบการชนกับ bounding box อื่น
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y
    }
    
    /// คำนวณพื้นที่
    pub fn area(&self) -> f32 {
        let size = self.max - self.min;
        size.x * size.y
    }
    
    /// ดึงตำแหน่งกลาง
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }
    
    /// ดึงขนาด
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }
}

/// ⭕ Circle Collider
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleCollider {
    pub center: Vec2,
    pub radius: f32,
}

impl CircleCollider {
    /// สร้าง circle collider ใหม่
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
    
    /// ตรวจสอบว่าจุดอยู่ในวงกลมหรือไม่
    pub fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance_to(&point) <= self.radius
    }
    
    /// ตรวจสอบการชนกับวงกลมอื่น
    pub fn intersects_circle(&self, other: &CircleCollider) -> bool {
        let distance = self.center.distance_to(&other.center);
        distance <= (self.radius + other.radius)
    }
    
    /// ตรวจสอบการชนกับ bounding box
    pub fn intersects_box(&self, bbox: &BoundingBox) -> bool {
        // หาจุดที่ใกล้ที่สุดใน bounding box
        let closest_x = self.center.x.clamp(bbox.min.x, bbox.max.x);
        let closest_y = self.center.y.clamp(bbox.min.y, bbox.max.y);
        let closest_point = Vec2::new(closest_x, closest_y);
        
        self.center.distance_to(&closest_point) <= self.radius
    }
    
    /// คำนวณ bounding box ของวงกลม
    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Vec2::new(self.center.x - self.radius, self.center.y - self.radius),
            Vec2::new(self.center.x + self.radius, self.center.y + self.radius)
        )
    }
}

/// 🎯 ประเภทของ Collider
#[derive(Debug, Clone, PartialEq)]
pub enum ColliderType {
    Box(BoundingBox),
    Circle(CircleCollider),
}

impl ColliderType {
    /// ตรวจสอบการชนกับ collider อื่น
    pub fn intersects(&self, other: &ColliderType) -> bool {
        match (self, other) {
            (ColliderType::Box(a), ColliderType::Box(b)) => a.intersects(b),
            (ColliderType::Circle(a), ColliderType::Circle(b)) => a.intersects_circle(b),
            (ColliderType::Box(bbox), ColliderType::Circle(circle)) |
            (ColliderType::Circle(circle), ColliderType::Box(bbox)) => {
                circle.intersects_box(bbox)
            }
        }
    }
    
    /// ดึง bounding box
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            ColliderType::Box(bbox) => *bbox,
            ColliderType::Circle(circle) => circle.bounding_box(),
        }
    }
}

/// 🏃 Rigid Body สำหรับฟิสิกส์
#[derive(Debug, Clone)]
pub struct RigidBody {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub restitution: f32, // ความยืดหยุ่นในการชน (0.0 - 1.0)
    pub friction: f32,    // แรงเสียดทาน (0.0 - 1.0)
    pub is_static: bool,  // วัตถุคงที่ (ไม่เคลื่อนที่)
    pub collider: ColliderType,
    pub forces: Vec<Vec2>, // แรงที่กระทำ
}

impl RigidBody {
    /// สร้าง rigid body ใหม่
    pub fn new(id: u32, position: Vec2, collider: ColliderType) -> Self {
        Self {
            id,
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            restitution: 0.5,
            friction: 0.1,
            is_static: false,
            collider,
            forces: Vec::new(),
        }
    }
    
    /// สร้าง static body (ไม่เคลื่อนที่)
    pub fn new_static(id: u32, position: Vec2, collider: ColliderType) -> Self {
        let mut body = Self::new(id, position, collider);
        body.is_static = true;
        body.mass = f32::INFINITY;
        body
    }
    
    /// เพิ่มแรง
    pub fn add_force(&mut self, force: Vec2) {
        if !self.is_static {
            self.forces.push(force);
        }
    }
    
    /// เพิ่ม impulse (แรงกระแทก)
    pub fn add_impulse(&mut self, impulse: Vec2) {
        if !self.is_static {
            self.velocity = self.velocity + impulse / self.mass;
        }
    }
    
    /// อัปเดตฟิสิกส์
    pub fn update(&mut self, delta_time: f32) {
        if self.is_static {
            return;
        }
        
        // คำนวณแรงรวม
        let total_force = self.forces.iter().fold(Vec2::ZERO, |acc, &force| acc + force);
        
        // F = ma -> a = F/m
        self.acceleration = total_force / self.mass;
        
        // อัปเดตความเร็ว: v = v0 + at
        self.velocity = self.velocity + self.acceleration * delta_time;
        
        // ใช้แรงเสียดทาน
        let friction_force = self.velocity * -self.friction;
        self.velocity = self.velocity + friction_force * delta_time;
        
        // อัปเดตตำแหน่ง: s = s0 + vt
        self.position = self.position + self.velocity * delta_time;
        
        // อัปเดตตำแหน่ง collider
        self.update_collider_position();
        
        // เคลียร์แรง
        self.forces.clear();
    }
    
    /// อัปเดตตำแหน่ง collider
    fn update_collider_position(&mut self) {
        match &mut self.collider {
            ColliderType::Circle(circle) => {
                circle.center = self.position;
            }
            ColliderType::Box(bbox) => {
                let size = bbox.size();
                *bbox = BoundingBox::from_center_size(self.position, size);
            }
        }
    }
    
    /// ตรวจสอบการชนกับ body อื่น
    pub fn intersects(&self, other: &RigidBody) -> bool {
        self.collider.intersects(&other.collider)
    }
}

/// 💥 ข้อมูลการชน
#[derive(Debug, Clone)]
pub struct Collision {
    pub body_a: u32,
    pub body_b: u32,
    pub contact_point: Vec2,
    pub normal: Vec2,      // ทิศทางการชน
    pub penetration: f32,  // ความลึกของการชน
}

/// ✨ Particle สำหรับ particle system
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub life_time: f32,
    pub max_life_time: f32,
    pub size: f32,
    pub color: (f32, f32, f32, f32), // RGBA
    pub is_alive: bool,
}

impl Particle {
    /// สร้าง particle ใหม่
    pub fn new(position: Vec2, velocity: Vec2, life_time: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vec2::ZERO,
            life_time,
            max_life_time: life_time,
            size: 1.0,
            color: (1.0, 1.0, 1.0, 1.0),
            is_alive: true,
        }
    }
    
    /// อัปเดต particle
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_alive {
            return;
        }
        
        // อัปเดตเวลา
        self.life_time -= delta_time;
        if self.life_time <= 0.0 {
            self.is_alive = false;
            return;
        }
        
        // อัปเดตฟิสิกส์
        self.velocity = self.velocity + self.acceleration * delta_time;
        self.position = self.position + self.velocity * delta_time;
        
        // อัปเดต alpha ตามอายุ
        let life_ratio = self.life_time / self.max_life_time;
        self.color.3 = life_ratio; // alpha
    }
}

/// ✨ Particle System
#[derive(Debug)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
    pub spawn_rate: f32,    // particles per second
    pub spawn_timer: f32,
    pub gravity: Vec2,
    pub wind: Vec2,
}

impl ParticleSystem {
    /// สร้าง particle system ใหม่
    pub fn new(max_particles: usize, spawn_rate: f32) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
            spawn_rate,
            spawn_timer: 0.0,
            gravity: Vec2::new(0.0, -9.81),
            wind: Vec2::ZERO,
        }
    }
    
    /// เพิ่ม particle
    pub fn emit(&mut self, position: Vec2, velocity: Vec2, life_time: f32) {
        if self.particles.len() < self.max_particles {
            let mut particle = Particle::new(position, velocity, life_time);
            particle.acceleration = self.gravity + self.wind;
            self.particles.push(particle);
        }
    }
    
    /// อัปเดต particle system
    pub fn update(&mut self, delta_time: f32) {
        // อัปเดต spawn timer
        self.spawn_timer += delta_time;
        
        // อัปเดต particles
        for particle in &mut self.particles {
            particle.acceleration = self.gravity + self.wind;
            particle.update(delta_time);
        }
        
        // ลบ particles ที่ตายแล้ว
        self.particles.retain(|p| p.is_alive);
    }
    
    /// ดึงจำนวน particles ที่มีชีวิต
    pub fn alive_count(&self) -> usize {
        self.particles.iter().filter(|p| p.is_alive).count()
    }
}

/// ⚡ Physics World - โลกฟิสิกส์
#[derive(Debug)]
pub struct PhysicsWorld {
    pub bodies: HashMap<u32, RigidBody>,
    pub gravity: Vec2,
    pub time_step: f32,
    pub collision_iterations: u32,
    pub particle_systems: Vec<ParticleSystem>,
}

impl PhysicsWorld {
    /// สร้าง physics world ใหม่
    pub fn new() -> Self {
        Self {
            bodies: HashMap::new(),
            gravity: Vec2::new(0.0, -9.81), // แรงโน้มถ่วง
            time_step: 1.0 / 60.0, // 60 FPS
            collision_iterations: 4,
            particle_systems: Vec::new(),
        }
    }
    
    /// เพิ่ม rigid body
    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.insert(body.id, body);
    }
    
    /// ลบ rigid body
    pub fn remove_body(&mut self, id: u32) {
        self.bodies.remove(&id);
    }
    
    /// ดึง rigid body
    pub fn get_body(&self, id: u32) -> Option<&RigidBody> {
        self.bodies.get(&id)
    }
    
    /// ดึง rigid body (mutable)
    pub fn get_body_mut(&mut self, id: u32) -> Option<&mut RigidBody> {
        self.bodies.get_mut(&id)
    }
    
    /// เพิ่ม particle system
    pub fn add_particle_system(&mut self, system: ParticleSystem) {
        self.particle_systems.push(system);
    }
    
    /// อัปเดตโลกฟิสิกส์
    pub fn update(&mut self, delta_time: f32) {
        // ใช้ fixed time step
        let mut remaining_time = delta_time;
        
        while remaining_time > 0.0 {
            let step = remaining_time.min(self.time_step);
            
            // ใช้แรงโน้มถ่วง
            for body in self.bodies.values_mut() {
                if !body.is_static {
                    body.add_force(self.gravity * body.mass);
                }
            }
            
            // อัปเดต rigid bodies
            for body in self.bodies.values_mut() {
                body.update(step);
            }
            
            // ตรวจสอบการชนและแก้ไข
            for _ in 0..self.collision_iterations {
                let collisions = self.detect_collisions();
                self.resolve_collisions(&collisions);
            }
            
            // อัปเดต particle systems
            for system in &mut self.particle_systems {
                system.update(step);
            }
            
            remaining_time -= step;
        }
    }
    
    /// ตรวจสอบการชน
    fn detect_collisions(&self) -> Vec<Collision> {
        let mut collisions = Vec::new();
        let body_ids: Vec<u32> = self.bodies.keys().cloned().collect();
        
        for i in 0..body_ids.len() {
            for j in (i + 1)..body_ids.len() {
                let id_a = body_ids[i];
                let id_b = body_ids[j];
                
                if let (Some(body_a), Some(body_b)) = (self.bodies.get(&id_a), self.bodies.get(&id_b)) {
                    if body_a.intersects(body_b) {
                        // สร้างข้อมูลการชน (simplified)
                        let collision = Collision {
                            body_a: id_a,
                            body_b: id_b,
                            contact_point: (body_a.position + body_b.position) / 2.0,
                            normal: (body_b.position - body_a.position).normalize(),
                            penetration: 0.1, // simplified
                        };
                        collisions.push(collision);
                    }
                }
            }
        }
        
        collisions
    }
    
    /// แก้ไขการชน
    fn resolve_collisions(&mut self, collisions: &[Collision]) {
        for collision in collisions {
            if let (Some(body_a), Some(body_b)) = (
                self.bodies.get(&collision.body_a).cloned(),
                self.bodies.get(&collision.body_b).cloned()
            ) {
                // คำนวณ relative velocity
                let relative_velocity = body_b.velocity - body_a.velocity;
                let velocity_along_normal = relative_velocity.dot(&collision.normal);
                
                // ไม่แก้ไขถ้าวัตถุกำลังแยกออกจากกัน
                if velocity_along_normal > 0.0 {
                    continue;
                }
                
                // คำนวณ restitution
                let restitution = (body_a.restitution + body_b.restitution) / 2.0;
                
                // คำนวณ impulse magnitude
                let mut impulse_magnitude = -(1.0 + restitution) * velocity_along_normal;
                impulse_magnitude /= 1.0 / body_a.mass + 1.0 / body_b.mass;
                
                // ใช้ impulse
                let impulse = collision.normal * impulse_magnitude;
                
                if let Some(body_a_mut) = self.bodies.get_mut(&collision.body_a) {
                    body_a_mut.add_impulse(impulse * -1.0);
                }
                
                if let Some(body_b_mut) = self.bodies.get_mut(&collision.body_b) {
                    body_b_mut.add_impulse(impulse);
                }
                
                // แก้ไข penetration
                let correction = collision.normal * (collision.penetration / (1.0 / body_a.mass + 1.0 / body_b.mass)) * 0.8;
                
                if let Some(body_a_mut) = self.bodies.get_mut(&collision.body_a) {
                    if !body_a_mut.is_static {
                        body_a_mut.position = body_a_mut.position - correction / body_a_mut.mass;
                        body_a_mut.update_collider_position();
                    }
                }
                
                if let Some(body_b_mut) = self.bodies.get_mut(&collision.body_b) {
                    if !body_b_mut.is_static {
                        body_b_mut.position = body_b_mut.position + correction / body_b_mut.mass;
                        body_b_mut.update_collider_position();
                    }
                }
            }
        }
    }
    
    /// ดึงสถิติ
    pub fn get_stats(&self) -> PhysicsStats {
        let total_particles: usize = self.particle_systems.iter()
            .map(|s| s.alive_count())
            .sum();
            
        PhysicsStats {
            rigid_bodies: self.bodies.len(),
            particle_systems: self.particle_systems.len(),
            total_particles,
            gravity: self.gravity,
        }
    }
}

/// 📊 สถิติฟิสิกส์
#[derive(Debug, Clone)]
pub struct PhysicsStats {
    pub rigid_bodies: usize,
    pub particle_systems: usize,
    pub total_particles: usize,
    pub gravity: Vec2,
}

/// ⚡ สาธิตการใช้งาน Physics Engine
pub fn demonstrate_physics_engine() {
    println!("⚡ === Physics Engine Demo ===");
    
    // สร้าง physics world
    let mut world = PhysicsWorld::new();
    println!("🌍 Created physics world with gravity: {:?}", world.gravity);
    
    // สร้าง rigid bodies
    println!("\n📦 Creating rigid bodies:");
    
    // พื้น (static)
    let ground = RigidBody::new_static(
        1,
        Vec2::new(0.0, -5.0),
        ColliderType::Box(BoundingBox::from_center_size(Vec2::new(0.0, -5.0), Vec2::new(20.0, 1.0)))
    );
    world.add_body(ground);
    println!("🏔️ Added ground at y=-5.0");
    
    // ลูกบอล
    let mut ball = RigidBody::new(
        2,
        Vec2::new(0.0, 10.0),
        ColliderType::Circle(CircleCollider::new(Vec2::new(0.0, 10.0), 1.0))
    );
    ball.restitution = 0.8; // ลูกบอลเด้ง
    ball.friction = 0.1;
    world.add_body(ball);
    println!("⚽ Added bouncing ball at y=10.0");
    
    // กล่อง
    let mut box_body = RigidBody::new(
        3,
        Vec2::new(3.0, 8.0),
        ColliderType::Box(BoundingBox::from_center_size(Vec2::new(3.0, 8.0), Vec2::new(2.0, 2.0)))
    );
    box_body.restitution = 0.3;
    box_body.friction = 0.5;
    world.add_body(box_body);
    println!("📦 Added box at (3.0, 8.0)");
    
    // สร้าง particle system
    println!("\n✨ Creating particle system:");
    let mut particles = ParticleSystem::new(100, 50.0);
    particles.gravity = Vec2::new(0.0, -5.0);
    particles.wind = Vec2::new(1.0, 0.0);
    
    // เพิ่ม particles
    for i in 0..20 {
        let angle = (i as f32) * 0.314; // รอบๆ วงกลม
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 5.0;
        particles.emit(Vec2::new(0.0, 5.0), velocity, 3.0);
    }
    
    world.add_particle_system(particles);
    println!("✨ Added particle system with 20 particles");
    
    // จำลองฟิสิกส์
    println!("\n🎬 Simulating physics:");
    for frame in 0..10 {
        println!("\n--- Frame {} ---", frame + 1);
        
        // อัปเดตโลก
        world.update(1.0 / 60.0); // 60 FPS
        
        // แสดงตำแหน่งของวัตถุ
        if let Some(ball) = world.get_body(2) {
            println!("⚽ Ball: pos({:.2}, {:.2}) vel({:.2}, {:.2})", 
                    ball.position.x, ball.position.y, ball.velocity.x, ball.velocity.y);
        }
        
        if let Some(box_body) = world.get_body(3) {
            println!("📦 Box: pos({:.2}, {:.2}) vel({:.2}, {:.2})", 
                    box_body.position.x, box_body.position.y, box_body.position.x, box_body.velocity.y);
        }
        
        // แสดงสถิติ
        let stats = world.get_stats();
        println!("📊 Stats: {} bodies, {} particle systems, {} particles",
                stats.rigid_bodies, stats.particle_systems, stats.total_particles);
        
        // เพิ่ม impulse ให้ลูกบอลในบางเฟรม
        if frame == 3 {
            if let Some(ball) = world.get_body_mut(2) {
                ball.add_impulse(Vec2::new(5.0, 2.0));
                println!("💥 Applied impulse to ball!");
            }
        }
    }
    
    // แสดง best practices
    println!("\n💡 Physics Engine Best Practices:");
    show_physics_best_practices();
}

/// 💡 Physics Best Practices
fn show_physics_best_practices() {
    let practices = vec![
        "⏱️ Use fixed time steps for consistent simulation",
        "🔄 Implement proper collision detection algorithms",
        "📐 Use spatial partitioning for performance",
        "⚖️ Balance accuracy vs performance",
        "🎯 Implement continuous collision detection for fast objects",
        "🔧 Tune physics parameters for gameplay feel",
        "💾 Consider object pooling for particles",
        "🌊 Implement proper fluid dynamics if needed",
        "🎪 Use constraint solvers for complex interactions",
        "📊 Profile physics performance regularly",
        "🎮 Make physics feel good, not necessarily realistic",
        "🔒 Implement proper sleeping/waking for static objects",
        "🎯 Use broad-phase collision detection",
        "⚡ Optimize hot paths in physics loops",
        "🧪 Test edge cases thoroughly",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Physics Libraries:");
    println!("   • rapier - Fast 2D/3D physics engine");
    println!("   • nphysics - Pure Rust physics engine");
    println!("   • bullet3-rs - Bullet physics bindings");
    println!("   • box2d-rs - Box2D bindings");
    println!("   • parry - Collision detection library");
    println!("   • salva - Fluid simulation");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(1.0, 2.0);
        
        assert_eq!(v1.magnitude(), 5.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v1 - v2, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(6.0, 8.0));
        assert_eq!(v1.dot(&v2), 11.0); // 3*1 + 4*2
    }
    
    #[test]
    fn test_bounding_box_intersection() {
        let box1 = BoundingBox::new(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let box2 = BoundingBox::new(Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0));
        let box3 = BoundingBox::new(Vec2::new(3.0, 3.0), Vec2::new(5.0, 5.0));
        
        assert!(box1.intersects(&box2));
        assert!(!box1.intersects(&box3));
    }
    
    #[test]
    fn test_circle_collision() {
        let circle1 = CircleCollider::new(Vec2::new(0.0, 0.0), 1.0);
        let circle2 = CircleCollider::new(Vec2::new(1.5, 0.0), 1.0);
        let circle3 = CircleCollider::new(Vec2::new(3.0, 0.0), 1.0);
        
        assert!(circle1.intersects_circle(&circle2));
        assert!(!circle1.intersects_circle(&circle3));
    }
    
    #[test]
    fn test_rigid_body() {
        let mut body = RigidBody::new(
            1,
            Vec2::new(0.0, 0.0),
            ColliderType::Circle(CircleCollider::new(Vec2::new(0.0, 0.0), 1.0))
        );
        
        body.add_force(Vec2::new(10.0, 0.0));
        body.update(1.0);
        
        assert!(body.velocity.x > 0.0); // Should have moved
        assert!(body.position.x > 0.0);
    }
    
    #[test]
    fn test_particle() {
        let mut particle = Particle::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            1.0
        );
        
        particle.update(0.5);
        
        assert_eq!(particle.position.x, 0.5);
        assert_eq!(particle.life_time, 0.5);
        assert!(particle.is_alive);
        
        particle.update(0.6);
        assert!(!particle.is_alive);
    }
    
    #[test]
    fn test_physics_world() {
        let mut world = PhysicsWorld::new();
        
        let body = RigidBody::new(
            1,
            Vec2::new(0.0, 10.0),
            ColliderType::Circle(CircleCollider::new(Vec2::new(0.0, 10.0), 1.0))
        );
        
        world.add_body(body);
        assert_eq!(world.bodies.len(), 1);
        
        world.update(1.0 / 60.0);
        
        // Body should have fallen due to gravity
        if let Some(body) = world.get_body(1) {
            assert!(body.position.y < 10.0);
        }
    }
}

// 🌍 "ฟิสิกส์ในเกมไม่ใช่ฟิสิกส์จริง
//     มันคือฟิสิกส์ที่ทำให้เกมสนุก!
//     บางทีลูกบอลเด้งสูงกว่าที่ควรจะเป็น
//     แต่ถ้ามันทำให้ผู้เล่นมีความสุข ก็พอแล้ว! ⚡🎮"