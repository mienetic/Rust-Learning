//! 🏗️ Blockchain and Cryptocurrency - เวิร์กช็อปสร้างระบบบล็อกเชนแบบมืออาชีพ
//!
//! 🎯 การสร้าง Blockchain และ Cryptocurrency สำหรับเว็บแอปพลิเคชัน - เหมือนการสร้างธนาคารดิจิทัลในเวิร์กช็อป!

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

/// 🔐 ฟังก์ชันแฮชแบบเวิร์กช็อป (simplified SHA-256) - เครื่องมือสร้างลายเซ็นดิจิทัล
fn simple_hash(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// 💰 โครงสร้างธุรกรรม - ระบบการโอนเงินดิจิทัลในเวิร์กช็อป
#[derive(Debug, Clone, PartialEq)]
struct Transaction {
    id: String,
    from: String,
    to: String,
    amount: f64,
    timestamp: u64,
    signature: Option<String>,
}

impl Transaction {
    fn new(from: String, to: String, amount: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let id = simple_hash(&format!("{from}{to}{amount}{timestamp}"));
        
        Self {
            id,
            from,
            to,
            amount,
            timestamp,
            signature: None,
        }
    }
    
    fn sign(&mut self, private_key: &str) {
        let data = format!("{}{}{}{}", self.from, self.to, self.amount, self.timestamp);
        self.signature = Some(simple_hash(&format!("{data}{private_key}")));
    }
    
    fn verify_signature(&self, public_key: &str) -> bool {
        if let Some(ref signature) = self.signature {
            let data = format!("{}{}{}{}", self.from, self.to, self.amount, self.timestamp);
            let expected_signature = simple_hash(&format!("{data}{public_key}"));
            signature == &expected_signature
        } else {
            false
        }
    }
    
    fn is_valid(&self) -> bool {
        !self.from.is_empty() && 
        !self.to.is_empty() && 
        self.amount > 0.0 &&
        self.signature.is_some()
    }
}

/// 🧱 โครงสร้างบล็อก - หน่วยพื้นฐานของโซ่บล็อกในเวิร์กช็อป
#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    nonce: u64,
    hash: String,
    merkle_root: String,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        let mut block = Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            merkle_root,
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.merkle_root,
            self.nonce,
            self.transactions.len()
        );
        simple_hash(&data)
    }
    
    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return simple_hash("");
        }
        
        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| simple_hash(&tx.id))
            .collect();
        
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(simple_hash(&format!("{}{}", chunk[0], chunk[1])));
                } else {
                    next_level.push(chunk[0].clone());
                }
            }
            
            hashes = next_level;
        }
        
        hashes[0].clone()
    }
    
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        println!("Mining block {}...", self.index);
        let start_time = SystemTime::now();
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                let duration = start_time.elapsed().unwrap();
                println!(
                    "Block mined: {} (nonce: {}, time: {:.2}s)",
                    self.hash,
                    self.nonce,
                    duration.as_secs_f64()
                );
                break;
            }
            
            self.nonce += 1;
        }
    }
    
    fn is_valid(&self, previous_block: Option<&Self>) -> bool {
        // Check hash
        if self.hash != self.calculate_hash() {
            return false;
        }
        
        // Check previous hash
        if let Some(prev) = previous_block {
            if self.previous_hash != prev.hash {
                return false;
            }
            
            if self.index != prev.index + 1 {
                return false;
            }
        }
        
        // Check merkle root
        if self.merkle_root != Self::calculate_merkle_root(&self.transactions) {
            return false;
        }
        
        // Check all transactions
        for transaction in &self.transactions {
            if !transaction.is_valid() {
                return false;
            }
        }
        
        true
    }
}

/// ⛓️ โครงสร้างบล็อกเชน - ระบบฐานข้อมูลแบบกระจายสำหรับเวิร์กช็อป
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    pending_transactions: Vec<Transaction>,
    mining_reward: f64,
    balances: HashMap<String, f64>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            difficulty: 2,
            pending_transactions: Vec::new(),
            mining_reward: 100.0,
            balances: HashMap::new(),
        };
        
        blockchain.create_genesis_block();
        blockchain
    }
    
    fn create_genesis_block(&mut self) {
        let genesis_transactions = vec![
            Transaction {
                id: "genesis".to_string(),
                from: "genesis".to_string(),
                to: "genesis".to_string(),
                amount: 0.0,
                timestamp: 0,
                signature: Some("genesis_signature".to_string()),
            }
        ];
        
        let mut genesis_block = Block::new(0, genesis_transactions, "0".to_string());
        genesis_block.mine_block(self.difficulty);
        
        self.chain.push(genesis_block);
    }
    
    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    fn add_transaction(&mut self, mut transaction: Transaction) {
        if transaction.from != "mining_reward" {
            // Check if sender has sufficient balance
            let balance = self.get_balance(&transaction.from);
            if balance < transaction.amount {
                println!("Insufficient balance for transaction: {}", transaction.id);
                return;
            }
        }
        
        // For simplicity, auto-sign with a mock private key
        transaction.sign(&format!("{}_private_key", transaction.from));
        
        self.pending_transactions.push(transaction);
    }
    
    fn mine_pending_transactions(&mut self, mining_reward_address: &str) {
        // Add mining reward transaction
        let reward_transaction = Transaction {
            id: simple_hash(&format!("mining_reward_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())),
            from: "mining_reward".to_string(),
            to: mining_reward_address.to_string(),
            amount: self.mining_reward,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            signature: Some("mining_reward_signature".to_string()),
        };
        
        self.pending_transactions.push(reward_transaction);
        
        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.get_latest_block().hash.clone(),
        );
        
        block.mine_block(self.difficulty);
        
        // Update balances
        for transaction in &block.transactions {
            if transaction.from != "mining_reward" && transaction.from != "genesis" {
                *self.balances.entry(transaction.from.clone()).or_insert(0.0) -= transaction.amount;
            }
            if transaction.to != "genesis" {
                *self.balances.entry(transaction.to.clone()).or_insert(0.0) += transaction.amount;
            }
        }
        
        self.chain.push(block);
        self.pending_transactions.clear();
    }
    
    fn get_balance(&self, address: &str) -> f64 {
        self.balances.get(address).copied().unwrap_or(0.0)
    }
    
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if !current_block.is_valid(Some(previous_block)) {
                return false;
            }
        }
        
        true
    }
    
    fn get_transaction_history(&self, address: &str) -> Vec<&Transaction> {
        let mut history = Vec::new();
        
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address || transaction.to == address {
                    history.push(transaction);
                }
            }
        }
        
        history
    }
}

/// 👛 โครงสร้างกระเป๋าเงินดิจิทัล - ระบบจัดการเงินดิจิทัลในเวิร์กช็อป
#[derive(Debug)]
struct Wallet {
    address: String,
    private_key: String,
    public_key: String,
}

impl Wallet {
    fn new(name: &str) -> Self {
        let private_key = simple_hash(&format!("{name}_private"));
        let public_key = simple_hash(&format!("{name}_public"));
        let address = simple_hash(&public_key)[..16].to_string();
        
        Self {
            address,
            private_key,
            public_key,
        }
    }
    
    fn create_transaction(&self, to: &str, amount: f64) -> Transaction {
        let mut transaction = Transaction::new(self.address.clone(), to.to_string(), amount);
        transaction.sign(&self.private_key);
        transaction
    }
    
    fn get_balance(&self, blockchain: &Blockchain) -> f64 {
        blockchain.get_balance(&self.address)
    }
}

/// 📜 โครงสร้างสัญญาอัจฉริยะ - ระบบสัญญาอัตโนมัติสำหรับเว็บแอป
#[derive(Debug, Clone)]
struct SmartContract {
    address: String,
    code: String,
    state: HashMap<String, String>,
    owner: String,
}

impl SmartContract {
    fn new(code: String, owner: String) -> Self {
        let address = simple_hash(&format!("{code}{owner}"))[..16].to_string();
        
        Self {
            address,
            code,
            state: HashMap::new(),
            owner,
        }
    }
    
    fn execute(&mut self, function: &str, params: &[&str]) -> Result<String, String> {
        match function {
            "set" => {
                if params.len() != 2 {
                    return Err("set requires 2 parameters: key, value".to_string());
                }
                self.state.insert(params[0].to_string(), params[1].to_string());
                Ok(format!("Set {} = {}", params[0], params[1]))
            }
            "get" => {
                if params.len() != 1 {
                    return Err("get requires 1 parameter: key".to_string());
                }
                match self.state.get(params[0]) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Key '{}' not found", params[0]))
                }
            }
            "transfer" => {
                if params.len() != 2 {
                    return Err("transfer requires 2 parameters: to, amount".to_string());
                }
                // Simplified transfer logic
                Ok(format!("Transferred {} to {}", params[1], params[0]))
            }
            _ => Err(format!("Unknown function: {function}"))
        }
    }
}

/// 💎 โครงสร้างสกุลเงินดิจิทัล - ระบบเงินดิจิทัลแบบครบครันสำหรับเวิร์กช็อป
#[derive(Debug)]
struct Cryptocurrency {
    name: String,
    symbol: String,
    total_supply: f64,
    blockchain: Blockchain,
    smart_contracts: HashMap<String, SmartContract>,
}

impl Cryptocurrency {
    fn new(name: String, symbol: String, total_supply: f64) -> Self {
        Self {
            name,
            symbol,
            total_supply,
            blockchain: Blockchain::new(),
            smart_contracts: HashMap::new(),
        }
    }
    
    fn deploy_contract(&mut self, contract: SmartContract) {
        let address = contract.address.clone();
        self.smart_contracts.insert(address, contract);
    }
    
    fn call_contract(&mut self, contract_address: &str, function: &str, params: &[&str]) -> Result<String, String> {
        if let Some(contract) = self.smart_contracts.get_mut(contract_address) {
            contract.execute(function, params)
        } else {
            Err(format!("Contract not found: {contract_address}"))
        }
    }
    
    fn get_network_stats(&self) -> NetworkStats {
        let total_transactions = self.blockchain.chain.iter()
            .map(|block| block.transactions.len())
            .sum();
        
        let total_blocks = self.blockchain.chain.len();
        
        let hash_rate = if total_blocks > 1 {
            let time_diff = self.blockchain.chain.last().unwrap().timestamp - 
                           self.blockchain.chain.first().unwrap().timestamp;
            if time_diff > 0 {
                (total_blocks - 1) as f64 / time_diff as f64
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        NetworkStats {
            total_blocks,
            total_transactions,
            hash_rate,
            difficulty: self.blockchain.difficulty,
            pending_transactions: self.blockchain.pending_transactions.len(),
        }
    }
}

/// 📊 สถิติเครือข่าย - ระบบติดตามประสิทธิภาพเครือข่ายบล็อกเชน
#[derive(Debug)]
struct NetworkStats {
    total_blocks: usize,
    total_transactions: usize,
    hash_rate: f64,
    difficulty: usize,
    pending_transactions: usize,
}

impl fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Network Stats:\n\
             - Total Blocks: {}\n\
             - Total Transactions: {}\n\
             - Hash Rate: {:.2} blocks/sec\n\
             - Difficulty: {}\n\
             - Pending Transactions: {}",
            self.total_blocks,
            self.total_transactions,
            self.hash_rate,
            self.difficulty,
            self.pending_transactions
        )
    }
}

/// ⛏️ กลุ่มขุดเหมือง - ระบบรวมพลังการขุดสำหรับเวิร์กช็อป
#[derive(Debug)]
struct MiningPool {
    name: String,
    miners: Vec<String>,
    total_hash_power: f64,
    rewards_distributed: f64,
}

impl MiningPool {
    const fn new(name: String) -> Self {
        Self {
            name,
            miners: Vec::new(),
            total_hash_power: 0.0,
            rewards_distributed: 0.0,
        }
    }
    
    fn add_miner(&mut self, miner_address: String, hash_power: f64) {
        self.miners.push(miner_address);
        self.total_hash_power += hash_power;
    }
    
    fn distribute_rewards(&mut self, total_reward: f64) -> HashMap<String, f64> {
        let mut rewards = HashMap::new();
        
        if self.total_hash_power > 0.0 {
            let individual_hash_power = self.total_hash_power / self.miners.len() as f64;
            let reward_per_miner = total_reward * (individual_hash_power / self.total_hash_power);
            
            for miner in &self.miners {
                rewards.insert(miner.clone(), reward_per_miner);
            }
            
            self.rewards_distributed += total_reward;
        }
        
        rewards
    }
}

/// 🎯 สาธิตการใช้งาน Blockchain และ Cryptocurrency ในเวิร์กช็อป
pub fn demonstrate_blockchain() {
    println!("⛓️ 🎓 ตัวอย่างการใช้งาน Blockchain และ Cryptocurrency ในเวิร์กช็อป Web Development!");
    
    // Create cryptocurrency
    println!("\n🪙 💎 การสร้างสกุลเงินดิจิทัลสำหรับเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let mut rustcoin = Cryptocurrency::new(
        "RustCoin".to_string(),
        "RST".to_string(),
        1_000_000.0
    );
    
    println!("✨ สร้าง {} ({}) สำเร็จ! จำนวนเหรียญทั้งหมด: {} เหรียญ", 
            rustcoin.name, rustcoin.symbol, rustcoin.total_supply);
    
    // Create wallets
    println!("\n👛 💼 การสร้างกระเป๋าเงินดิจิทัลสำหรับผู้เข้าร่วมเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let alice_wallet = Wallet::new("Alice");
    let bob_wallet = Wallet::new("Bob");
    let charlie_wallet = Wallet::new("Charlie");
    
    println!("🎯 ที่อยู่กระเป๋าของ Alice: {}", alice_wallet.address);
    println!("🎯 ที่อยู่กระเป๋าของ Bob: {}", bob_wallet.address);
    println!("🎯 ที่อยู่กระเป๋าของ Charlie: {}", charlie_wallet.address);
    
    // Initial mining to give Alice some coins
    println!("\n⛏️ 🚀 การขุดเหรียญเริ่มต้นสำหรับเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    rustcoin.blockchain.mine_pending_transactions(&alice_wallet.address);
    
    println!("💰 ยอดเงินของ Alice หลังขุดเหรียญ: {} เหรียญ", alice_wallet.get_balance(&rustcoin.blockchain));
    
    // Create and add transactions
    println!("\n💸 📝 การสร้างธุรกรรมในระบบเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let tx1 = alice_wallet.create_transaction(&bob_wallet.address, 30.0);
    let tx2 = alice_wallet.create_transaction(&charlie_wallet.address, 20.0);
    
    rustcoin.blockchain.add_transaction(tx1.clone());
    rustcoin.blockchain.add_transaction(tx2.clone());
    
    println!("✅ เพิ่มธุรกรรม: {} -> {} ({} เหรียญ)", tx1.from, tx1.to, tx1.amount);
    println!("✅ เพิ่มธุรกรรม: {} -> {} ({} เหรียญ)", tx2.from, tx2.to, tx2.amount);
    
    // Mine transactions
    println!("\n⛏️ 🔨 การขุดธุรกรรมในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    rustcoin.blockchain.mine_pending_transactions(&bob_wallet.address);
    
    // Check balances
    println!("\n💰 🔍 ตรวจสอบยอดเงินหลังธุรกรรมในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    println!("💳 ยอดเงินของ Alice: {} เหรียญ", alice_wallet.get_balance(&rustcoin.blockchain));
    println!("💳 ยอดเงินของ Bob: {} เหรียญ", bob_wallet.get_balance(&rustcoin.blockchain));
    println!("💳 ยอดเงินของ Charlie: {} เหรียญ", charlie_wallet.get_balance(&rustcoin.blockchain));
    
    // Validate blockchain
    println!("\n✅ 🔐 การตรวจสอบความถูกต้องของบล็อกเชน:");
    println!("{:-<50}", "");
    
    println!("🛡️ บล็อกเชนถูกต้องหรือไม่? {}", rustcoin.blockchain.is_chain_valid());
    
    // Smart Contract Demo
    println!("\n📜 🤖 การสาธิตสัญญาอัจฉริยะในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let contract = SmartContract::new(
        "simple_storage".to_string(),
        alice_wallet.address.clone()
    );
    
    println!("🚀 ติดตั้งสัญญาอัจฉริยะสำเร็จที่ที่อยู่: {}", contract.address);
    let contract_address = contract.address.clone();
    rustcoin.deploy_contract(contract);
    
    // Execute contract functions
    match rustcoin.call_contract(&contract_address, "set", &["name", "RustCoin"]) {
        Ok(result) => println!("✨ ผลการดำเนินการสัญญา: {result}"),
        Err(error) => println!("❌ ข้อผิดพลาดในการดำเนินการ: {error}"),
    }
    
    match rustcoin.call_contract(&contract_address, "get", &["name"]) {
        Ok(result) => println!("📊 ผลการสอบถามข้อมูล: {result}"),
        Err(error) => println!("❌ ข้อผิดพลาดในการสอบถาม: {error}"),
    }
    
    // Mining Pool Demo
    println!("\n🏊 ⛏️ การสาธิตกลุ่มขุดเหมืองในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let mut mining_pool = MiningPool::new("RustPool".to_string());
    mining_pool.add_miner(alice_wallet.address.clone(), 100.0);
    mining_pool.add_miner(bob_wallet.address.clone(), 150.0);
    mining_pool.add_miner(charlie_wallet.address, 75.0);
    
    println!("🎯 สร้างกลุ่มขุดเหมือง '{}' สำเร็จ!", mining_pool.name);
    println!("⚡ พลังการขุดรวม: {} หน่วย", mining_pool.total_hash_power);
    
    let rewards = mining_pool.distribute_rewards(300.0);
    println!("\n💰 การแจกจ่ายรางวัลในเวิร์กช็อป:");
    for (miner, reward) in rewards {
        println!("  🎁 {miner}: {reward:.2} เหรียญ");
    }
    
    // Network Statistics
    println!("\n📊 📈 สถิติเครือข่ายบล็อกเชนในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let stats = rustcoin.get_network_stats();
    println!("{stats}");
    
    // Transaction History
    println!("\n📋 📜 ประวัติธุรกรรมของ Alice ในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    let alice_history = rustcoin.blockchain.get_transaction_history(&alice_wallet.address);
    for (i, tx) in alice_history.iter().enumerate() {
        println!("{}. 💸 {} -> {} (จำนวน: {} เหรียญ, เวลา: {})", 
                i + 1, tx.from, tx.to, tx.amount, tx.timestamp);
    }
    
    // Blockchain Explorer
    println!("\n🔍 🌐 เครื่องมือสำรวจบล็อกเชนในเวิร์กช็อป:");
    println!("{:-<50}", "");
    
    for (i, block) in rustcoin.blockchain.chain.iter().enumerate() {
        println!("🧱 บล็อก #{}: {} ธุรกรรม, แฮช: {}", 
                i, block.transactions.len(), &block.hash[..16]);
        
        for (j, tx) in block.transactions.iter().enumerate() {
            println!("  💳 ธุรกรรม {}: {} -> {} ({} เหรียญ)", 
                    j + 1, tx.from, tx.to, tx.amount);
        }
    }
    
    println!("\n🎉 ✅ สาธิตการใช้งาน Blockchain และ Cryptocurrency ในเวิร์กช็อปเสร็จสิ้น!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
        assert_eq!(tx.from, "Alice");
        assert_eq!(tx.to, "Bob");
        assert_eq!(tx.amount, 50.0);
        assert!(tx.signature.is_none());
    }
    
    #[test]
    fn test_transaction_signing() {
        let mut tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
        tx.sign("alice_private_key");
        assert!(tx.signature.is_some());
        assert!(tx.is_valid());
    }
    
    #[test]
    fn test_block_creation() {
        let transactions = vec![
            Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0)
        ];
        let block = Block::new(1, transactions, "previous_hash".to_string());
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.transactions.len(), 1);
    }
    
    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Genesis block
        assert!(blockchain.is_chain_valid());
    }
    
    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new("Alice");
        assert!(!wallet.address.is_empty());
        assert!(!wallet.private_key.is_empty());
        assert!(!wallet.public_key.is_empty());
    }
    
    #[test]
    fn test_smart_contract() {
        let mut contract = SmartContract::new(
            "test_contract".to_string(),
            "owner".to_string()
        );
        
        let result = contract.execute("set", &["key", "value"]);
        assert!(result.is_ok());
        
        let result = contract.execute("get", &["key"]);
        assert_eq!(result.unwrap(), "value");
    }
    
    #[test]
    fn test_mining_pool() {
        let mut pool = MiningPool::new("TestPool".to_string());
        pool.add_miner("miner1".to_string(), 100.0);
        pool.add_miner("miner2".to_string(), 200.0);
        
        assert_eq!(pool.total_hash_power, 300.0);
        assert_eq!(pool.miners.len(), 2);
        
        let rewards = pool.distribute_rewards(300.0);
        assert_eq!(rewards.len(), 2);
    }
}