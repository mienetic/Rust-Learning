//! 🔐 Encryption Workshop - การเข้ารหัสข้อมูล! 🛡️
//!
//! ยินดีต้อนรับสู่ Workshop การเรียนรู้การเข้ารหัส! 🎉
//! เหมือนการสร้างรหัสลับเพื่อปกป้องข้อมูล! 🔒
//!
//! 🎯 สิ่งที่จะได้เรียนรู้:
//! - 🔄 Caesar Cipher - การเลื่อนตัวอักษร
//! - ⚡ XOR Cipher - การเข้ารหัสแบบ XOR
//! - 🔀 Substitution Cipher - การแทนที่ตัวอักษร
//! - 📝 Base64 Encoding - การเข้ารหัสแบบ Base64
//!
//! หมายเหตุ: นี่คือการจำลองเพื่อการศึกษา! 📚

use std::collections::HashMap;

/// 🔄 Caesar Cipher - การเลื่อนตัวอักษร!
/// เหมือนการเลื่อนตัวอักษรในวงล้อ! 🎡
struct CaesarCipher {
    shift: u8,
}

impl CaesarCipher {
    const fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }
    
    fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (c as u8 - base + self.shift) % 26;
                    (base + shifted) as char
                } else {
                    c
                }
            })
            .collect()
    }
    
    fn decrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (c as u8 - base + 26 - self.shift) % 26;
                    (base + shifted) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

/// ⚡ XOR Cipher - การเข้ารหัสแบบ XOR!
/// เหมือนการใช้กุญแจวิเศษเปลี่ยนข้อมูล! 🗝️
struct XorCipher {
    key: Vec<u8>,
}

impl XorCipher {
    fn new(key: &str) -> Self {
        Self {
            key: key.bytes().collect(),
        }
    }
    
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ self.key[i % self.key.len()])
            .collect()
    }
    
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // XOR is symmetric, so decrypt is the same as encrypt
        self.encrypt(data)
    }
    
    fn encrypt_string(&self, text: &str) -> Vec<u8> {
        self.encrypt(text.as_bytes())
    }
    
    fn decrypt_to_string(&self, data: &[u8]) -> Result<String, std::string::FromUtf8Error> {
        let decrypted = self.decrypt(data);
        String::from_utf8(decrypted)
    }
}

/// 🔀 Substitution Cipher - การแทนที่ตัวอักษร!
/// เหมือนการใช้รหัสลับแทนตัวอักษร! 🔤
struct SubstitutionCipher {
    encrypt_map: HashMap<char, char>,
    decrypt_map: HashMap<char, char>,
}

impl SubstitutionCipher {
    fn new(key: &str) -> Result<Self, String> {
        if key.len() != 26 {
            return Err("Key must be exactly 26 characters".to_string());
        }
        
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut encrypt_map = HashMap::new();
        let mut decrypt_map = HashMap::new();
        
        for (i, c) in alphabet.chars().enumerate() {
            let key_char = key.chars().nth(i).unwrap().to_ascii_lowercase();
            encrypt_map.insert(c, key_char);
            encrypt_map.insert(c.to_ascii_uppercase(), key_char.to_ascii_uppercase());
            decrypt_map.insert(key_char, c);
            decrypt_map.insert(key_char.to_ascii_uppercase(), c.to_ascii_uppercase());
        }
        
        Ok(Self {
            encrypt_map,
            decrypt_map,
        })
    }
    
    fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.encrypt_map.get(&c).copied().unwrap_or(c))
            .collect()
    }
    
    fn decrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.decrypt_map.get(&c).copied().unwrap_or(c))
            .collect()
    }
}

/// 📝 Base64 Encoder - การเข้ารหัสแบบ Base64!
/// เหมือนการแปลงข้อมูลให้อ่านได้ง่าย! 📄
struct Base64 {
    chars: &'static str,
}

impl Base64 {
    const fn new() -> Self {
        Self {
            chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        }
    }
    
    fn encode(&self, data: &[u8]) -> String {
        let mut result = String::new();
        let chars: Vec<char> = self.chars.chars().collect();
        
        for chunk in data.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b = (u32::from(buf[0]) << 16) | (u32::from(buf[1]) << 8) | u32::from(buf[2]);
            
            result.push(chars[((b >> 18) & 63) as usize]);
            result.push(chars[((b >> 12) & 63) as usize]);
            
            if chunk.len() > 1 {
                result.push(chars[((b >> 6) & 63) as usize]);
            } else {
                result.push('=');
            }
            
            if chunk.len() > 2 {
                result.push(chars[(b & 63) as usize]);
            } else {
                result.push('=');
            }
        }
        
        result
    }
    
    fn decode(&self, encoded: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let chars: HashMap<char, u8> = self.chars.chars().enumerate()
            .map(|(i, c)| (c, i as u8))
            .collect();
        
        let cleaned: String = encoded.chars().filter(|&c| c != '=').collect();
        
        for chunk in cleaned.as_bytes().chunks(4) {
            if chunk.len() < 2 {
                return Err("Invalid base64 input".to_string());
            }
            
            let mut values = [0u8; 4];
            for (i, &byte) in chunk.iter().enumerate() {
                let c = byte as char;
                values[i] = *chars.get(&c).ok_or("Invalid character in base64")?;
            }
            
            let b = (u32::from(values[0]) << 18) |
                   (u32::from(values[1]) << 12) |
                   (u32::from(values[2]) << 6) |
                   u32::from(values[3]);
            
            result.push((b >> 16) as u8);
            if chunk.len() > 2 {
                result.push((b >> 8) as u8);
            }
            if chunk.len() > 3 {
                result.push(b as u8);
            }
        }
        
        Ok(result)
    }
}

/// 🔐 Encryption Manager - ตัวจัดการการเข้ารหัส!
/// เหมือนการมีกล่องเครื่องมือเข้ารหัสครบชุด! 🧰
struct EncryptionManager {
    caesar: CaesarCipher,
    xor: XorCipher,
    substitution: Option<SubstitutionCipher>,
    base64: Base64,
}

impl EncryptionManager {
    fn new(caesar_shift: u8, xor_key: &str, substitution_key: Option<&str>) -> Result<Self, String> {
        let substitution = if let Some(key) = substitution_key {
            Some(SubstitutionCipher::new(key)?)
        } else {
            None
        };
        
        Ok(Self {
            caesar: CaesarCipher::new(caesar_shift),
            xor: XorCipher::new(xor_key),
            substitution,
            base64: Base64::new(),
        })
    }
    
    fn encrypt_caesar(&self, text: &str) -> String {
        self.caesar.encrypt(text)
    }
    
    fn decrypt_caesar(&self, text: &str) -> String {
        self.caesar.decrypt(text)
    }
    
    fn encrypt_xor(&self, text: &str) -> Vec<u8> {
        self.xor.encrypt_string(text)
    }
    
    fn decrypt_xor(&self, data: &[u8]) -> Result<String, std::string::FromUtf8Error> {
        self.xor.decrypt_to_string(data)
    }
    
    fn encrypt_substitution(&self, text: &str) -> Result<String, String> {
        match &self.substitution {
            Some(cipher) => Ok(cipher.encrypt(text)),
            None => Err("Substitution cipher not initialized".to_string()),
        }
    }
    
    fn decrypt_substitution(&self, text: &str) -> Result<String, String> {
        match &self.substitution {
            Some(cipher) => Ok(cipher.decrypt(text)),
            None => Err("Substitution cipher not initialized".to_string()),
        }
    }
    
    fn encode_base64(&self, data: &[u8]) -> String {
        self.base64.encode(data)
    }
    
    fn decode_base64(&self, encoded: &str) -> Result<Vec<u8>, String> {
        self.base64.decode(encoded)
    }
}

/// 🚀 สาธิต Encryption Workshop!
/// เหมือนการเรียนรู้วิธีสร้างรหัสลับ! 🔒
pub fn demonstrate_encryption() {
    println!("🎉 ยินดีต้อนรับสู่ Encryption Workshop! 🔐");
    println!("เหมือนการเรียนรู้วิธีสร้างรหัสลับเพื่อปกป้องข้อมูล! 🛡️\n");
    
    let manager = EncryptionManager::new(
        3, // Caesar shift
        "secretkey", // XOR key
        Some("zyxwvutsrqponmlkjihgfedcba"), // Substitution key
    ).unwrap();
    
    let original_text = "Hello, World! This is a secret message.";
    println!("📝 ข้อความต้นฉบับ: {original_text}");
    
    // Caesar Cipher
    println!("\n🔄 Caesar Cipher - การเลื่อนตัวอักษร (เลื่อน 3 ตำแหน่ง):");
    let caesar_encrypted = manager.encrypt_caesar(original_text);
    println!("  🔒 เข้ารหัสแล้ว: {caesar_encrypted}");
    let caesar_decrypted = manager.decrypt_caesar(&caesar_encrypted);
    println!("  🔓 ถอดรหัสแล้ว: {caesar_decrypted}");
    
    // XOR Cipher
    println!("\n⚡ XOR Cipher - การเข้ารหัสแบบ XOR:");
    let xor_encrypted = manager.encrypt_xor(original_text);
    println!("  🔒 เข้ารหัสแล้ว (hex): {}", hex_encode(&xor_encrypted));
    let xor_decrypted = manager.decrypt_xor(&xor_encrypted).unwrap();
    println!("  🔓 ถอดรหัสแล้ว: {xor_decrypted}");
    
    // Substitution Cipher
    println!("\n🔀 Substitution Cipher - การแทนที่ตัวอักษร:");
    let sub_encrypted = manager.encrypt_substitution(original_text).unwrap();
    println!("  🔒 เข้ารหัสแล้ว: {sub_encrypted}");
    let sub_decrypted = manager.decrypt_substitution(&sub_encrypted).unwrap();
    println!("  🔓 ถอดรหัสแล้ว: {sub_decrypted}");
    
    // Base64 Encoding
    println!("\n📝 Base64 Encoding - การเข้ารหัสแบบ Base64:");
    let base64_encoded = manager.encode_base64(original_text.as_bytes());
    println!("  🔒 เข้ารหัสแล้ว: {base64_encoded}");
    let base64_decoded = manager.decode_base64(&base64_encoded).unwrap();
    let base64_text = String::from_utf8(base64_decoded).unwrap();
    println!("  🔓 ถอดรหัสแล้ว: {base64_text}");
    
    // Combined encryption
    println!("\n🔗 การเข้ารหัสแบบรวม (Caesar + XOR + Base64):");
    println!("เหมือนการใส่ข้อมูลในกล่องหลายชั้น! 📦");
    let step1 = manager.encrypt_caesar(original_text);
    let step2 = manager.encrypt_xor(&step1);
    let step3 = manager.encode_base64(&step2);
    println!("  🔒 เข้ารหัสขั้นสุดท้าย: {step3}");
    
    // Combined decryption
    println!("\n🔓 การถอดรหัสแบบรวม:");
    let decode1 = manager.decode_base64(&step3).unwrap();
    let decode2 = manager.decrypt_xor(&decode1).unwrap();
    let decode3 = manager.decrypt_caesar(&decode2);
    println!("  🔓 ถอดรหัสขั้นสุดท้าย: {decode3}");
    
    println!("\n🎉 ยินดีด้วย! คุณได้เรียนรู้การเข้ารหัสเรียบร้อยแล้ว!");
    println!("💡 ตอนนี้คุณรู้วิธีปกป้องข้อมูลด้วยรหัสลับแล้ว! 🔐");
}

/// Helper function to encode bytes as hex
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_caesar_cipher() {
        let cipher = CaesarCipher::new(3);
        let original = "Hello";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(original, decrypted);
    }
    
    #[test]
    fn test_xor_cipher() {
        let cipher = XorCipher::new("key");
        let original = "Hello, World!";
        let encrypted = cipher.encrypt_string(original);
        let decrypted = cipher.decrypt_to_string(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
    
    #[test]
    fn test_base64() {
        let base64 = Base64::new();
        let original = b"Hello, World!";
        let encoded = base64.encode(original);
        let decoded = base64.decode(&encoded).unwrap();
        assert_eq!(original.to_vec(), decoded);
    }
    
    #[test]
    fn test_substitution_cipher() {
        let cipher = SubstitutionCipher::new("zyxwvutsrqponmlkjihgfedcba").unwrap();
        let original = "Hello";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(original, decrypted);
    }
}