//! 🎵 Audio System - ระบบเสียงสำหรับเกม
//! 
//! โมดูลนี้สาธิตการสร้างระบบเสียงสำหรับเกม
//! รวมถึง sound effects, music, 3D audio, และ audio mixing
//! 
//! 🎧 "เสียงดีทำให้เกมดี เสียงแย่ทำให้เกมแย่!"

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

/// 🎵 ประเภทของไฟล์เสียง
#[derive(Debug, Clone, PartialEq)]
pub enum AudioFormat {
    WAV,
    MP3,
    OGG,
    FLAC,
    AAC,
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioFormat::WAV => write!(f, "WAV (Uncompressed)"),
            AudioFormat::MP3 => write!(f, "MP3 (Compressed)"),
            AudioFormat::OGG => write!(f, "OGG (Open Source)"),
            AudioFormat::FLAC => write!(f, "FLAC (Lossless)"),
            AudioFormat::AAC => write!(f, "AAC (Advanced)"),
        }
    }
}

/// 🎯 ตำแหน่งในพื้นที่ 3D สำหรับเสียง
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AudioPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub const ZERO: AudioPosition = AudioPosition { x: 0.0, y: 0.0, z: 0.0 };
    
    /// คำนวณระยะห่างจากตำแหน่งอื่น
    pub fn distance_to(&self, other: &AudioPosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// คำนวณทิศทางไปยังตำแหน่งอื่น
    pub fn direction_to(&self, other: &AudioPosition) -> AudioPosition {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();
        
        if length > 0.0 {
            AudioPosition::new(dx / length, dy / length, dz / length)
        } else {
            AudioPosition::ZERO
        }
    }
}

/// 🎵 Audio Clip - ไฟล์เสียง
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub id: u32,
    pub name: String,
    pub file_path: String,
    pub format: AudioFormat,
    pub duration: f32,      // วินาที
    pub sample_rate: u32,   // Hz (เช่น 44100)
    pub channels: u8,       // 1 = mono, 2 = stereo
    pub bit_depth: u8,      // bits per sample
    pub file_size: usize,   // bytes
    pub is_loaded: bool,
    pub data: Vec<f32>,     // audio samples (-1.0 to 1.0)
}

impl AudioClip {
    /// สร้าง audio clip ใหม่
    pub fn new(id: u32, name: &str, file_path: &str, format: AudioFormat) -> Self {
        Self {
            id,
            name: name.to_string(),
            file_path: file_path.to_string(),
            format,
            duration: 0.0,
            sample_rate: 44100,
            channels: 2,
            bit_depth: 16,
            file_size: 0,
            is_loaded: false,
            data: Vec::new(),
        }
    }
    
    /// โหลดไฟล์เสียง (จำลอง)
    pub fn load(&mut self) -> Result<(), String> {
        if self.is_loaded {
            return Ok(());
        }
        
        println!("🎵 Loading audio: {} ({})", self.name, self.format);
        
        // จำลองการโหลดไฟล์
        match self.format {
            AudioFormat::WAV => {
                self.duration = 2.5;
                self.sample_rate = 44100;
                self.channels = 2;
                self.bit_depth = 16;
                self.file_size = (self.duration * self.sample_rate as f32 * self.channels as f32 * (self.bit_depth as f32 / 8.0)) as usize;
            }
            AudioFormat::MP3 => {
                self.duration = 3.0;
                self.sample_rate = 44100;
                self.channels = 2;
                self.bit_depth = 16;
                self.file_size = (self.duration * self.sample_rate as f32 * 0.1) as usize; // compressed
            }
            AudioFormat::OGG => {
                self.duration = 2.8;
                self.sample_rate = 48000;
                self.channels = 2;
                self.bit_depth = 16;
                self.file_size = (self.duration * self.sample_rate as f32 * 0.12) as usize; // compressed
            }
            _ => {
                return Err(format!("Unsupported format: {:?}", self.format));
            }
        }
        
        // สร้างข้อมูลเสียงจำลอง (sine wave)
        let sample_count = (self.duration * self.sample_rate as f32) as usize * self.channels as usize;
        self.data = Vec::with_capacity(sample_count);
        
        for i in 0..sample_count {
            let time = i as f32 / self.sample_rate as f32;
            let frequency = 440.0; // A4 note
            let sample = (time * frequency * 2.0 * std::f32::consts::PI).sin() * 0.3;
            self.data.push(sample);
        }
        
        self.is_loaded = true;
        println!("✅ Loaded: {:.1}s, {}Hz, {} channels, {} samples", 
                self.duration, self.sample_rate, self.channels, self.data.len());
        
        Ok(())
    }
    
    /// ยกเลิกการโหลด
    pub fn unload(&mut self) {
        if self.is_loaded {
            self.data.clear();
            self.is_loaded = false;
            println!("🗑️ Unloaded audio: {}", self.name);
        }
    }
    
    /// ดึงข้อมูลเสียงในช่วงเวลา
    pub fn get_samples(&self, start_time: f32, duration: f32) -> Vec<f32> {
        if !self.is_loaded {
            return Vec::new();
        }
        
        let start_sample = (start_time * self.sample_rate as f32) as usize * self.channels as usize;
        let sample_count = (duration * self.sample_rate as f32) as usize * self.channels as usize;
        let end_sample = (start_sample + sample_count).min(self.data.len());
        
        if start_sample >= self.data.len() {
            return Vec::new();
        }
        
        self.data[start_sample..end_sample].to_vec()
    }
}

/// 🎮 Audio Source - แหล่งเสียง
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub id: u32,
    pub clip_id: u32,
    pub position: AudioPosition,
    pub velocity: AudioPosition,  // สำหรับ Doppler effect
    pub volume: f32,              // 0.0 - 1.0
    pub pitch: f32,               // 1.0 = normal, 2.0 = double speed
    pub pan: f32,                 // -1.0 = left, 0.0 = center, 1.0 = right
    pub is_playing: bool,
    pub is_looping: bool,
    pub is_paused: bool,
    pub playback_position: f32,   // วินาที
    pub fade_in_duration: f32,
    pub fade_out_duration: f32,
    pub current_fade: f32,
    pub max_distance: f32,        // ระยะไกลสุดที่ยังได้ยิน
    pub rolloff_factor: f32,      // ความเร็วในการลดเสียงตามระยะ
}

impl AudioSource {
    /// สร้าง audio source ใหม่
    pub fn new(id: u32, clip_id: u32) -> Self {
        Self {
            id,
            clip_id,
            position: AudioPosition::ZERO,
            velocity: AudioPosition::ZERO,
            volume: 1.0,
            pitch: 1.0,
            pan: 0.0,
            is_playing: false,
            is_looping: false,
            is_paused: false,
            playback_position: 0.0,
            fade_in_duration: 0.0,
            fade_out_duration: 0.0,
            current_fade: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
        }
    }
    
    /// เล่นเสียง
    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
        self.playback_position = 0.0;
        println!("▶️ Playing audio source {} (clip {})", self.id, self.clip_id);
    }
    
    /// หยุดเสียง
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        self.playback_position = 0.0;
        println!("⏹️ Stopped audio source {}", self.id);
    }
    
    /// พักเสียง
    pub fn pause(&mut self) {
        if self.is_playing {
            self.is_paused = true;
            println!("⏸️ Paused audio source {}", self.id);
        }
    }
    
    /// เล่นต่อ
    pub fn resume(&mut self) {
        if self.is_paused {
            self.is_paused = false;
            println!("▶️ Resumed audio source {}", self.id);
        }
    }
    
    /// ตั้งค่า fade in
    pub fn fade_in(&mut self, duration: f32) {
        self.fade_in_duration = duration;
        self.current_fade = 0.0;
    }
    
    /// ตั้งค่า fade out
    pub fn fade_out(&mut self, duration: f32) {
        self.fade_out_duration = duration;
    }
    
    /// อัปเดต audio source
    pub fn update(&mut self, delta_time: f32, clip_duration: f32) {
        if !self.is_playing || self.is_paused {
            return;
        }
        
        // อัปเดตตำแหน่งการเล่น
        self.playback_position += delta_time * self.pitch;
        
        // ตรวจสอบการจบเสียง
        if self.playback_position >= clip_duration {
            if self.is_looping {
                self.playback_position = 0.0;
            } else {
                self.stop();
                return;
            }
        }
        
        // อัปเดต fade
        if self.fade_in_duration > 0.0 && self.current_fade < 1.0 {
            self.current_fade += delta_time / self.fade_in_duration;
            self.current_fade = self.current_fade.min(1.0);
        }
        
        if self.fade_out_duration > 0.0 {
            let remaining_time = clip_duration - self.playback_position;
            if remaining_time <= self.fade_out_duration {
                self.current_fade = remaining_time / self.fade_out_duration;
            }
        }
    }
    
    /// คำนวณระดับเสียงตามระยะห่าง
    pub fn calculate_distance_volume(&self, listener_position: &AudioPosition) -> f32 {
        let distance = self.position.distance_to(listener_position);
        
        if distance >= self.max_distance {
            return 0.0;
        }
        
        // Linear rolloff
        let volume_factor = 1.0 - (distance / self.max_distance).powf(self.rolloff_factor);
        volume_factor.max(0.0)
    }
    
    /// คำนวณ pan ตามตำแหน่ง
    pub fn calculate_spatial_pan(&self, listener_position: &AudioPosition, listener_forward: &AudioPosition) -> f32 {
        let to_source = self.position.direction_to(listener_position);
        
        // คำนวณมุมระหว่าง listener forward และทิศทางไปยัง source
        let dot_product = listener_forward.x * to_source.x + listener_forward.z * to_source.z;
        let cross_product = listener_forward.x * to_source.z - listener_forward.z * to_source.x;
        
        // แปลงเป็น pan value (-1.0 ถึง 1.0)
        cross_product.clamp(-1.0, 1.0)
    }
}

/// 🎧 Audio Listener - ผู้ฟัง
#[derive(Debug, Clone)]
pub struct AudioListener {
    pub position: AudioPosition,
    pub velocity: AudioPosition,
    pub forward: AudioPosition,   // ทิศทางที่หันหน้าไป
    pub up: AudioPosition,        // ทิศทางด้านบน
    pub master_volume: f32,
}

impl AudioListener {
    /// สร้าง audio listener ใหม่
    pub fn new() -> Self {
        Self {
            position: AudioPosition::ZERO,
            velocity: AudioPosition::ZERO,
            forward: AudioPosition::new(0.0, 0.0, -1.0), // หันหน้าไป -Z
            up: AudioPosition::new(0.0, 1.0, 0.0),       // ด้านบนคือ +Y
            master_volume: 1.0,
        }
    }
    
    /// ตั้งค่าตำแหน่งและการหมุน
    pub fn set_transform(&mut self, position: AudioPosition, forward: AudioPosition, up: AudioPosition) {
        self.position = position;
        self.forward = forward;
        self.up = up;
    }
}

/// 🎛️ Audio Mixer - ตัวผสมเสียง
#[derive(Debug)]
pub struct AudioMixer {
    pub channels: HashMap<String, MixerChannel>,
    pub master_volume: f32,
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub output_buffer: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct MixerChannel {
    pub name: String,
    pub volume: f32,
    pub mute: bool,
    pub solo: bool,
    pub effects: Vec<AudioEffect>,
}

#[derive(Debug, Clone)]
pub enum AudioEffect {
    Reverb { room_size: f32, damping: f32, wet_level: f32 },
    Echo { delay: f32, feedback: f32, wet_level: f32 },
    LowPass { cutoff_frequency: f32, resonance: f32 },
    HighPass { cutoff_frequency: f32, resonance: f32 },
    Distortion { drive: f32, tone: f32 },
    Chorus { rate: f32, depth: f32, feedback: f32 },
}

impl AudioMixer {
    /// สร้าง audio mixer ใหม่
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        let mut mixer = Self {
            channels: HashMap::new(),
            master_volume: 1.0,
            sample_rate,
            buffer_size,
            output_buffer: vec![0.0; buffer_size * 2], // stereo
        };
        
        // สร้าง channel พื้นฐาน
        mixer.add_channel("Master", 1.0);
        mixer.add_channel("Music", 0.8);
        mixer.add_channel("SFX", 1.0);
        mixer.add_channel("Voice", 0.9);
        mixer.add_channel("Ambient", 0.6);
        
        mixer
    }
    
    /// เพิ่ม channel
    pub fn add_channel(&mut self, name: &str, volume: f32) {
        let channel = MixerChannel {
            name: name.to_string(),
            volume,
            mute: false,
            solo: false,
            effects: Vec::new(),
        };
        self.channels.insert(name.to_string(), channel);
        println!("🎛️ Added mixer channel: {} (volume: {:.2})", name, volume);
    }
    
    /// ตั้งค่าระดับเสียงของ channel
    pub fn set_channel_volume(&mut self, channel_name: &str, volume: f32) {
        if let Some(channel) = self.channels.get_mut(channel_name) {
            channel.volume = volume.clamp(0.0, 2.0);
            println!("🔊 Set {} volume to {:.2}", channel_name, channel.volume);
        }
    }
    
    /// mute/unmute channel
    pub fn set_channel_mute(&mut self, channel_name: &str, mute: bool) {
        if let Some(channel) = self.channels.get_mut(channel_name) {
            channel.mute = mute;
            println!("🔇 {} channel: {}", if mute { "Muted" } else { "Unmuted" }, channel_name);
        }
    }
    
    /// เพิ่ม effect ให้ channel
    pub fn add_effect(&mut self, channel_name: &str, effect: AudioEffect) {
        if let Some(channel) = self.channels.get_mut(channel_name) {
            channel.effects.push(effect.clone());
            println!("🎚️ Added effect to {}: {:?}", channel_name, effect);
        }
    }
    
    /// ผสมเสียงจาก sources
    pub fn mix(&mut self, sources: &[&AudioSource], clips: &HashMap<u32, &AudioClip>) {
        // เคลียร์ buffer
        self.output_buffer.fill(0.0);
        
        for source in sources {
            if !source.is_playing || source.is_paused {
                continue;
            }
            
            if let Some(clip) = clips.get(&source.clip_id) {
                if !clip.is_loaded {
                    continue;
                }
                
                // ดึงข้อมูลเสียง
                let sample_duration = self.buffer_size as f32 / self.sample_rate as f32;
                let samples = clip.get_samples(source.playback_position, sample_duration);
                
                // ผสมเสียงเข้า buffer
                for (i, &sample) in samples.iter().enumerate() {
                    if i >= self.output_buffer.len() {
                        break;
                    }
                    
                    let volume = source.volume * source.current_fade * self.master_volume;
                    self.output_buffer[i] += sample * volume;
                }
            }
        }
        
        // จำกัดระดับเสียงไม่ให้เกิน clipping
        for sample in &mut self.output_buffer {
            *sample = sample.clamp(-1.0, 1.0);
        }
    }
    
    /// ดึงสถิติ mixer
    pub fn get_stats(&self) -> MixerStats {
        let active_channels = self.channels.values().filter(|c| !c.mute).count();
        let total_effects = self.channels.values().map(|c| c.effects.len()).sum();
        
        MixerStats {
            total_channels: self.channels.len(),
            active_channels,
            total_effects,
            sample_rate: self.sample_rate,
            buffer_size: self.buffer_size,
            master_volume: self.master_volume,
        }
    }
}

/// 📊 สถิติ mixer
#[derive(Debug, Clone)]
pub struct MixerStats {
    pub total_channels: usize,
    pub active_channels: usize,
    pub total_effects: usize,
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub master_volume: f32,
}

/// 🎵 Audio System - ระบบเสียงหลัก
#[derive(Debug)]
pub struct AudioSystem {
    pub clips: HashMap<u32, AudioClip>,
    pub sources: HashMap<u32, AudioSource>,
    pub listener: AudioListener,
    pub mixer: AudioMixer,
    pub next_clip_id: u32,
    pub next_source_id: u32,
    pub is_enabled: bool,
}

impl AudioSystem {
    /// สร้าง audio system ใหม่
    pub fn new() -> Self {
        Self {
            clips: HashMap::new(),
            sources: HashMap::new(),
            listener: AudioListener::new(),
            mixer: AudioMixer::new(44100, 1024),
            next_clip_id: 1,
            next_source_id: 1,
            is_enabled: true,
        }
    }
    
    /// โหลด audio clip
    pub fn load_clip(&mut self, name: &str, file_path: &str, format: AudioFormat) -> Result<u32, String> {
        let id = self.next_clip_id;
        self.next_clip_id += 1;
        
        let mut clip = AudioClip::new(id, name, file_path, format);
        clip.load()?;
        
        self.clips.insert(id, clip);
        println!("📁 Loaded audio clip: {} (ID: {})", name, id);
        
        Ok(id)
    }
    
    /// สร้าง audio source
    pub fn create_source(&mut self, clip_id: u32) -> Result<u32, String> {
        if !self.clips.contains_key(&clip_id) {
            return Err(format!("Clip {} not found", clip_id));
        }
        
        let id = self.next_source_id;
        self.next_source_id += 1;
        
        let source = AudioSource::new(id, clip_id);
        self.sources.insert(id, source);
        
        println!("🎮 Created audio source: {} for clip {}", id, clip_id);
        Ok(id)
    }
    
    /// เล่นเสียง
    pub fn play(&mut self, source_id: u32) -> Result<(), String> {
        if let Some(source) = self.sources.get_mut(&source_id) {
            source.play();
            Ok(())
        } else {
            Err(format!("Source {} not found", source_id))
        }
    }
    
    /// เล่นเสียงแบบ one-shot
    pub fn play_one_shot(&mut self, clip_id: u32, volume: f32, position: Option<AudioPosition>) -> Result<u32, String> {
        let source_id = self.create_source(clip_id)?;
        
        if let Some(source) = self.sources.get_mut(&source_id) {
            source.volume = volume;
            if let Some(pos) = position {
                source.position = pos;
            }
            source.play();
        }
        
        Ok(source_id)
    }
    
    /// หยุดเสียงทั้งหมด
    pub fn stop_all(&mut self) {
        for source in self.sources.values_mut() {
            source.stop();
        }
        println!("⏹️ Stopped all audio sources");
    }
    
    /// อัปเดตระบบเสียง
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_enabled {
            return;
        }
        
        // อัปเดต audio sources
        let mut finished_sources = Vec::new();
        
        for (id, source) in &mut self.sources {
            if let Some(clip) = self.clips.get(&source.clip_id) {
                source.update(delta_time, clip.duration);
                
                // ลบเฉพาะ one-shot sources ที่เล่นจบแล้ว
                // (ไม่ลบ sources ปกติเพื่อให้สามารถเล่นซ้ำได้)
                // if !source.is_playing && !source.is_looping {
                //     finished_sources.push(*id);
                // }
            }
        }
        
        // ลบ sources ที่เล่นจบแล้ว
        for id in finished_sources {
            self.sources.remove(&id);
        }
        
        // อัปเดต 3D audio
        self.update_3d_audio();
        
        // ผสมเสียง
        let source_refs: Vec<&AudioSource> = self.sources.values().collect();
        let clip_refs: HashMap<u32, &AudioClip> = self.clips.iter().map(|(k, v)| (*k, v)).collect();
        self.mixer.mix(&source_refs, &clip_refs);
    }
    
    /// อัปเดต 3D audio
    fn update_3d_audio(&mut self) {
        for source in self.sources.values_mut() {
            // คำนวณระดับเสียงตามระยะห่าง
            let distance_volume = source.calculate_distance_volume(&self.listener.position);
            
            // คำนวณ pan ตามตำแหน่ง
            let spatial_pan = source.calculate_spatial_pan(&self.listener.position, &self.listener.forward);
            
            // อัปเดตค่า (ในการใช้งานจริงจะส่งไปยัง audio engine)
            source.volume = (source.volume * distance_volume).min(1.0);
            source.pan = spatial_pan;
        }
    }
    
    /// ดึงสถิติระบบเสียง
    pub fn get_stats(&self) -> AudioSystemStats {
        let playing_sources = self.sources.values().filter(|s| s.is_playing && !s.is_paused).count();
        let loaded_clips = self.clips.values().filter(|c| c.is_loaded).count();
        let total_memory = self.clips.values().map(|c| c.data.len() * 4).sum::<usize>(); // 4 bytes per f32
        
        AudioSystemStats {
            total_clips: self.clips.len(),
            loaded_clips,
            total_sources: self.sources.len(),
            playing_sources,
            memory_usage_bytes: total_memory,
            mixer_stats: self.mixer.get_stats(),
        }
    }
}

/// 📊 สถิติระบบเสียง
#[derive(Debug, Clone)]
pub struct AudioSystemStats {
    pub total_clips: usize,
    pub loaded_clips: usize,
    pub total_sources: usize,
    pub playing_sources: usize,
    pub memory_usage_bytes: usize,
    pub mixer_stats: MixerStats,
}

/// 🎵 สาธิตการใช้งาน Audio System
pub fn demonstrate_audio_system() {
    println!("🎵 === Audio System Demo ===");
    
    // สร้าง audio system
    let mut audio = AudioSystem::new();
    println!("🎧 Created audio system ({}Hz, {} samples buffer)", 
            audio.mixer.sample_rate, audio.mixer.buffer_size);
    
    // โหลด audio clips
    println!("\n📁 Loading audio clips:");
    let music_id = audio.load_clip("background_music", "assets/music.mp3", AudioFormat::MP3)
        .expect("Failed to load music");
    let jump_id = audio.load_clip("jump_sound", "assets/jump.wav", AudioFormat::WAV)
        .expect("Failed to load jump sound");
    let explosion_id = audio.load_clip("explosion", "assets/explosion.ogg", AudioFormat::OGG)
        .expect("Failed to load explosion");
    
    // ตั้งค่า mixer
    println!("\n🎛️ Configuring mixer:");
    audio.mixer.set_channel_volume("Music", 0.6);
    audio.mixer.set_channel_volume("SFX", 0.8);
    
    // เพิ่ม effects
    audio.mixer.add_effect("Music", AudioEffect::Reverb {
        room_size: 0.5,
        damping: 0.3,
        wet_level: 0.2,
    });
    
    audio.mixer.add_effect("SFX", AudioEffect::Echo {
        delay: 0.3,
        feedback: 0.4,
        wet_level: 0.1,
    });
    
    // สร้าง audio sources
    println!("\n🎮 Creating audio sources:");
    let music_source = audio.create_source(music_id).expect("Failed to create music source");
    let jump_source = audio.create_source(jump_id).expect("Failed to create jump source");
    
    // ตั้งค่า music source
    if let Some(source) = audio.sources.get_mut(&music_source) {
        source.is_looping = true;
        source.volume = 0.7;
        source.fade_in(2.0);
    }
    
    // เล่น background music
    audio.play(music_source).expect("Failed to play music");
    println!("🎵 Started background music with fade-in");
    
    // ตั้งค่า listener
    audio.listener.set_transform(
        AudioPosition::new(0.0, 0.0, 0.0),
        AudioPosition::new(0.0, 0.0, -1.0),
        AudioPosition::new(0.0, 1.0, 0.0)
    );
    
    // จำลองการเล่นเกม
    println!("\n🎬 Simulating gameplay:");
    for frame in 0..10 {
        println!("\n--- Frame {} ---", frame + 1);
        
        // อัปเดตระบบเสียง
        audio.update(1.0 / 60.0); // 60 FPS
        
        // เล่น sound effects ในบางเฟรม
        match frame {
            2 => {
                // เล่นเสียงกระโดด
                audio.play(jump_source).expect("Failed to play jump");
                println!("🦘 Player jumped!");
            }
            5 => {
                // เล่นเสียงระเบิดแบบ 3D
                let explosion_pos = AudioPosition::new(10.0, 0.0, -5.0);
                audio.play_one_shot(explosion_id, 1.0, Some(explosion_pos))
                    .expect("Failed to play explosion");
                println!("💥 Explosion at (10, 0, -5)!");
            }
            7 => {
                // เคลื่อนย้าย listener
                audio.listener.position = AudioPosition::new(5.0, 0.0, 0.0);
                println!("🚶 Listener moved to (5, 0, 0)");
            }
            8 => {
                // ลดเสียงเพลง
                audio.mixer.set_channel_volume("Music", 0.3);
                println!("🔉 Lowered music volume");
            }
            _ => {}
        }
        
        // แสดงสถิติ
        let stats = audio.get_stats();
        println!("📊 Stats: {} clips, {} sources playing, {:.1}KB memory",
                stats.loaded_clips, stats.playing_sources, 
                stats.memory_usage_bytes as f32 / 1024.0);
    }
    
    // ทดสอบ audio effects
    println!("\n🎚️ Testing audio effects:");
    test_audio_effects(&mut audio);
    
    // แสดง best practices
    println!("\n💡 Audio System Best Practices:");
    show_audio_best_practices();
}

/// 🎚️ ทดสอบ audio effects
fn test_audio_effects(audio: &mut AudioSystem) {
    let effects = vec![
        AudioEffect::LowPass { cutoff_frequency: 1000.0, resonance: 0.7 },
        AudioEffect::HighPass { cutoff_frequency: 200.0, resonance: 0.5 },
        AudioEffect::Distortion { drive: 0.8, tone: 0.6 },
        AudioEffect::Chorus { rate: 1.5, depth: 0.3, feedback: 0.2 },
    ];
    
    for effect in effects {
        audio.mixer.add_effect("SFX", effect.clone());
        println!("🎛️ Added effect: {:?}", effect);
    }
    
    println!("🎵 Effects applied to SFX channel");
}

/// 💡 Audio Best Practices
fn show_audio_best_practices() {
    let practices = vec![
        "🎯 Use appropriate audio formats for different purposes",
        "💾 Implement audio streaming for large files",
        "🔄 Use object pooling for frequently played sounds",
        "📊 Monitor audio memory usage",
        "🎚️ Implement proper audio mixing and ducking",
        "🌍 Use 3D audio for immersive experiences",
        "⚡ Optimize audio processing for target platforms",
        "🎵 Implement audio compression and decompression",
        "🔇 Provide audio accessibility options",
        "📱 Consider mobile audio limitations",
        "🎧 Test with different audio devices",
        "🔊 Implement proper volume controls",
        "🎪 Use audio occlusion and obstruction",
        "⏱️ Implement audio synchronization",
        "🎭 Create dynamic audio systems",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n🦀 Rust Audio Libraries:");
    println!("   • rodio - High-level audio playback");
    println!("   • cpal - Cross-platform audio I/O");
    println!("   • hound - WAV encoding/decoding");
    println!("   • symphonia - Audio decoding");
    println!("   • dasp - Digital audio signal processing");
    println!("   • fundsp - Audio synthesis and processing");
    println!("   • kira - Game audio engine");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_position() {
        let pos1 = AudioPosition::new(0.0, 0.0, 0.0);
        let pos2 = AudioPosition::new(3.0, 4.0, 0.0);
        
        assert_eq!(pos1.distance_to(&pos2), 5.0); // 3-4-5 triangle
    }
    
    #[test]
    fn test_audio_clip_creation() {
        let clip = AudioClip::new(1, "test", "test.wav", AudioFormat::WAV);
        
        assert_eq!(clip.id, 1);
        assert_eq!(clip.name, "test");
        assert_eq!(clip.format, AudioFormat::WAV);
        assert!(!clip.is_loaded);
    }
    
    #[test]
    fn test_audio_source() {
        let mut source = AudioSource::new(1, 1);
        
        assert!(!source.is_playing);
        
        source.play();
        assert!(source.is_playing);
        assert!(!source.is_paused);
        
        source.pause();
        assert!(source.is_paused);
        
        source.stop();
        assert!(!source.is_playing);
        assert!(!source.is_paused);
    }
    
    #[test]
    fn test_audio_mixer() {
        let mut mixer = AudioMixer::new(44100, 512);
        
        assert_eq!(mixer.sample_rate, 44100);
        assert_eq!(mixer.buffer_size, 512);
        
        mixer.set_channel_volume("Music", 0.5);
        if let Some(channel) = mixer.channels.get("Music") {
            assert_eq!(channel.volume, 0.5);
        }
    }
    
    #[test]
    fn test_audio_system() {
        let mut audio = AudioSystem::new();
        
        let clip_id = audio.load_clip("test", "test.wav", AudioFormat::WAV)
            .expect("Failed to load clip");
        
        let source_id = audio.create_source(clip_id)
            .expect("Failed to create source");
        
        audio.play(source_id).expect("Failed to play");
        
        assert!(audio.sources.get(&source_id).unwrap().is_playing);
    }
    
    #[test]
    fn test_distance_volume_calculation() {
        let mut source = AudioSource::new(1, 1);
        source.position = AudioPosition::new(0.0, 0.0, 0.0);
        source.max_distance = 10.0;
        
        let listener_pos = AudioPosition::new(5.0, 0.0, 0.0);
        let volume = source.calculate_distance_volume(&listener_pos);
        
        assert!(volume > 0.0 && volume < 1.0); // Should be reduced but not zero
    }
}

// 🎵 "เสียงคือครึ่งหนึ่งของประสบการณ์
//     เกมที่ไม่มีเสียงเหมือนหนังเงียบ
//     แต่เกมที่เสียงแย่เหมือนหนังที่มีเสียงรบกวน
//     ดังนั้นทำเสียงให้ดี ผู้เล่นจะรักคุณ! 🎧❤️"