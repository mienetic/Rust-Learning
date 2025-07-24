//! 🔐 Web Development Workshop: Authentication and Authorization Systems 🔐
//!
//! 🎯 ยินดีต้อนรับสู่เวิร์คช็อป "ระบบยืนยันตัวตนและการควบคุมการเข้าถึง"!
//! 🛡️ เรียนรู้วิธีสร้างระบบรักษาความปลอดภัยแบบมืออาชีพ
//! 🎭 เหมือนการเป็นยามรักษาความปลอดภัยของเว็บไซต์!

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fmt;

/// 👑 บทบาทผู้ใช้ - เหมือนตำแหน่งงานในบริษัท
/// 🎭 แต่ละบทบาทมีสิทธิ์และหน้าที่ที่แตกต่างกัน!
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Moderator,
    User,
    Guest,
    Custom(String),
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "Admin"),
            Self::Moderator => write!(f, "Moderator"),
            Self::User => write!(f, "User"),
            Self::Guest => write!(f, "Guest"),
            Self::Custom(name) => write!(f, "Custom({name})"),
        }
    }
}

/// 🔑 สิทธิ์การเข้าถึง - เหมือนกุญแจที่เปิดประตูต่างๆ
/// 🚪 แต่ละสิทธิ์เปิดประตูไปยังฟีเจอร์ที่แตกต่างกัน!
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Execute,
    Admin,
    CreateUser,
    DeleteUser,
    ModifyUser,
    ViewLogs,
    ManageSystem,
    Custom(String),
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Read => write!(f, "Read"),
            Self::Write => write!(f, "Write"),
            Self::Delete => write!(f, "Delete"),
            Self::Execute => write!(f, "Execute"),
            Self::Admin => write!(f, "Admin"),
            Self::CreateUser => write!(f, "CreateUser"),
            Self::DeleteUser => write!(f, "DeleteUser"),
            Self::ModifyUser => write!(f, "ModifyUser"),
            Self::ViewLogs => write!(f, "ViewLogs"),
            Self::ManageSystem => write!(f, "ManageSystem"),
            Self::Custom(name) => write!(f, "Custom({name})"),
        }
    }
}

/// 👤 บัญชีผู้ใช้ - ข้อมูลประจำตัวของสมาชิก
/// 📋 เก็บข้อมูลทุกอย่างที่จำเป็นสำหรับการยืนยันตัวตน!
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub roles: HashSet<Role>,
    pub permissions: HashSet<Permission>,
    pub is_active: bool,
    pub created_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub failed_login_attempts: u32,
    pub locked_until: Option<SystemTime>,
}

impl User {
    fn new(id: u64, username: String, email: String, password_hash: String) -> Self {
        let mut user = Self {
            id,
            username,
            email,
            password_hash,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            is_active: true,
            created_at: SystemTime::now(),
            last_login: None,
            failed_login_attempts: 0,
            locked_until: None,
        };
        
        // Default role
        user.roles.insert(Role::User);
        user.permissions.insert(Permission::Read);
        
        user
    }
    
    fn add_role(&mut self, role: Role) {
        self.roles.insert(role.clone());
        
        // Add default permissions for role
        match role {
            Role::Admin => {
                self.permissions.extend([
                    Permission::Read, Permission::Write, Permission::Delete,
                    Permission::Execute, Permission::Admin, Permission::CreateUser,
                    Permission::DeleteUser, Permission::ModifyUser, Permission::ViewLogs,
                    Permission::ManageSystem,
                ]);
            },
            Role::Moderator => {
                self.permissions.extend([
                    Permission::Read, Permission::Write, Permission::Delete,
                    Permission::ModifyUser, Permission::ViewLogs,
                ]);
            },
            Role::User => {
                self.permissions.extend([
                    Permission::Read, Permission::Write,
                ]);
            },
            Role::Guest => {
                self.permissions.insert(Permission::Read);
            },
            Role::Custom(_) => {
                // Custom roles need explicit permission assignment
            },
        }
    }
    
    fn remove_role(&mut self, role: &Role) {
        self.roles.remove(role);
    }
    
    fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }
    
    fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission) || self.permissions.contains(&Permission::Admin)
    }
    
    fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            SystemTime::now() < locked_until
        } else {
            false
        }
    }
    
    fn lock_account(&mut self, duration: Duration) {
        self.locked_until = Some(SystemTime::now() + duration);
    }
    
    const fn unlock_account(&mut self) {
        self.locked_until = None;
        self.failed_login_attempts = 0;
    }
}

/// 🎫 โทเค็นเซสชัน - ตั้วเข้าชมที่มีอายุจำกัด
/// ⏰ เหมือนตั๋วเข้าชมที่หมดอายุเมื่อครบกำหนด!
#[derive(Debug, Clone)]
pub struct SessionToken {
    pub token: String,
    pub user_id: u64,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub is_active: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl SessionToken {
    fn new(user_id: u64, duration: Duration) -> Self {
        let now = SystemTime::now();
        let token = Self::generate_token();
        
        Self {
            token,
            user_id,
            created_at: now,
            expires_at: now + duration,
            is_active: true,
            ip_address: None,
            user_agent: None,
        }
    }
    
    fn generate_token() -> String {
        // Simple token generation (in production, use crypto-secure random)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        format!("token_{timestamp:x}")
    }
    
    fn is_valid(&self) -> bool {
        self.is_active && SystemTime::now() < self.expires_at
    }
    
    fn refresh(&mut self, duration: Duration) {
        if self.is_valid() {
            self.expires_at = SystemTime::now() + duration;
        }
    }
    
    const fn revoke(&mut self) {
        self.is_active = false;
    }
}

/// Authentication Result
#[derive(Debug, Clone)]
pub enum AuthResult {
    Success(SessionToken),
    InvalidCredentials,
    AccountLocked,
    AccountDisabled,
    TooManyAttempts,
    TokenExpired,
    TokenInvalid,
}

impl fmt::Display for AuthResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success(_) => write!(f, "Authentication successful"),
            Self::InvalidCredentials => write!(f, "Invalid username or password"),
            Self::AccountLocked => write!(f, "Account is locked"),
            Self::AccountDisabled => write!(f, "Account is disabled"),
            Self::TooManyAttempts => write!(f, "Too many failed attempts"),
            Self::TokenExpired => write!(f, "Session token expired"),
            Self::TokenInvalid => write!(f, "Invalid session token"),
        }
    }
}

/// Password Hasher
struct PasswordHasher;

impl PasswordHasher {
    fn hash_password(password: &str, salt: &str) -> String {
        // Simple password hashing (in production, use bcrypt, scrypt, or argon2)
        let combined = format!("{password}{salt}");
        let mut hash = 0u64;
        
        for byte in combined.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(u64::from(byte));
        }
        
        format!("hash_{hash:x}")
    }
    
    fn verify_password(password: &str, salt: &str, hash: &str) -> bool {
        let computed_hash = Self::hash_password(password, salt);
        computed_hash == hash
    }
    
    fn generate_salt() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        format!("salt_{timestamp:x}")
    }
}

/// Authentication Manager
pub struct AuthManager {
    users: HashMap<String, User>, // username -> user
    sessions: HashMap<String, SessionToken>, // token -> session
    user_salts: HashMap<String, String>, // username -> salt
    max_failed_attempts: u32,
    lockout_duration: Duration,
    session_duration: Duration,
}

impl AuthManager {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            user_salts: HashMap::new(),
            max_failed_attempts: 5,
            lockout_duration: Duration::from_secs(300), // 5 minutes
            session_duration: Duration::from_secs(3600), // 1 hour
        }
    }
    
    fn register_user(&mut self, username: String, email: String, password: String) -> Result<u64, String> {
        if self.users.contains_key(&username) {
            return Err("Username already exists".to_string());
        }
        
        let salt = PasswordHasher::generate_salt();
        let password_hash = PasswordHasher::hash_password(&password, &salt);
        let user_id = self.users.len() as u64 + 1;
        
        let user = User::new(user_id, username.clone(), email, password_hash);
        
        self.users.insert(username.clone(), user);
        self.user_salts.insert(username, salt);
        
        Ok(user_id)
    }
    
    fn authenticate(&mut self, username: &str, password: &str) -> AuthResult {
        let user = match self.users.get_mut(username) {
            Some(user) => user,
            None => return AuthResult::InvalidCredentials,
        };
        
        if !user.is_active {
            return AuthResult::AccountDisabled;
        }
        
        if user.is_locked() {
            return AuthResult::AccountLocked;
        }
        
        let salt = match self.user_salts.get(username) {
            Some(salt) => salt,
            None => return AuthResult::InvalidCredentials,
        };
        
        if PasswordHasher::verify_password(password, salt, &user.password_hash) {
            // Successful authentication
            user.last_login = Some(SystemTime::now());
            user.failed_login_attempts = 0;
            user.unlock_account();
            
            let session = SessionToken::new(user.id, self.session_duration);
            let token = session.token.clone();
            self.sessions.insert(token, session.clone());
            
            AuthResult::Success(session)
        } else {
            // Failed authentication
            user.failed_login_attempts += 1;
            
            if user.failed_login_attempts >= self.max_failed_attempts {
                user.lock_account(self.lockout_duration);
                AuthResult::TooManyAttempts
            } else {
                AuthResult::InvalidCredentials
            }
        }
    }
    
    fn validate_session(&self, token: &str) -> AuthResult {
        match self.sessions.get(token) {
            Some(session) => {
                if session.is_valid() {
                    AuthResult::Success(session.clone())
                } else {
                    AuthResult::TokenExpired
                }
            },
            None => AuthResult::TokenInvalid,
        }
    }
    
    fn refresh_session(&mut self, token: &str) -> AuthResult {
        match self.sessions.get_mut(token) {
            Some(session) => {
                session.refresh(self.session_duration);
                AuthResult::Success(session.clone())
            },
            None => AuthResult::TokenInvalid,
        }
    }
    
    fn logout(&mut self, token: &str) -> bool {
        match self.sessions.get_mut(token) {
            Some(session) => {
                session.revoke();
                true
            },
            None => false,
        }
    }
    
    fn get_user_by_token(&self, token: &str) -> Option<&User> {
        let session = self.sessions.get(token)?;
        if !session.is_valid() {
            return None;
        }
        
        self.users.values().find(|user| user.id == session.user_id)
    }
    
    fn cleanup_expired_sessions(&mut self) {
        self.sessions.retain(|_, session| session.is_valid());
    }
    
    fn get_user_sessions(&self, user_id: u64) -> Vec<&SessionToken> {
        self.sessions.values()
            .filter(|session| session.user_id == user_id && session.is_valid())
            .collect()
    }
    
    fn revoke_all_user_sessions(&mut self, user_id: u64) {
        for session in self.sessions.values_mut() {
            if session.user_id == user_id {
                session.revoke();
            }
        }
    }
}

/// 🛡️ ผู้จัดการการอนุญาต - ระบบรักษาความปลอดภัยของเวิร์กช็อป
/// Authorization Manager - ผู้ดูแลสิทธิ์การเข้าถึงทรัพยากรต่างๆ ในเวิร์กช็อป
pub struct AuthzManager {
    role_permissions: HashMap<Role, HashSet<Permission>>,
    resource_permissions: HashMap<String, HashSet<Permission>>,
}

impl AuthzManager {
    fn new() -> Self {
        let mut manager = Self {
            role_permissions: HashMap::new(),
            resource_permissions: HashMap::new(),
        };
        
        manager.setup_default_permissions();
        manager
    }
    
    fn setup_default_permissions(&mut self) {
        // Admin permissions
        let admin_perms = HashSet::from([
            Permission::Read, Permission::Write, Permission::Delete,
            Permission::Execute, Permission::Admin, Permission::CreateUser,
            Permission::DeleteUser, Permission::ModifyUser, Permission::ViewLogs,
            Permission::ManageSystem,
        ]);
        self.role_permissions.insert(Role::Admin, admin_perms);
        
        // Moderator permissions
        let mod_perms = HashSet::from([
            Permission::Read, Permission::Write, Permission::Delete,
            Permission::ModifyUser, Permission::ViewLogs,
        ]);
        self.role_permissions.insert(Role::Moderator, mod_perms);
        
        // User permissions
        let user_perms = HashSet::from([
            Permission::Read, Permission::Write,
        ]);
        self.role_permissions.insert(Role::User, user_perms);
        
        // Guest permissions
        let guest_perms = HashSet::from([Permission::Read]);
        self.role_permissions.insert(Role::Guest, guest_perms);
    }
    
    fn check_permission(&self, user: &User, permission: &Permission) -> bool {
        // Check direct user permissions
        if user.has_permission(permission) {
            return true;
        }
        
        // Check role-based permissions
        for role in &user.roles {
            if let Some(role_perms) = self.role_permissions.get(role) {
                if role_perms.contains(permission) || role_perms.contains(&Permission::Admin) {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn check_resource_access(&self, user: &User, resource: &str, permission: &Permission) -> bool {
        // Check if user has general permission
        if self.check_permission(user, permission) {
            return true;
        }
        
        // Check resource-specific permissions
        if let Some(resource_perms) = self.resource_permissions.get(resource) {
            return resource_perms.contains(permission);
        }
        
        false
    }
    
    fn grant_resource_permission(&mut self, resource: String, permission: Permission) {
        self.resource_permissions
            .entry(resource)
            .or_default()
            .insert(permission);
    }
    
    fn revoke_resource_permission(&mut self, resource: &str, permission: &Permission) {
        if let Some(perms) = self.resource_permissions.get_mut(resource) {
            perms.remove(permission);
        }
    }
}

/// 🔐 การยืนยันตัวตนหลายขั้นตอน - ระบบรักษาความปลอดภัยขั้นสูง
/// Multi-Factor Authentication - ระบบล็อคสองชั้นเพื่อความปลอดภัยสูงสุด
#[derive(Debug, Clone)]
pub struct MfaToken {
    pub code: String,
    pub user_id: u64,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub used: bool,
}

impl MfaToken {
    fn new(user_id: u64) -> Self {
        let now = SystemTime::now();
        let code = Self::generate_code();
        
        Self {
            code,
            user_id,
            created_at: now,
            expires_at: now + Duration::from_secs(300), // 5 minutes
            used: false,
        }
    }
    
    fn generate_code() -> String {
        // Generate 6-digit code
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        format!("{:06}", timestamp % 1000000)
    }
    
    fn is_valid(&self) -> bool {
        !self.used && SystemTime::now() < self.expires_at
    }
    
    fn use_token(&mut self) -> bool {
        if self.is_valid() {
            self.used = true;
            true
        } else {
            false
        }
    }
}

/// 🔑 ผู้จัดการ MFA - ระบบจัดการโทเค็นยืนยันตัวตน
pub struct MfaManager {
    tokens: HashMap<u64, Vec<MfaToken>>, // user_id -> tokens
    enabled_users: HashSet<u64>,
}

impl MfaManager {
    fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            enabled_users: HashSet::new(),
        }
    }
    
    fn enable_mfa(&mut self, user_id: u64) {
        self.enabled_users.insert(user_id);
    }
    
    fn disable_mfa(&mut self, user_id: u64) {
        self.enabled_users.remove(&user_id);
        self.tokens.remove(&user_id);
    }
    
    fn is_enabled(&self, user_id: u64) -> bool {
        self.enabled_users.contains(&user_id)
    }
    
    fn generate_token(&mut self, user_id: u64) -> Option<MfaToken> {
        if !self.is_enabled(user_id) {
            return None;
        }
        
        let token = MfaToken::new(user_id);
        let code = token.code.clone();
        
        self.tokens
            .entry(user_id)
            .or_default()
            .push(token.clone());
        
        // Clean up old tokens
        self.cleanup_expired_tokens(user_id);
        
        Some(token)
    }
    
    fn verify_token(&mut self, user_id: u64, code: &str) -> bool {
        if let Some(tokens) = self.tokens.get_mut(&user_id) {
            for token in tokens.iter_mut() {
                if token.code == code && token.use_token() {
                    return true;
                }
            }
        }
        false
    }
    
    fn cleanup_expired_tokens(&mut self, user_id: u64) {
        if let Some(tokens) = self.tokens.get_mut(&user_id) {
            tokens.retain(MfaToken::is_valid);
        }
    }
}

/// 🏛️ ระบบยืนยันตัวตนแบบครบครัน - ศูนย์รวมความปลอดภัยของเวิร์กช็อป
/// Complete Authentication System - ระบบรักษาความปลอดภัยแบบองค์รวม
pub struct AuthenticationSystem {
    auth_manager: AuthManager,
    authz_manager: AuthzManager,
    mfa_manager: MfaManager,
}

impl AuthenticationSystem {
    fn new() -> Self {
        Self {
            auth_manager: AuthManager::new(),
            authz_manager: AuthzManager::new(),
            mfa_manager: MfaManager::new(),
        }
    }
    
    fn register_user(&mut self, username: String, email: String, password: String) -> Result<u64, String> {
        self.auth_manager.register_user(username, email, password)
    }
    
    fn login(&mut self, username: &str, password: &str, mfa_code: Option<&str>) -> AuthResult {
        let auth_result = self.auth_manager.authenticate(username, password);
        
        match auth_result {
            AuthResult::Success(session) => {
                // Check if MFA is required
                if self.mfa_manager.is_enabled(session.user_id) {
                    if let Some(code) = mfa_code {
                        if self.mfa_manager.verify_token(session.user_id, code) {
                            AuthResult::Success(session)
                        } else {
                            // Revoke the session since MFA failed
                            self.auth_manager.logout(&session.token);
                            AuthResult::InvalidCredentials
                        }
                    } else {
                        // MFA required but not provided
                        self.auth_manager.logout(&session.token);
                        AuthResult::InvalidCredentials
                    }
                } else {
                    AuthResult::Success(session)
                }
            },
            other => other,
        }
    }
    
    fn check_access(&self, token: &str, resource: &str, permission: Permission) -> bool {
        if let Some(user) = self.auth_manager.get_user_by_token(token) {
            self.authz_manager.check_resource_access(user, resource, &permission)
        } else {
            false
        }
    }
    
    fn enable_mfa(&mut self, user_id: u64) {
        self.mfa_manager.enable_mfa(user_id);
    }
    
    fn generate_mfa_token(&mut self, user_id: u64) -> Option<MfaToken> {
        self.mfa_manager.generate_token(user_id)
    }
    
    fn get_user_by_token(&self, token: &str) -> Option<&User> {
        self.auth_manager.get_user_by_token(token)
    }
    
    fn logout(&mut self, token: &str) -> bool {
        self.auth_manager.logout(token)
    }
    
    fn cleanup_expired_sessions(&mut self) {
        self.auth_manager.cleanup_expired_sessions();
    }
}

/// 🎭 สาธิตระบบยืนยันตัวตนและการอนุญาต - เวิร์กช็อปความปลอดภัยดิจิทัล
pub fn demonstrate_authentication() {
    println!("🏛️ === เวิร์กช็อประบบรักษาความปลอดภัยดิจิทัล === 🔐");
    println!("🎯 สาธิตการยืนยันตัวตนและการจัดการสิทธิ์การเข้าถึง");
    
    let mut auth_system = AuthenticationSystem::new();
    
    // User Registration
    println!("\n👥 === การลงทะเบียนผู้ใช้งาน === 📝");
    println!("🎪 ขั้นตอนการสมัครสมาชิกเข้าร่วมเวิร์กช็อป");
    println!("{:-<50}", "");
    
    let users = [
        ("alice", "alice@example.com", "password123"),
        ("bob", "bob@example.com", "securepass456"),
        ("charlie", "charlie@example.com", "mypassword789"),
    ];
    
    let mut user_ids = Vec::new();
    
    for (username, email, password) in users {
        match auth_system.register_user(username.to_string(), email.to_string(), password.to_string()) {
            Ok(user_id) => {
                println!("✅ ลงทะเบียนผู้ใช้ '{username}' สำเร็จ! 🎉 ID: {user_id}");
                user_ids.push(user_id);
            },
            Err(e) => println!("❌ ลงทะเบียน '{username}' ล้มเหลว: {e}"),
        }
    }
    
    // Set up roles and permissions
    println!("\n🎭 === การกำหนดบทบาทและสิทธิ์ === 👑");
    println!("🏢 จัดตำแหน่งงานในองค์กรเวิร์กช็อป");
    println!("{:-<50}", "");
    
    // Make alice an admin
    if let Some(alice) = auth_system.auth_manager.users.get_mut("alice") {
        alice.add_role(Role::Admin);
        println!("👑 Alice ได้รับตำแหน่ง Admin - ผู้ดูแลระบบสูงสุด!");
    }
    
    // Make bob a moderator
    if let Some(bob) = auth_system.auth_manager.users.get_mut("bob") {
        bob.add_role(Role::Moderator);
        println!("🛡️ Bob ได้รับตำแหน่ง Moderator - ผู้ดูแลเนื้อหา!");
    }
    
    // Charlie remains a regular user
    println!("👤 Charlie คือ User ทั่วไป - สมาชิกเวิร์กช็อป!");
    
    // Authentication Tests
    println!("\n🔑 === การทดสอบระบบยืนยันตัวตน === 🎪");
    println!("🎯 ทดสอบการเข้าสู่ระบบและความปลอดภัย");
    println!("{:-<50}", "");
    
    // Successful login
    match auth_system.login("alice", "password123", None) {
        AuthResult::Success(session) => {
            println!("✅ Alice เข้าสู่ระบบสำเร็จ! 🎉");
            println!("   🎫 โทเค็น: {}", session.token);
            println!("   ⏰ หมดอายุ: {:?}", session.expires_at);
            
            // Test authorization
            println!("\n🛡️ === การทดสอบระบบอนุญาต === 🔐");
            println!("🎯 ทดสอบสิทธิ์การเข้าถึงทรัพยากรต่างๆ");
            println!("{:-<50}", "");
            
            let permissions_to_test = [
                (Permission::Read, "documents"),
                (Permission::Write, "documents"),
                (Permission::Delete, "documents"),
                (Permission::Admin, "system"),
                (Permission::CreateUser, "users"),
            ];
            
            for (permission, resource) in permissions_to_test {
                let has_access = auth_system.check_access(&session.token, resource, permission.clone());
                let status = if has_access { "✅" } else { "❌" };
                println!("{status} Alice สามารถ {permission} บน {resource}: {has_access}");
            }
        },
        result => println!("❌ Alice เข้าสู่ระบบล้มเหลว: {result}"),
    }
    
    // Failed login
    match auth_system.login("alice", "wrongpassword", None) {
        AuthResult::InvalidCredentials => println!("✅ ปฏิเสธรหัสผ่านผิดอย่างถูกต้อง! 🚫"),
        result => println!("❌ ผลลัพธ์ไม่คาดคิด: {result}"),
    }
    
    // Test account lockout
    println!("\n🔒 === การทดสอบการล็อคบัญชี === 🚨");
    println!("⚠️ ทดสอบระบบป้องกันการเข้าถึงที่ผิดปกติ");
    println!("{:-<50}", "");
    
    for attempt in 1..=6 {
        match auth_system.login("bob", "wrongpassword", None) {
            AuthResult::InvalidCredentials => {
                println!("❌ ครั้งที่ {attempt}: ข้อมูลประจำตัวไม่ถูกต้อง");
            },
            AuthResult::TooManyAttempts => {
                println!("🔒 ครั้งที่ {attempt}: บัญชีถูกล็อคเนื่องจากพยายามเข้าถึงมากเกินไป! 🚨");
                break;
            },
            result => println!("❌ ครั้งที่ {attempt}: ผลลัพธ์ไม่คาดคิด: {result}"),
        }
    }
    
    // Multi-Factor Authentication
    println!("\n🔐 === การยืนยันตัวตนหลายขั้นตอน === 📱");
    println!("🛡️ ระบบรักษาความปลอดภัยขั้นสูงด้วย MFA");
    println!("{:-<50}", "");
    
    // Enable MFA for Charlie
    if let Some(&charlie_id) = user_ids.get(2) {
        auth_system.enable_mfa(charlie_id);
        println!("✅ เปิดใช้งาน MFA สำหรับ Charlie แล้ว! 🔐 (ID: {charlie_id})");
        
        // Generate MFA token
        if let Some(mfa_token) = auth_system.generate_mfa_token(charlie_id) {
            println!("📱 สร้างรหัส MFA แล้ว: {}", mfa_token.code);
            
            // Try login without MFA
            match auth_system.login("charlie", "mypassword789", None) {
                AuthResult::InvalidCredentials => {
                    println!("❌ เข้าสู่ระบบล้มเหลวเนื่องจากไม่มีรหัส MFA (คาดหวัง) 🚫");
                },
                result => println!("❌ ผลลัพธ์ไม่คาดคิด: {result}"),
            }
            
            // Try login with wrong MFA
            match auth_system.login("charlie", "mypassword789", Some("000000")) {
                AuthResult::InvalidCredentials => {
                    println!("❌ เข้าสู่ระบบล้มเหลวด้วยรหัส MFA ผิด (คาดหวัง) 🔢");
                },
                result => println!("❌ ผลลัพธ์ไม่คาดคิด: {result}"),
            }
            
            // Try login with correct MFA
            match auth_system.login("charlie", "mypassword789", Some(&mfa_token.code)) {
                AuthResult::Success(session) => {
                    println!("✅ Charlie เข้าสู่ระบบสำเร็จด้วย MFA! 🎉");
                    println!("   🎫 โทเค็น: {}", session.token);
                },
                result => println!("❌ ผลลัพธ์ไม่คาดคิด: {result}"),
            }
        }
    }
    
    // Session Management
    println!("\n📋 === การจัดการเซสชัน === 🎪");
    println!("🎯 ระบบติดตามและจัดการการเข้าใช้งาน");
    println!("{:-<50}", "");
    
    // Login multiple users
    let mut active_sessions = Vec::new();
    
    for (username, _, password) in &users[..2] { // Alice and Bob
        match auth_system.login(username, password, None) {
            AuthResult::Success(session) => {
                println!("✅ {} เข้าสู่ระบบแล้ว 🎉 โทเค็น: {}", username, session.token);
                active_sessions.push(((*username).to_string(), session.token));
            },
            AuthResult::AccountLocked => {
                println!("🔒 บัญชี {username} ถูกล็อค! 🚨");
            },
            result => println!("❌ {username} เข้าสู่ระบบล้มเหลว: {result}"),
        }
    }
    
    // Show active sessions
    println!("\n📊 เซสชันที่ใช้งานอยู่:");
    for (username, token) in &active_sessions {
        if let Some(user) = auth_system.get_user_by_token(token) {
            println!("  {}: {} (ID: {}, บทบาท: {:?})", 
                    username, 
                    &token[..20], 
                    user.id, 
                    user.roles.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(", "));
        }
    }
    
    // Logout
    if let Some((username, token)) = active_sessions.first() {
        if auth_system.logout(token) {
            println!("✅ {username} ออกจากระบบสำเร็จ! 👋");
        } else {
            println!("❌ {username} ออกจากระบบล้มเหลว");
        }
    }
    
    // Resource-based Authorization
    println!("\n🗂️ === การอนุญาตตามทรัพยากร === 📁");
    println!("🎯 ระบบควบคุมการเข้าถึงทรัพยากรเฉพาะ");
    println!("{:-<50}", "");
    
    // Grant specific resource permissions
    auth_system.authz_manager.grant_resource_permission(
        "sensitive_documents".to_string(), 
        Permission::Read
    );
    
    auth_system.authz_manager.grant_resource_permission(
        "admin_panel".to_string(), 
        Permission::Admin
    );
    
    // Test resource access
    if let Some((_, token)) = active_sessions.get(1) { // Bob's session
        let resources = [
            ("documents", Permission::Read),
            ("documents", Permission::Write),
            ("sensitive_documents", Permission::Read),
            ("admin_panel", Permission::Admin),
        ];
        
        for (resource, permission) in resources {
            let has_access = auth_system.check_access(token, resource, permission.clone());
            let status = if has_access { "✅" } else { "❌" };
            println!("{status} เข้าถึง {resource} ด้วยสิทธิ์ {permission}: {has_access}");
        }
    }
    
    // Cleanup
    println!("\n🧹 === การทำความสะอาด === 🗑️");
    println!("🎯 ล้างข้อมูลเซสชันที่หมดอายุ");
    println!("{:-<50}", "");
    
    auth_system.cleanup_expired_sessions();
    println!("✅ ล้างเซสชันที่หมดอายุแล้ว! 🧽");
    
    println!("\n🎉 === เวิร์กช็อประบบรักษาความปลอดภัยเสร็จสิ้น! === 🏆");
    println!("🎯 สาธิตระบบยืนยันตัวตนและการอนุญาตครบถ้วนแล้ว!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "test".to_string(), "test@example.com".to_string(), "hash".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "test");
        assert!(user.has_role(&Role::User));
        assert!(user.has_permission(&Permission::Read));
    }
    
    #[test]
    fn test_role_permissions() {
        let mut user = User::new(1, "test".to_string(), "test@example.com".to_string(), "hash".to_string());
        user.add_role(Role::Admin);
        
        assert!(user.has_role(&Role::Admin));
        assert!(user.has_permission(&Permission::Admin));
        assert!(user.has_permission(&Permission::CreateUser));
    }
    
    #[test]
    fn test_session_token() {
        let mut token = SessionToken::new(1, Duration::from_secs(3600));
        assert!(token.is_valid());
        
        token.revoke();
        assert!(!token.is_valid());
    }
    
    #[test]
    fn test_password_hashing() {
        let password = "test_password";
        let salt = "test_salt";
        
        let hash1 = PasswordHasher::hash_password(password, salt);
        let hash2 = PasswordHasher::hash_password(password, salt);
        
        assert_eq!(hash1, hash2);
        assert!(PasswordHasher::verify_password(password, salt, &hash1));
        assert!(!PasswordHasher::verify_password("wrong", salt, &hash1));
    }
    
    #[test]
    fn test_mfa_token() {
        let mut token = MfaToken::new(1);
        assert!(token.is_valid());
        
        assert!(token.use_token());
        assert!(!token.is_valid()); // Should be invalid after use
        assert!(!token.use_token()); // Should fail second use
    }
}