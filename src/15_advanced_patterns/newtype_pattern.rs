//! Newtype Pattern Implementation
//!
//! การใช้งาน Newtype Pattern ใน Rust
//! ครอบคลุม Type Safety, Domain Modeling, และ Zero-Cost Abstractions

use std::fmt;
use std::ops::{Add, Sub, Deref, DerefMut};
use std::str::FromStr;

/// Email newtype for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if Self::is_valid(&email) {
            Ok(Self(email))
        } else {
            Err(format!("Invalid email format: {email}"))
        }
    }

    #[must_use] pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use] pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }

    #[must_use] pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }

    fn is_valid(email: &str) -> bool {
        email.contains('@') && 
        email.chars().filter(|&c| c == '@').count() == 1 &&
        !email.starts_with('@') &&
        !email.ends_with('@') &&
        email.len() > 3
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// `UserId` newtype for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserId(u64);

impl UserId {
    #[must_use] pub const fn new(id: u64) -> Self {
        Self(id)
    }

    #[must_use] pub const fn value(&self) -> u64 {
        self.0
    }

    #[must_use] pub const fn is_admin(&self) -> bool {
        self.0 < 100 // Admin users have IDs < 100
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User#{}", self.0)
    }
}

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// Password newtype with security considerations
#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, String> {
        if Self::is_strong(&password) {
            Ok(Self(password))
        } else {
            Err("Password does not meet strength requirements".to_string())
        }
    }

    #[must_use] pub fn verify(&self, input: &str) -> bool {
        // In real implementation, this would use proper hashing
        self.0 == input
    }

    #[must_use] pub fn strength(&self) -> PasswordStrength {
        let password = &self.0;
        let mut score = 0;
        
        if password.len() >= 8 { score += 1; }
        if password.len() >= 12 { score += 1; }
        if password.chars().any(char::is_lowercase) { score += 1; }
        if password.chars().any(char::is_uppercase) { score += 1; }
        if password.chars().any(char::is_numeric) { score += 1; }
        if password.chars().any(|c| !c.is_alphanumeric()) { score += 1; }
        
        match score {
            0..=2 => PasswordStrength::Weak,
            3..=4 => PasswordStrength::Medium,
            5..=6 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    fn is_strong(password: &str) -> bool {
        password.len() >= 8 &&
        password.chars().any(char::is_lowercase) &&
        password.chars().any(char::is_uppercase) &&
        password.chars().any(char::is_numeric)
    }
}

// Don't implement Display for Password for security
impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[HIDDEN]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weak => write!(f, "Weak 🔴"),
            Self::Medium => write!(f, "Medium 🟡"),
            Self::Strong => write!(f, "Strong 🟢"),
            Self::VeryStrong => write!(f, "Very Strong 💚"),
        }
    }
}

/// Money newtype with currency support
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Money {
    amount: i64, // Store as cents to avoid floating point issues
    currency: Currency,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    THB,
}

impl Currency {
    #[must_use] pub const fn symbol(&self) -> &'static str {
        match self {
            Self::USD => "$",
            Self::EUR => "€",
            Self::GBP => "£",
            Self::JPY => "¥",
            Self::THB => "฿",
        }
    }

    #[must_use] pub const fn code(&self) -> &'static str {
        match self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::JPY => "JPY",
            Self::THB => "THB",
        }
    }
}

impl Money {
    #[must_use] pub fn new(amount: f64, currency: Currency) -> Self {
        Self {
            amount: (amount * 100.0).round() as i64,
            currency,
        }
    }

    #[must_use] pub const fn from_cents(cents: i64, currency: Currency) -> Self {
        Self {
            amount: cents,
            currency,
        }
    }

    #[must_use] pub fn amount(&self) -> f64 {
        self.amount as f64 / 100.0
    }

    #[must_use] pub const fn cents(&self) -> i64 {
        self.amount
    }

    #[must_use] pub const fn currency(&self) -> &Currency {
        &self.currency
    }

    #[must_use] pub fn is_same_currency(&self, other: &Self) -> bool {
        self.currency == other.currency
    }

    #[must_use] pub const fn is_positive(&self) -> bool {
        self.amount > 0
    }

    #[must_use] pub const fn is_zero(&self) -> bool {
        self.amount == 0
    }

    #[must_use] pub fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency.clone(),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:.2}", self.currency.symbol(), self.amount())
    }
}

impl Add for Money {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(format!(
                "Cannot add different currencies: {} and {}",
                self.currency.code(),
                other.currency.code()
            ));
        }
        
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }
}

impl Sub for Money {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(format!(
                "Cannot subtract different currencies: {} and {}",
                self.currency.code(),
                other.currency.code()
            ));
        }
        
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency,
        })
    }
}

/// Distance newtype with unit conversions
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance {
    meters: f64,
}

impl Distance {
    #[must_use] pub const fn from_meters(meters: f64) -> Self {
        Self { meters }
    }

    #[must_use] pub fn from_kilometers(km: f64) -> Self {
        Self { meters: km * 1000.0 }
    }

    #[must_use] pub fn from_miles(miles: f64) -> Self {
        Self { meters: miles * 1609.344 }
    }

    #[must_use] pub fn from_feet(feet: f64) -> Self {
        Self { meters: feet * 0.3048 }
    }

    #[must_use] pub const fn meters(&self) -> f64 {
        self.meters
    }

    #[must_use] pub fn kilometers(&self) -> f64 {
        self.meters / 1000.0
    }

    #[must_use] pub fn miles(&self) -> f64 {
        self.meters / 1609.344
    }

    #[must_use] pub fn feet(&self) -> f64 {
        self.meters / 0.3048
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.meters < 1000.0 {
            write!(f, "{:.2} m", self.meters)
        } else {
            write!(f, "{:.2} km", self.kilometers())
        }
    }
}

impl Add for Distance {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            meters: self.meters + other.meters,
        }
    }
}

impl Sub for Distance {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            meters: self.meters - other.meters,
        }
    }
}

/// Temperature newtype with scale conversions
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature {
    celsius: f64,
}

impl Temperature {
    #[must_use] pub const fn from_celsius(celsius: f64) -> Self {
        Self { celsius }
    }

    #[must_use] pub fn from_fahrenheit(fahrenheit: f64) -> Self {
        Self {
            celsius: (fahrenheit - 32.0) * 5.0 / 9.0,
        }
    }

    #[must_use] pub fn from_kelvin(kelvin: f64) -> Self {
        Self {
            celsius: kelvin - 273.15,
        }
    }

    #[must_use] pub const fn celsius(&self) -> f64 {
        self.celsius
    }

    #[must_use] pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    #[must_use] pub fn kelvin(&self) -> f64 {
        self.celsius + 273.15
    }

    #[must_use] pub fn is_freezing(&self) -> bool {
        self.celsius <= 0.0
    }

    #[must_use] pub fn is_boiling(&self) -> bool {
        self.celsius >= 100.0
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

/// Wrapper for Vec with additional functionality
#[derive(Debug, Clone)]
pub struct SafeVec<T>(Vec<T>);

impl<T> Default for SafeVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SafeVec<T> {
    #[must_use] pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use] pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    #[must_use] pub fn safe_get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    pub fn safe_get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    #[must_use] pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use] pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use] pub const fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.0.iter_mut()
    }
}

impl<T> Deref for SafeVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SafeVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for SafeVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> From<SafeVec<T>> for Vec<T> {
    fn from(val: SafeVec<T>) -> Self {
        val.0
    }
}

/// User account with type-safe fields
#[derive(Debug, Clone)]
pub struct UserAccount {
    id: UserId,
    email: Email,
    password: Password,
    balance: Money,
}

impl UserAccount {
    #[must_use] pub const fn new(
        id: UserId,
        email: Email,
        password: Password,
        initial_balance: Money,
    ) -> Self {
        Self {
            id,
            email,
            password,
            balance: initial_balance,
        }
    }

    #[must_use] pub const fn id(&self) -> UserId {
        self.id
    }

    #[must_use] pub const fn email(&self) -> &Email {
        &self.email
    }

    #[must_use] pub const fn balance(&self) -> &Money {
        &self.balance
    }

    #[must_use] pub fn verify_password(&self, input: &str) -> bool {
        self.password.verify(input)
    }

    pub fn deposit(&mut self, amount: Money) -> Result<(), String> {
        if !self.balance.is_same_currency(&amount) {
            return Err("Currency mismatch".to_string());
        }
        
        if !amount.is_positive() {
            return Err("Deposit amount must be positive".to_string());
        }
        
        match self.balance.clone() + amount {
            Ok(new_balance) => {
                self.balance = new_balance;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<(), String> {
        if !self.balance.is_same_currency(&amount) {
            return Err("Currency mismatch".to_string());
        }
        
        if !amount.is_positive() {
            return Err("Withdrawal amount must be positive".to_string());
        }
        
        match self.balance.clone() - amount {
            Ok(new_balance) => {
                if new_balance.is_positive() || new_balance.is_zero() {
                    self.balance = new_balance;
                    Ok(())
                } else {
                    Err("Insufficient funds".to_string())
                }
            }
            Err(e) => Err(e),
        }
    }

    #[must_use] pub const fn is_admin(&self) -> bool {
        self.id.is_admin()
    }
}

impl fmt::Display for UserAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {} <{}> - Balance: {}",
            self.id, self.email, self.balance
        )
    }
}

/// สาธิตการใช้งาน Newtype Pattern
///
/// # Panics
/// 
/// ฟังก์ชันนี้จะ panic หากไม่สามารถสร้าง Email, Password หรือ Money ได้
pub fn demonstrate_newtype() {
    println!("🔒 Newtype Pattern Examples:");
    
    // Email validation
    println!("\n📧 Email Validation:");
    let valid_emails = vec![
        "user@example.com",
        "test.email+tag@domain.co.uk",
        "admin@company.org",
    ];
    
    let invalid_emails = vec![
        "invalid-email",
        "@domain.com",
        "user@",
        "a@b",
    ];
    
    for email_str in valid_emails {
        match Email::new(email_str.to_string()) {
            Ok(email) => {
                println!("✅ Valid email: {} (domain: {})", email, email.domain());
            }
            Err(e) => println!("❌ {e}"),
        }
    }
    
    for email_str in invalid_emails {
        match Email::new(email_str.to_string()) {
            Ok(email) => println!("✅ Valid email: {email}"),
            Err(e) => println!("❌ {e}"),
        }
    }
    
    // Password strength
    println!("\n🔐 Password Strength:");
    let passwords = vec![
        "weak",
        "Password123",
        "VeryStr0ng!Pass",
        "Super$ecure123!@#",
    ];
    
    for pwd_str in passwords {
        match Password::new(pwd_str.to_string()) {
            Ok(password) => {
                println!("✅ Password accepted - Strength: {}", password.strength());
            }
            Err(e) => println!("❌ Password rejected: {e}"),
        }
    }
    
    // Money operations
    println!("\n💰 Money Operations:");
    let usd1 = Money::new(100.50, Currency::USD);
    let usd2 = Money::new(50.25, Currency::USD);
    let eur1 = Money::new(75.00, Currency::EUR);
    
    println!("Amount 1: {usd1}");
    println!("Amount 2: {usd2}");
    println!("Amount 3: {eur1}");
    
    match usd1.clone() + usd2.clone() {
        Ok(sum) => println!("✅ {usd1} + {usd2} = {sum}"),
        Err(e) => println!("❌ Addition failed: {e}"),
    }
    
    match usd1 + eur1 {
        Ok(sum) => println!("✅ Sum: {sum}"),
        Err(e) => println!("❌ Addition failed: {e}"),
    }
    
    // Distance conversions
    println!("\n📏 Distance Conversions:");
    let distance1 = Distance::from_meters(1000.0);
    let distance2 = Distance::from_miles(1.0);
    let distance3 = Distance::from_feet(100.0);
    
    println!("1000 meters = {distance1}");
    println!("1 mile = {} = {:.2} km", distance2, distance2.kilometers());
    println!("100 feet = {} = {:.2} m", distance3, distance3.meters());
    
    let total_distance = distance1 + distance2 + distance3;
    println!("Total distance: {} = {:.2} miles", total_distance, total_distance.miles());
    
    // Temperature conversions
    println!("\n🌡️  Temperature Conversions:");
    let temps = vec![
        Temperature::from_celsius(0.0),
        Temperature::from_fahrenheit(32.0),
        Temperature::from_kelvin(273.15),
        Temperature::from_celsius(100.0),
        Temperature::from_fahrenheit(212.0),
    ];
    
    for temp in temps {
        println!(
            "{} = {:.1}°F = {:.1}K (Freezing: {}, Boiling: {})",
            temp,
            temp.fahrenheit(),
            temp.kelvin(),
            temp.is_freezing(),
            temp.is_boiling()
        );
    }
    
    // User account example
    println!("\n👤 User Account Example:");
    
    let email = Email::new("alice@example.com".to_string()).unwrap();
    let password = Password::new("SecurePass123!".to_string()).unwrap();
    let initial_balance = Money::new(1000.0, Currency::USD);
    
    let mut account = UserAccount::new(
        UserId::new(1),
        email,
        password,
        initial_balance,
    );
    
    println!("Created account: {account}");
    println!("Is admin: {}", account.is_admin());
    
    // Deposit money
    let deposit = Money::new(250.0, Currency::USD);
    match account.deposit(deposit) {
        Ok(()) => println!("✅ Deposited {}. New balance: {}", Money::new(250.0, Currency::USD), account.balance()),
        Err(e) => println!("❌ Deposit failed: {e}"),
    }
    
    // Try to withdraw
    let withdrawal = Money::new(500.0, Currency::USD);
    match account.withdraw(withdrawal) {
        Ok(()) => println!("✅ Withdrew {}. New balance: {}", Money::new(500.0, Currency::USD), account.balance()),
        Err(e) => println!("❌ Withdrawal failed: {e}"),
    }
    
    // Try to withdraw more than balance
    let large_withdrawal = Money::new(2000.0, Currency::USD);
    match account.withdraw(large_withdrawal) {
        Ok(()) => println!("✅ Withdrew {}", Money::new(2000.0, Currency::USD)),
        Err(e) => println!("❌ Withdrawal failed: {e}"),
    }
    
    // Safe vector example
    println!("\n🛡️  Safe Vector Example:");
    let mut safe_vec = SafeVec::new();
    safe_vec.push("Hello");
    safe_vec.push("World");
    safe_vec.push("Rust");
    
    println!("Vector length: {}", safe_vec.len());
    
    // Safe access
    match safe_vec.safe_get(1) {
        Some(value) => println!("✅ Index 1: {value}"),
        None => println!("❌ Index 1 out of bounds"),
    }
    
    match safe_vec.safe_get(10) {
        Some(value) => println!("✅ Index 10: {value}"),
        None => println!("❌ Index 10 out of bounds"),
    }
    
    println!("\n✅ Newtype pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com".to_string()).is_ok());
        assert!(Email::new("invalid-email".to_string()).is_err());
        assert!(Email::new("@domain.com".to_string()).is_err());
    }

    #[test]
    fn test_email_parts() {
        let email = Email::new("user@domain.com".to_string()).unwrap();
        assert_eq!(email.local_part(), "user");
        assert_eq!(email.domain(), "domain.com");
    }

    #[test]
    fn test_user_id() {
        let admin_id = UserId::new(50);
        let user_id = UserId::new(1000);
        
        assert!(admin_id.is_admin());
        assert!(!user_id.is_admin());
    }

    #[test]
    fn test_password_strength() {
        let weak = Password::new("weak".to_string());
        assert!(weak.is_err());
        
        let strong = Password::new("StrongPass123".to_string()).unwrap();
        assert_eq!(strong.strength(), PasswordStrength::Strong);
    }

    #[test]
    fn test_money_operations() {
        let money1 = Money::new(100.0, Currency::USD);
        let money2 = Money::new(50.0, Currency::USD);
        
        let sum = (money1.clone() + money2.clone()).unwrap();
        assert_eq!(sum.amount(), 150.0);
        
        let diff = (money1 - money2).unwrap();
        assert_eq!(diff.amount(), 50.0);
    }

    #[test]
    fn test_money_currency_mismatch() {
        let usd = Money::new(100.0, Currency::USD);
        let eur = Money::new(100.0, Currency::EUR);
        
        assert!((usd + eur).is_err());
    }

    #[test]
    fn test_distance_conversions() {
        let distance = Distance::from_kilometers(1.0);
        assert_eq!(distance.meters(), 1000.0);
        
        let mile_distance = Distance::from_miles(1.0);
        assert!((mile_distance.kilometers() - 1.609344).abs() < 0.001);
    }

    #[test]
    fn test_temperature_conversions() {
        let freezing = Temperature::from_celsius(0.0);
        assert_eq!(freezing.fahrenheit(), 32.0);
        assert_eq!(freezing.kelvin(), 273.15);
        assert!(freezing.is_freezing());
        
        let boiling = Temperature::from_celsius(100.0);
        assert_eq!(boiling.fahrenheit(), 212.0);
        assert!(boiling.is_boiling());
    }

    #[test]
    fn test_user_account() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let password = Password::new("TestPass123".to_string()).unwrap();
        let balance = Money::new(100.0, Currency::USD);
        
        let mut account = UserAccount::new(
            UserId::new(1),
            email,
            password,
            balance,
        );
        
        let deposit = Money::new(50.0, Currency::USD);
        assert!(account.deposit(deposit).is_ok());
        assert_eq!(account.balance().amount(), 150.0);
        
        let withdrawal = Money::new(200.0, Currency::USD);
        assert!(account.withdraw(withdrawal).is_err());
    }

    #[test]
    fn test_safe_vec() {
        let mut vec = SafeVec::new();
        vec.push(42);
        vec.push(84);
        
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.safe_get(0), Some(&42));
        assert_eq!(vec.safe_get(1), Some(&84));
        assert_eq!(vec.safe_get(2), None);
    }
}