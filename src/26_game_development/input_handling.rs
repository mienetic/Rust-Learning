//! 🎮 Input Handling - ระบบจัดการการป้อนข้อมูล
//! 
//! โมดูลนี้สาธิตการสร้างระบบจัดการ input สำหรับเกม
//! รวมถึง keyboard, mouse, gamepad, และ touch input
//! 
//! 🕹️ "Input ที่ดีทำให้เกมเล่นสนุก Input ที่แย่ทำให้อยากปิดเกม!"

use std::collections::{HashMap, HashSet};
use std::fmt;

/// ⌨️ Key codes สำหรับ keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Arrow keys
    Up, Down, Left, Right,
    
    // Special keys
    Space, Enter, Escape, Tab, Backspace, Delete,
    Shift, Ctrl, Alt, Super,
    
    // Other
    Comma, Period, Slash, Semicolon, Quote,
    LeftBracket, RightBracket, Backslash,
    Minus, Equal,
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyCode::A => write!(f, "A"),
            KeyCode::B => write!(f, "B"),
            KeyCode::Space => write!(f, "Space"),
            KeyCode::Enter => write!(f, "Enter"),
            KeyCode::Up => write!(f, "Up Arrow"),
            KeyCode::Down => write!(f, "Down Arrow"),
            KeyCode::Left => write!(f, "Left Arrow"),
            KeyCode::Right => write!(f, "Right Arrow"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// 🖱️ Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// 🎮 Gamepad buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    // Face buttons (Xbox naming)
    A, B, X, Y,
    
    // D-pad
    DPadUp, DPadDown, DPadLeft, DPadRight,
    
    // Shoulder buttons
    LeftShoulder, RightShoulder,
    
    // Triggers (as buttons)
    LeftTrigger, RightTrigger,
    
    // Stick buttons
    LeftStick, RightStick,
    
    // Menu buttons
    Start, Select, Home,
}

/// 🎮 Gamepad axes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX, LeftStickY,
    RightStickX, RightStickY,
    LeftTrigger, RightTrigger,
}

/// 📱 Touch input
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 - 1.0
    pub is_active: bool,
}

impl TouchPoint {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            id,
            x,
            y,
            pressure: 1.0,
            is_active: true,
        }
    }
    
    /// คำนวณระยะห่างจาก touch point อื่น
    pub fn distance_to(&self, other: &TouchPoint) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// 🎯 Input state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputState {
    Released,  // ไม่ได้กด
    Pressed,   // เพิ่งกด (frame นี้)
    Held,      // กดค้างอยู่
    JustReleased, // เพิ่งปล่อย (frame นี้)
}

/// ⌨️ Keyboard Input Manager
#[derive(Debug)]
pub struct KeyboardInput {
    pub current_keys: HashSet<KeyCode>,
    pub previous_keys: HashSet<KeyCode>,
    pub key_states: HashMap<KeyCode, InputState>,
    pub repeat_delay: f32,    // วินาทีก่อนเริ่ม repeat
    pub repeat_rate: f32,     // วินาทีระหว่าง repeat
    pub key_timers: HashMap<KeyCode, f32>,
}

impl KeyboardInput {
    /// สร้าง keyboard input ใหม่
    pub fn new() -> Self {
        Self {
            current_keys: HashSet::new(),
            previous_keys: HashSet::new(),
            key_states: HashMap::new(),
            repeat_delay: 0.5,
            repeat_rate: 0.05,
            key_timers: HashMap::new(),
        }
    }
    
    /// อัปเดต keyboard state
    pub fn update(&mut self, delta_time: f32) {
        // เก็บ previous state ก่อนอัปเดต
        let old_previous_keys = self.previous_keys.clone();
        
        // อัปเดต key states
        for &key in &self.current_keys {
            let was_pressed = old_previous_keys.contains(&key);
            
            if was_pressed {
                self.key_states.insert(key, InputState::Held);
            } else {
                self.key_states.insert(key, InputState::Pressed);
                self.key_timers.insert(key, 0.0);
            }
        }
        
        // ตรวจสอบ keys ที่ปล่อย
        for &key in &old_previous_keys {
            if !self.current_keys.contains(&key) {
                self.key_states.insert(key, InputState::JustReleased);
                self.key_timers.remove(&key);
            }
        }
        
        // อัปเดต previous state หลังจากประมวลผล
        self.previous_keys = self.current_keys.clone();
        
        // อัปเดต key timers สำหรับ repeat
        for (key, timer) in &mut self.key_timers {
            *timer += delta_time;
        }
        
        // ลบ keys ที่ไม่ได้กดแล้ว
        self.key_states.retain(|key, _| {
            self.current_keys.contains(key) || old_previous_keys.contains(key)
        });
    }
    
    /// กดปุ่ม
    pub fn press_key(&mut self, key: KeyCode) {
        self.current_keys.insert(key);
    }
    
    /// ปล่อยปุ่ม
    pub fn release_key(&mut self, key: KeyCode) {
        self.current_keys.remove(&key);
    }
    
    /// ตรวจสอบว่าปุ่มถูกกดอยู่หรือไม่
    pub fn is_key_held(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::Held) | Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่าปุ่มเพิ่งถูกกดหรือไม่
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่าปุ่มเพิ่งถูกปล่อยหรือไม่
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::JustReleased))
    }
    
    /// ตรวจสอบ key repeat
    pub fn is_key_repeating(&self, key: KeyCode) -> bool {
        if let Some(&timer) = self.key_timers.get(&key) {
            if timer > self.repeat_delay {
                let repeat_time = timer - self.repeat_delay;
                return (repeat_time / self.repeat_rate).fract() < (self.repeat_rate / 2.0);
            }
        }
        false
    }
    
    /// ดึงปุ่มที่กดอยู่ทั้งหมด
    pub fn get_pressed_keys(&self) -> Vec<KeyCode> {
        self.current_keys.iter().cloned().collect()
    }
}

/// 🖱️ Mouse Input Manager
#[derive(Debug)]
pub struct MouseInput {
    pub position: (f32, f32),
    pub previous_position: (f32, f32),
    pub delta: (f32, f32),
    pub wheel_delta: f32,
    pub button_states: HashMap<MouseButton, InputState>,
    pub current_buttons: HashSet<MouseButton>,
    pub previous_buttons: HashSet<MouseButton>,
    pub sensitivity: f32,
}

impl MouseInput {
    /// สร้าง mouse input ใหม่
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            previous_position: (0.0, 0.0),
            delta: (0.0, 0.0),
            wheel_delta: 0.0,
            button_states: HashMap::new(),
            current_buttons: HashSet::new(),
            previous_buttons: HashSet::new(),
            sensitivity: 1.0,
        }
    }
    
    /// อัปเดต mouse state
    pub fn update(&mut self) {
        // อัปเดต delta
        self.delta = (
            (self.position.0 - self.previous_position.0) * self.sensitivity,
            (self.position.1 - self.previous_position.1) * self.sensitivity
        );
        
        self.previous_position = self.position;
        
        // เก็บ previous buttons ก่อนอัปเดต
        let old_previous_buttons = self.previous_buttons.clone();
        
        // อัปเดต button states
        for &button in &self.current_buttons {
            let was_pressed = old_previous_buttons.contains(&button);
            
            if was_pressed {
                self.button_states.insert(button, InputState::Held);
            } else {
                self.button_states.insert(button, InputState::Pressed);
            }
        }
        
        // ตรวจสอบ buttons ที่ปล่อย
        for &button in &old_previous_buttons {
            if !self.current_buttons.contains(&button) {
                self.button_states.insert(button, InputState::JustReleased);
            }
        }
        
        // อัปเดต previous buttons หลังจากประมวลผล
        self.previous_buttons = self.current_buttons.clone();
        
        // รีเซ็ต wheel delta
        self.wheel_delta = 0.0;
        
        // ลบ buttons ที่ไม่ได้กดแล้ว
        self.button_states.retain(|button, _| {
            self.current_buttons.contains(button) || old_previous_buttons.contains(button)
        });
    }
    
    /// ตั้งตำแหน่ง mouse
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }
    
    /// กดปุ่ม mouse
    pub fn press_button(&mut self, button: MouseButton) {
        self.current_buttons.insert(button);
    }
    
    /// ปล่อยปุ่ม mouse
    pub fn release_button(&mut self, button: MouseButton) {
        self.current_buttons.remove(&button);
    }
    
    /// ตั้งค่า wheel delta
    pub fn set_wheel_delta(&mut self, delta: f32) {
        self.wheel_delta = delta;
    }
    
    /// ตรวจสอบว่าปุ่มถูกกดอยู่หรือไม่
    pub fn is_button_held(&self, button: MouseButton) -> bool {
        matches!(self.button_states.get(&button), Some(InputState::Held) | Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่าปุ่มเพิ่งถูกกดหรือไม่
    pub fn is_button_just_pressed(&self, button: MouseButton) -> bool {
        matches!(self.button_states.get(&button), Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่าปุ่มเพิ่งถูกปล่อยหรือไม่
    pub fn is_button_just_released(&self, button: MouseButton) -> bool {
        matches!(self.button_states.get(&button), Some(InputState::JustReleased))
    }
    
    /// ดึงการเคลื่อนไหวของ mouse
    pub fn get_delta(&self) -> (f32, f32) {
        self.delta
    }
}

/// 🎮 Gamepad Input Manager
#[derive(Debug)]
pub struct GamepadInput {
    pub id: u32,
    pub is_connected: bool,
    pub button_states: HashMap<GamepadButton, InputState>,
    pub current_buttons: HashSet<GamepadButton>,
    pub previous_buttons: HashSet<GamepadButton>,
    pub axis_values: HashMap<GamepadAxis, f32>,
    pub deadzone: f32,
    pub vibration: (f32, f32), // (left motor, right motor)
}

impl GamepadInput {
    /// สร้าง gamepad input ใหม่
    pub fn new(id: u32) -> Self {
        Self {
            id,
            is_connected: false,
            button_states: HashMap::new(),
            current_buttons: HashSet::new(),
            previous_buttons: HashSet::new(),
            axis_values: HashMap::new(),
            deadzone: 0.1,
            vibration: (0.0, 0.0),
        }
    }
    
    /// เชื่อมต่อ gamepad
    pub fn connect(&mut self) {
        self.is_connected = true;
        println!("🎮 Gamepad {} connected", self.id);
    }
    
    /// ตัดการเชื่อมต่อ gamepad
    pub fn disconnect(&mut self) {
        self.is_connected = false;
        self.current_buttons.clear();
        self.axis_values.clear();
        println!("🔌 Gamepad {} disconnected", self.id);
    }
    
    /// อัปเดต gamepad state
    pub fn update(&mut self) {
        if !self.is_connected {
            return;
        }
        
        // เก็บ previous buttons ก่อนอัปเดต
        let old_previous_buttons = self.previous_buttons.clone();
        
        // อัปเดต button states
        for &button in &self.current_buttons {
            let was_pressed = old_previous_buttons.contains(&button);
            
            if was_pressed {
                self.button_states.insert(button, InputState::Held);
            } else {
                self.button_states.insert(button, InputState::Pressed);
            }
        }
        
        // ตรวจสอบ buttons ที่ปล่อย
        for &button in &old_previous_buttons {
            if !self.current_buttons.contains(&button) {
                self.button_states.insert(button, InputState::JustReleased);
            }
        }
        
        // อัปเดต previous buttons หลังจากประมวลผล
        self.previous_buttons = self.current_buttons.clone();
        
        // ลบ buttons ที่ไม่ได้กดแล้ว
        self.button_states.retain(|button, _| {
            self.current_buttons.contains(button) || old_previous_buttons.contains(button)
        });
    }
    
    /// กดปุ่ม gamepad
    pub fn press_button(&mut self, button: GamepadButton) {
        if self.is_connected {
            self.current_buttons.insert(button);
        }
    }
    
    /// ปล่อยปุ่ม gamepad
    pub fn release_button(&mut self, button: GamepadButton) {
        self.current_buttons.remove(&button);
    }
    
    /// ตั้งค่า axis
    pub fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
        if self.is_connected {
            // ใช้ deadzone
            let adjusted_value = if value.abs() < self.deadzone {
                0.0
            } else {
                value
            };
            self.axis_values.insert(axis, adjusted_value);
        }
    }
    
    /// ดึงค่า axis
    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        self.axis_values.get(&axis).copied().unwrap_or(0.0)
    }
    
    /// ดึงค่า stick เป็น vector
    pub fn get_left_stick(&self) -> (f32, f32) {
        (
            self.get_axis(GamepadAxis::LeftStickX),
            self.get_axis(GamepadAxis::LeftStickY)
        )
    }
    
    /// ดึงค่า stick เป็น vector
    pub fn get_right_stick(&self) -> (f32, f32) {
        (
            self.get_axis(GamepadAxis::RightStickX),
            self.get_axis(GamepadAxis::RightStickY)
        )
    }
    
    /// ตั้งค่าการสั่น
    pub fn set_vibration(&mut self, left: f32, right: f32) {
        if self.is_connected {
            self.vibration = (left.clamp(0.0, 1.0), right.clamp(0.0, 1.0));
        }
    }
    
    /// ตรวจสอบว่าปุ่มถูกกดอยู่หรือไม่
    pub fn is_button_held(&self, button: GamepadButton) -> bool {
        matches!(self.button_states.get(&button), Some(InputState::Held) | Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่าปุ่มเพิ่งถูกกดหรือไม่
    pub fn is_button_just_pressed(&self, button: GamepadButton) -> bool {
        matches!(self.button_states.get(&button), Some(InputState::Pressed))
    }
}

/// 📱 Touch Input Manager
#[derive(Debug)]
pub struct TouchInput {
    pub touches: HashMap<u32, TouchPoint>,
    pub previous_touches: HashMap<u32, TouchPoint>,
    pub max_touches: usize,
    pub tap_threshold: f32,    // ระยะทางสูงสุดสำหรับ tap
    pub tap_time_limit: f32,   // เวลาสูงสุดสำหรับ tap
    pub touch_timers: HashMap<u32, f32>,
}

impl TouchInput {
    /// สร้าง touch input ใหม่
    pub fn new(max_touches: usize) -> Self {
        Self {
            touches: HashMap::new(),
            previous_touches: HashMap::new(),
            max_touches,
            tap_threshold: 10.0,
            tap_time_limit: 0.3,
            touch_timers: HashMap::new(),
        }
    }
    
    /// อัปเดต touch state
    pub fn update(&mut self, delta_time: f32) {
        self.previous_touches = self.touches.clone();
        
        // อัปเดต touch timers
        for (id, timer) in &mut self.touch_timers {
            *timer += delta_time;
        }
        
        // ลบ touches ที่หมดเวลา
        let expired_touches: Vec<u32> = self.touch_timers
            .iter()
            .filter(|&(_, timer)| *timer > self.tap_time_limit * 2.0)
            .map(|(&id, _)| id)
            .collect();
            
        for id in expired_touches {
            self.touch_timers.remove(&id);
        }
    }
    
    /// เริ่ม touch
    pub fn start_touch(&mut self, id: u32, x: f32, y: f32, pressure: f32) {
        if self.touches.len() < self.max_touches {
            let touch = TouchPoint {
                id,
                x,
                y,
                pressure,
                is_active: true,
            };
            self.touches.insert(id, touch);
            self.touch_timers.insert(id, 0.0);
            println!("👆 Touch {} started at ({:.1}, {:.1})", id, x, y);
        }
    }
    
    /// อัปเดต touch
    pub fn update_touch(&mut self, id: u32, x: f32, y: f32, pressure: f32) {
        if let Some(touch) = self.touches.get_mut(&id) {
            touch.x = x;
            touch.y = y;
            touch.pressure = pressure;
        }
    }
    
    /// จบ touch
    pub fn end_touch(&mut self, id: u32) {
        if let Some(touch) = self.touches.remove(&id) {
            // ตรวจสอบว่าเป็น tap หรือไม่
            if let Some(&timer) = self.touch_timers.get(&id) {
                if let Some(previous_touch) = self.previous_touches.get(&id) {
                    let distance = touch.distance_to(previous_touch);
                    if distance <= self.tap_threshold && timer <= self.tap_time_limit {
                        println!("👆 Tap detected at ({:.1}, {:.1})", touch.x, touch.y);
                    }
                }
            }
            println!("👆 Touch {} ended at ({:.1}, {:.1})", id, touch.x, touch.y);
        }
    }
    
    /// ดึง touch ที่ active
    pub fn get_active_touches(&self) -> Vec<&TouchPoint> {
        self.touches.values().filter(|t| t.is_active).collect()
    }
    
    /// ตรวจสอบ pinch gesture
    pub fn detect_pinch(&self) -> Option<(f32, f32)> { // (distance, center)
        let active_touches: Vec<&TouchPoint> = self.get_active_touches();
        
        if active_touches.len() == 2 {
            let touch1 = active_touches[0];
            let touch2 = active_touches[1];
            
            let distance = touch1.distance_to(touch2);
            let center_x = (touch1.x + touch2.x) / 2.0;
            let center_y = (touch1.y + touch2.y) / 2.0;
            
            return Some((distance, center_x));
        }
        
        None
    }
    
    /// ตรวจสอบ swipe gesture
    pub fn detect_swipe(&self, id: u32) -> Option<(f32, f32)> { // (dx, dy)
        if let (Some(current), Some(previous)) = (self.touches.get(&id), self.previous_touches.get(&id)) {
            let dx = current.x - previous.x;
            let dy = current.y - previous.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 50.0 { // threshold สำหรับ swipe
                return Some((dx, dy));
            }
        }
        
        None
    }
}

/// 🎮 Input Action - การกระทำที่ผูกกับ input
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Defend,
    Interact,
    Menu,
    Pause,
    Custom(String),
}

/// 🎯 Input Binding - การผูก input กับ action
#[derive(Debug, Clone)]
pub enum InputBinding {
    Key(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButton),
    GamepadAxis { axis: GamepadAxis, threshold: f32, positive: bool },
    Combination(Vec<InputBinding>),
}

/// 🎮 Input Manager - ตัวจัดการ input หลัก
#[derive(Debug)]
pub struct InputManager {
    pub keyboard: KeyboardInput,
    pub mouse: MouseInput,
    pub gamepads: HashMap<u32, GamepadInput>,
    pub touch: TouchInput,
    pub bindings: HashMap<InputAction, Vec<InputBinding>>,
    pub action_states: HashMap<InputAction, InputState>,
    pub is_enabled: bool,
}

impl InputManager {
    /// สร้าง input manager ใหม่
    pub fn new() -> Self {
        let mut manager = Self {
            keyboard: KeyboardInput::new(),
            mouse: MouseInput::new(),
            gamepads: HashMap::new(),
            touch: TouchInput::new(10),
            bindings: HashMap::new(),
            action_states: HashMap::new(),
            is_enabled: true,
        };
        
        // ตั้งค่า default bindings
        manager.setup_default_bindings();
        
        manager
    }
    
    /// ตั้งค่า default bindings
    fn setup_default_bindings(&mut self) {
        // Movement
        self.bind_action(InputAction::MoveUp, vec![
            InputBinding::Key(KeyCode::W),
            InputBinding::Key(KeyCode::Up),
            InputBinding::GamepadAxis { axis: GamepadAxis::LeftStickY, threshold: 0.5, positive: true },
        ]);
        
        self.bind_action(InputAction::MoveDown, vec![
            InputBinding::Key(KeyCode::S),
            InputBinding::Key(KeyCode::Down),
            InputBinding::GamepadAxis { axis: GamepadAxis::LeftStickY, threshold: 0.5, positive: false },
        ]);
        
        self.bind_action(InputAction::MoveLeft, vec![
            InputBinding::Key(KeyCode::A),
            InputBinding::Key(KeyCode::Left),
            InputBinding::GamepadAxis { axis: GamepadAxis::LeftStickX, threshold: 0.5, positive: false },
        ]);
        
        self.bind_action(InputAction::MoveRight, vec![
            InputBinding::Key(KeyCode::D),
            InputBinding::Key(KeyCode::Right),
            InputBinding::GamepadAxis { axis: GamepadAxis::LeftStickX, threshold: 0.5, positive: true },
        ]);
        
        // Actions
        self.bind_action(InputAction::Jump, vec![
            InputBinding::Key(KeyCode::Space),
            InputBinding::GamepadButton(GamepadButton::A),
        ]);
        
        self.bind_action(InputAction::Attack, vec![
            InputBinding::MouseButton(MouseButton::Left),
            InputBinding::GamepadButton(GamepadButton::X),
        ]);
        
        self.bind_action(InputAction::Defend, vec![
            InputBinding::MouseButton(MouseButton::Right),
            InputBinding::GamepadButton(GamepadButton::Y),
        ]);
        
        self.bind_action(InputAction::Interact, vec![
            InputBinding::Key(KeyCode::E),
            InputBinding::GamepadButton(GamepadButton::B),
        ]);
        
        self.bind_action(InputAction::Menu, vec![
            InputBinding::Key(KeyCode::Escape),
            InputBinding::GamepadButton(GamepadButton::Start),
        ]);
        
        self.bind_action(InputAction::Pause, vec![
            InputBinding::Key(KeyCode::P),
            InputBinding::GamepadButton(GamepadButton::Select),
        ]);
    }
    
    /// ผูก action กับ input bindings
    pub fn bind_action(&mut self, action: InputAction, bindings: Vec<InputBinding>) {
        self.bindings.insert(action, bindings);
    }
    
    /// เพิ่ม gamepad
    pub fn add_gamepad(&mut self, id: u32) {
        let mut gamepad = GamepadInput::new(id);
        gamepad.connect();
        self.gamepads.insert(id, gamepad);
    }
    
    /// ลบ gamepad
    pub fn remove_gamepad(&mut self, id: u32) {
        if let Some(mut gamepad) = self.gamepads.remove(&id) {
            gamepad.disconnect();
        }
    }
    
    /// อัปเดต input manager
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_enabled {
            return;
        }
        
        // อัปเดต input devices
        self.keyboard.update(delta_time);
        self.mouse.update();
        self.touch.update(delta_time);
        
        for gamepad in self.gamepads.values_mut() {
            gamepad.update();
        }
        
        // อัปเดต action states
        self.update_action_states();
    }
    
    /// อัปเดต action states
    fn update_action_states(&mut self) {
        for (action, bindings) in &self.bindings {
            let mut is_active = false;
            
            for binding in bindings {
                if self.is_binding_active(binding) {
                    is_active = true;
                    break;
                }
            }
            
            // อัปเดต state
            let previous_state = self.action_states.get(action).copied().unwrap_or(InputState::Released);
            
            let new_state = match (previous_state, is_active) {
                (InputState::Released, true) => InputState::Pressed,
                (InputState::Pressed, true) => InputState::Held,
                (InputState::Held, true) => InputState::Held,
                (InputState::JustReleased, true) => InputState::Pressed,
                (_, false) => {
                    if matches!(previous_state, InputState::Pressed | InputState::Held) {
                        InputState::JustReleased
                    } else {
                        InputState::Released
                    }
                }
            };
            
            self.action_states.insert(action.clone(), new_state);
        }
    }
    
    /// ตรวจสอบว่า binding active หรือไม่
    fn is_binding_active(&self, binding: &InputBinding) -> bool {
        match binding {
            InputBinding::Key(key) => self.keyboard.is_key_held(*key),
            InputBinding::MouseButton(button) => self.mouse.is_button_held(*button),
            InputBinding::GamepadButton(button) => {
                self.gamepads.values().any(|g| g.is_button_held(*button))
            }
            InputBinding::GamepadAxis { axis, threshold, positive } => {
                self.gamepads.values().any(|g| {
                    let value = g.get_axis(*axis);
                    if *positive {
                        value >= *threshold
                    } else {
                        value <= -*threshold
                    }
                })
            }
            InputBinding::Combination(bindings) => {
                bindings.iter().all(|b| self.is_binding_active(b))
            }
        }
    }
    
    /// ตรวจสอบว่า action ถูกกดอยู่หรือไม่
    pub fn is_action_held(&self, action: &InputAction) -> bool {
        matches!(self.action_states.get(action), Some(InputState::Held) | Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่า action เพิ่งถูกกดหรือไม่
    pub fn is_action_just_pressed(&self, action: &InputAction) -> bool {
        matches!(self.action_states.get(action), Some(InputState::Pressed))
    }
    
    /// ตรวจสอบว่า action เพิ่งถูกปล่อยหรือไม่
    pub fn is_action_just_released(&self, action: &InputAction) -> bool {
        matches!(self.action_states.get(action), Some(InputState::JustReleased))
    }
    
    /// ดึงค่า movement vector
    pub fn get_movement_vector(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        
        if self.is_action_held(&InputAction::MoveLeft) {
            x -= 1.0;
        }
        if self.is_action_held(&InputAction::MoveRight) {
            x += 1.0;
        }
        if self.is_action_held(&InputAction::MoveDown) {
            y -= 1.0;
        }
        if self.is_action_held(&InputAction::MoveUp) {
            y += 1.0;
        }
        
        // ใช้ gamepad stick ถ้ามี
        for gamepad in self.gamepads.values() {
            if gamepad.is_connected {
                let (stick_x, stick_y) = gamepad.get_left_stick();
                if stick_x.abs() > 0.1 || stick_y.abs() > 0.1 {
                    x = stick_x;
                    y = stick_y;
                    break;
                }
            }
        }
        
        (x, y)
    }
    
    /// ดึงสถิติ input
    pub fn get_stats(&self) -> InputStats {
        let connected_gamepads = self.gamepads.values().filter(|g| g.is_connected).count();
        let active_touches = self.touch.get_active_touches().len();
        let pressed_keys = self.keyboard.get_pressed_keys().len();
        
        InputStats {
            connected_gamepads,
            active_touches,
            pressed_keys,
            total_bindings: self.bindings.len(),
            mouse_position: self.mouse.position,
        }
    }
}

/// 📊 สถิติ input
#[derive(Debug, Clone)]
pub struct InputStats {
    pub connected_gamepads: usize,
    pub active_touches: usize,
    pub pressed_keys: usize,
    pub total_bindings: usize,
    pub mouse_position: (f32, f32),
}

/// 🎮 สาธิตการใช้งาน Input Handling
pub fn demonstrate_input_handling() {
    println!("🎮 === Input Handling Demo ===");
    
    // สร้าง input manager
    let mut input = InputManager::new();
    println!("🕹️ Created input manager with default bindings");
    
    // เพิ่ม gamepad
    input.add_gamepad(0);
    
    // จำลองการป้อน input
    println!("\n⌨️ Simulating keyboard input:");
    input.keyboard.press_key(KeyCode::W);
    input.keyboard.press_key(KeyCode::Space);
    
    println!("\n🖱️ Simulating mouse input:");
    input.mouse.set_position(100.0, 200.0);
    input.mouse.press_button(MouseButton::Left);
    input.mouse.set_wheel_delta(1.5);
    
    println!("\n🎮 Simulating gamepad input:");
    if let Some(gamepad) = input.gamepads.get_mut(&0) {
        gamepad.press_button(GamepadButton::A);
        gamepad.set_axis(GamepadAxis::LeftStickX, 0.8);
        gamepad.set_axis(GamepadAxis::LeftStickY, -0.6);
        gamepad.set_vibration(0.5, 0.3);
    }
    
    println!("\n📱 Simulating touch input:");
    input.touch.start_touch(1, 150.0, 300.0, 1.0);
    input.touch.start_touch(2, 250.0, 400.0, 0.8);
    
    // อัปเดตและทดสอบ
    println!("\n🔄 Updating input system:");
    for frame in 0..5 {
        println!("\n--- Frame {} ---", frame + 1);
        
        input.update(1.0 / 60.0);
        
        // ทดสอบ actions
        if input.is_action_just_pressed(&InputAction::Jump) {
            println!("🦘 Jump action triggered!");
        }
        
        if input.is_action_held(&InputAction::MoveUp) {
            println!("⬆️ Moving up");
        }
        
        if input.is_action_just_pressed(&InputAction::Attack) {
            println!("⚔️ Attack action triggered!");
        }
        
        // ทดสอบ movement
        let (move_x, move_y) = input.get_movement_vector();
        if move_x != 0.0 || move_y != 0.0 {
            println!("🏃 Movement vector: ({:.2}, {:.2})", move_x, move_y);
        }
        
        // ทดสอบ mouse
        let (delta_x, delta_y) = input.mouse.get_delta();
        if delta_x != 0.0 || delta_y != 0.0 {
            println!("🖱️ Mouse delta: ({:.1}, {:.1})", delta_x, delta_y);
        }
        
        // ทดสอบ touch gestures
        if let Some((distance, center)) = input.touch.detect_pinch() {
            println!("🤏 Pinch detected: distance={:.1}, center={:.1}", distance, center);
        }
        
        // แสดงสถิติ
        let stats = input.get_stats();
        println!("📊 Stats: {} gamepads, {} touches, {} keys, mouse at ({:.1}, {:.1})",
                stats.connected_gamepads, stats.active_touches, 
                stats.pressed_keys, stats.mouse_position.0, stats.mouse_position.1);
        
        // จำลองการเปลี่ยนแปลง input
        match frame {
            1 => {
                input.keyboard.release_key(KeyCode::W);
                input.keyboard.press_key(KeyCode::D);
            }
            2 => {
                input.mouse.release_button(MouseButton::Left);
                input.touch.update_touch(1, 160.0, 310.0, 0.9);
            }
            3 => {
                input.keyboard.release_key(KeyCode::Space);
                if let Some(gamepad) = input.gamepads.get_mut(&0) {
                    gamepad.release_button(GamepadButton::A);
                }
            }
            4 => {
                input.touch.end_touch(1);
                input.touch.end_touch(2);
            }
            _ => {}
        }
    }
    
    // ทดสอบ custom bindings
    println!("\n🎯 Testing custom bindings:");
    test_custom_bindings(&mut input);
    
    // แสดง best practices
    println!("\n💡 Input Handling Best Practices:");
    show_input_best_practices();
}

/// 🎯 ทดสอบ custom bindings
fn test_custom_bindings(input: &mut InputManager) {
    // สร้าง custom action
    let custom_action = InputAction::Custom("SuperJump".to_string());
    
    // ผูกกับ combination
    input.bind_action(custom_action.clone(), vec![
        InputBinding::Combination(vec![
            InputBinding::Key(KeyCode::Shift),
            InputBinding::Key(KeyCode::Space),
        ])
    ]);
    
    println!("🎯 Created custom action: SuperJump (Shift + Space)");
    
    // ทดสอบ
    input.keyboard.press_key(KeyCode::Shift);
    input.keyboard.press_key(KeyCode::Space);
    input.update(1.0 / 60.0);
    
    if input.is_action_just_pressed(&custom_action) {
        println!("🚀 SuperJump activated!");
    }
}

/// 💡 Input Best Practices
fn show_input_best_practices() {
    let practices = vec![
        "🎯 Provide customizable key bindings",
        "🎮 Support multiple input devices simultaneously",
        "📱 Implement proper touch gesture recognition",
        "⚡ Use input buffering for responsive controls",
        "🔄 Implement input prediction for network games",
        "♿ Provide accessibility options",
        "🎚️ Allow sensitivity adjustments",
        "🔇 Implement input deadzone configuration",
        "📊 Monitor input latency",
        "🎪 Use input actions instead of raw input",
        "🔒 Implement input validation and sanitization",
        "🎭 Provide visual feedback for input",
        "⏱️ Handle input timing correctly",
        "🌍 Support different input layouts",
        "🎯 Implement context-sensitive input",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Input Libraries:");
    println!("   • winit - Cross-platform window and input");
    println!("   • gilrs - Game controller input");
    println!("   • device_query - Simple input polling");
    println!("   • inputbot - Input simulation and monitoring");
    println!("   • enigo - Cross-platform input simulation");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyboard_input() {
        let mut keyboard = KeyboardInput::new();
        
        keyboard.press_key(KeyCode::A);
        keyboard.update(0.016);
        
        assert!(keyboard.is_key_just_pressed(KeyCode::A));
        assert!(keyboard.is_key_held(KeyCode::A));
        
        keyboard.update(0.016);
        assert!(!keyboard.is_key_just_pressed(KeyCode::A));
        assert!(keyboard.is_key_held(KeyCode::A));
        
        keyboard.release_key(KeyCode::A);
        keyboard.update(0.016);
        assert!(keyboard.is_key_just_released(KeyCode::A));
        assert!(!keyboard.is_key_held(KeyCode::A));
    }
    
    #[test]
    fn test_mouse_input() {
        let mut mouse = MouseInput::new();
        
        mouse.set_position(100.0, 200.0);
        mouse.press_button(MouseButton::Left);
        mouse.update();
        
        assert!(mouse.is_button_just_pressed(MouseButton::Left));
        assert_eq!(mouse.position, (100.0, 200.0));
    }
    
    #[test]
    fn test_gamepad_input() {
        let mut gamepad = GamepadInput::new(0);
        gamepad.connect();
        
        gamepad.press_button(GamepadButton::A);
        gamepad.set_axis(GamepadAxis::LeftStickX, 0.8);
        gamepad.update();
        
        assert!(gamepad.is_button_just_pressed(GamepadButton::A));
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftStickX), 0.8);
    }
    
    #[test]
    fn test_touch_input() {
        let mut touch = TouchInput::new(5);
        
        touch.start_touch(1, 100.0, 200.0, 1.0);
        assert_eq!(touch.touches.len(), 1);
        
        touch.end_touch(1);
        assert_eq!(touch.touches.len(), 0);
    }
    
    #[test]
    fn test_input_manager() {
        let mut input = InputManager::new();
        
        input.keyboard.press_key(KeyCode::Space);
        input.update(0.016);
        
        assert!(input.is_action_just_pressed(&InputAction::Jump));
    }
    
    #[test]
    fn test_movement_vector() {
        let mut input = InputManager::new();
        
        input.keyboard.press_key(KeyCode::W);
        input.keyboard.press_key(KeyCode::D);
        input.update(0.016);
        
        let (x, y) = input.get_movement_vector();
        assert_eq!(x, 1.0);
        assert_eq!(y, 1.0);
    }
}

// 🎮 "Input ที่ดีคือหัวใจของเกมที่ดี
//     ผู้เล่นไม่ควรต้องต่อสู้กับ controls
//     แต่ควรต้องต่อสู้กับเกมเท่านั้น!
//     ทำให้ input รู้สึกเป็นธรรมชาติ ผู้เล่นจะลืมว่ากำลังใช้ controller! 🕹️✨"