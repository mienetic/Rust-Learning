//! 🔐 Web Development Workshop: Hashing and Digital Signatures 🔐
//!
//! 🎯 ยินดีต้อนรับสู่เวิร์คช็อป "การสร้างลายเซ็นดิจิทัลและการแฮช"!
//! 📚 เรียนรู้วิธีการสร้างลายนิ้วมือดิจิทัลสำหรับข้อมูล
//! 🛡️ เหมือนการสร้างตราประทับที่ไม่สามารถปลอมแปลงได้!

use std::collections::HashMap;
use std::fmt;

/// 🔍 ประเภทของอัลกอริทึมแฮช - เหมือนเครื่องมือสร้างลายนิ้วมือดิจิทัล
/// 🎨 แต่ละแบบจะสร้างลายเซ็นที่แตกต่างกัน!
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake2b,
    Argon2,
}

/// 📋 ผลลัพธ์การแฮช - เหมือนใบรับรองดิจิทัล
/// 🎯 เก็บข้อมูลลายเซ็นและวิธีการตรวจสอบ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashResult {
    pub algorithm: HashAlgorithm,
    pub hash: Vec<u8>,
    pub hex_string: String,
}

impl HashResult {
    fn new(algorithm: HashAlgorithm, hash: Vec<u8>) -> Self {
        let hex_string = hash.iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        
        Self {
            algorithm,
            hash,
            hex_string,
        }
    }
    
    fn verify(&self, input: &[u8], hasher: &dyn Hasher) -> bool {
        let computed = hasher.hash(input);
        computed.hash == self.hash
    }
}

impl fmt::Display for HashResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.algorithm, self.hex_string)
    }
}

/// 🔧 Trait สำหรับเครื่องมือแฮช - พิมพ์เขียวสำหรับเครื่องสร้างลายเซ็น
/// 🎪 ทุกเครื่องมือต้องมีความสามารถพื้นฐานเหล่านี้!
trait Hasher {
    fn hash(&self, input: &[u8]) -> HashResult;
    fn algorithm(&self) -> HashAlgorithm;
}

/// 🔴 MD5 Hasher - เครื่องมือแฮชรุ่นเก่า (ใช้เพื่อการเรียนรู้เท่านั้น)
/// ⚠️ หมายเหตุ: ไม่ปลอดภัยสำหรับการใช้งานจริง!
struct Md5Hasher;

impl Hasher for Md5Hasher {
    fn hash(&self, input: &[u8]) -> HashResult {
        // Simplified MD5 simulation
        let mut hash = vec![0u8; 16];
        let mut state = 0x67452301u32;
        
        for (i, &byte) in input.iter().enumerate() {
            state = state.wrapping_add(u32::from(byte));
            state = state.rotate_left((i % 32) as u32);
            hash[i % 16] = (state ^ u32::from(byte)) as u8;
        }
        
        // Add some more mixing
        for i in 0..16 {
            hash[i] = hash[i].wrapping_add((state >> (i * 2)) as u8);
        }
        
        HashResult::new(HashAlgorithm::Md5, hash)
    }
    
    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Md5
    }
}

/// 🟢 SHA-256 Hasher - เครื่องมือแฮชมาตรฐานที่ปลอดภัย
/// 🏆 เป็นที่นิยมใช้ในระบบ Bitcoin และเว็บไซต์ต่างๆ!
struct Sha256Hasher;

impl Hasher for Sha256Hasher {
    fn hash(&self, input: &[u8]) -> HashResult {
        // Simplified SHA-256 simulation
        let mut hash = vec![0u8; 32];
        let mut state = [
            0x6a09e667u32, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        
        // Process input in chunks
        let mut processed = 0;
        for chunk in input.chunks(64) {
            for (i, &byte) in chunk.iter().enumerate() {
                let state_idx = i % 8;
                state[state_idx] = state[state_idx]
                    .wrapping_add(u32::from(byte))
                    .rotate_left(((i + processed) % 32) as u32);
            }
            processed += chunk.len();
        }
        
        // Convert state to bytes
        for (i, &word) in state.iter().enumerate() {
            let bytes = word.to_be_bytes();
            hash[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }
        
        HashResult::new(HashAlgorithm::Sha256, hash)
    }
    
    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Sha256
    }
}

/// 🔵 SHA-512 Hasher - เครื่องมือแฮชความปลอดภัยสูง
/// 💪 แข็งแกร่งกว่า SHA-256 เหมาะสำหรับข้อมูลสำคัญ!
struct Sha512Hasher;

impl Hasher for Sha512Hasher {
    fn hash(&self, input: &[u8]) -> HashResult {
        // Simplified SHA-512 simulation
        let mut hash = vec![0u8; 64];
        let mut state = [
            0x6a09e667f3bcc908u64, 0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
            0x510e527fade682d1, 0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
        ];
        
        // Process input
        let mut processed = 0;
        for chunk in input.chunks(128) {
            for (i, &byte) in chunk.iter().enumerate() {
                let state_idx = i % 8;
                state[state_idx] = state[state_idx]
                    .wrapping_add(u64::from(byte))
                    .rotate_left(((i + processed) % 64) as u32);
            }
            processed += chunk.len();
        }
        
        // Convert state to bytes
        for (i, &word) in state.iter().enumerate() {
            let bytes = word.to_be_bytes();
            hash[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
        }
        
        HashResult::new(HashAlgorithm::Sha512, hash)
    }
    
    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Sha512
    }
}

/// 🟣 `BLAKE2b` Hasher - เครื่องมือแฮชรุ่นใหม่ที่เร็วและปลอดภัย
/// ⚡ เร็วกว่า SHA-3 แต่ปลอดภัยเท่ากัน!
struct Blake2bHasher;

impl Hasher for Blake2bHasher {
    fn hash(&self, input: &[u8]) -> HashResult {
        // Simplified BLAKE2b simulation
        let mut hash = vec![0u8; 64];
        let mut state = [
            0x6a09e667f2bdc948u64, 0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
            0x510e527fade682d1, 0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
        ];
        
        // BLAKE2b specific initialization
        state[0] ^= 0x01010040; // Parameter block
        
        // Process input
        for (i, &byte) in input.iter().enumerate() {
            let state_idx = i % 8;
            state[state_idx] = state[state_idx]
                .wrapping_add(u64::from(byte))
                .rotate_right(((i % 63) + 1) as u32); // Different rotation
        }
        
        // Final mixing
        for i in 0..8 {
            state[i] ^= state[(i + 4) % 8];
        }
        
        // Convert to bytes
        for (i, &word) in state.iter().enumerate() {
            let bytes = word.to_le_bytes(); // Little-endian for BLAKE2b
            hash[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
        }
        
        HashResult::new(HashAlgorithm::Blake2b, hash)
    }
    
    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Blake2b
    }
}

/// 🔑 การแฮชรหัสผ่านพร้อมเกลือ - เหมือนการเก็บรหัสผ่านในตู้เซฟ
/// 🧂 เกลือช่วยป้องกันการโจมตีแบบ Rainbow Table!
#[derive(Debug, Clone)]
pub struct PasswordHash {
    pub algorithm: HashAlgorithm,
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
    pub iterations: u32,
}

impl PasswordHash {
    const fn new(algorithm: HashAlgorithm, salt: Vec<u8>, hash: Vec<u8>, iterations: u32) -> Self {
        Self {
            algorithm,
            salt,
            hash,
            iterations,
        }
    }
    
    fn verify(&self, password: &str, hasher: &PasswordHasher) -> bool {
        let computed = hasher.hash_password(password, &self.salt, self.iterations);
        computed.hash == self.hash
    }
    
    fn to_string(&self) -> String {
        let salt_hex = self.salt.iter().map(|b| format!("{b:02x}")).collect::<String>();
        let hash_hex = self.hash.iter().map(|b| format!("{b:02x}")).collect::<String>();
        format!("{:?}:{}:{}:{}", self.algorithm, self.iterations, salt_hex, hash_hex)
    }
}

/// 🔐 เครื่องแฮชรหัสผ่าน - ผู้เชี่ยวชาญด้านการปกป้องรหัสผ่าน
/// 💪 ใช้เทคนิค Salt และ Iteration เพื่อความปลอดภัยสูงสุด!
struct PasswordHasher {
    base_hasher: Box<dyn Hasher>,
}

impl PasswordHasher {
    fn new(algorithm: HashAlgorithm) -> Self {
        let base_hasher: Box<dyn Hasher> = match algorithm {
            HashAlgorithm::Sha256 => Box::new(Sha256Hasher),
            HashAlgorithm::Sha512 => Box::new(Sha512Hasher),
            HashAlgorithm::Blake2b => Box::new(Blake2bHasher),
            _ => Box::new(Sha256Hasher), // Default
        };
        
        Self { base_hasher }
    }
    
    fn generate_salt(&self) -> Vec<u8> {
        // Simple salt generation (in real implementation, use crypto-secure random)
        let mut salt = Vec::new();
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        for i in 0..32 {
            salt.push(((seed.wrapping_mul(i + 1)) % 256) as u8);
        }
        
        salt
    }
    
    fn hash_password(&self, password: &str, salt: &[u8], iterations: u32) -> PasswordHash {
        let mut current = password.as_bytes().to_vec();
        current.extend_from_slice(salt);
        
        // Perform iterations
        for _ in 0..iterations {
            let result = self.base_hasher.hash(&current);
            current = result.hash;
            current.extend_from_slice(salt); // Re-add salt for next iteration
        }
        
        // Final hash without salt
        let final_result = self.base_hasher.hash(&current[..current.len() - salt.len()]);
        
        PasswordHash::new(
            self.base_hasher.algorithm(),
            salt.to_vec(),
            final_result.hash,
            iterations,
        )
    }
    
    fn hash_password_with_salt(&self, password: &str, iterations: u32) -> PasswordHash {
        let salt = self.generate_salt();
        self.hash_password(password, &salt, iterations)
    }
}

/// 🔐 HMAC Generator - เครื่องสร้างลายเซ็นข้อความ
/// 🛡️ ใช้คีย์ลับเพื่อยืนยันความถูกต้องของข้อความ!
struct HmacGenerator {
    hasher: Box<dyn Hasher>,
    block_size: usize,
}

impl HmacGenerator {
    fn new(algorithm: HashAlgorithm) -> Self {
        let (hasher, block_size): (Box<dyn Hasher>, usize) = match algorithm {
            HashAlgorithm::Sha256 => (Box::new(Sha256Hasher), 64),
            HashAlgorithm::Sha512 => (Box::new(Sha512Hasher), 128),
            HashAlgorithm::Blake2b => (Box::new(Blake2bHasher), 128),
            _ => (Box::new(Sha256Hasher), 64),
        };
        
        Self { hasher, block_size }
    }
    
    fn generate(&self, key: &[u8], message: &[u8]) -> HashResult {
        let mut key = key.to_vec();
        
        // If key is longer than block size, hash it
        if key.len() > self.block_size {
            key = self.hasher.hash(&key).hash;
        }
        
        // Pad key to block size
        key.resize(self.block_size, 0);
        
        // Create inner and outer padding
        let mut ipad = vec![0x36; self.block_size];
        let mut opad = vec![0x5c; self.block_size];
        
        // XOR key with padding
        for i in 0..self.block_size {
            ipad[i] ^= key[i];
            opad[i] ^= key[i];
        }
        
        // Inner hash: H(K ⊕ ipad || message)
        let mut inner_input = ipad;
        inner_input.extend_from_slice(message);
        let inner_hash = self.hasher.hash(&inner_input);
        
        // Outer hash: H(K ⊕ opad || inner_hash)
        let mut outer_input = opad;
        outer_input.extend_from_slice(&inner_hash.hash);
        self.hasher.hash(&outer_input)
    }
    
    fn verify(&self, key: &[u8], message: &[u8], expected_hmac: &[u8]) -> bool {
        let computed_hmac = self.generate(key, message);
        
        // Constant-time comparison
        if computed_hmac.hash.len() != expected_hmac.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (a, b) in computed_hmac.hash.iter().zip(expected_hmac.iter()) {
            result |= a ^ b;
        }
        
        result == 0
    }
}

/// ✍️ ลายเซ็นดิจิทัล - เหมือนลายเซ็นจริงแต่ปลอมแปลงไม่ได้
/// 🔒 ใช้คู่กุญแจเพื่อยืนยันตัวตนและความถูกต้อง!
#[derive(Debug, Clone)]
pub struct DigitalSignature {
    pub signature: Vec<u8>,
    pub algorithm: HashAlgorithm,
}

/// 🗝️ คู่กุญแจ - กุญแจสาธารณะและกุญแจส่วนตัว
/// 🎭 เหมือนการมีกุญแจสำหรับล็อคและปลดล็อค!
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

/// ✍️ ผู้ลงลายเซ็นดิจิทัล - ช่างฝีมือด้านการสร้างลายเซ็น
/// 🎨 สร้างและตรวจสอบลายเซ็นที่ไม่สามารถปลอมแปลงได้!
struct DigitalSigner {
    hasher: Box<dyn Hasher>,
}

impl DigitalSigner {
    fn new(algorithm: HashAlgorithm) -> Self {
        let hasher: Box<dyn Hasher> = match algorithm {
            HashAlgorithm::Sha256 => Box::new(Sha256Hasher),
            HashAlgorithm::Sha512 => Box::new(Sha512Hasher),
            _ => Box::new(Sha256Hasher),
        };
        
        Self { hasher }
    }
    
    fn generate_keypair(&self) -> KeyPair {
        // Simplified key generation (not cryptographically secure)
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut private_key = Vec::new();
        let mut public_key = Vec::new();
        
        // Generate private key
        for i in 0..32 {
            private_key.push(((seed.wrapping_mul(i + 1)) % 256) as u8);
        }
        
        // Generate public key from private key
        let private_hash = self.hasher.hash(&private_key);
        public_key = private_hash.hash;
        
        KeyPair {
            public_key,
            private_key,
        }
    }
    
    fn sign(&self, message: &[u8], private_key: &[u8]) -> DigitalSignature {
        // Hash the message
        let message_hash = self.hasher.hash(message);
        
        // "Encrypt" hash with private key (simplified)
        // In this simplified implementation, we'll derive the public key from private key
        let public_key_hash = self.hasher.hash(private_key);
        let public_key = &public_key_hash.hash;
        
        let mut signature = Vec::new();
        for (i, &hash_byte) in message_hash.hash.iter().enumerate() {
            let key_byte = public_key[i % public_key.len()];
            signature.push(hash_byte.wrapping_add(key_byte));
        }
        
        DigitalSignature {
            signature,
            algorithm: self.hasher.algorithm(),
        }
    }
    
    fn verify(&self, message: &[u8], signature: &DigitalSignature, public_key: &[u8]) -> bool {
        // Hash the message
        let message_hash = self.hasher.hash(message);
        
        // In this simplified implementation, we'll use the public key directly
        // to verify the signature (this is not how real digital signatures work)
        let mut decrypted = Vec::new();
        for (i, &sig_byte) in signature.signature.iter().enumerate() {
            let key_byte = public_key[i % public_key.len()];
            decrypted.push(sig_byte.wrapping_sub(key_byte));
        }
        
        // Compare with message hash
        decrypted == message_hash.hash
    }
}

/// 🎛️ ตัวจัดการแฮช - ศูนย์รวมเครื่องมือแฮชทุกประเภท
/// 🔧 เหมือนกล่องเครื่องมือที่มีเครื่องมือครบครัน!
pub struct HashManager {
    hashers: HashMap<HashAlgorithm, Box<dyn Hasher>>,
}

impl HashManager {
    fn new() -> Self {
        let mut hashers: HashMap<HashAlgorithm, Box<dyn Hasher>> = HashMap::new();
        
        hashers.insert(HashAlgorithm::Md5, Box::new(Md5Hasher));
        hashers.insert(HashAlgorithm::Sha256, Box::new(Sha256Hasher));
        hashers.insert(HashAlgorithm::Sha512, Box::new(Sha512Hasher));
        hashers.insert(HashAlgorithm::Blake2b, Box::new(Blake2bHasher));
        
        Self { hashers }
    }
    
    fn hash(&self, algorithm: HashAlgorithm, input: &[u8]) -> Option<HashResult> {
        self.hashers.get(&algorithm).map(|hasher| hasher.hash(input))
    }
    
    fn hash_string(&self, algorithm: HashAlgorithm, input: &str) -> Option<HashResult> {
        self.hash(algorithm, input.as_bytes())
    }
    
    fn verify_hash(&self, input: &[u8], expected: &HashResult) -> bool {
        if let Some(hasher) = self.hashers.get(&expected.algorithm) {
            let computed = hasher.hash(input);
            computed.hash == expected.hash
        } else {
            false
        }
    }
    
    fn compare_algorithms(&self, input: &[u8]) -> Vec<HashResult> {
        let mut results = Vec::new();
        
        for algorithm in [HashAlgorithm::Md5, HashAlgorithm::Sha256, HashAlgorithm::Sha512, HashAlgorithm::Blake2b] {
            if let Some(result) = self.hash(algorithm, input) {
                results.push(result);
            }
        }
        
        results
    }
}

/// 🎓 สาธิตการใช้งาน Hashing และ Digital Signatures - เวิร์คช็อปความปลอดภัย
pub fn demonstrate_hashing() {
    println!("🎯 ยินดีต้อนรับสู่เวิร์คช็อป 'การสร้างลายนิ้วมือดิจิทัล'! 🔐");
    println!("📚 วันนี้เราจะเรียนรู้วิธีปกป้องข้อมูลด้วยเทคนิคแฮชและลายเซ็นดิจิทัล!");
    
    let manager = HashManager::new();
    
    // Basic Hashing
    println!("\n🔨 บทที่ 1: การสร้างลายนิ้วมือดิจิทัลพื้นฐาน");
    println!("🎨 เหมือนการสร้างลายนิ้วมือที่ไม่ซ้ำใครสำหรับข้อมูล!");
    println!("{:-<60}", "");
    
    let test_data = "Hello, World!";
    println!("📝 ข้อมูลต้นฉบับ: {test_data}");
    println!("🔍 มาดูลายนิ้วมือที่สร้างจากอัลกอริทึมต่างๆ:");
    
    let results = manager.compare_algorithms(test_data.as_bytes());
    for result in results {
        println!("  🎯 {result}");
    }
    
    // Hash Verification
    println!("\n🔍 บทที่ 2: การตรวจสอบความถูกต้องของข้อมูล");
    println!("🛡️ เหมือนการตรวจสอบว่าเอกสารถูกแก้ไขหรือไม่!");
    println!("{:-<60}", "");
    
    let original = "secret message";
    let sha256_hash = manager.hash_string(HashAlgorithm::Sha256, original).unwrap();
    
    println!("📄 ข้อความต้นฉบับ: {original}");
    println!("🔐 ลายนิ้วมือ SHA-256: {}", sha256_hash.hex_string);
    
    // Verify correct input
    let is_valid = manager.verify_hash(original.as_bytes(), &sha256_hash);
    println!("✅ ตรวจสอบข้อความถูกต้อง: {is_valid} 🎉");
    
    // Verify incorrect input
    let is_invalid = manager.verify_hash(b"wrong message", &sha256_hash);
    println!("❌ ตรวจสอบข้อความผิด: {is_invalid} ⚠️");
    
    // Password Hashing
    println!("\n🔑 บทที่ 3: การปกป้องรหัสผ่านด้วยเกลือและการทำซ้ำ");
    println!("🧂 เหมือนการใส่เกลือในอาหารเพื่อให้รสชาติไม่ซ้ำใคร!");
    println!("{:-<60}", "");
    
    let password_hasher = PasswordHasher::new(HashAlgorithm::Sha256);
    
    let password = "my_secure_password123";
    let password_hash = password_hasher.hash_password_with_salt(password, 10000);
    
    println!("🔐 รหัสผ่าน: {password}");
    println!("🧂 เกลือ (Salt): {}", password_hash.salt.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("🔒 แฮชที่ได้: {}", password_hash.hash.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("🔄 จำนวนรอบ: {} รอบ", password_hash.iterations);
    println!("📦 รหัสที่เข้ารหัสแล้ว: {}", password_hash.to_string());
    
    // Verify password
    let correct_password = password_hash.verify(password, &password_hasher);
    let wrong_password = password_hash.verify("wrong_password", &password_hasher);
    
    println!("✅ ตรวจสอบรหัสผ่านถูกต้อง: {correct_password} 🎯");
    println!("❌ ตรวจสอบรหัสผ่านผิด: {wrong_password} 🚫");
    
    // HMAC
    println!("\n🔐 บทที่ 4: HMAC - การยืนยันตัวตนของข้อความ");
    println!("🛡️ เหมือนการใส่ตราประทับลับที่มีเพียงเราเท่านั้นที่รู้!");
    println!("{:-<60}", "");
    
    let hmac_generator = HmacGenerator::new(HashAlgorithm::Sha256);
    let key = b"secret_key";
    let message = b"important message";
    
    let hmac = hmac_generator.generate(key, message);
    println!("🗝️ กุญแจลับ: {}", String::from_utf8_lossy(key));
    println!("📝 ข้อความ: {}", String::from_utf8_lossy(message));
    println!("🔐 ลายเซ็น HMAC: {}", hmac.hex_string);
    
    // Verify HMAC
    let valid_hmac = hmac_generator.verify(key, message, &hmac.hash);
    let invalid_hmac = hmac_generator.verify(b"wrong_key", message, &hmac.hash);
    
    println!("✅ ตรวจสอบ HMAC ถูกต้อง: {valid_hmac} 🎉");
    println!("❌ ตรวจสอบ HMAC ผิด: {invalid_hmac} ⚠️");
    
    // Digital Signatures
    println!("\n✍️ บทที่ 5: ลายเซ็นดิจิทัล - การยืนยันตัวตนแบบดิจิทัล");
    println!("🎭 เหมือนลายเซ็นจริงแต่ปลอมแปลงไม่ได้!");
    println!("{:-<60}", "");
    
    let signer = DigitalSigner::new(HashAlgorithm::Sha256);
    let keypair = signer.generate_keypair();
    
    println!("🔓 กุญแจสาธารณะ: {}", keypair.public_key.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("🔐 กุญแจส่วนตัว: {}... (เก็บเป็นความลับ!)", keypair.private_key[..8].iter().map(|b| format!("{b:02x}")).collect::<String>());
    
    let document = b"This is an important document that needs to be signed.";
    let signature = signer.sign(document, &keypair.private_key);
    
    println!("📄 เอกสาร: {}", String::from_utf8_lossy(document));
    println!("✍️ ลายเซ็นดิจิทัล: {}", signature.signature.iter().map(|b| format!("{b:02x}")).collect::<String>());
    
    // Verify signature
    let valid_signature = signer.verify(document, &signature, &keypair.public_key);
    let invalid_signature = signer.verify(b"tampered document", &signature, &keypair.public_key);
    
    println!("✅ ตรวจสอบลายเซ็นถูกต้อง: {valid_signature} 🎯");
    println!("❌ ตรวจสอบเอกสารที่ถูกแก้ไข: {invalid_signature} 🚨");
    
    // File Integrity Check
    println!("\n📁 บทที่ 6: การตรวจสอบความสมบูรณ์ของไฟล์");
    println!("🛡️ เหมือนการตรวจสอบว่าไฟล์ถูกแก้ไขหรือไม่!");
    println!("{:-<60}", "");
    
    let file_content = b"This is the content of an important file.\nIt contains sensitive data.\nWe need to ensure its integrity.";
    let file_hashes = manager.compare_algorithms(file_content);
    
    println!("📄 เนื้อหาไฟล์ ({} ไบต์):", file_content.len());
    println!("{}", String::from_utf8_lossy(file_content));
    println!("\n🔐 ลายนิ้วมือสำหรับตรวจสอบความสมบูรณ์:");
    
    for hash in file_hashes {
        println!("  🎯 {hash}");
    }
    
    // Simulate file modification
    let modified_content = b"This is the content of an important file.\nIt contains MODIFIED data.\nWe need to ensure its integrity.";
    let modified_hash = manager.hash(HashAlgorithm::Sha256, modified_content).unwrap();
    let original_hash = manager.hash(HashAlgorithm::Sha256, file_content).unwrap();
    
    println!("\n🔍 ผลการตรวจสอบความสมบูรณ์:");
    println!("📄 ไฟล์ต้นฉบับ SHA-256:  {}", original_hash.hex_string);
    println!("📝 ไฟล์ที่แก้ไข SHA-256:  {}", modified_hash.hex_string);
    println!("🎯 ไฟล์เหมือนกัน: {} {}", original_hash.hash == modified_hash.hash, if original_hash.hash == modified_hash.hash { "✅" } else { "❌" });
    
    // Performance Comparison
    println!("\n⚡ บทที่ 7: การเปรียบเทียบประสิทธิภาพ");
    println!("🏃‍♂️ มาดูกันว่าอัลกอริทึมไหนเร็วที่สุด!");
    println!("{:-<60}", "");
    
    let large_data = vec![0u8; 10000]; // 10KB of data
    println!("📊 ทดสอบกับข้อมูล 10KB:");
    
    for algorithm in [HashAlgorithm::Md5, HashAlgorithm::Sha256, HashAlgorithm::Sha512, HashAlgorithm::Blake2b] {
        let start = std::time::Instant::now();
        let result = manager.hash(algorithm.clone(), &large_data).unwrap();
        let duration = start.elapsed();
        
        println!("🚀 {:?}: {} ({}μs)", 
                algorithm, 
                &result.hex_string[..16], 
                duration.as_micros());
    }
    
    println!("\n🎉 ยินดีด้วย! คุณได้เรียนรู้เทคนิคการแฮชและลายเซ็นดิจิทัลเรียบร้อยแล้ว!");
    println!("💡 ตอนนี้คุณสามารถปกป้องข้อมูลและยืนยันตัวตนได้แล้ว! 🔐");
    println!("🛡️ จำไว้: ความปลอดภัยคือสิ่งสำคัญที่สุดในโลกดิจิทัล!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_consistency() {
        let hasher = Sha256Hasher;
        let input = b"test input";
        
        let hash1 = hasher.hash(input);
        let hash2 = hasher.hash(input);
        
        assert_eq!(hash1.hash, hash2.hash);
        assert_eq!(hash1.hex_string, hash2.hex_string);
    }
    
    #[test]
    fn test_hash_verification() {
        let hasher = Sha256Hasher;
        let input = b"test message";
        let hash_result = hasher.hash(input);
        
        assert!(hash_result.verify(input, &hasher));
        assert!(!hash_result.verify(b"different message", &hasher));
    }
    
    #[test]
    fn test_password_hashing() {
        let hasher = PasswordHasher::new(HashAlgorithm::Sha256);
        let password = "test_password";
        
        let hash1 = hasher.hash_password_with_salt(password, 1000);
        let hash2 = hasher.hash_password_with_salt(password, 1000);
        
        // Different salts should produce different hashes
        assert_ne!(hash1.hash, hash2.hash);
        
        // But both should verify correctly
        assert!(hash1.verify(password, &hasher));
        assert!(hash2.verify(password, &hasher));
    }
    
    #[test]
    fn test_hmac() {
        let hmac_gen = HmacGenerator::new(HashAlgorithm::Sha256);
        let key = b"test_key";
        let message = b"test_message";
        
        let hmac = hmac_gen.generate(key, message);
        
        assert!(hmac_gen.verify(key, message, &hmac.hash));
        assert!(!hmac_gen.verify(b"wrong_key", message, &hmac.hash));
        assert!(!hmac_gen.verify(key, b"wrong_message", &hmac.hash));
    }
    
    #[test]
    fn test_digital_signature() {
        let signer = DigitalSigner::new(HashAlgorithm::Sha256);
        let keypair = signer.generate_keypair();
        let message = b"test document";
        
        let signature = signer.sign(message, &keypair.private_key);
        
        assert!(signer.verify(message, &signature, &keypair.public_key));
        assert!(!signer.verify(b"tampered message", &signature, &keypair.public_key));
    }
}