//! 💾 Mobile Data Storage
//! 
//! สาธิตการจัดการข้อมูลในแอปพลิเคชันมือถือ
//! รวมถึง Local Storage, Cloud Storage, Caching, และ Data Synchronization

use std::collections::{HashMap, BTreeMap};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::{Arc, Mutex};
use std::fmt;

/// 📁 Storage Type
#[derive(Debug, Clone, PartialEq)]
pub enum StorageType {
    UserDefaults,    // iOS NSUserDefaults / Android SharedPreferences
    Keychain,        // iOS Keychain / Android Keystore
    SQLite,          // Local database
    CoreData,        // iOS Core Data
    Room,            // Android Room
    FileSystem,      // Local file storage
    CloudKit,        // iOS CloudKit
    FirebaseFirestore, // Firebase Firestore
    RealmDB,         // Realm Database
    AsyncStorage,    // React Native AsyncStorage
}

/// 🔐 Data Security Level
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    None,           // No encryption
    Basic,          // Basic encryption
    Standard,       // Standard encryption
    High,           // High-level encryption
    Biometric,      // Biometric protection
}

/// 🌐 Sync Strategy
#[derive(Debug, Clone, PartialEq)]
pub enum SyncStrategy {
    Manual,         // Manual sync only
    Automatic,      // Auto sync when possible
    RealTime,       // Real-time sync
    Periodic,       // Periodic sync
    OnDemand,       // Sync on demand
    Conflict,       // Conflict resolution
}

/// 📊 Data Model
#[derive(Debug, Clone)]
pub struct DataModel {
    pub id: String,
    pub data: serde_json::Value,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: u32,
    pub is_synced: bool,
    pub is_deleted: bool,
    pub metadata: HashMap<String, String>,
}

impl DataModel {
    pub fn new(id: String, data: serde_json::Value) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            id,
            data,
            created_at: timestamp,
            updated_at: timestamp,
            version: 1,
            is_synced: false,
            is_deleted: false,
            metadata: HashMap::new(),
        }
    }
    
    pub fn update(&mut self, data: serde_json::Value) {
        self.data = data;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.version += 1;
        self.is_synced = false;
    }
    
    pub fn mark_synced(&mut self) {
        self.is_synced = true;
    }
    
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.version += 1;
        self.is_synced = false;
    }
}

/// 🗄️ Storage Manager
#[derive(Debug)]
pub struct StorageManager {
    storage_type: StorageType,
    security_level: SecurityLevel,
    encryption_key: Option<String>,
    data_store: HashMap<String, DataModel>,
    cache: HashMap<String, CachedData>,
    max_cache_size: usize,
    auto_cleanup: bool,
    compression_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CachedData {
    pub data: DataModel,
    pub access_count: u32,
    pub last_accessed: u64,
    pub expires_at: Option<u64>,
    pub size_bytes: usize,
}

impl StorageManager {
    pub fn new(storage_type: StorageType, security_level: SecurityLevel) -> Self {
        Self {
            storage_type,
            security_level,
            encryption_key: None,
            data_store: HashMap::new(),
            cache: HashMap::new(),
            max_cache_size: 50 * 1024 * 1024, // 50 MB
            auto_cleanup: true,
            compression_enabled: true,
        }
    }
    
    pub fn set_encryption_key(&mut self, key: String) {
        self.encryption_key = Some(key);
        println!("🔐 Encryption key set for {:?} storage", self.storage_type);
    }
    
    pub fn store(&mut self, key: String, data: serde_json::Value) -> Result<(), StorageError> {
        // Validate security requirements
        if self.security_level != SecurityLevel::None && self.encryption_key.is_none() {
            return Err(StorageError::EncryptionRequired);
        }
        
        // Create data model
        let mut model = DataModel::new(key.clone(), data);
        
        // Add metadata based on storage type
        match self.storage_type {
            StorageType::Keychain => {
                model.metadata.insert("access_group".to_string(), "app.group".to_string());
                model.metadata.insert("accessible".to_string(), "when_unlocked".to_string());
            }
            StorageType::SQLite | StorageType::Room => {
                model.metadata.insert("table".to_string(), "user_data".to_string());
                model.metadata.insert("indexed".to_string(), "true".to_string());
            }
            StorageType::CloudKit | StorageType::FirebaseFirestore => {
                model.metadata.insert("collection".to_string(), "user_documents".to_string());
                model.metadata.insert("public".to_string(), "false".to_string());
            }
            _ => {}
        }
        
        // Encrypt if required
        if self.security_level != SecurityLevel::None {
            self.encrypt_data(&mut model)?;
        }
        
        // Compress if enabled
        if self.compression_enabled {
            self.compress_data(&mut model)?;
        }
        
        // Store data
        self.data_store.insert(key.clone(), model.clone());
        
        // Add to cache
        self.add_to_cache(key.clone(), model);
        
        println!("💾 Stored data with key: {} using {:?}", key, self.storage_type);
        Ok(())
    }
    
    pub fn retrieve(&mut self, key: &str) -> Result<Option<serde_json::Value>, StorageError> {
        // Check cache first
        if let Some(cached) = self.cache.get_mut(key) {
            cached.access_count += 1;
            cached.last_accessed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            println!("🚀 Retrieved from cache: {}", key);
            return Ok(Some(cached.data.data.clone()));
        }
        
        // Retrieve from storage
        if let Some(mut model) = self.data_store.get(key).cloned() {
            // Decompress if needed
            if self.compression_enabled {
                self.decompress_data(&mut model)?;
            }
            
            // Decrypt if needed
            if self.security_level != SecurityLevel::None {
                self.decrypt_data(&mut model)?;
            }
            
            // Add to cache
            self.add_to_cache(key.to_string(), model.clone());
            
            println!("📁 Retrieved from storage: {}", key);
            Ok(Some(model.data))
        } else {
            Ok(None)
        }
    }
    
    pub fn update(&mut self, key: String, data: serde_json::Value) -> Result<(), StorageError> {
        if let Some(model) = self.data_store.get_mut(&key) {
            model.update(data.clone());
            
            // Update cache
            if let Some(cached) = self.cache.get_mut(&key) {
                cached.data.update(data);
            }
            
            println!("✏️ Updated data: {}", key);
            Ok(())
        } else {
            Err(StorageError::KeyNotFound)
        }
    }
    
    pub fn delete(&mut self, key: &str) -> Result<(), StorageError> {
        if let Some(model) = self.data_store.get_mut(key) {
            model.soft_delete();
            
            // Remove from cache
            self.cache.remove(key);
            
            println!("🗑️ Deleted data: {}", key);
            Ok(())
        } else {
            Err(StorageError::KeyNotFound)
        }
    }
    
    pub fn clear_all(&mut self) {
        self.data_store.clear();
        self.cache.clear();
        println!("🧹 Cleared all data from {:?} storage", self.storage_type);
    }
    
    pub fn get_storage_info(&self) -> StorageInfo {
        let total_items = self.data_store.len();
        let cached_items = self.cache.len();
        let total_size: usize = self.cache.values()
            .map(|cached| cached.size_bytes)
            .sum();
        
        let synced_items = self.data_store.values()
            .filter(|model| model.is_synced)
            .count();
        
        let deleted_items = self.data_store.values()
            .filter(|model| model.is_deleted)
            .count();
        
        StorageInfo {
            storage_type: self.storage_type.clone(),
            security_level: self.security_level.clone(),
            total_items,
            cached_items,
            synced_items,
            deleted_items,
            total_size_bytes: total_size,
            cache_hit_ratio: self.calculate_cache_hit_ratio(),
            encryption_enabled: self.encryption_key.is_some(),
            compression_enabled: self.compression_enabled,
        }
    }
    
    fn add_to_cache(&mut self, key: String, model: DataModel) {
        let size_bytes = self.estimate_size(&model);
        
        // Check cache size limit
        let current_cache_size: usize = self.cache.values()
            .map(|cached| cached.size_bytes)
            .sum();
        
        if current_cache_size + size_bytes > self.max_cache_size {
            self.evict_cache_items(size_bytes);
        }
        
        let cached_data = CachedData {
            data: model,
            access_count: 1,
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            expires_at: None,
            size_bytes,
        };
        
        self.cache.insert(key, cached_data);
    }
    
    fn evict_cache_items(&mut self, needed_size: usize) {
        // LRU eviction strategy
        let mut items: Vec<_> = self.cache.iter().collect();
        items.sort_by_key(|(_, cached)| cached.last_accessed);
        
        let mut freed_size = 0;
        let mut keys_to_remove = Vec::new();
        
        for (key, cached) in items {
            if freed_size >= needed_size {
                break;
            }
            
            freed_size += cached.size_bytes;
            keys_to_remove.push(key.clone());
        }
        
        for key in keys_to_remove {
            self.cache.remove(&key);
        }
        
        println!("🧹 Evicted cache items, freed {} bytes", freed_size);
    }
    
    fn estimate_size(&self, model: &DataModel) -> usize {
        // Rough estimation of data size
        model.data.to_string().len() + 
        model.id.len() + 
        model.metadata.iter()
            .map(|(k, v)| k.len() + v.len())
            .sum::<usize>() + 
        64 // overhead
    }
    
    fn calculate_cache_hit_ratio(&self) -> f32 {
        if self.cache.is_empty() {
            return 0.0;
        }
        
        let total_accesses: u32 = self.cache.values()
            .map(|cached| cached.access_count)
            .sum();
        
        let cache_hits = total_accesses.saturating_sub(self.cache.len() as u32);
        
        if total_accesses > 0 {
            cache_hits as f32 / total_accesses as f32
        } else {
            0.0
        }
    }
    
    fn encrypt_data(&self, model: &mut DataModel) -> Result<(), StorageError> {
        // Simulate encryption
        match self.security_level {
            SecurityLevel::Basic => {
                println!("🔒 Applied basic encryption");
            }
            SecurityLevel::Standard => {
                println!("🔒 Applied AES-256 encryption");
            }
            SecurityLevel::High => {
                println!("🔒 Applied AES-256 with key derivation");
            }
            SecurityLevel::Biometric => {
                println!("🔒 Applied biometric-protected encryption");
            }
            SecurityLevel::None => {}
        }
        
        model.metadata.insert("encrypted".to_string(), "true".to_string());
        model.metadata.insert("encryption_level".to_string(), format!("{:?}", self.security_level));
        
        Ok(())
    }
    
    fn decrypt_data(&self, model: &mut DataModel) -> Result<(), StorageError> {
        if model.metadata.get("encrypted") == Some(&"true".to_string()) {
            println!("🔓 Decrypted data for key: {}", model.id);
        }
        Ok(())
    }
    
    fn compress_data(&self, model: &mut DataModel) -> Result<(), StorageError> {
        // Simulate compression
        let original_size = self.estimate_size(model);
        let compressed_size = (original_size as f32 * 0.7) as usize; // 30% compression
        
        model.metadata.insert("compressed".to_string(), "true".to_string());
        model.metadata.insert("original_size".to_string(), original_size.to_string());
        model.metadata.insert("compressed_size".to_string(), compressed_size.to_string());
        
        println!("🗜️ Compressed data: {} -> {} bytes", original_size, compressed_size);
        Ok(())
    }
    
    fn decompress_data(&self, model: &mut DataModel) -> Result<(), StorageError> {
        if model.metadata.get("compressed") == Some(&"true".to_string()) {
            println!("📦 Decompressed data for key: {}", model.id);
        }
        Ok(())
    }
    
    pub fn cleanup_deleted_items(&mut self) {
        let before_count = self.data_store.len();
        
        self.data_store.retain(|_, model| !model.is_deleted);
        
        let after_count = self.data_store.len();
        let cleaned_count = before_count - after_count;
        
        if cleaned_count > 0 {
            println!("🧹 Cleaned up {} deleted items", cleaned_count);
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub storage_type: StorageType,
    pub security_level: SecurityLevel,
    pub total_items: usize,
    pub cached_items: usize,
    pub synced_items: usize,
    pub deleted_items: usize,
    pub total_size_bytes: usize,
    pub cache_hit_ratio: f32,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
}

/// ❌ Storage Errors
#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    KeyNotFound,
    EncryptionRequired,
    DecryptionFailed,
    CompressionFailed,
    DecompressionFailed,
    StorageFull,
    PermissionDenied,
    NetworkError,
    SyncConflict,
    InvalidData,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::KeyNotFound => write!(f, "Key not found"),
            StorageError::EncryptionRequired => write!(f, "Encryption required but no key provided"),
            StorageError::DecryptionFailed => write!(f, "Failed to decrypt data"),
            StorageError::CompressionFailed => write!(f, "Failed to compress data"),
            StorageError::DecompressionFailed => write!(f, "Failed to decompress data"),
            StorageError::StorageFull => write!(f, "Storage is full"),
            StorageError::PermissionDenied => write!(f, "Permission denied"),
            StorageError::NetworkError => write!(f, "Network error"),
            StorageError::SyncConflict => write!(f, "Synchronization conflict"),
            StorageError::InvalidData => write!(f, "Invalid data format"),
        }
    }
}

impl std::error::Error for StorageError {}

/// ☁️ Cloud Sync Manager
#[derive(Debug)]
pub struct CloudSyncManager {
    sync_strategy: SyncStrategy,
    local_storage: Arc<Mutex<StorageManager>>,
    pending_uploads: Vec<String>,
    pending_downloads: Vec<String>,
    conflict_resolution: ConflictResolution,
    last_sync: Option<u64>,
    sync_interval: Duration,
    is_syncing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConflictResolution {
    LocalWins,      // Local changes take precedence
    RemoteWins,     // Remote changes take precedence
    LastWriteWins,  // Most recent change wins
    Manual,         // Manual conflict resolution
    Merge,          // Attempt to merge changes
}

impl CloudSyncManager {
    pub fn new(local_storage: Arc<Mutex<StorageManager>>, sync_strategy: SyncStrategy) -> Self {
        Self {
            sync_strategy,
            local_storage,
            pending_uploads: Vec::new(),
            pending_downloads: Vec::new(),
            conflict_resolution: ConflictResolution::LastWriteWins,
            last_sync: None,
            sync_interval: Duration::from_secs(300), // 5 minutes
            is_syncing: false,
        }
    }
    
    pub fn set_conflict_resolution(&mut self, resolution: ConflictResolution) {
        self.conflict_resolution = resolution;
        println!("🔄 Set conflict resolution strategy: {:?}", resolution);
    }
    
    pub fn sync_now(&mut self) -> Result<SyncResult, StorageError> {
        if self.is_syncing {
            return Err(StorageError::NetworkError);
        }
        
        self.is_syncing = true;
        println!("☁️ Starting cloud synchronization...");
        
        let mut result = SyncResult {
            uploaded_count: 0,
            downloaded_count: 0,
            conflicts_resolved: 0,
            errors: Vec::new(),
            sync_duration: Duration::new(0, 0),
        };
        
        let start_time = SystemTime::now();
        
        // Simulate sync process
        match self.sync_strategy {
            SyncStrategy::Manual => {
                result = self.perform_manual_sync()?;
            }
            SyncStrategy::Automatic => {
                result = self.perform_automatic_sync()?;
            }
            SyncStrategy::RealTime => {
                result = self.perform_realtime_sync()?;
            }
            SyncStrategy::Periodic => {
                if self.should_sync() {
                    result = self.perform_periodic_sync()?;
                }
            }
            SyncStrategy::OnDemand => {
                result = self.perform_ondemand_sync()?;
            }
            SyncStrategy::Conflict => {
                result = self.resolve_conflicts()?;
            }
        }
        
        result.sync_duration = start_time.elapsed().unwrap_or_default();
        self.last_sync = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        
        self.is_syncing = false;
        
        println!("✅ Sync completed: {} uploaded, {} downloaded, {} conflicts resolved", 
                result.uploaded_count, result.downloaded_count, result.conflicts_resolved);
        
        Ok(result)
    }
    
    fn should_sync(&self) -> bool {
        if let Some(last_sync) = self.last_sync {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            now - last_sync >= self.sync_interval.as_secs()
        } else {
            true // First sync
        }
    }
    
    fn perform_manual_sync(&mut self) -> Result<SyncResult, StorageError> {
        println!("📋 Performing manual sync");
        
        let mut result = SyncResult::default();
        
        // Upload pending changes
        for key in &self.pending_uploads {
            println!("   ⬆️ Uploading: {}", key);
            result.uploaded_count += 1;
        }
        self.pending_uploads.clear();
        
        // Download remote changes
        for key in &self.pending_downloads {
            println!("   ⬇️ Downloading: {}", key);
            result.downloaded_count += 1;
        }
        self.pending_downloads.clear();
        
        Ok(result)
    }
    
    fn perform_automatic_sync(&self) -> Result<SyncResult, StorageError> {
        println!("🤖 Performing automatic sync");
        
        let mut result = SyncResult::default();
        
        // Simulate automatic detection of changes
        result.uploaded_count = 3;
        result.downloaded_count = 2;
        
        println!("   📊 Auto-detected {} local changes", result.uploaded_count);
        println!("   📊 Auto-detected {} remote changes", result.downloaded_count);
        
        Ok(result)
    }
    
    fn perform_realtime_sync(&self) -> Result<SyncResult, StorageError> {
        println!("⚡ Performing real-time sync");
        
        let mut result = SyncResult::default();
        
        // Real-time sync processes changes immediately
        result.uploaded_count = 1;
        result.downloaded_count = 1;
        
        println!("   🔄 Real-time sync active");
        
        Ok(result)
    }
    
    fn perform_periodic_sync(&self) -> Result<SyncResult, StorageError> {
        println!("⏰ Performing periodic sync");
        
        let mut result = SyncResult::default();
        
        // Batch sync at intervals
        result.uploaded_count = 5;
        result.downloaded_count = 3;
        
        println!("   📦 Batched {} changes for upload", result.uploaded_count);
        println!("   📦 Batched {} changes for download", result.downloaded_count);
        
        Ok(result)
    }
    
    fn perform_ondemand_sync(&self) -> Result<SyncResult, StorageError> {
        println!("🎯 Performing on-demand sync");
        
        let mut result = SyncResult::default();
        
        // Sync specific items on demand
        result.uploaded_count = 2;
        result.downloaded_count = 1;
        
        println!("   🎯 Synced requested items only");
        
        Ok(result)
    }
    
    fn resolve_conflicts(&self) -> Result<SyncResult, StorageError> {
        println!("⚔️ Resolving sync conflicts");
        
        let mut result = SyncResult::default();
        
        // Simulate conflict resolution
        let conflicts = vec![
            "user_profile".to_string(),
            "app_settings".to_string(),
        ];
        
        for conflict_key in conflicts {
            match self.conflict_resolution {
                ConflictResolution::LocalWins => {
                    println!("   🏠 Local wins for: {}", conflict_key);
                }
                ConflictResolution::RemoteWins => {
                    println!("   ☁️ Remote wins for: {}", conflict_key);
                }
                ConflictResolution::LastWriteWins => {
                    println!("   ⏰ Last write wins for: {}", conflict_key);
                }
                ConflictResolution::Manual => {
                    println!("   👤 Manual resolution needed for: {}", conflict_key);
                }
                ConflictResolution::Merge => {
                    println!("   🔀 Merged changes for: {}", conflict_key);
                }
            }
            
            result.conflicts_resolved += 1;
        }
        
        Ok(result)
    }
    
    pub fn add_pending_upload(&mut self, key: String) {
        if !self.pending_uploads.contains(&key) {
            self.pending_uploads.push(key.clone());
            println!("📤 Added to upload queue: {}", key);
        }
    }
    
    pub fn add_pending_download(&mut self, key: String) {
        if !self.pending_downloads.contains(&key) {
            self.pending_downloads.push(key.clone());
            println!("📥 Added to download queue: {}", key);
        }
    }
    
    pub fn get_sync_status(&self) -> SyncStatus {
        SyncStatus {
            is_syncing: self.is_syncing,
            last_sync: self.last_sync,
            pending_uploads: self.pending_uploads.len(),
            pending_downloads: self.pending_downloads.len(),
            sync_strategy: self.sync_strategy.clone(),
            conflict_resolution: self.conflict_resolution.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    pub uploaded_count: u32,
    pub downloaded_count: u32,
    pub conflicts_resolved: u32,
    pub errors: Vec<String>,
    pub sync_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub is_syncing: bool,
    pub last_sync: Option<u64>,
    pub pending_uploads: usize,
    pub pending_downloads: usize,
    pub sync_strategy: SyncStrategy,
    pub conflict_resolution: ConflictResolution,
}

/// 🗃️ Database Manager
#[derive(Debug)]
pub struct DatabaseManager {
    db_type: DatabaseType,
    tables: HashMap<String, TableSchema>,
    indexes: HashMap<String, Vec<String>>,
    migrations: Vec<Migration>,
    current_version: u32,
    connection_pool_size: usize,
    query_cache: HashMap<String, QueryResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {
    SQLite,
    CoreData,
    Room,
    Realm,
    WatermelonDB,
}

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
    pub primary_key: String,
    pub foreign_keys: Vec<ForeignKey>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub unique: bool,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub column: String,
    pub references_table: String,
    pub references_column: String,
    pub on_delete: String,
    pub on_update: String,
}

#[derive(Debug, Clone)]
pub struct Migration {
    pub version: u32,
    pub description: String,
    pub up_sql: String,
    pub down_sql: String,
    pub applied_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<HashMap<String, serde_json::Value>>,
    pub affected_rows: u32,
    pub execution_time: Duration,
    pub cached: bool,
}

impl DatabaseManager {
    pub fn new(db_type: DatabaseType) -> Self {
        Self {
            db_type,
            tables: HashMap::new(),
            indexes: HashMap::new(),
            migrations: Vec::new(),
            current_version: 0,
            connection_pool_size: 5,
            query_cache: HashMap::new(),
        }
    }
    
    pub fn create_table(&mut self, schema: TableSchema) -> Result<(), StorageError> {
        println!("🏗️ Creating table: {}", schema.name);
        
        // Validate schema
        if schema.columns.is_empty() {
            return Err(StorageError::InvalidData);
        }
        
        // Generate SQL based on database type
        let sql = self.generate_create_table_sql(&schema);
        println!("   SQL: {}", sql);
        
        self.tables.insert(schema.name.clone(), schema);
        
        Ok(())
    }
    
    pub fn create_index(&mut self, table: String, columns: Vec<String>) -> Result<(), StorageError> {
        let index_name = format!("idx_{}_{}", table, columns.join("_"));
        println!("📇 Creating index: {} on {}", index_name, table);
        
        let sql = self.generate_create_index_sql(&table, &columns, &index_name);
        println!("   SQL: {}", sql);
        
        self.indexes.insert(index_name, columns);
        
        Ok(())
    }
    
    pub fn add_migration(&mut self, migration: Migration) {
        println!("🔄 Added migration v{}: {}", migration.version, migration.description);
        self.migrations.push(migration);
    }
    
    pub fn run_migrations(&mut self) -> Result<u32, StorageError> {
        println!("🚀 Running database migrations...");
        
        let mut applied_count = 0;
        
        for migration in &mut self.migrations {
            if migration.version > self.current_version {
                println!("   ⬆️ Applying migration v{}: {}", migration.version, migration.description);
                
                // Simulate migration execution
                migration.applied_at = Some(SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs());
                
                self.current_version = migration.version;
                applied_count += 1;
            }
        }
        
        if applied_count > 0 {
            println!("   ✅ Applied {} migrations, current version: {}", applied_count, self.current_version);
        } else {
            println!("   ℹ️ Database is up to date (version {})", self.current_version);
        }
        
        Ok(applied_count)
    }
    
    pub fn execute_query(&mut self, sql: String) -> Result<QueryResult, StorageError> {
        let start_time = SystemTime::now();
        
        // Check query cache
        if let Some(cached_result) = self.query_cache.get(&sql) {
            println!("🚀 Query result from cache");
            let mut result = cached_result.clone();
            result.cached = true;
            return Ok(result);
        }
        
        // Simulate query execution
        println!("🔍 Executing query: {}", sql);
        
        let result = QueryResult {
            rows: vec![
                {
                    let mut row = HashMap::new();
                    row.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    row.insert("name".to_string(), serde_json::Value::String("Sample Data".to_string()));
                    row
                }
            ],
            affected_rows: 1,
            execution_time: start_time.elapsed().unwrap_or_default(),
            cached: false,
        };
        
        // Cache the result
        self.query_cache.insert(sql, result.clone());
        
        println!("   ✅ Query executed in {:?}", result.execution_time);
        
        Ok(result)
    }
    
    fn generate_create_table_sql(&self, schema: &TableSchema) -> String {
        match self.db_type {
            DatabaseType::SQLite => {
                let columns: Vec<String> = schema.columns.iter()
                    .map(|col| format!("{} {} {}", 
                        col.name, 
                        col.data_type,
                        if col.nullable { "" } else { "NOT NULL" }
                    ))
                    .collect();
                
                format!("CREATE TABLE {} ({})", schema.name, columns.join(", "))
            }
            DatabaseType::Room => {
                format!("@Entity(tableName = \"{}\")", schema.name)
            }
            DatabaseType::CoreData => {
                format!("Core Data Entity: {}", schema.name)
            }
            DatabaseType::Realm => {
                format!("class {} : RealmObject", schema.name)
            }
            DatabaseType::WatermelonDB => {
                format!("tableSchema({{ name: '{}', columns: [...] }})", schema.name)
            }
        }
    }
    
    fn generate_create_index_sql(&self, table: &str, columns: &[String], index_name: &str) -> String {
        match self.db_type {
            DatabaseType::SQLite => {
                format!("CREATE INDEX {} ON {} ({})", index_name, table, columns.join(", "))
            }
            DatabaseType::Room => {
                format!("@Index(value = [{}])", columns.iter().map(|c| format!("\"{}\")", c)).collect::<Vec<_>>().join(", "))
            }
            _ => {
                format!("Index on {}: {}", table, columns.join(", "))
            }
        }
    }
    
    pub fn get_database_stats(&self) -> DatabaseStats {
        DatabaseStats {
            db_type: self.db_type.clone(),
            table_count: self.tables.len(),
            index_count: self.indexes.len(),
            migration_count: self.migrations.len(),
            current_version: self.current_version,
            cache_size: self.query_cache.len(),
            connection_pool_size: self.connection_pool_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub db_type: DatabaseType,
    pub table_count: usize,
    pub index_count: usize,
    pub migration_count: usize,
    pub current_version: u32,
    pub cache_size: usize,
    pub connection_pool_size: usize,
}

/// 💾 สาธิตการใช้งาน Mobile Data Storage
pub fn demonstrate_mobile_data_storage() {
    println!("💾 === Mobile Data Storage Demo ===");
    
    // Storage Manager
    println!("\n🗄️ Storage Manager:");
    demonstrate_storage_manager();
    
    // Cloud Synchronization
    println!("\n☁️ Cloud Synchronization:");
    demonstrate_cloud_sync();
    
    // Database Management
    println!("\n🗃️ Database Management:");
    demonstrate_database_management();
    
    // Best Practices
    println!("\n💡 Data Storage Best Practices:");
    show_data_storage_best_practices();
}

/// 🗄️ สาธิต Storage Manager
fn demonstrate_storage_manager() {
    // Test different storage types
    let storage_types = vec![
        (StorageType::UserDefaults, SecurityLevel::None),
        (StorageType::Keychain, SecurityLevel::High),
        (StorageType::SQLite, SecurityLevel::Standard),
        (StorageType::CloudKit, SecurityLevel::Standard),
    ];
    
    for (storage_type, security_level) in storage_types {
        println!("\n   📁 Testing {:?} with {:?} security:", storage_type, security_level);
        
        let mut storage = StorageManager::new(storage_type.clone(), security_level.clone());
        
        if security_level != SecurityLevel::None {
            storage.set_encryption_key("my_secret_key_123".to_string());
        }
        
        // Store data
        let user_data = serde_json::json!({
            "name": "John Doe",
            "email": "john@example.com",
            "preferences": {
                "theme": "dark",
                "notifications": true
            }
        });
        
        let _ = storage.store("user_profile".to_string(), user_data);
        
        // Retrieve data
        if let Ok(Some(data)) = storage.retrieve("user_profile") {
            println!("   ✅ Retrieved data: {}", data["name"]);
        }
        
        // Update data
        let updated_data = serde_json::json!({
            "name": "John Doe",
            "email": "john.doe@example.com",
            "preferences": {
                "theme": "light",
                "notifications": false
            }
        });
        
        let _ = storage.update("user_profile".to_string(), updated_data);
        
        // Show storage info
        let info = storage.get_storage_info();
        println!("   📊 Storage Info:");
        println!("      • Total items: {}", info.total_items);
        println!("      • Cached items: {}", info.cached_items);
        println!("      • Cache size: {:.1} KB", info.total_size_bytes as f64 / 1024.0);
        println!("      • Cache hit ratio: {:.1}%", info.cache_hit_ratio * 100.0);
        println!("      • Encryption: {}", info.encryption_enabled);
        println!("      • Compression: {}", info.compression_enabled);
    }
}

/// ☁️ สาธิต Cloud Synchronization
fn demonstrate_cloud_sync() {
    let storage = Arc::new(Mutex::new(StorageManager::new(
        StorageType::CloudKit,
        SecurityLevel::Standard,
    )));
    
    let sync_strategies = vec![
        SyncStrategy::Manual,
        SyncStrategy::Automatic,
        SyncStrategy::RealTime,
        SyncStrategy::Periodic,
    ];
    
    for strategy in sync_strategies {
        println!("\n   🔄 Testing {:?} sync strategy:", strategy);
        
        let mut sync_manager = CloudSyncManager::new(storage.clone(), strategy.clone());
        sync_manager.set_conflict_resolution(ConflictResolution::LastWriteWins);
        
        // Add some pending operations
        sync_manager.add_pending_upload("user_profile".to_string());
        sync_manager.add_pending_upload("app_settings".to_string());
        sync_manager.add_pending_download("shared_data".to_string());
        
        // Perform sync
        if let Ok(result) = sync_manager.sync_now() {
            println!("   ✅ Sync completed:");
            println!("      • Uploaded: {}", result.uploaded_count);
            println!("      • Downloaded: {}", result.downloaded_count);
            println!("      • Conflicts resolved: {}", result.conflicts_resolved);
            println!("      • Duration: {:?}", result.sync_duration);
        }
        
        // Show sync status
        let status = sync_manager.get_sync_status();
        println!("   📊 Sync Status:");
        println!("      • Is syncing: {}", status.is_syncing);
        println!("      • Pending uploads: {}", status.pending_uploads);
        println!("      • Pending downloads: {}", status.pending_downloads);
    }
}

/// 🗃️ สาธิต Database Management
fn demonstrate_database_management() {
    let db_types = vec![
        DatabaseType::SQLite,
        DatabaseType::Room,
        DatabaseType::CoreData,
        DatabaseType::Realm,
    ];
    
    for db_type in db_types {
        println!("\n   🗃️ Testing {:?} database:", db_type);
        
        let mut db_manager = DatabaseManager::new(db_type.clone());
        
        // Create table schema
        let user_table = TableSchema {
            name: "users".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                    default_value: None,
                    unique: true,
                },
                ColumnDefinition {
                    name: "name".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: false,
                    default_value: None,
                    unique: false,
                },
                ColumnDefinition {
                    name: "email".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: false,
                    default_value: None,
                    unique: true,
                },
                ColumnDefinition {
                    name: "created_at".to_string(),
                    data_type: "DATETIME".to_string(),
                    nullable: false,
                    default_value: Some("CURRENT_TIMESTAMP".to_string()),
                    unique: false,
                },
            ],
            primary_key: "id".to_string(),
            foreign_keys: Vec::new(),
            constraints: vec!["UNIQUE(email)".to_string()],
        };
        
        let _ = db_manager.create_table(user_table);
        
        // Create indexes
        let _ = db_manager.create_index("users".to_string(), vec!["email".to_string()]);
        let _ = db_manager.create_index("users".to_string(), vec!["name".to_string(), "created_at".to_string()]);
        
        // Add migrations
        db_manager.add_migration(Migration {
            version: 1,
            description: "Create users table".to_string(),
            up_sql: "CREATE TABLE users (...)".to_string(),
            down_sql: "DROP TABLE users".to_string(),
            applied_at: None,
        });
        
        db_manager.add_migration(Migration {
            version: 2,
            description: "Add email index".to_string(),
            up_sql: "CREATE INDEX idx_users_email ON users(email)".to_string(),
            down_sql: "DROP INDEX idx_users_email".to_string(),
            applied_at: None,
        });
        
        // Run migrations
        if let Ok(applied) = db_manager.run_migrations() {
            println!("   ✅ Applied {} migrations", applied);
        }
        
        // Execute queries
        let queries = vec![
            "SELECT * FROM users WHERE email = 'john@example.com'".to_string(),
            "INSERT INTO users (name, email) VALUES ('John', 'john@example.com')".to_string(),
            "UPDATE users SET name = 'John Doe' WHERE id = 1".to_string(),
        ];
        
        for query in queries {
            if let Ok(result) = db_manager.execute_query(query) {
                println!("   📊 Query result: {} rows, {:?} execution time", 
                        result.rows.len(), result.execution_time);
            }
        }
        
        // Show database stats
        let stats = db_manager.get_database_stats();
        println!("   📊 Database Stats:");
        println!("      • Tables: {}", stats.table_count);
        println!("      • Indexes: {}", stats.index_count);
        println!("      • Migrations: {}", stats.migration_count);
        println!("      • Current version: {}", stats.current_version);
        println!("      • Query cache size: {}", stats.cache_size);
    }
}

/// 💡 Data Storage Best Practices
fn show_data_storage_best_practices() {
    let practices = vec![
        "🔐 Use appropriate security levels for sensitive data",
        "💾 Choose the right storage type for your data",
        "🗜️ Enable compression for large data sets",
        "🚀 Implement caching for frequently accessed data",
        "☁️ Design for offline-first with sync capabilities",
        "🔄 Handle sync conflicts gracefully",
        "📊 Monitor storage usage and performance",
        "🧹 Implement data cleanup and archival strategies",
        "🔍 Use database indexes for query optimization",
        "🚀 Implement connection pooling for databases",
        "📱 Consider device storage limitations",
        "🔒 Encrypt sensitive data at rest",
        "⚡ Use lazy loading for large datasets",
        "🎯 Implement proper error handling",
        "📈 Use analytics to understand data usage patterns",
    ];
    
    for practice in practices {
        println!("   {}", practice);
    }
    
    println!("\n   📱 Platform-Specific Considerations:");
    println!("      🍎 iOS:");
    println!("         • Use Keychain for sensitive data");
    println!("         • Leverage Core Data for complex relationships");
    println!("         • Consider CloudKit for cloud sync");
    println!("         • Use NSUserDefaults for simple preferences");
    
    println!("\n      🤖 Android:");
    println!("         • Use Android Keystore for encryption keys");
    println!("         • Leverage Room for type-safe database access");
    println!("         • Consider Firebase for cloud services");
    println!("         • Use SharedPreferences for simple data");
    
    println!("\n   🛠️ Storage Selection Guide:");
    println!("      • Simple key-value: UserDefaults/SharedPreferences");
    println!("      • Sensitive data: Keychain/Keystore");
    println!("      • Complex queries: SQLite/Room/Core Data");
    println!("      • Real-time sync: Firebase/CloudKit");
    println!("      • Offline-first: Realm/WatermelonDB");
    println!("      • Large files: File system with cloud backup");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_manager() {
        let mut storage = StorageManager::new(StorageType::SQLite, SecurityLevel::None);
        
        let data = serde_json::json!({"test": "value"});
        assert!(storage.store("test_key".to_string(), data).is_ok());
        
        let retrieved = storage.retrieve("test_key").unwrap();
        assert!(retrieved.is_some());
        
        assert!(storage.update("test_key".to_string(), serde_json::json!({"updated": "value"})).is_ok());
        assert!(storage.delete("test_key").is_ok());
    }
    
    #[test]
    fn test_data_model() {
        let mut model = DataModel::new("test".to_string(), serde_json::json!({"data": "value"}));
        
        assert_eq!(model.version, 1);
        assert!(!model.is_synced);
        assert!(!model.is_deleted);
        
        model.update(serde_json::json!({"updated": "data"}));
        assert_eq!(model.version, 2);
        
        model.mark_synced();
        assert!(model.is_synced);
        
        model.soft_delete();
        assert!(model.is_deleted);
        assert_eq!(model.version, 3);
    }
    
    #[test]
    fn test_cloud_sync_manager() {
        let storage = Arc::new(Mutex::new(StorageManager::new(
            StorageType::CloudKit,
            SecurityLevel::None,
        )));
        
        let mut sync_manager = CloudSyncManager::new(storage, SyncStrategy::Manual);
        
        sync_manager.add_pending_upload("test_key".to_string());
        assert_eq!(sync_manager.get_sync_status().pending_uploads, 1);
        
        let result = sync_manager.sync_now().unwrap();
        assert!(result.uploaded_count > 0);
    }
    
    #[test]
    fn test_database_manager() {
        let mut db_manager = DatabaseManager::new(DatabaseType::SQLite);
        
        let schema = TableSchema {
            name: "test_table".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                    default_value: None,
                    unique: true,
                }
            ],
            primary_key: "id".to_string(),
            foreign_keys: Vec::new(),
            constraints: Vec::new(),
        };
        
        assert!(db_manager.create_table(schema).is_ok());
        assert_eq!(db_manager.get_database_stats().table_count, 1);
        
        let migration = Migration {
            version: 1,
            description: "Test migration".to_string(),
            up_sql: "CREATE TABLE test (...)".to_string(),
            down_sql: "DROP TABLE test".to_string(),
            applied_at: None,
        };
        
        db_manager.add_migration(migration);
        let applied = db_manager.run_migrations().unwrap();
        assert_eq!(applied, 1);
    }
}