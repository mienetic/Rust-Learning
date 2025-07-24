//! 🎯 Game Logic - ระบบตรรกะหลักของเกม
//! 
//! โมดูลนี้สาธิตการสร้างระบบตรรกะหลักของเกม รวมถึง game states, 
//! rules engine, event system, และ gameplay mechanics
//! 
//! 🎮 "เกมที่ดีต้องมีตรรกะที่ชัดเจน แต่ความสนุกที่ไม่คาดคิด!"

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::fmt;

/// 🎮 Game State Types
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    MainMenu,
    Loading { progress: f32 },
    Playing { level: u32, score: u64 },
    Paused { previous_state: Box<GameState> },
    GameOver { final_score: u64, reason: GameOverReason },
    Victory { score: u64, time: Duration },
    Settings,
    Credits,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameOverReason {
    PlayerDied,
    TimeUp,
    QuitGame,
    LevelFailed,
}

/// 🎯 Game Rules Engine
#[derive(Debug, Clone)]
pub struct GameRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub priority: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleCondition {
    ScoreReached(u64),
    TimeElapsed(Duration),
    PlayerHealthBelow(f32),
    EnemiesDefeated(u32),
    ItemCollected(String),
    LevelCompleted(u32),
    PlayerPosition { x: f32, y: f32, radius: f32 },
    And(Vec<RuleCondition>),
    Or(Vec<RuleCondition>),
    Not(Box<RuleCondition>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleAction {
    AddScore(u64),
    SubtractScore(u64),
    HealPlayer(f32),
    DamagePlayer(f32),
    SpawnEnemy(String),
    SpawnItem(String),
    TriggerEvent(String),
    ChangeLevel(u32),
    EndGame(GameOverReason),
    ShowMessage(String),
    PlaySound(String),
    Multiple(Vec<RuleAction>),
}

/// 📅 Game Event System
#[derive(Debug, Clone, PartialEq)]
pub struct GameEvent {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: Instant,
    pub data: EventData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    PlayerAction,
    GameStateChange,
    ScoreUpdate,
    HealthChange,
    ItemPickup,
    EnemySpawn,
    EnemyDefeat,
    LevelComplete,
    Achievement,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventData {
    PlayerMoved { from: (f32, f32), to: (f32, f32) },
    PlayerAttacked { target: String, damage: f32 },
    ScoreChanged { old_score: u64, new_score: u64 },
    HealthChanged { old_health: f32, new_health: f32 },
    ItemPickedUp { item_type: String, value: f32 },
    EnemySpawned { enemy_type: String, position: (f32, f32) },
    EnemyDefeated { enemy_type: String, score_reward: u64 },
    LevelCompleted { level: u32, time: Duration },
    AchievementUnlocked { achievement_id: String },
    Custom(HashMap<String, String>),
}

/// 🏆 Achievement System
#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub points: u32,
    pub unlocked: bool,
    pub unlock_time: Option<Instant>,
    pub requirements: Vec<AchievementRequirement>,
    pub progress: HashMap<String, f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AchievementRequirement {
    ScoreReached(u64),
    EnemiesKilled(u32),
    LevelsCompleted(u32),
    TimePlayedTotal(Duration),
    ItemsCollected(u32),
    DeathsBelow(u32),
    ConsecutiveWins(u32),
    Custom { key: String, target: f32 },
}

/// 🎮 Player Stats
#[derive(Debug, Clone)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub experience: u64,
    pub level: u32,
    pub score: u64,
    pub lives: u32,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub inventory: Inventory,
    pub abilities: Vec<Ability>,
    pub status_effects: Vec<StatusEffect>,
}

/// 🎒 Inventory System
#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: HashMap<String, InventoryItem>,
    pub max_capacity: u32,
    pub current_weight: f32,
    pub max_weight: f32,
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub weight: f32,
    pub value: u64,
    pub item_type: ItemType,
    pub usable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon { damage: f32, range: f32 },
    Armor { defense: f32, durability: f32 },
    Consumable { effect: String, duration: Option<Duration> },
    Key { door_id: String },
    Quest { quest_id: String },
    Collectible,
}

/// ⚡ Ability System
#[derive(Debug, Clone)]
pub struct Ability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cooldown: Duration,
    pub last_used: Option<Instant>,
    pub mana_cost: f32,
    pub level: u32,
    pub max_level: u32,
    pub ability_type: AbilityType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbilityType {
    Attack { damage: f32, range: f32 },
    Heal { amount: f32 },
    Buff { stat: String, multiplier: f32, duration: Duration },
    Teleport { max_distance: f32 },
    Shield { absorption: f32, duration: Duration },
    Custom { effects: Vec<String> },
}

/// 🌟 Status Effects
#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect_type: StatusEffectType,
    pub duration: Duration,
    pub remaining_time: Duration,
    pub stacks: u32,
    pub max_stacks: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusEffectType {
    Poison { damage_per_second: f32 },
    Regeneration { heal_per_second: f32 },
    SpeedBoost { multiplier: f32 },
    Slow { multiplier: f32 },
    Invulnerable,
    Stunned,
    Burning { damage_per_second: f32 },
    Frozen,
    Custom { effects: HashMap<String, f32> },
}

/// 🎯 Game Logic Manager
#[derive(Debug)]
pub struct GameLogicManager {
    pub current_state: GameState,
    pub player_stats: PlayerStats,
    pub rules: Vec<GameRule>,
    pub events: VecDeque<GameEvent>,
    pub achievements: HashMap<String, Achievement>,
    pub game_time: Duration,
    pub level_data: HashMap<u32, LevelData>,
    pub event_listeners: HashMap<EventType, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct LevelData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub time_limit: Option<Duration>,
    pub enemy_spawns: Vec<EnemySpawn>,
    pub item_spawns: Vec<ItemSpawn>,
    pub background_music: String,
    pub difficulty_multiplier: f32,
}

#[derive(Debug, Clone)]
pub struct Objective {
    pub id: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub completed: bool,
    pub progress: f32,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveType {
    KillEnemies(u32),
    CollectItems(String, u32),
    ReachLocation { x: f32, y: f32, radius: f32 },
    SurviveTime(Duration),
    ScorePoints(u64),
    DefeatBoss(String),
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct EnemySpawn {
    pub enemy_type: String,
    pub position: (f32, f32),
    pub spawn_time: Duration,
    pub spawn_condition: Option<RuleCondition>,
}

#[derive(Debug, Clone)]
pub struct ItemSpawn {
    pub item_type: String,
    pub position: (f32, f32),
    pub spawn_time: Duration,
    pub respawn_time: Option<Duration>,
}

impl GameLogicManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_state: GameState::MainMenu,
            player_stats: PlayerStats::new(),
            rules: Vec::new(),
            events: VecDeque::new(),
            achievements: HashMap::new(),
            game_time: Duration::from_secs(0),
            level_data: HashMap::new(),
            event_listeners: HashMap::new(),
        };
        
        // เพิ่ม default rules และ achievements
        manager.setup_default_rules();
        manager.setup_default_achievements();
        manager.setup_default_levels();
        
        manager
    }
    
    /// ตั้งค่า rules เริ่มต้น
    fn setup_default_rules(&mut self) {
        // Rule: เมื่อคะแนนถึง 1000 ให้เพิ่มชีวิต
        self.add_rule(GameRule {
            id: "score_1000_bonus".to_string(),
            name: "Score Milestone".to_string(),
            description: "Gain extra life at 1000 points".to_string(),
            condition: RuleCondition::ScoreReached(1000),
            action: RuleAction::Multiple(vec![
                RuleAction::HealPlayer(50.0),
                RuleAction::ShowMessage("🎉 Bonus life! Keep going!".to_string()),
            ]),
            priority: 1,
            enabled: true,
        });
        
        // Rule: เมื่อเลือดต่ำกว่า 20% ให้เตือน
        self.add_rule(GameRule {
            id: "low_health_warning".to_string(),
            name: "Low Health Warning".to_string(),
            description: "Warn player when health is low".to_string(),
            condition: RuleCondition::PlayerHealthBelow(20.0),
            action: RuleAction::ShowMessage("⚠️ Health is low! Find healing!".to_string()),
            priority: 10,
            enabled: true,
        });
        
        // Rule: เมื่อเวลาหมดให้จบเกม
        self.add_rule(GameRule {
            id: "time_limit".to_string(),
            name: "Time Limit".to_string(),
            description: "End game when time runs out".to_string(),
            condition: RuleCondition::TimeElapsed(Duration::from_secs(300)), // 5 minutes
            action: RuleAction::EndGame(GameOverReason::TimeUp),
            priority: 100,
            enabled: true,
        });
    }
    
    /// ตั้งค่า achievements เริ่มต้น
    fn setup_default_achievements(&mut self) {
        // Achievement: First Kill
        let mut first_kill = Achievement {
            id: "first_kill".to_string(),
            name: "First Blood".to_string(),
            description: "Defeat your first enemy".to_string(),
            icon: "🗡️".to_string(),
            points: 10,
            unlocked: false,
            unlock_time: None,
            requirements: vec![AchievementRequirement::EnemiesKilled(1)],
            progress: HashMap::new(),
        };
        first_kill.progress.insert("enemies_killed".to_string(), 0.0);
        self.achievements.insert(first_kill.id.clone(), first_kill);
        
        // Achievement: Score Master
        let mut score_master = Achievement {
            id: "score_master".to_string(),
            name: "Score Master".to_string(),
            description: "Reach 10,000 points".to_string(),
            icon: "🏆".to_string(),
            points: 50,
            unlocked: false,
            unlock_time: None,
            requirements: vec![AchievementRequirement::ScoreReached(10000)],
            progress: HashMap::new(),
        };
        score_master.progress.insert("score".to_string(), 0.0);
        self.achievements.insert(score_master.id.clone(), score_master);
        
        // Achievement: Survivor
        let mut survivor = Achievement {
            id: "survivor".to_string(),
            name: "Survivor".to_string(),
            description: "Survive for 10 minutes".to_string(),
            icon: "⏰".to_string(),
            points: 30,
            unlocked: false,
            unlock_time: None,
            requirements: vec![AchievementRequirement::TimePlayedTotal(Duration::from_secs(600))],
            progress: HashMap::new(),
        };
        survivor.progress.insert("time_played".to_string(), 0.0);
        self.achievements.insert(survivor.id.clone(), survivor);
    }
    
    /// ตั้งค่า levels เริ่มต้น
    fn setup_default_levels(&mut self) {
        // Level 1
        let level1 = LevelData {
            id: 1,
            name: "Forest Entrance".to_string(),
            description: "A peaceful forest path".to_string(),
            objectives: vec![
                Objective {
                    id: "kill_5_goblins".to_string(),
                    description: "Defeat 5 goblins".to_string(),
                    objective_type: ObjectiveType::KillEnemies(5),
                    completed: false,
                    progress: 0.0,
                    required: true,
                },
                Objective {
                    id: "collect_key".to_string(),
                    description: "Find the forest key".to_string(),
                    objective_type: ObjectiveType::CollectItems("forest_key".to_string(), 1),
                    completed: false,
                    progress: 0.0,
                    required: true,
                },
            ],
            time_limit: Some(Duration::from_secs(300)),
            enemy_spawns: vec![
                EnemySpawn {
                    enemy_type: "goblin".to_string(),
                    position: (100.0, 100.0),
                    spawn_time: Duration::from_secs(10),
                    spawn_condition: None,
                },
                EnemySpawn {
                    enemy_type: "goblin".to_string(),
                    position: (200.0, 150.0),
                    spawn_time: Duration::from_secs(30),
                    spawn_condition: None,
                },
            ],
            item_spawns: vec![
                ItemSpawn {
                    item_type: "health_potion".to_string(),
                    position: (50.0, 50.0),
                    spawn_time: Duration::from_secs(5),
                    respawn_time: Some(Duration::from_secs(60)),
                },
            ],
            background_music: "forest_theme.ogg".to_string(),
            difficulty_multiplier: 1.0,
        };
        self.level_data.insert(1, level1);
        
        // Level 2
        let level2 = LevelData {
            id: 2,
            name: "Dark Cave".to_string(),
            description: "A mysterious underground cave".to_string(),
            objectives: vec![
                Objective {
                    id: "defeat_cave_boss".to_string(),
                    description: "Defeat the Cave Troll".to_string(),
                    objective_type: ObjectiveType::DefeatBoss("cave_troll".to_string()),
                    completed: false,
                    progress: 0.0,
                    required: true,
                },
            ],
            time_limit: Some(Duration::from_secs(600)),
            enemy_spawns: vec![
                EnemySpawn {
                    enemy_type: "bat".to_string(),
                    position: (150.0, 200.0),
                    spawn_time: Duration::from_secs(20),
                    spawn_condition: None,
                },
                EnemySpawn {
                    enemy_type: "cave_troll".to_string(),
                    position: (300.0, 300.0),
                    spawn_time: Duration::from_secs(120),
                    spawn_condition: Some(RuleCondition::EnemiesDefeated(3)),
                },
            ],
            item_spawns: vec![
                ItemSpawn {
                    item_type: "magic_sword".to_string(),
                    position: (250.0, 100.0),
                    spawn_time: Duration::from_secs(60),
                    respawn_time: None,
                },
            ],
            background_music: "cave_theme.ogg".to_string(),
            difficulty_multiplier: 1.5,
        };
        self.level_data.insert(2, level2);
    }
    
    /// เพิ่ม rule ใหม่
    pub fn add_rule(&mut self, rule: GameRule) {
        self.rules.push(rule);
        // เรียงตาม priority (สูงไปต่ำ)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    /// ลบ rule
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.retain(|rule| rule.id != rule_id);
    }
    
    /// เปิด/ปิด rule
    pub fn toggle_rule(&mut self, rule_id: &str, enabled: bool) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
        }
    }
    
    /// เพิ่ม event
    pub fn add_event(&mut self, event: GameEvent) {
        println!("📅 Event: {:?}", event.event_type);
        
        // เก็บ event ไว้ในประวัติ
        self.events.push_back(event.clone());
        
        // จำกัดจำนวน events ที่เก็บไว้
        while self.events.len() > 100 {
            self.events.pop_front();
        }
        
        // ประมวลผล event
        self.process_event(&event);
    }
    
    /// ประมวลผล event
    fn process_event(&mut self, event: &GameEvent) {
        // อัปเดต achievement progress
        self.update_achievement_progress(event);
        
        // ตรวจสอบ rules
        self.check_rules();
        
        // ประมวลผลตาม event type
        match &event.event_type {
            EventType::ScoreUpdate => {
                if let EventData::ScoreChanged { new_score, .. } = &event.data {
                    self.player_stats.score = *new_score;
                }
            }
            EventType::HealthChange => {
                if let EventData::HealthChanged { new_health, .. } = &event.data {
                    self.player_stats.health = *new_health;
                    
                    // ตรวจสอบการตาย
                    if self.player_stats.health <= 0.0 {
                        self.change_state(GameState::GameOver {
                            final_score: self.player_stats.score,
                            reason: GameOverReason::PlayerDied,
                        });
                    }
                }
            }
            EventType::LevelComplete => {
                if let EventData::LevelCompleted { level, time } = &event.data {
                    println!("🎉 Level {} completed in {:?}!", level, time);
                    
                    // เปลี่ยนไปเลเวลถัดไป
                    if self.level_data.contains_key(&(level + 1)) {
                        self.change_state(GameState::Playing {
                            level: level + 1,
                            score: self.player_stats.score,
                        });
                    } else {
                        // ชนะเกม!
                        self.change_state(GameState::Victory {
                            score: self.player_stats.score,
                            time: self.game_time,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    
    /// อัปเดต achievement progress
    fn update_achievement_progress(&mut self, event: &GameEvent) {
        // อัปเดต progress ก่อน
        for achievement in self.achievements.values_mut() {
            if achievement.unlocked {
                continue;
            }
            
            match &event.data {
                EventData::EnemyDefeated { .. } => {
                    if let Some(progress) = achievement.progress.get_mut("enemies_killed") {
                        *progress += 1.0;
                    }
                }
                EventData::ScoreChanged { new_score, .. } => {
                    if let Some(progress) = achievement.progress.get_mut("score") {
                        *progress = *new_score as f32;
                    }
                }
                _ => {}
            }
        }
        
        // ตรวจสอบ achievements ที่ควรปลดล็อคแยกต่างหาก
        let mut achievements_to_unlock = Vec::new();
        for achievement in self.achievements.values() {
            if !achievement.unlocked && self.check_achievement_requirements(achievement) {
                achievements_to_unlock.push(achievement.id.clone());
            }
        }
        
        // ปลดล็อค achievements และเพิ่ม events แยกต่างหาก
        for achievement_id in achievements_to_unlock {
            if let Some(achievement) = self.achievements.get_mut(&achievement_id) {
                achievement.unlocked = true;
                achievement.unlock_time = Some(Instant::now());
                
                println!("🏆 Achievement Unlocked: {} - {}", achievement.name, achievement.description);
            }
            
            // เพิ่ม event สำหรับ achievement (แยกจากการ borrow achievement)
            self.add_event(GameEvent {
                id: format!("achievement_{}", achievement_id),
                event_type: EventType::Achievement,
                timestamp: Instant::now(),
                data: EventData::AchievementUnlocked {
                    achievement_id: achievement_id.clone(),
                },
            });
        }
    }
    
    /// ตรวจสอบเงื่อนไข achievement
    fn check_achievement_requirements(&self, achievement: &Achievement) -> bool {
        for requirement in &achievement.requirements {
            match requirement {
                AchievementRequirement::EnemiesKilled(target) => {
                    if let Some(progress) = achievement.progress.get("enemies_killed") {
                        if *progress < *target as f32 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                AchievementRequirement::ScoreReached(target) => {
                    if let Some(progress) = achievement.progress.get("score") {
                        if *progress < *target as f32 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                AchievementRequirement::TimePlayedTotal(target) => {
                    if let Some(progress) = achievement.progress.get("time_played") {
                        if *progress < target.as_secs_f32() {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }
    
    /// ตรวจสอบ rules
    fn check_rules(&mut self) {
        let mut actions_to_execute = Vec::new();
        
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if self.evaluate_condition(&rule.condition) {
                actions_to_execute.push(rule.action.clone());
            }
        }
        
        // ดำเนินการ actions
        for action in actions_to_execute {
            self.execute_action(action);
        }
    }
    
    /// ประเมินเงื่อนไข
    fn evaluate_condition(&self, condition: &RuleCondition) -> bool {
        match condition {
            RuleCondition::ScoreReached(target) => self.player_stats.score >= *target,
            RuleCondition::TimeElapsed(target) => self.game_time >= *target,
            RuleCondition::PlayerHealthBelow(target) => self.player_stats.health < *target,
            RuleCondition::PlayerPosition { x, y, radius } => {
                let dx = self.player_stats.position.0 - x;
                let dy = self.player_stats.position.1 - y;
                (dx * dx + dy * dy).sqrt() <= *radius
            }
            RuleCondition::And(conditions) => {
                conditions.iter().all(|c| self.evaluate_condition(c))
            }
            RuleCondition::Or(conditions) => {
                conditions.iter().any(|c| self.evaluate_condition(c))
            }
            RuleCondition::Not(condition) => !self.evaluate_condition(condition),
            _ => false, // TODO: implement other conditions
        }
    }
    
    /// ดำเนินการ action
    fn execute_action(&mut self, action: RuleAction) {
        match action {
            RuleAction::AddScore(points) => {
                let old_score = self.player_stats.score;
                self.player_stats.score += points;
                
                self.add_event(GameEvent {
                    id: "score_added".to_string(),
                    event_type: EventType::ScoreUpdate,
                    timestamp: Instant::now(),
                    data: EventData::ScoreChanged {
                        old_score,
                        new_score: self.player_stats.score,
                    },
                });
            }
            RuleAction::HealPlayer(amount) => {
                let old_health = self.player_stats.health;
                self.player_stats.health = (self.player_stats.health + amount).min(self.player_stats.max_health);
                
                // ไม่สร้าง event เพื่อป้องกัน infinite loop
                println!("💚 Rule healed player from {:.1} to {:.1} HP", old_health, self.player_stats.health);
            }
            RuleAction::DamagePlayer(amount) => {
                let old_health = self.player_stats.health;
                self.player_stats.health = (self.player_stats.health - amount).max(0.0);
                
                // ไม่สร้าง event เพื่อป้องกัน infinite loop
                println!("💔 Rule damaged player from {:.1} to {:.1} HP", old_health, self.player_stats.health);
            }
            RuleAction::EndGame(reason) => {
                self.change_state(GameState::GameOver {
                    final_score: self.player_stats.score,
                    reason,
                });
            }
            RuleAction::ShowMessage(message) => {
                println!("💬 {}", message);
            }
            RuleAction::Multiple(actions) => {
                for action in actions {
                    self.execute_action(action);
                }
            }
            _ => {
                // TODO: implement other actions
            }
        }
    }
    
    /// เปลี่ยน game state
    pub fn change_state(&mut self, new_state: GameState) {
        let old_state = self.current_state.clone();
        self.current_state = new_state.clone();
        
        println!("🔄 State changed: {:?} -> {:?}", old_state, new_state);
        
        self.add_event(GameEvent {
            id: "state_changed".to_string(),
            event_type: EventType::GameStateChange,
            timestamp: Instant::now(),
            data: EventData::Custom({
                let mut data = HashMap::new();
                data.insert("old_state".to_string(), format!("{:?}", old_state));
                data.insert("new_state".to_string(), format!("{:?}", new_state));
                data
            }),
        });
    }
    
    /// อัปเดต game logic
    pub fn update(&mut self, delta_time: Duration) {
        self.game_time += delta_time;
        
        // อัปเดต status effects
        self.update_status_effects(delta_time);
        
        // อัปเดต ability cooldowns
        self.update_ability_cooldowns();
        
        // ตรวจสอบ rules
        self.check_rules();
        
        // อัปเดต achievement progress สำหรับเวลา
        for achievement in self.achievements.values_mut() {
            if let Some(progress) = achievement.progress.get_mut("time_played") {
                *progress += delta_time.as_secs_f32();
            }
        }
    }
    
    /// อัปเดต status effects
    fn update_status_effects(&mut self, delta_time: Duration) {
        self.player_stats.status_effects.retain_mut(|effect| {
            effect.remaining_time = effect.remaining_time.saturating_sub(delta_time);
            
            // ใช้ effect
            match &effect.effect_type {
                StatusEffectType::Poison { damage_per_second } => {
                    let damage = damage_per_second * delta_time.as_secs_f32();
                    self.player_stats.health = (self.player_stats.health - damage).max(0.0);
                }
                StatusEffectType::Regeneration { heal_per_second } => {
                    let heal = heal_per_second * delta_time.as_secs_f32();
                    self.player_stats.health = (self.player_stats.health + heal).min(self.player_stats.max_health);
                }
                StatusEffectType::Burning { damage_per_second } => {
                    let damage = damage_per_second * delta_time.as_secs_f32();
                    self.player_stats.health = (self.player_stats.health - damage).max(0.0);
                }
                _ => {}
            }
            
            // เก็บ effect ไว้ถ้ายังไม่หมดเวลา
            effect.remaining_time > Duration::from_secs(0)
        });
    }
    
    /// อัปเดต ability cooldowns
    fn update_ability_cooldowns(&mut self) {
        let now = Instant::now();
        
        for ability in &mut self.player_stats.abilities {
            if let Some(last_used) = ability.last_used {
                if now.duration_since(last_used) >= ability.cooldown {
                    // Ability พร้อมใช้แล้ว
                }
            }
        }
    }
    
    /// ใช้ ability
    pub fn use_ability(&mut self, ability_id: &str) -> bool {
        let now = Instant::now();
        
        if let Some(ability) = self.player_stats.abilities.iter_mut().find(|a| a.id == ability_id) {
            // ตรวจสอบ cooldown
            if let Some(last_used) = ability.last_used {
                if now.duration_since(last_used) < ability.cooldown {
                    return false; // ยังไม่พร้อมใช้
                }
            }
            
            // ตรวจสอบ mana
            if self.player_stats.mana < ability.mana_cost {
                return false; // mana ไม่พอ
            }
            
            // ใช้ ability
            self.player_stats.mana -= ability.mana_cost;
            ability.last_used = Some(now);
            
            // ใช้ effect ของ ability
            match &ability.ability_type {
                AbilityType::Heal { amount } => {
                    let old_health = self.player_stats.health;
                    self.player_stats.health = (self.player_stats.health + amount).min(self.player_stats.max_health);
                    
                    // ไม่สร้าง event เพื่อป้องกัน infinite loop
                    println!("💚 Healed from {:.1} to {:.1} HP", old_health, self.player_stats.health);
                }
                AbilityType::Attack { damage, .. } => {
                    println!("⚔️ Used {} for {} damage!", ability.name, damage);
                }
                _ => {
                    println!("✨ Used ability: {}", ability.name);
                }
            }
            
            true
        } else {
            false
        }
    }
    
    /// เพิ่ม status effect
    pub fn add_status_effect(&mut self, effect: StatusEffect) {
        // ตรวจสอบว่ามี effect แบบเดียวกันอยู่แล้วหรือไม่
        if let Some(existing) = self.player_stats.status_effects.iter_mut().find(|e| e.id == effect.id) {
            // Stack หรือ refresh duration
            if existing.stacks < existing.max_stacks {
                existing.stacks += 1;
            }
            existing.remaining_time = effect.duration;
        } else {
            self.player_stats.status_effects.push(effect);
        }
    }
    
    /// ดึงสถิติเกม
    pub fn get_game_stats(&self) -> GameStats {
        GameStats {
            current_state: self.current_state.clone(),
            player_stats: self.player_stats.clone(),
            game_time: self.game_time,
            total_events: self.events.len(),
            unlocked_achievements: self.achievements.values().filter(|a| a.unlocked).count(),
            total_achievements: self.achievements.len(),
            active_rules: self.rules.iter().filter(|r| r.enabled).count(),
            total_rules: self.rules.len(),
        }
    }
}

/// 📊 Game Statistics
#[derive(Debug, Clone)]
pub struct GameStats {
    pub current_state: GameState,
    pub player_stats: PlayerStats,
    pub game_time: Duration,
    pub total_events: usize,
    pub unlocked_achievements: usize,
    pub total_achievements: usize,
    pub active_rules: usize,
    pub total_rules: usize,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            mana: 50.0,
            max_mana: 50.0,
            experience: 0,
            level: 1,
            score: 0,
            lives: 3,
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            inventory: Inventory::new(),
            abilities: vec![
                Ability {
                    id: "heal".to_string(),
                    name: "Heal".to_string(),
                    description: "Restore health".to_string(),
                    cooldown: Duration::from_secs(10),
                    last_used: None,
                    mana_cost: 20.0,
                    level: 1,
                    max_level: 5,
                    ability_type: AbilityType::Heal { amount: 30.0 },
                },
                Ability {
                    id: "fireball".to_string(),
                    name: "Fireball".to_string(),
                    description: "Launch a fireball".to_string(),
                    cooldown: Duration::from_secs(3),
                    last_used: None,
                    mana_cost: 15.0,
                    level: 1,
                    max_level: 10,
                    ability_type: AbilityType::Attack { damage: 25.0, range: 100.0 },
                },
            ],
            status_effects: Vec::new(),
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            max_capacity: 20,
            current_weight: 0.0,
            max_weight: 100.0,
        }
    }
    
    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        // ตรวจสอบน้ำหนักและความจุ
        if self.current_weight + item.weight > self.max_weight {
            return false;
        }
        
        if self.items.len() >= self.max_capacity as usize {
            return false;
        }
        
        // เพิ่มไอเทม
        if let Some(existing) = self.items.get_mut(&item.id) {
            existing.quantity += item.quantity;
        } else {
            self.current_weight += item.weight;
            self.items.insert(item.id.clone(), item);
        }
        
        true
    }
    
    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                
                if item.quantity == 0 {
                    self.current_weight -= item.weight;
                    self.items.remove(item_id);
                }
                
                return true;
            }
        }
        false
    }
    
    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        if let Some(item) = self.items.get(item_id) {
            item.quantity >= quantity
        } else {
            false
        }
    }
}

/// 🎮 สาธิตการใช้งาน Game Logic
pub fn demonstrate_game_logic() {
    println!("🎯 === Game Logic Demo ===");
    
    // สร้าง game logic manager
    let mut game = GameLogicManager::new();
    println!("🎮 Created game logic manager");
    
    // เริ่มเกม
    game.change_state(GameState::Playing { level: 1, score: 0 });
    
    println!("\n🎯 Starting gameplay simulation:");
    
    // จำลองการเล่นเกม
    for second in 0..20 {
        let delta_time = Duration::from_secs(1);
        
        // จำลอง events ต่างๆ
        match second {
            2 => {
                // ผู้เล่นฆ่าศัตรู
                game.add_event(GameEvent {
                    id: "enemy_killed_1".to_string(),
                    event_type: EventType::EnemyDefeat,
                    timestamp: Instant::now(),
                    data: EventData::EnemyDefeated {
                        enemy_type: "goblin".to_string(),
                        score_reward: 100,
                    },
                });
                
                // เพิ่มคะแนน
                game.add_event(GameEvent {
                    id: "score_update_1".to_string(),
                    event_type: EventType::ScoreUpdate,
                    timestamp: Instant::now(),
                    data: EventData::ScoreChanged {
                        old_score: 0,
                        new_score: 100,
                    },
                });
            }
            5 => {
                // ผู้เล่นได้รับความเสียหาย
                game.add_event(GameEvent {
                    id: "player_damaged_1".to_string(),
                    event_type: EventType::HealthChange,
                    timestamp: Instant::now(),
                    data: EventData::HealthChanged {
                        old_health: 100.0,
                        new_health: 75.0,
                    },
                });
            }
            8 => {
                // ใช้ ability heal
                if game.use_ability("heal") {
                    println!("✨ Used heal ability!");
                } else {
                    println!("❌ Cannot use heal ability");
                }
            }
            10 => {
                // เพิ่ม status effect
                let poison = StatusEffect {
                    id: "poison".to_string(),
                    name: "Poison".to_string(),
                    description: "Taking damage over time".to_string(),
                    effect_type: StatusEffectType::Poison { damage_per_second: 2.0 },
                    duration: Duration::from_secs(5),
                    remaining_time: Duration::from_secs(5),
                    stacks: 1,
                    max_stacks: 3,
                };
                game.add_status_effect(poison);
                println!("🐍 Applied poison effect!");
            }
            12 => {
                // เก็บไอเทม
                let health_potion = InventoryItem {
                    id: "health_potion".to_string(),
                    name: "Health Potion".to_string(),
                    description: "Restores 50 HP".to_string(),
                    quantity: 1,
                    weight: 0.5,
                    value: 25,
                    item_type: ItemType::Consumable {
                        effect: "heal_50".to_string(),
                        duration: None,
                    },
                    usable: true,
                };
                
                if game.player_stats.inventory.add_item(health_potion) {
                    println!("🧪 Picked up health potion!");
                    
                    game.add_event(GameEvent {
                        id: "item_pickup_1".to_string(),
                        event_type: EventType::ItemPickup,
                        timestamp: Instant::now(),
                        data: EventData::ItemPickedUp {
                            item_type: "health_potion".to_string(),
                            value: 25.0,
                        },
                    });
                }
            }
            15 => {
                // ฆ่าศัตรูหลายตัวเพื่อทดสอบ achievement
                for i in 0..5 {
                    game.add_event(GameEvent {
                        id: format!("enemy_killed_{}", i + 2),
                        event_type: EventType::EnemyDefeat,
                        timestamp: Instant::now(),
                        data: EventData::EnemyDefeated {
                            enemy_type: "goblin".to_string(),
                            score_reward: 100,
                        },
                    });
                }
                
                // อัปเดตคะแนน
                game.add_event(GameEvent {
                    id: "score_update_big".to_string(),
                    event_type: EventType::ScoreUpdate,
                    timestamp: Instant::now(),
                    data: EventData::ScoreChanged {
                        old_score: 100,
                        new_score: 1500,
                    },
                });
            }
            _ => {}
        }
        
        // อัปเดต game logic
        game.update(delta_time);
        
        // แสดงสถานะทุก 5 วินาที
        if second % 5 == 0 && second > 0 {
            println!("\n--- Second {} ---", second);
            let stats = game.get_game_stats();
            
            println!("🎮 State: {:?}", stats.current_state);
            println!("❤️ Health: {:.1}/{:.1}", stats.player_stats.health, stats.player_stats.max_health);
            println!("💙 Mana: {:.1}/{:.1}", stats.player_stats.mana, stats.player_stats.max_mana);
            println!("🏆 Score: {}", stats.player_stats.score);
            println!("⏰ Time: {:?}", stats.game_time);
            println!("🏅 Achievements: {}/{}", stats.unlocked_achievements, stats.total_achievements);
            println!("📜 Active Rules: {}/{}", stats.active_rules, stats.total_rules);
            println!("🎒 Inventory: {}/{} items", stats.player_stats.inventory.items.len(), stats.player_stats.inventory.max_capacity);
            
            // แสดง status effects
            if !stats.player_stats.status_effects.is_empty() {
                println!("🌟 Status Effects:");
                for effect in &stats.player_stats.status_effects {
                    println!("   {} - {:.1}s remaining", effect.name, effect.remaining_time.as_secs_f32());
                }
            }
        }
    }
    
    // แสดงผลสรุป
    println!("\n📊 Final Game Statistics:");
    let final_stats = game.get_game_stats();
    println!("🏆 Final Score: {}", final_stats.player_stats.score);
    println!("⏰ Total Time: {:?}", final_stats.game_time);
    println!("🏅 Achievements Unlocked: {}/{}", final_stats.unlocked_achievements, final_stats.total_achievements);
    println!("📅 Total Events: {}", final_stats.total_events);
    
    // แสดง unlocked achievements
    println!("\n🏆 Unlocked Achievements:");
    for achievement in game.achievements.values().filter(|a| a.unlocked) {
        println!("   {} {} - {} ({} points)", achievement.icon, achievement.name, achievement.description, achievement.points);
    }
    
    // แสดง best practices
    println!("\n💡 Game Logic Best Practices:");
    show_game_logic_best_practices();
}

/// 💡 Game Logic Best Practices
fn show_game_logic_best_practices() {
    let practices = vec![
        "🎯 Keep game rules simple and understandable",
        "🔄 Use event-driven architecture for loose coupling",
        "📊 Track player progress with meaningful metrics",
        "🏆 Design achievements that encourage exploration",
        "⚖️ Balance difficulty progression carefully",
        "🎮 Provide clear feedback for player actions",
        "💾 Save game state frequently and reliably",
        "🔧 Make game rules configurable and moddable",
        "📈 Use analytics to understand player behavior",
        "🎯 Focus on core gameplay loop first",
        "🛡️ Validate all player input and actions",
        "⏱️ Optimize performance for smooth gameplay",
        "🎨 Separate game logic from presentation",
        "🔄 Implement proper state management",
        "🧪 Test edge cases and error conditions",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Game Logic Libraries:");
    println!("   • specs - Entity Component System");
    println!("   • legion - High-performance ECS");
    println!("   • bevy - Game engine with built-in ECS");
    println!("   • amethyst - Data-driven game engine");
    println!("   • bracket-lib - Roguelike toolkit");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_state_transitions() {
        let mut game = GameLogicManager::new();
        
        assert_eq!(game.current_state, GameState::MainMenu);
        
        game.change_state(GameState::Playing { level: 1, score: 0 });
        assert!(matches!(game.current_state, GameState::Playing { .. }));
    }
    
    #[test]
    fn test_rule_evaluation() {
        let game = GameLogicManager::new();
        
        let condition = RuleCondition::ScoreReached(100);
        assert!(!game.evaluate_condition(&condition));
        
        let condition = RuleCondition::PlayerHealthBelow(150.0);
        assert!(game.evaluate_condition(&condition));
    }
    
    #[test]
    fn test_inventory_system() {
        let mut inventory = Inventory::new();
        
        let item = InventoryItem {
            id: "test_item".to_string(),
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
            quantity: 1,
            weight: 1.0,
            value: 10,
            item_type: ItemType::Collectible,
            usable: false,
        };
        
        assert!(inventory.add_item(item));
        assert!(inventory.has_item("test_item", 1));
        assert!(inventory.remove_item("test_item", 1));
        assert!(!inventory.has_item("test_item", 1));
    }
    
    #[test]
    fn test_ability_system() {
        let mut game = GameLogicManager::new();
        
        // ทดสอบการใช้ ability
        assert!(game.use_ability("heal"));
        
        // ทดสอบ cooldown
        assert!(!game.use_ability("heal")); // ควรไม่สามารถใช้ได้เพราะยังไม่หมด cooldown
    }
    
    #[test]
    fn test_achievement_system() {
        let mut game = GameLogicManager::new();
        
        // จำลองการฆ่าศัตรู
        game.add_event(GameEvent {
            id: "test_kill".to_string(),
            event_type: EventType::EnemyDefeat,
            timestamp: Instant::now(),
            data: EventData::EnemyDefeated {
                enemy_type: "test_enemy".to_string(),
                score_reward: 100,
            },
        });
        
        // ตรวจสอบว่า achievement ปลดล็อคหรือไม่
        let achievement = game.achievements.get("first_kill").unwrap();
        assert!(achievement.unlocked);
    }
}

// 🎯 "เกมที่ดีไม่ได้มาจากกราฟิกสวยหรือเสียงเพราะ
//     แต่มาจากตรรกะที่ชัดเจน กฎที่ยุติธรรม
//     และความสนุกที่เกิดจากการตัดสินใจของผู้เล่น
//     ทุกระบบต้องทำงานร่วมกันเป็นหนึ่งเดียว! 🎮✨"