//! 🌐 Game Networking - ระบบเครือข่ายสำหรับเกม
//! 
//! โมดูลนี้สาธิตการสร้างระบบเครือข่ายสำหรับเกมแบบ multiplayer
//! รวมถึง client-server architecture, state synchronization, และ lag compensation
//! 
//! 🎮 "ในโลกของ multiplayer ทุกอย่างคือ lag แต่เราต้องทำให้มันไม่รู้สึก!"

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::fmt;

/// 🆔 Player ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player({})", self.0)
    }
}

/// 📦 Network Message Types
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkMessage {
    // Connection messages
    Connect { player_name: String },
    Disconnect { player_id: PlayerId },
    
    // Game state messages
    PlayerJoined { player_id: PlayerId, name: String },
    PlayerLeft { player_id: PlayerId },
    
    // Input messages
    PlayerInput { 
        player_id: PlayerId, 
        input: PlayerInput, 
        timestamp: u64,
        sequence: u32,
    },
    
    // State sync messages
    GameState { 
        state: GameState, 
        timestamp: u64,
        tick: u32,
    },
    PlayerState { 
        player_id: PlayerId, 
        position: (f32, f32), 
        velocity: (f32, f32),
        timestamp: u64,
    },
    
    // Game events
    PlayerDamaged { 
        player_id: PlayerId, 
        damage: f32, 
        source: PlayerId,
        timestamp: u64,
    },
    PlayerDied { 
        player_id: PlayerId, 
        killer: Option<PlayerId>,
        timestamp: u64,
    },
    
    // Chat messages
    ChatMessage { 
        player_id: PlayerId, 
        message: String, 
        timestamp: u64,
    },
    
    // Server messages
    Ping { timestamp: u64 },
    Pong { timestamp: u64 },
    ServerInfo { 
        tick_rate: u32, 
        max_players: u32, 
        current_players: u32,
    },
}

/// 🎮 Player Input
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInput {
    pub move_x: f32,
    pub move_y: f32,
    pub shoot: bool,
    pub jump: bool,
    pub interact: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub buttons: u32, // Bit flags for various buttons
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            move_x: 0.0,
            move_y: 0.0,
            shoot: false,
            jump: false,
            interact: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            buttons: 0,
        }
    }
    
    pub fn set_button(&mut self, button: u32, pressed: bool) {
        if pressed {
            self.buttons |= 1 << button;
        } else {
            self.buttons &= !(1 << button);
        }
    }
    
    pub fn is_button_pressed(&self, button: u32) -> bool {
        (self.buttons & (1 << button)) != 0
    }
}

/// 🎯 Game State
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub players: HashMap<PlayerId, PlayerState>,
    pub projectiles: Vec<Projectile>,
    pub items: Vec<Item>,
    pub game_time: f32,
    pub match_state: MatchState,
}

/// 👤 Player State
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub id: PlayerId,
    pub name: String,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub health: f32,
    pub max_health: f32,
    pub score: u32,
    pub is_alive: bool,
    pub last_input_sequence: u32,
}

/// 🚀 Projectile
#[derive(Debug, Clone, PartialEq)]
pub struct Projectile {
    pub id: u32,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub damage: f32,
    pub owner: PlayerId,
    pub lifetime: f32,
}

/// 📦 Item
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: u32,
    pub position: (f32, f32),
    pub item_type: ItemType,
    pub respawn_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    HealthPack,
    Ammo,
    Weapon { weapon_type: String },
    PowerUp { effect: String, duration: f32 },
}

/// 🏆 Match State
#[derive(Debug, Clone, PartialEq)]
pub enum MatchState {
    Waiting,
    Starting { countdown: f32 },
    InProgress,
    Finished { winner: Option<PlayerId> },
}

/// 📊 Network Statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub ping: Duration,
    pub packet_loss: f32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u32,
    pub messages_received: u32,
    pub last_update: Instant,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            ping: Duration::from_millis(0),
            packet_loss: 0.0,
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
            last_update: Instant::now(),
        }
    }
    
    pub fn update_ping(&mut self, ping: Duration) {
        self.ping = ping;
        self.last_update = Instant::now();
    }
    
    pub fn add_sent_message(&mut self, size: usize) {
        self.messages_sent += 1;
        self.bytes_sent += size as u64;
    }
    
    pub fn add_received_message(&mut self, size: usize) {
        self.messages_received += 1;
        self.bytes_received += size as u64;
    }
}

/// 🕰️ Input Buffer - สำหรับ lag compensation
#[derive(Debug)]
pub struct InputBuffer {
    inputs: VecDeque<(u32, PlayerInput, u64)>, // (sequence, input, timestamp)
    max_size: usize,
}

impl InputBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            inputs: VecDeque::new(),
            max_size,
        }
    }
    
    pub fn add_input(&mut self, sequence: u32, input: PlayerInput, timestamp: u64) {
        // เพิ่ม input ใหม่
        self.inputs.push_back((sequence, input, timestamp));
        
        // ลบ input เก่าถ้าเกิน max_size
        while self.inputs.len() > self.max_size {
            self.inputs.pop_front();
        }
        
        // เรียงตาม sequence
        self.inputs.make_contiguous().sort_by_key(|(seq, _, _)| *seq);
    }
    
    pub fn get_input_at_time(&self, timestamp: u64) -> Option<&PlayerInput> {
        // หา input ที่ใกล้เคียงกับ timestamp ที่ต้องการ
        self.inputs
            .iter()
            .min_by_key(|(_, _, ts)| (*ts as i64 - timestamp as i64).abs())
            .map(|(_, input, _)| input)
    }
    
    pub fn get_latest_input(&self) -> Option<&PlayerInput> {
        self.inputs.back().map(|(_, input, _)| input)
    }
    
    pub fn clear_old_inputs(&mut self, cutoff_timestamp: u64) {
        while let Some((_, _, timestamp)) = self.inputs.front() {
            if *timestamp < cutoff_timestamp {
                self.inputs.pop_front();
            } else {
                break;
            }
        }
    }
}

/// 🔄 State Interpolation - สำหรับ smooth movement
#[derive(Debug)]
pub struct StateInterpolator {
    states: VecDeque<(PlayerState, u64)>, // (state, timestamp)
    max_states: usize,
    interpolation_delay: Duration,
}

impl StateInterpolator {
    pub fn new(max_states: usize, interpolation_delay: Duration) -> Self {
        Self {
            states: VecDeque::new(),
            max_states,
            interpolation_delay,
        }
    }
    
    pub fn add_state(&mut self, state: PlayerState, timestamp: u64) {
        self.states.push_back((state, timestamp));
        
        // ลบ state เก่า
        while self.states.len() > self.max_states {
            self.states.pop_front();
        }
        
        // เรียงตาม timestamp
        self.states.make_contiguous().sort_by_key(|(_, ts)| *ts);
    }
    
    pub fn interpolate_at_time(&self, current_time: u64) -> Option<PlayerState> {
        // คำนวณ target time (ลบ interpolation delay)
        let target_time = current_time.saturating_sub(self.interpolation_delay.as_millis() as u64);
        
        // หา 2 states ที่อยู่รอบๆ target time
        let mut before_state = None;
        let mut after_state = None;
        
        for (state, timestamp) in &self.states {
            if *timestamp <= target_time {
                before_state = Some((state, *timestamp));
            } else if after_state.is_none() {
                after_state = Some((state, *timestamp));
                break;
            }
        }
        
        match (before_state, after_state) {
            (Some((before, before_time)), Some((after, after_time))) => {
                // Interpolate between two states
                let time_diff = after_time - before_time;
                if time_diff == 0 {
                    return Some(before.clone());
                }
                
                let t = (target_time - before_time) as f32 / time_diff as f32;
                let t = t.clamp(0.0, 1.0);
                
                let mut interpolated = before.clone();
                interpolated.position.0 = before.position.0 + (after.position.0 - before.position.0) * t;
                interpolated.position.1 = before.position.1 + (after.position.1 - before.position.1) * t;
                interpolated.velocity.0 = before.velocity.0 + (after.velocity.0 - before.velocity.0) * t;
                interpolated.velocity.1 = before.velocity.1 + (after.velocity.1 - before.velocity.1) * t;
                
                Some(interpolated)
            }
            (Some((state, _)), None) => {
                // ใช้ state ล่าสุด
                Some(state.clone())
            }
            _ => None,
        }
    }
    
    pub fn clear_old_states(&mut self, cutoff_timestamp: u64) {
        while let Some((_, timestamp)) = self.states.front() {
            if *timestamp < cutoff_timestamp {
                self.states.pop_front();
            } else {
                break;
            }
        }
    }
}

/// 🖥️ Game Server
#[derive(Debug)]
pub struct GameServer {
    pub players: HashMap<PlayerId, PlayerState>,
    pub input_buffers: HashMap<PlayerId, InputBuffer>,
    pub game_state: GameState,
    pub tick_rate: u32,
    pub current_tick: u32,
    pub start_time: Instant,
    pub max_players: u32,
    pub network_stats: HashMap<PlayerId, NetworkStats>,
}

impl GameServer {
    pub fn new(tick_rate: u32, max_players: u32) -> Self {
        Self {
            players: HashMap::new(),
            input_buffers: HashMap::new(),
            game_state: GameState {
                players: HashMap::new(),
                projectiles: Vec::new(),
                items: Vec::new(),
                game_time: 0.0,
                match_state: MatchState::Waiting,
            },
            tick_rate,
            current_tick: 0,
            start_time: Instant::now(),
            max_players,
            network_stats: HashMap::new(),
        }
    }
    
    /// เพิ่มผู้เล่นใหม่
    pub fn add_player(&mut self, player_id: PlayerId, name: String) -> bool {
        if self.players.len() >= self.max_players as usize {
            return false;
        }
        
        let player_state = PlayerState {
            id: player_id,
            name: name.clone(),
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            health: 100.0,
            max_health: 100.0,
            score: 0,
            is_alive: true,
            last_input_sequence: 0,
        };
        
        self.players.insert(player_id, player_state.clone());
        self.game_state.players.insert(player_id, player_state);
        self.input_buffers.insert(player_id, InputBuffer::new(60)); // 1 second at 60fps
        self.network_stats.insert(player_id, NetworkStats::new());
        
        println!("👋 Player {} ({}) joined the game", player_id, name);
        true
    }
    
    /// ลบผู้เล่น
    pub fn remove_player(&mut self, player_id: PlayerId) {
        if let Some(player) = self.players.remove(&player_id) {
            self.game_state.players.remove(&player_id);
            self.input_buffers.remove(&player_id);
            self.network_stats.remove(&player_id);
            println!("👋 Player {} ({}) left the game", player_id, player.name);
        }
    }
    
    /// ประมวลผล input ของผู้เล่น
    pub fn process_player_input(&mut self, player_id: PlayerId, input: PlayerInput, sequence: u32, timestamp: u64) {
        if let Some(buffer) = self.input_buffers.get_mut(&player_id) {
            buffer.add_input(sequence, input.clone(), timestamp);
        }
        
        if let Some(player) = self.players.get_mut(&player_id) {
            // อัปเดต sequence number
            if sequence > player.last_input_sequence {
                player.last_input_sequence = sequence;
                
                // ประมวลผล input
                self.apply_input_to_player(player_id, &input);
            }
        }
    }
    
    /// ใช้ input กับผู้เล่น
    fn apply_input_to_player(&mut self, player_id: PlayerId, input: &PlayerInput) {
        let mut should_shoot = false;
        let mut should_jump = false;
        
        // อัปเดตข้อมูลผู้เล่นก่อน
        if let Some(player) = self.players.get_mut(&player_id) {
            if !player.is_alive {
                return;
            }
            
            // อัปเดตความเร็ว
            let speed = 100.0; // pixels per second
            player.velocity.0 = input.move_x * speed;
            player.velocity.1 = input.move_y * speed;
            
            // เก็บข้อมูลการกระทำ
            should_shoot = input.shoot;
            should_jump = input.jump;
            
            if should_jump {
                // จัดการการกระโดด
                player.velocity.1 += 200.0;
            }
        }
        
        // ประมวลผลการกระทำที่ต้องใช้ mutable borrow ของ self แยกต่างหาก
        if should_shoot {
            self.spawn_projectile(player_id, input.mouse_x, input.mouse_y);
        }
    }
    
    /// สร้าง projectile
    fn spawn_projectile(&mut self, owner: PlayerId, target_x: f32, target_y: f32) {
        if let Some(player) = self.players.get(&owner) {
            let dx = target_x - player.position.0;
            let dy = target_y - player.position.1;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 0.0 {
                let speed = 500.0;
                let velocity = (dx / distance * speed, dy / distance * speed);
                
                let projectile = Projectile {
                    id: self.current_tick, // ใช้ tick เป็น ID
                    position: player.position,
                    velocity,
                    damage: 25.0,
                    owner,
                    lifetime: 2.0, // 2 seconds
                };
                
                self.game_state.projectiles.push(projectile);
                println!("🚀 {} fired a projectile", owner);
            }
        }
    }
    
    /// อัปเดต game state
    pub fn update(&mut self, delta_time: f32) {
        self.current_tick += 1;
        self.game_state.game_time += delta_time;
        
        // อัปเดตตำแหน่งผู้เล่น
        for player in self.players.values_mut() {
            if player.is_alive {
                player.position.0 += player.velocity.0 * delta_time;
                player.position.1 += player.velocity.1 * delta_time;
                
                // จำกัดขอบเขต
                player.position.0 = player.position.0.clamp(-500.0, 500.0);
                player.position.1 = player.position.1.clamp(-500.0, 500.0);
                
                // ลดความเร็วลง (friction)
                player.velocity.0 *= 0.9;
                player.velocity.1 *= 0.9;
            }
        }
        
        // อัปเดต projectiles
        self.update_projectiles(delta_time);
        
        // ตรวจสอบการชน
        self.check_collisions();
        
        // อัปเดต game state
        self.game_state.players = self.players.clone();
        
        // ตรวจสอบสภาพการแข่งขัน
        self.update_match_state();
    }
    
    /// อัปเดต projectiles
    fn update_projectiles(&mut self, delta_time: f32) {
        self.game_state.projectiles.retain_mut(|projectile| {
            // อัปเดตตำแหน่ง
            projectile.position.0 += projectile.velocity.0 * delta_time;
            projectile.position.1 += projectile.velocity.1 * delta_time;
            
            // ลด lifetime
            projectile.lifetime -= delta_time;
            
            // ลบถ้าหมดอายุหรือออกนอกขอบเขต
            projectile.lifetime > 0.0 && 
            projectile.position.0.abs() < 1000.0 && 
            projectile.position.1.abs() < 1000.0
        });
    }
    
    /// ตรวจสอบการชน
    fn check_collisions(&mut self) {
        let mut hits = Vec::new();
        
        // ตรวจสอบการชนระหว่าง projectiles กับผู้เล่น
        for projectile in &self.game_state.projectiles {
            for player in self.players.values() {
                if player.id == projectile.owner || !player.is_alive {
                    continue;
                }
                
                let dx = projectile.position.0 - player.position.0;
                let dy = projectile.position.1 - player.position.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance < 20.0 { // collision radius
                    hits.push((projectile.id, player.id, projectile.damage, projectile.owner));
                }
            }
        }
        
        // ประมวลผลการชน
        for (projectile_id, player_id, damage, attacker) in hits {
            // ลบ projectile
            self.game_state.projectiles.retain(|p| p.id != projectile_id);
            
            // ทำความเสียหายให้ผู้เล่น
            if let Some(player) = self.players.get_mut(&player_id) {
                player.health -= damage;
                println!("💥 {} hit {} for {} damage", attacker, player_id, damage);
                
                if player.health <= 0.0 {
                    player.is_alive = false;
                    player.health = 0.0;
                    println!("💀 {} was killed by {}", player_id, attacker);
                }
            }
            
            // เพิ่มคะแนนให้ผู้โจมตี (แยกออกมาเพื่อหลีกเลี่ยง double borrow)
            if let Some(attacker_player) = self.players.get_mut(&attacker) {
                attacker_player.score += 1;
            }
        }
    }
    
    /// อัปเดตสภาพการแข่งขัน
    fn update_match_state(&mut self) {
        match &self.game_state.match_state {
            MatchState::Waiting => {
                if self.players.len() >= 2 {
                    self.game_state.match_state = MatchState::Starting { countdown: 3.0 };
                    println!("🎮 Match starting in 3 seconds...");
                }
            }
            MatchState::Starting { countdown } => {
                let new_countdown = countdown - (1.0 / self.tick_rate as f32);
                if new_countdown <= 0.0 {
                    self.game_state.match_state = MatchState::InProgress;
                    println!("🚀 Match started!");
                } else {
                    self.game_state.match_state = MatchState::Starting { countdown: new_countdown };
                }
            }
            MatchState::InProgress => {
                // ตรวจสอบเงื่อนไขการชนะ
                let alive_players: Vec<_> = self.players.values().filter(|p| p.is_alive).collect();
                
                if alive_players.len() <= 1 {
                    let winner = alive_players.first().map(|p| p.id);
                    self.game_state.match_state = MatchState::Finished { winner };
                    
                    if let Some(winner_id) = winner {
                        println!("🏆 {} wins the match!", winner_id);
                    } else {
                        println!("🤝 Match ended in a draw!");
                    }
                }
            }
            MatchState::Finished { .. } => {
                // รอการเริ่มเกมใหม่
            }
        }
    }
    
    /// ดึงข้อมูลสำหรับส่งให้ client
    pub fn get_game_state_for_client(&self, _client_id: PlayerId) -> GameState {
        // ในเกมจริงอาจจะต้องกรองข้อมูลตาม client
        self.game_state.clone()
    }
    
    /// ดึงสถิติเซิร์ฟเวอร์
    pub fn get_server_stats(&self) -> ServerStats {
        ServerStats {
            current_players: self.players.len() as u32,
            max_players: self.max_players,
            current_tick: self.current_tick,
            uptime: self.start_time.elapsed(),
            total_projectiles: self.game_state.projectiles.len() as u32,
        }
    }
}

/// 📊 Server Statistics
#[derive(Debug, Clone)]
pub struct ServerStats {
    pub current_players: u32,
    pub max_players: u32,
    pub current_tick: u32,
    pub uptime: Duration,
    pub total_projectiles: u32,
}

/// 💻 Game Client
#[derive(Debug)]
pub struct GameClient {
    pub player_id: Option<PlayerId>,
    pub local_state: GameState,
    pub interpolators: HashMap<PlayerId, StateInterpolator>,
    pub input_sequence: u32,
    pub network_stats: NetworkStats,
    pub prediction_enabled: bool,
    pub interpolation_enabled: bool,
}

impl GameClient {
    pub fn new() -> Self {
        Self {
            player_id: None,
            local_state: GameState {
                players: HashMap::new(),
                projectiles: Vec::new(),
                items: Vec::new(),
                game_time: 0.0,
                match_state: MatchState::Waiting,
            },
            interpolators: HashMap::new(),
            input_sequence: 0,
            network_stats: NetworkStats::new(),
            prediction_enabled: true,
            interpolation_enabled: true,
        }
    }
    
    /// เชื่อมต่อกับเซิร์ฟเวอร์
    pub fn connect(&mut self, player_name: String) -> NetworkMessage {
        println!("🔌 Connecting to server as {}", player_name);
        NetworkMessage::Connect { player_name }
    }
    
    /// ส่ง input ไปยังเซิร์ฟเวอร์
    pub fn send_input(&mut self, input: PlayerInput) -> Option<NetworkMessage> {
        if let Some(player_id) = self.player_id {
            self.input_sequence += 1;
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            // Client-side prediction
            if self.prediction_enabled {
                self.apply_input_prediction(&input);
            }
            
            Some(NetworkMessage::PlayerInput {
                player_id,
                input,
                timestamp,
                sequence: self.input_sequence,
            })
        } else {
            None
        }
    }
    
    /// ใช้ client-side prediction
    fn apply_input_prediction(&mut self, input: &PlayerInput) {
        if let Some(player_id) = self.player_id {
            if let Some(player) = self.local_state.players.get_mut(&player_id) {
                // ทำนายการเคลื่อนไหว
                let speed = 100.0;
                let delta_time = 1.0 / 60.0; // สมมติ 60 FPS
                
                player.velocity.0 = input.move_x * speed;
                player.velocity.1 = input.move_y * speed;
                
                player.position.0 += player.velocity.0 * delta_time;
                player.position.1 += player.velocity.1 * delta_time;
                
                // จำกัดขอบเขต
                player.position.0 = player.position.0.clamp(-500.0, 500.0);
                player.position.1 = player.position.1.clamp(-500.0, 500.0);
            }
        }
    }
    
    /// ประมวลผลข้อความจากเซิร์ฟเวอร์
    pub fn process_server_message(&mut self, message: NetworkMessage) {
        match message {
            NetworkMessage::PlayerJoined { player_id, name } => {
                if self.player_id.is_none() {
                    self.player_id = Some(player_id);
                    println!("✅ Connected as {} ({})", name, player_id);
                } else {
                    println!("👋 {} ({}) joined the game", name, player_id);
                }
            }
            
            NetworkMessage::GameState { state, timestamp: _, tick: _ } => {
                self.update_game_state(state);
            }
            
            NetworkMessage::PlayerState { player_id, position, velocity, timestamp } => {
                if self.interpolation_enabled {
                    // เพิ่มเข้า interpolator
                    let interpolator = self.interpolators.entry(player_id)
                        .or_insert_with(|| StateInterpolator::new(10, Duration::from_millis(100)));
                    
                    let player_state = PlayerState {
                        id: player_id,
                        name: String::new(), // จะได้จาก game state
                        position,
                        velocity,
                        health: 100.0, // จะได้จาก game state
                        max_health: 100.0,
                        score: 0,
                        is_alive: true,
                        last_input_sequence: 0,
                    };
                    
                    interpolator.add_state(player_state, timestamp);
                }
            }
            
            NetworkMessage::Pong { timestamp } => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                
                let ping = Duration::from_millis(now - timestamp);
                self.network_stats.update_ping(ping);
            }
            
            NetworkMessage::ChatMessage { player_id, message, timestamp: _ } => {
                println!("💬 {}: {}", player_id, message);
            }
            
            _ => {
                // ประมวลผลข้อความอื่นๆ
            }
        }
    }
    
    /// อัปเดต game state จากเซิร์ฟเวอร์
    fn update_game_state(&mut self, server_state: GameState) {
        // Server reconciliation - แก้ไข prediction ที่ผิด
        if let Some(player_id) = self.player_id {
            if let (Some(server_player), Some(local_player)) = (
                server_state.players.get(&player_id),
                self.local_state.players.get(&player_id)
            ) {
                // ตรวจสอบความแตกต่างของตำแหน่ง
                let dx = server_player.position.0 - local_player.position.0;
                let dy = server_player.position.1 - local_player.position.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 10.0 { // threshold สำหรับการแก้ไข
                    println!("🔧 Server reconciliation: correcting position by {:.1} units", distance);
                    // แก้ไขตำแหน่ง
                    if let Some(local_player) = self.local_state.players.get_mut(&player_id) {
                        local_player.position = server_player.position;
                    }
                }
            }
        }
        
        // อัปเดต state อื่นๆ
        self.local_state = server_state;
    }
    
    /// อัปเดต client
    pub fn update(&mut self, delta_time: f32) {
        // อัปเดต interpolation
        if self.interpolation_enabled {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            for (player_id, interpolator) in &self.interpolators {
                if let Some(interpolated_state) = interpolator.interpolate_at_time(current_time) {
                    if let Some(player) = self.local_state.players.get_mut(player_id) {
                        // อัปเดตเฉพาะผู้เล่นอื่น (ไม่ใช่ตัวเอง)
                        if Some(*player_id) != self.player_id {
                            player.position = interpolated_state.position;
                            player.velocity = interpolated_state.velocity;
                        }
                    }
                }
            }
        }
        
        // อัปเดต local game time
        self.local_state.game_time += delta_time;
    }
    
    /// ส่ง ping ไปยังเซิร์ฟเวอร์
    pub fn send_ping(&self) -> NetworkMessage {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        NetworkMessage::Ping { timestamp }
    }
    
    /// ดึงสถิติเครือข่าย
    pub fn get_network_stats(&self) -> &NetworkStats {
        &self.network_stats
    }
}

/// 🎮 สาธิตการใช้งาน Game Networking
pub fn demonstrate_game_networking() {
    println!("🌐 === Game Networking Demo ===");
    
    // สร้างเซิร์ฟเวอร์
    let mut server = GameServer::new(60, 4); // 60 TPS, 4 players max
    println!("🖥️ Created game server (60 TPS, 4 players max)");
    
    // สร้าง clients
    let mut client1 = GameClient::new();
    let mut client2 = GameClient::new();
    
    println!("💻 Created 2 game clients");
    
    // จำลองการเชื่อมต่อ
    println!("\n🔌 Simulating connections:");
    
    let connect_msg1 = client1.connect("Alice".to_string());
    let connect_msg2 = client2.connect("Bob".to_string());
    
    // เซิร์ฟเวอร์ประมวลผลการเชื่อมต่อ
    if let NetworkMessage::Connect { player_name } = connect_msg1 {
        let player_id = PlayerId(1);
        server.add_player(player_id, player_name);
        client1.process_server_message(NetworkMessage::PlayerJoined { 
            player_id, 
            name: "Alice".to_string() 
        });
    }
    
    if let NetworkMessage::Connect { player_name } = connect_msg2 {
        let player_id = PlayerId(2);
        server.add_player(player_id, player_name);
        client2.process_server_message(NetworkMessage::PlayerJoined { 
            player_id, 
            name: "Bob".to_string() 
        });
    }
    
    println!("\n🎮 Starting game simulation:");
    
    // จำลองเกม
    for frame in 0..300 { // 5 seconds at 60 FPS
        let delta_time = 1.0 / 60.0;
        
        // จำลอง input จาก clients
        if frame % 10 == 0 { // ส่ง input ทุก 10 frames
            // Client 1 input
            let mut input1 = PlayerInput::new();
            input1.move_x = if frame < 120 { 1.0 } else { -0.5 };
            input1.move_y = (frame as f32 * 0.1).sin() * 0.5;
            input1.shoot = frame % 30 == 0;
            
            if let Some(msg) = client1.send_input(input1.clone()) {
                if let NetworkMessage::PlayerInput { player_id, input, sequence, timestamp } = msg {
                    server.process_player_input(player_id, input, sequence, timestamp);
                }
            }
            
            // Client 2 input
            let mut input2 = PlayerInput::new();
            input2.move_x = if frame < 180 { -0.8 } else { 1.0 };
            input2.move_y = (frame as f32 * 0.05).cos() * 0.3;
            input2.shoot = frame % 45 == 0;
            
            if let Some(msg) = client2.send_input(input2.clone()) {
                if let NetworkMessage::PlayerInput { player_id, input, sequence, timestamp } = msg {
                    server.process_player_input(player_id, input, sequence, timestamp);
                }
            }
        }
        
        // อัปเดตเซิร์ฟเวอร์
        server.update(delta_time);
        
        // ส่ง game state ไปยัง clients (ทุก 3 frames)
        if frame % 3 == 0 {
            let game_state = server.get_game_state_for_client(PlayerId(1));
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            let state_msg = NetworkMessage::GameState {
                state: game_state.clone(),
                timestamp,
                tick: server.current_tick,
            };
            
            client1.process_server_message(state_msg.clone());
            client2.process_server_message(state_msg);
        }
        
        // อัปเดต clients
        client1.update(delta_time);
        client2.update(delta_time);
        
        // แสดงข้อมูลทุก 60 frames (1 second)
        if frame % 60 == 0 && frame > 0 {
            println!("\n--- Second {} ---", frame / 60);
            
            let server_stats = server.get_server_stats();
            println!("🖥️ Server: {} players, tick {}, {} projectiles", 
                    server_stats.current_players, server_stats.current_tick, 
                    server_stats.total_projectiles);
            
            // แสดงตำแหน่งผู้เล่น
            for (id, player) in &server.players {
                println!("👤 {} ({}): pos=({:.1}, {:.1}), hp={:.1}, score={}, alive={}", 
                        player.name, id, player.position.0, player.position.1, 
                        player.health, player.score, player.is_alive);
            }
            
            // แสดงสถานะการแข่งขัน
            match &server.game_state.match_state {
                MatchState::Waiting => println!("⏳ Waiting for players..."),
                MatchState::Starting { countdown } => println!("🚀 Starting in {:.1}s", countdown),
                MatchState::InProgress => println!("🎮 Match in progress"),
                MatchState::Finished { winner } => {
                    if let Some(winner_id) = winner {
                        println!("🏆 Match finished! Winner: {}", winner_id);
                    } else {
                        println!("🤝 Match finished in a draw");
                    }
                }
            }
        }
        
        // หยุดถ้าเกมจบ
        if matches!(server.game_state.match_state, MatchState::Finished { .. }) {
            break;
        }
    }
    
    // ทดสอบ ping
    println!("\n📡 Testing ping:");
    let ping_msg = client1.send_ping();
    if let NetworkMessage::Ping { timestamp } = ping_msg {
        let pong_msg = NetworkMessage::Pong { timestamp };
        client1.process_server_message(pong_msg);
        
        let stats = client1.get_network_stats();
        println!("🏓 Client 1 ping: {}ms", stats.ping.as_millis());
    }
    
    // แสดง best practices
    println!("\n💡 Game Networking Best Practices:");
    show_networking_best_practices();
}

/// 💡 Game Networking Best Practices
fn show_networking_best_practices() {
    let practices = vec![
        "🎯 Use client-side prediction for responsive controls",
        "🔄 Implement server reconciliation for accuracy",
        "📈 Use interpolation for smooth movement",
        "⏱️ Implement lag compensation for fair gameplay",
        "📦 Compress network messages efficiently",
        "🔒 Validate all client input on server",
        "📊 Monitor network statistics continuously",
        "🎮 Prioritize gameplay-critical messages",
        "🔄 Use delta compression for state updates",
        "⚡ Batch multiple small messages together",
        "🛡️ Implement anti-cheat measures",
        "📱 Handle network disconnections gracefully",
        "🎯 Use different update rates for different data",
        "🔍 Implement proper error handling",
        "📈 Use adaptive quality based on connection",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Networking Libraries:");
    println!("   • tokio - Async runtime for networking");
    println!("   • quinn - QUIC protocol implementation");
    println!("   • laminar - UDP-based game networking");
    println!("   • renet - High-level networking for games");
    println!("   • bevy_networking_turbulence - Bevy networking plugin");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_player_input() {
        let mut input = PlayerInput::new();
        
        input.set_button(0, true);
        assert!(input.is_button_pressed(0));
        
        input.set_button(0, false);
        assert!(!input.is_button_pressed(0));
    }
    
    #[test]
    fn test_input_buffer() {
        let mut buffer = InputBuffer::new(5);
        let input = PlayerInput::new();
        
        buffer.add_input(1, input.clone(), 100);
        buffer.add_input(2, input.clone(), 200);
        
        assert!(buffer.get_input_at_time(150).is_some());
        assert!(buffer.get_latest_input().is_some());
    }
    
    #[test]
    fn test_game_server() {
        let mut server = GameServer::new(60, 4);
        
        assert!(server.add_player(PlayerId(1), "Test".to_string()));
        assert_eq!(server.players.len(), 1);
        
        server.remove_player(PlayerId(1));
        assert_eq!(server.players.len(), 0);
    }
    
    #[test]
    fn test_health_system() {
        let mut server = GameServer::new(60, 4);
        server.add_player(PlayerId(1), "Test".to_string());
        
        // ทำความเสียหาย
        if let Some(player) = server.players.get_mut(&PlayerId(1)) {
            player.health = 10.0;
        }
        
        server.update(1.0 / 60.0);
        
        let player = server.players.get(&PlayerId(1)).unwrap();
        assert_eq!(player.health, 10.0);
    }
    
    #[test]
    fn test_network_stats() {
        let mut stats = NetworkStats::new();
        
        stats.add_sent_message(100);
        assert_eq!(stats.messages_sent, 1);
        assert_eq!(stats.bytes_sent, 100);
        
        stats.update_ping(Duration::from_millis(50));
        assert_eq!(stats.ping.as_millis(), 50);
    }
}

// 🌐 "ในโลกของ multiplayer ทุกอย่างคือเรื่องของเวลา:
//     Client prediction ทำให้รู้สึกเร็ว
//     Server reconciliation ทำให้ถูกต้อง
//     Interpolation ทำให้ลื่นไหล
//     Lag compensation ทำให้ยุติธรรม
//     รวมกันแล้วได้เกมที่เล่นสนุก! 🎮✨"