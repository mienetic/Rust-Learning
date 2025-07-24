//! 🏗️ ECS Architecture - Entity Component System
//! 
//! โมดูลนี้สาธิตการสร้างระบบ ECS (Entity Component System)
//! ซึ่งเป็น architectural pattern ที่นิยมใช้ในการพัฒนาเกม
//! 
//! 🎭 "ECS: Entity มีตัวตน Component มีข้อมูล System มีพฤติกรรม!"

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// 🆔 Entity ID - ตัวระบุเอนทิตี้
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

/// 🧩 Component trait - สำหรับ components ทั้งหมด
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

// Implement Component for specific types
macro_rules! impl_component {
    ($($t:ty),*) => {
        $(
            impl Component for $t {
                fn as_any(&self) -> &dyn Any {
                    self
                }
                
                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self
                }
            }
        )*
    };
}

impl_component!(Position, Velocity, Renderable, Health, AI, Tag);

/// 📍 Position Component
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// 🏃 Velocity Component
#[derive(Debug, Clone, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }
}

/// 🎨 Renderable Component
#[derive(Debug, Clone)]
pub struct Renderable {
    pub sprite: String,
    pub color: (f32, f32, f32, f32), // RGBA
    pub scale: f32,
    pub rotation: f32,
    pub visible: bool,
    pub layer: i32,
}

impl Renderable {
    pub fn new(sprite: String) -> Self {
        Self {
            sprite,
            color: (1.0, 1.0, 1.0, 1.0),
            scale: 1.0,
            rotation: 0.0,
            visible: true,
            layer: 0,
        }
    }
    
    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = (r, g, b, a);
        self
    }
    
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// ❤️ Health Component
#[derive(Debug, Clone, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub regeneration_rate: f32,
    pub is_invulnerable: bool,
    pub last_damage_time: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health,
            max: max_health,
            regeneration_rate: 0.0,
            is_invulnerable: false,
            last_damage_time: 0.0,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32, current_time: f32) {
        if !self.is_invulnerable {
            self.current = (self.current - damage).max(0.0);
            self.last_damage_time = current_time;
        }
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn health_percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// 🎯 AI Component
#[derive(Debug, Clone)]
pub struct AI {
    pub behavior: AIBehavior,
    pub target: Option<EntityId>,
    pub state: AIState,
    pub detection_range: f32,
    pub attack_range: f32,
    pub speed: f32,
    pub last_action_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AIBehavior {
    Idle,
    Patrol { points: Vec<Position> },
    Chase,
    Attack,
    Flee,
    Guard { position: Position },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AIState {
    Idle,
    Moving,
    Attacking,
    Searching,
    Returning,
}

impl AI {
    pub fn new(behavior: AIBehavior) -> Self {
        Self {
            behavior,
            target: None,
            state: AIState::Idle,
            detection_range: 100.0,
            attack_range: 50.0,
            speed: 50.0,
            last_action_time: 0.0,
        }
    }
}

/// 🏷️ Tag Component - สำหรับการจัดกลุ่ม
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

impl Tag {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

/// 🗂️ Component Storage - เก็บ components
pub struct ComponentStorage {
    components: HashMap<TypeId, HashMap<EntityId, Box<dyn Component>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    
    /// เพิ่ม component ให้ entity
    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }
    
    /// ดึง component ของ entity
    pub fn get_component<T: Component + 'static>(&self, entity: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(&entity)?
            .as_any()
            .downcast_ref::<T>()
    }
    
    /// ดึง component ของ entity (mutable)
    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(&entity)?
            .as_any_mut()
            .downcast_mut::<T>()
    }
    
    /// ลบ component ของ entity
    pub fn remove_component<T: Component + 'static>(&mut self, entity: EntityId) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(components) = self.components.get_mut(&type_id) {
            components.remove(&entity).is_some()
        } else {
            false
        }
    }
    
    /// ตรวจสอบว่า entity มี component หรือไม่
    pub fn has_component<T: Component + 'static>(&self, entity: EntityId) -> bool {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .map(|components| components.contains_key(&entity))
            .unwrap_or(false)
    }
    
    /// ลบ components ทั้งหมดของ entity
    pub fn remove_entity(&mut self, entity: EntityId) {
        for components in self.components.values_mut() {
            components.remove(&entity);
        }
    }
    
    /// ดึง entities ที่มี component type นี้
    pub fn get_entities_with_component<T: Component + 'static>(&self) -> Vec<EntityId> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .map(|components| components.keys().cloned().collect())
            .unwrap_or_default()
    }
}

/// 🏭 Entity Manager - จัดการ entities
#[derive(Debug)]
pub struct EntityManager {
    next_id: u32,
    active_entities: HashSet<EntityId>,
    entities_to_destroy: Vec<EntityId>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            active_entities: HashSet::new(),
            entities_to_destroy: Vec::new(),
        }
    }
    
    /// สร้าง entity ใหม่
    pub fn create_entity(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.active_entities.insert(id);
        println!("🆕 Created {}", id);
        id
    }
    
    /// ทำลาย entity (จะถูกลบในรอบถัดไป)
    pub fn destroy_entity(&mut self, entity: EntityId) {
        if self.active_entities.contains(&entity) {
            self.entities_to_destroy.push(entity);
            println!("💥 Marked {} for destruction", entity);
        }
    }
    
    /// ตรวจสอบว่า entity ยังมีอยู่หรือไม่
    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.active_entities.contains(&entity) && !self.entities_to_destroy.contains(&entity)
    }
    
    /// ดึง entities ที่ active ทั้งหมด
    pub fn get_active_entities(&self) -> Vec<EntityId> {
        self.active_entities.iter().cloned().collect()
    }
    
    /// ประมวลผลการทำลาย entities
    pub fn process_destructions(&mut self, storage: &mut ComponentStorage) {
        for entity in &self.entities_to_destroy {
            self.active_entities.remove(entity);
            storage.remove_entity(*entity);
            println!("🗑️ Destroyed {}", entity);
        }
        self.entities_to_destroy.clear();
    }
    
    /// ดึงสถิติ
    pub fn get_stats(&self) -> EntityStats {
        EntityStats {
            active_count: self.active_entities.len(),
            pending_destruction: self.entities_to_destroy.len(),
            next_id: self.next_id,
        }
    }
}

/// 📊 สถิติ entities
#[derive(Debug, Clone)]
pub struct EntityStats {
    pub active_count: usize,
    pub pending_destruction: usize,
    pub next_id: u32,
}

/// ⚙️ System trait - สำหรับ systems ทั้งหมด
pub trait System {
    fn update(&mut self, world: &mut World, delta_time: f32);
    fn name(&self) -> &'static str;
}

/// 🏃 Movement System - จัดการการเคลื่อนไหว
#[derive(Debug)]
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        let entities_with_position = world.storage.get_entities_with_component::<Position>();
        
        // เก็บข้อมูล velocity ก่อนเพื่อหลีกเลี่ยงการ borrow ซ้ำ
        let mut position_updates = Vec::new();
        
        for entity in entities_with_position {
            if let Some(velocity) = world.storage.get_component::<Velocity>(entity) {
                let velocity_clone = velocity.clone();
                position_updates.push((entity, velocity_clone));
            }
        }
        
        // อัปเดต position แยกต่างหาก
        for (entity, velocity) in position_updates {
            if let Some(position) = world.storage.get_component_mut::<Position>(entity) {
                position.x += velocity.x * delta_time;
                position.y += velocity.y * delta_time;
                position.z += velocity.z * delta_time;
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "MovementSystem"
    }
}

/// ❤️ Health System - จัดการ health และ regeneration
#[derive(Debug)]
pub struct HealthSystem {
    current_time: f32,
}

impl HealthSystem {
    pub fn new() -> Self {
        Self { current_time: 0.0 }
    }
}

impl System for HealthSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.current_time += delta_time;
        
        let entities_with_health = world.storage.get_entities_with_component::<Health>();
        let mut entities_to_destroy = Vec::new();
        
        for entity in entities_with_health {
            if let Some(health) = world.storage.get_component_mut::<Health>(entity) {
                // Regeneration
                if health.regeneration_rate > 0.0 {
                    health.heal(health.regeneration_rate * delta_time);
                }
                
                // ตรวจสอบว่าตายหรือไม่
                if !health.is_alive() {
                    entities_to_destroy.push(entity);
                }
            }
        }
        
        // ทำลาย entities ที่ตาย
        for entity in entities_to_destroy {
            world.entity_manager.destroy_entity(entity);
        }
    }
    
    fn name(&self) -> &'static str {
        "HealthSystem"
    }
}

/// 🎯 AI System - จัดการ AI behavior
#[derive(Debug)]
pub struct AISystem {
    current_time: f32,
}

impl AISystem {
    pub fn new() -> Self {
        Self { current_time: 0.0 }
    }
    
    fn find_nearest_target(&self, world: &World, ai_entity: EntityId, ai_pos: &Position) -> Option<EntityId> {
        let mut nearest_target = None;
        let mut nearest_distance = f32::INFINITY;
        
        for entity in world.entity_manager.get_active_entities() {
            if entity == ai_entity {
                continue;
            }
            
            // ตรวจสอบว่าเป็น player หรือไม่ (มี tag "Player")
            if let Some(tag) = world.storage.get_component::<Tag>(entity) {
                if tag.0 == "Player" {
                    if let Some(target_pos) = world.storage.get_component::<Position>(entity) {
                        let distance = ai_pos.distance_to(target_pos);
                        if distance < nearest_distance {
                            nearest_distance = distance;
                            nearest_target = Some(entity);
                        }
                    }
                }
            }
        }
        
        nearest_target
    }
}

impl System for AISystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.current_time += delta_time;
        
        let entities_with_ai = world.storage.get_entities_with_component::<AI>();
        
        for entity in entities_with_ai {
            if let (Some(ai), Some(position)) = (
                world.storage.get_component::<AI>(entity),
                world.storage.get_component::<Position>(entity)
            ) {
                let ai_clone = ai.clone();
                let position_clone = position.clone();
                
                // ค้นหา target
                let nearest_target = self.find_nearest_target(world, entity, &position_clone);
                
                // อัปเดต AI - แยกการ borrow เพื่อหลีกเลี่ยงปัญหา
                let mut ai_updates = Vec::new();
                let mut velocity_updates = Vec::new();
                let mut health_updates = Vec::new();
                
                // ตรวจสอบและเก็บข้อมูลที่ต้องอัปเดต
                match &ai_clone.behavior {
                    AIBehavior::Idle => {
                        // ค้นหา target ในระยะ detection
                        if let Some(target) = nearest_target {
                            if let Some(target_pos) = world.storage.get_component::<Position>(target) {
                                let distance = position_clone.distance_to(target_pos);
                                if distance <= ai_clone.detection_range {
                                    ai_updates.push((entity, Some(target), AIBehavior::Chase, AIState::Moving, None));
                                }
                            }
                        }
                    }
                    AIBehavior::Chase => {
                        if let Some(target) = ai_clone.target {
                            if let Some(target_pos) = world.storage.get_component::<Position>(target) {
                                let target_pos = target_pos.clone();
                                let distance = position_clone.distance_to(&target_pos);
                                
                                if distance <= ai_clone.attack_range {
                                    ai_updates.push((entity, Some(target), AIBehavior::Attack, AIState::Attacking, None));
                                } else if distance > ai_clone.detection_range * 1.5 {
                                    // หยุดไล่ถ้าไกลเกินไป
                                    ai_updates.push((entity, None, AIBehavior::Idle, AIState::Idle, None));
                                } else {
                                    // เคลื่อนที่ไปหา target
                                    let dx = target_pos.x - position_clone.x;
                                    let dy = target_pos.y - position_clone.y;
                                    let distance = (dx * dx + dy * dy).sqrt();
                                    
                                    if distance > 0.0 {
                                        let vel_x = (dx / distance) * ai_clone.speed;
                                        let vel_y = (dy / distance) * ai_clone.speed;
                                        velocity_updates.push((entity, vel_x, vel_y));
                                    }
                                }
                            }
                        }
                    }
                    AIBehavior::Attack => {
                        if let Some(target) = ai_clone.target {
                            if let Some(target_pos) = world.storage.get_component::<Position>(target) {
                                let target_pos = target_pos.clone();
                                let distance = position_clone.distance_to(&target_pos);
                                
                                if distance > ai_clone.attack_range {
                                    ai_updates.push((entity, Some(target), AIBehavior::Chase, AIState::Moving, None));
                                } else {
                                    // โจมตี (ทุก 1 วินาที)
                                    if self.current_time - ai_clone.last_action_time >= 1.0 {
                                        health_updates.push((target, 10.0));
                                        ai_updates.push((entity, Some(target), ai_clone.behavior.clone(), ai_clone.state.clone(), Some(self.current_time)));
                                        println!("🗡️ {} attacks {}!", entity, target);
                                    }
                                }
                            }
                        }
                    }
                    _ => {} // อื่นๆ
                }
                
                // ทำการอัปเดตแยกกัน
                for (entity_id, target, behavior, state, last_action_time) in ai_updates {
                    if let Some(ai_mut) = world.storage.get_component_mut::<AI>(entity_id) {
                        ai_mut.target = target;
                        ai_mut.behavior = behavior;
                        ai_mut.state = state;
                        if let Some(time) = last_action_time {
                            ai_mut.last_action_time = time;
                        }
                    }
                }
                
                for (entity_id, vel_x, vel_y) in velocity_updates {
                    if let Some(velocity) = world.storage.get_component_mut::<Velocity>(entity_id) {
                        velocity.x = vel_x;
                        velocity.y = vel_y;
                    }
                }
                
                for (target_id, damage) in health_updates {
                    if let Some(target_health) = world.storage.get_component_mut::<Health>(target_id) {
                        target_health.take_damage(damage, self.current_time);
                    }
                }
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "AISystem"
    }
}

/// 🎨 Render System - จัดการการเรนเดอร์
#[derive(Debug)]
pub struct RenderSystem {
    rendered_count: usize,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self { rendered_count: 0 }
    }
}

impl System for RenderSystem {
    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let entities_with_renderable = world.storage.get_entities_with_component::<Renderable>();
        self.rendered_count = 0;
        
        // เรียงตาม layer
        let mut render_data: Vec<(EntityId, i32, Position, Renderable)> = Vec::new();
        
        for entity in entities_with_renderable {
            if let (Some(position), Some(renderable)) = (
                world.storage.get_component::<Position>(entity),
                world.storage.get_component::<Renderable>(entity)
            ) {
                if renderable.visible {
                    render_data.push((entity, renderable.layer, position.clone(), renderable.clone()));
                }
            }
        }
        
        // เรียงตาม layer
        render_data.sort_by_key(|(_, layer, _, _)| *layer);
        
        // "เรนเดอร์" (แค่ print ในตัวอย่างนี้)
        for (entity, layer, position, renderable) in render_data {
            println!("🎨 Rendering {} at ({:.1}, {:.1}) layer {} sprite: {}", 
                    entity, position.x, position.y, layer, renderable.sprite);
            self.rendered_count += 1;
        }
    }
    
    fn name(&self) -> &'static str {
        "RenderSystem"
    }
}

/// 🔧 Dummy System - ใช้สำหรับ placeholder ใน memory replacement
#[derive(Debug)]
struct DummySystem;

impl System for DummySystem {
    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // ไม่ทำอะไร - เป็นแค่ placeholder
    }
    
    fn name(&self) -> &'static str {
        "DummySystem"
    }
}

/// 🌍 World - โลกของ ECS
pub struct World {
    pub entity_manager: EntityManager,
    pub storage: ComponentStorage,
    pub systems: Vec<Box<dyn System>>,
    pub delta_time: f32,
    pub total_time: f32,
}

impl World {
    /// สร้าง world ใหม่
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            storage: ComponentStorage::new(),
            systems: Vec::new(),
            delta_time: 0.0,
            total_time: 0.0,
        }
    }
    
    /// เพิ่ม system
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        println!("⚙️ Added system: {}", system.name());
        self.systems.push(Box::new(system));
    }
    
    /// สร้าง entity พร้อม components
    pub fn spawn_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_manager.create_entity();
        EntityBuilder::new(entity, &mut self.storage)
    }
    
    /// อัปเดต world
    pub fn update(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.total_time += delta_time;
        
        // อัปเดต systems - ใช้ index เพื่อหลีกเลี่ยงปัญหา borrowing
        let system_count = self.systems.len();
        for i in 0..system_count {
            // แยก systems ออกจาก self เพื่อหลีกเลี่ยงปัญหา borrow
            let mut system = std::mem::replace(&mut self.systems[i], Box::new(DummySystem));
            system.update(self, delta_time);
            self.systems[i] = system;
        }
        
        // ประมวลผลการทำลาย entities
        self.entity_manager.process_destructions(&mut self.storage);
    }
    
    /// ดึงสถิติ world
    pub fn get_stats(&self) -> WorldStats {
        let entity_stats = self.entity_manager.get_stats();
        
        WorldStats {
            entity_count: entity_stats.active_count,
            system_count: self.systems.len(),
            total_time: self.total_time,
            delta_time: self.delta_time,
        }
    }
}

/// 📊 สถิติ world
#[derive(Debug, Clone)]
pub struct WorldStats {
    pub entity_count: usize,
    pub system_count: usize,
    pub total_time: f32,
    pub delta_time: f32,
}

/// 🏗️ Entity Builder - สำหรับสร้าง entity พร้อม components
pub struct EntityBuilder<'a> {
    entity: EntityId,
    storage: &'a mut ComponentStorage,
}

impl<'a> EntityBuilder<'a> {
    fn new(entity: EntityId, storage: &'a mut ComponentStorage) -> Self {
        Self { entity, storage }
    }
    
    /// เพิ่ม component
    pub fn with<T: Component + 'static>(self, component: T) -> Self {
        self.storage.add_component(self.entity, component);
        self
    }
    
    /// สร้าง entity เสร็จสิ้น
    pub fn build(self) -> EntityId {
        println!("🏗️ Built {} with components", self.entity);
        self.entity
    }
}

/// 🎮 สาธิตการใช้งาน ECS Architecture
pub fn demonstrate_ecs_architecture() {
    println!("🏗️ === ECS Architecture Demo ===");
    
    // สร้าง world
    let mut world = World::new();
    println!("🌍 Created ECS world");
    
    // เพิ่ม systems
    world.add_system(MovementSystem);
    world.add_system(HealthSystem::new());
    world.add_system(AISystem::new());
    world.add_system(RenderSystem::new());
    
    // สร้าง player entity
    let player = world.spawn_entity()
        .with(Position::new(0.0, 0.0, 0.0))
        .with(Velocity::new(0.0, 0.0, 0.0))
        .with(Health::new(100.0))
        .with(Renderable::new("player.png".to_string()).with_color(0.0, 1.0, 0.0, 1.0))
        .with(Tag::new("Player"))
        .build();
    
    println!("🧙 Created player: {}", player);
    
    // สร้าง enemy entities
    let enemy1 = world.spawn_entity()
        .with(Position::new(50.0, 50.0, 0.0))
        .with(Velocity::new(0.0, 0.0, 0.0))
        .with(Health::new(50.0))
        .with(AI::new(AIBehavior::Idle))
        .with(Renderable::new("enemy.png".to_string()).with_color(1.0, 0.0, 0.0, 1.0))
        .with(Tag::new("Enemy"))
        .build();
    
    let enemy2 = world.spawn_entity()
        .with(Position::new(-30.0, 80.0, 0.0))
        .with(Velocity::new(0.0, 0.0, 0.0))
        .with(Health::new(30.0))
        .with(AI::new(AIBehavior::Patrol { 
            points: vec![
                Position::new(-30.0, 80.0, 0.0),
                Position::new(30.0, 80.0, 0.0),
            ]
        }))
        .with(Renderable::new("guard.png".to_string()).with_color(0.8, 0.4, 0.0, 1.0))
        .with(Tag::new("Enemy"))
        .build();
    
    println!("👹 Created enemies: {} and {}", enemy1, enemy2);
    
    // สร้าง item entity
    let _item = world.spawn_entity()
        .with(Position::new(20.0, -10.0, 0.0))
        .with(Renderable::new("potion.png".to_string()).with_color(0.0, 0.0, 1.0, 1.0))
        .with(Tag::new("Item"))
        .build();
    
    // จำลองการเคลื่อนไหวของ player
    if let Some(player_velocity) = world.storage.get_component_mut::<Velocity>(player) {
        player_velocity.x = 10.0;
        player_velocity.y = 5.0;
    }
    
    println!("\n🎮 Starting simulation:");
    
    // รันการจำลอง
    for frame in 0..10 {
        println!("\n--- Frame {} ---", frame + 1);
        
        world.update(1.0 / 60.0); // 60 FPS
        
        // แสดงสถิติ
        let stats = world.get_stats();
        println!("📊 World stats: {} entities, {} systems, time: {:.2}s", 
                stats.entity_count, stats.system_count, stats.total_time);
        
        // แสดงตำแหน่ง player
        if let Some(player_pos) = world.storage.get_component::<Position>(player) {
            println!("🧙 Player position: ({:.1}, {:.1}, {:.1})", 
                    player_pos.x, player_pos.y, player_pos.z);
        }
        
        // แสดง health ของ entities
        for entity in world.entity_manager.get_active_entities() {
            if let Some(health) = world.storage.get_component::<Health>(entity) {
                if let Some(tag) = world.storage.get_component::<Tag>(entity) {
                    println!("❤️ {} {}: {:.1}/{:.1} HP ({:.0}%)", 
                            tag.0, entity, health.current, health.max, 
                            health.health_percentage() * 100.0);
                }
            }
        }
        
        // เปลี่ยนทิศทางของ player บางครั้ง
        if frame == 3 {
            if let Some(player_velocity) = world.storage.get_component_mut::<Velocity>(player) {
                player_velocity.x = -5.0;
                player_velocity.y = 10.0;
                println!("🔄 Player changed direction");
            }
        }
        
        // ทำให้ player ได้รับความเสียหาย
        if frame == 5 {
            if let Some(player_health) = world.storage.get_component_mut::<Health>(player) {
                player_health.take_damage(25.0, world.total_time);
                println!("💥 Player took damage!");
            }
        }
    }
    
    // ทดสอบการทำลาย entity
    println!("\n💥 Testing entity destruction:");
    world.entity_manager.destroy_entity(enemy2);
    world.update(1.0 / 60.0);
    
    let final_stats = world.get_stats();
    println!("📊 Final stats: {} entities remaining", final_stats.entity_count);
    
    // แสดง best practices
    println!("\n💡 ECS Best Practices:");
    show_ecs_best_practices();
}

/// 💡 ECS Best Practices
fn show_ecs_best_practices() {
    let practices = vec![
        "🧩 Keep components as pure data (no behavior)",
        "⚙️ Put all logic in systems",
        "🔄 Make systems stateless when possible",
        "📦 Use composition over inheritance",
        "🎯 Design components for specific purposes",
        "🚀 Optimize for cache-friendly data access",
        "🔗 Avoid tight coupling between systems",
        "📊 Use events for system communication",
        "🏷️ Use tags for entity categorization",
        "⚡ Batch operations when possible",
        "🧪 Make systems testable in isolation",
        "📝 Document component relationships",
        "🔍 Use queries for efficient entity filtering",
        "🎭 Separate rendering from game logic",
        "🌊 Handle entity lifecycle properly",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust ECS Libraries:");
    println!("   • bevy_ecs - Part of Bevy game engine");
    println!("   • specs - Parallel ECS library");
    println!("   • legion - High-performance ECS");
    println!("   • hecs - Minimal ECS library");
    println!("   • shipyard - Flexible ECS with unique features");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_creation() {
        let mut entity_manager = EntityManager::new();
        let entity = entity_manager.create_entity();
        
        assert!(entity_manager.is_alive(entity));
        assert_eq!(entity.0, 1);
    }
    
    #[test]
    fn test_component_storage() {
        let mut storage = ComponentStorage::new();
        let entity = EntityId(1);
        
        storage.add_component(entity, Position::new(10.0, 20.0, 30.0));
        
        assert!(storage.has_component::<Position>(entity));
        
        let position = storage.get_component::<Position>(entity).expect("Position component should exist");
        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 20.0);
        assert_eq!(position.z, 30.0);
    }
    
    #[test]
    fn test_health_component() {
        let mut health = Health::new(100.0);
        
        assert!(health.is_alive());
        assert_eq!(health.health_percentage(), 1.0);
        
        health.take_damage(30.0, 0.0);
        assert_eq!(health.current, 70.0);
        assert_eq!(health.health_percentage(), 0.7);
        
        health.heal(20.0);
        assert_eq!(health.current, 90.0);
    }
    
    #[test]
    fn test_velocity_component() {
        let mut velocity = Velocity::new(3.0, 4.0, 0.0);
        
        assert_eq!(velocity.magnitude(), 5.0);
        
        velocity.normalize();
        assert!((velocity.magnitude() - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_world_creation() {
        let mut world = World::new();
        
        let entity = world.spawn_entity()
            .with(Position::new(0.0, 0.0, 0.0))
            .with(Health::new(50.0))
            .build();
        
        assert!(world.entity_manager.is_alive(entity));
        assert!(world.storage.has_component::<Position>(entity));
        assert!(world.storage.has_component::<Health>(entity));
    }
    
    #[test]
    fn test_movement_system() {
        let mut world = World::new();
        world.add_system(MovementSystem);
        
        let entity = world.spawn_entity()
            .with(Position::new(0.0, 0.0, 0.0))
            .with(Velocity::new(10.0, 5.0, 0.0))
            .build();
        
        world.update(1.0); // 1 second
        
        let position = world.storage.get_component::<Position>(entity).expect("Position component should exist after movement");
        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 5.0);
    }
}

// 🏗️ "ECS ไม่ใช่แค่ pattern แต่เป็นปรัชญา:
//     Entity คือสิ่งที่มีอยู่
//     Component คือสิ่งที่มันเป็น
//     System คือสิ่งที่มันทำ
//     แยกข้อมูลออกจากพฤติกรรม แล้วโลกจะเป็นระเบียบ! 🌍✨"