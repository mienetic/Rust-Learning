//! Phantom Types Implementation
//!
//! การใช้งาน Phantom Types ใน Rust
//! ครอบคลุม Type Safety, Zero-Cost Abstractions, และ Compile-time Guarantees

use std::marker::PhantomData;
use std::fmt;

/// Unit types for different measurement systems
#[derive(Debug, Clone, Copy)]
pub struct Metric;

#[derive(Debug, Clone, Copy)]
pub struct Imperial;

/// Length with phantom type for unit system
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Length<Unit> {
    #[must_use] pub const fn value(&self) -> f64 {
        self.value
    }
}

/// Metric length implementations
impl Length<Metric> {
    #[must_use] pub const fn meters(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }

    #[must_use] pub fn kilometers(value: f64) -> Self {
        Self {
            value: value * 1000.0,
            _unit: PhantomData,
        }
    }

    #[must_use] pub fn centimeters(value: f64) -> Self {
        Self {
            value: value / 100.0,
            _unit: PhantomData,
        }
    }

    #[must_use] pub const fn to_meters(&self) -> f64 {
        self.value
    }

    #[must_use] pub fn to_kilometers(&self) -> f64 {
        self.value / 1000.0
    }

    #[must_use] pub fn to_centimeters(&self) -> f64 {
        self.value * 100.0
    }

    #[must_use] pub fn to_imperial(self) -> Length<Imperial> {
        Length {
            value: self.value * 3.28084, // meters to feet
            _unit: PhantomData,
        }
    }
}

/// Imperial length implementations
impl Length<Imperial> {
    #[must_use] pub const fn feet(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }

    #[must_use] pub fn inches(value: f64) -> Self {
        Self {
            value: value / 12.0,
            _unit: PhantomData,
        }
    }

    #[must_use] pub fn yards(value: f64) -> Self {
        Self {
            value: value * 3.0,
            _unit: PhantomData,
        }
    }

    #[must_use] pub fn miles(value: f64) -> Self {
        Self {
            value: value * 5280.0,
            _unit: PhantomData,
        }
    }

    #[must_use] pub const fn to_feet(&self) -> f64 {
        self.value
    }

    #[must_use] pub fn to_inches(&self) -> f64 {
        self.value * 12.0
    }

    #[must_use] pub fn to_yards(&self) -> f64 {
        self.value / 3.0
    }

    #[must_use] pub fn to_miles(&self) -> f64 {
        self.value / 5280.0
    }

    #[must_use] pub fn to_metric(self) -> Length<Metric> {
        Length {
            value: self.value / 3.28084, // feet to meters
            _unit: PhantomData,
        }
    }
}

impl fmt::Display for Length<Metric> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value >= 1000.0 {
            write!(f, "{:.2} km", self.to_kilometers())
        } else if self.value < 1.0 {
            write!(f, "{:.1} cm", self.to_centimeters())
        } else {
            write!(f, "{:.2} m", self.value)
        }
    }
}

impl fmt::Display for Length<Imperial> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value >= 5280.0 {
            write!(f, "{:.2} miles", self.to_miles())
        } else if self.value >= 3.0 {
            write!(f, "{:.1} yards", self.to_yards())
        } else if self.value < 1.0 {
            write!(f, "{:.1} inches", self.to_inches())
        } else {
            write!(f, "{:.1} feet", self.value)
        }
    }
}

/// Addition for same unit types
impl<Unit> std::ops::Add for Length<Unit> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
            _unit: PhantomData,
        }
    }
}

/// Subtraction for same unit types
impl<Unit> std::ops::Sub for Length<Unit> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value,
            _unit: PhantomData,
        }
    }
}

/// Currency types
#[derive(Debug, Clone, Copy)]
pub struct USD;

#[derive(Debug, Clone, Copy)]
pub struct EUR;

#[derive(Debug, Clone, Copy)]
pub struct GBP;

/// Money with phantom type for currency
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Money<Currency> {
    amount: f64,
    _currency: PhantomData<Currency>,
}

impl<Currency> Money<Currency> {
    #[must_use] pub const fn new(amount: f64) -> Self {
        Self {
            amount,
            _currency: PhantomData,
        }
    }

    #[must_use] pub const fn amount(&self) -> f64 {
        self.amount
    }

    #[must_use] pub fn is_positive(&self) -> bool {
        self.amount > 0.0
    }

    #[must_use] pub fn is_zero(&self) -> bool {
        self.amount == 0.0
    }

    #[must_use] pub const fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            _currency: PhantomData,
        }
    }
}

/// USD specific implementations
impl Money<USD> {
    #[must_use] pub const fn dollars(amount: f64) -> Self {
        Self::new(amount)
    }

    #[must_use] pub fn cents(cents: i64) -> Self {
        Self::new(cents as f64 / 100.0)
    }

    #[must_use] pub fn to_eur(self, exchange_rate: f64) -> Money<EUR> {
        Money::new(self.amount * exchange_rate)
    }

    #[must_use] pub fn to_gbp(self, exchange_rate: f64) -> Money<GBP> {
        Money::new(self.amount * exchange_rate)
    }
}

/// EUR specific implementations
impl Money<EUR> {
    #[must_use] pub const fn euros(amount: f64) -> Self {
        Self::new(amount)
    }

    #[must_use] pub fn cents(cents: i64) -> Self {
        Self::new(cents as f64 / 100.0)
    }

    #[must_use] pub fn to_usd(self, exchange_rate: f64) -> Money<USD> {
        Money::new(self.amount * exchange_rate)
    }

    #[must_use] pub fn to_gbp(self, exchange_rate: f64) -> Money<GBP> {
        Money::new(self.amount * exchange_rate)
    }
}

/// GBP specific implementations
impl Money<GBP> {
    #[must_use] pub const fn pounds(amount: f64) -> Self {
        Self::new(amount)
    }

    #[must_use] pub fn pence(pence: i64) -> Self {
        Self::new(pence as f64 / 100.0)
    }

    #[must_use] pub fn to_usd(self, exchange_rate: f64) -> Money<USD> {
        Money::new(self.amount * exchange_rate)
    }

    #[must_use] pub fn to_eur(self, exchange_rate: f64) -> Money<EUR> {
        Money::new(self.amount * exchange_rate)
    }
}

impl fmt::Display for Money<USD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.amount)
    }
}

impl fmt::Display for Money<EUR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "€{:.2}", self.amount)
    }
}

impl fmt::Display for Money<GBP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "£{:.2}", self.amount)
    }
}

/// Addition for same currency types
impl<Currency> std::ops::Add for Money<Currency> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            amount: self.amount + other.amount,
            _currency: PhantomData,
        }
    }
}

/// Subtraction for same currency types
impl<Currency> std::ops::Sub for Money<Currency> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            amount: self.amount - other.amount,
            _currency: PhantomData,
        }
    }
}

/// Security levels
#[derive(Debug, Clone, Copy)]
pub struct Public;

#[derive(Debug, Clone, Copy)]
pub struct Private;

#[derive(Debug, Clone, Copy)]
pub struct Secret;

#[derive(Debug, Clone, Copy)]
pub struct TopSecret;

/// Document with security level phantom type
#[derive(Debug, Clone)]
pub struct Document<SecurityLevel> {
    title: String,
    content: String,
    author: String,
    _security: PhantomData<SecurityLevel>,
}

impl<SecurityLevel> Document<SecurityLevel> {
    #[must_use] pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use] pub fn author(&self) -> &str {
        &self.author
    }

    #[must_use] pub const fn content_length(&self) -> usize {
        self.content.len()
    }
}

/// Public document implementations
impl Document<Public> {
    #[must_use] pub const fn new_public(title: String, content: String, author: String) -> Self {
        Self {
            title,
            content,
            author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use] pub fn publish(&self) -> String {
        format!("Publishing: {} by {}", self.title, self.author)
    }

    #[must_use] pub fn classify_as_private(self) -> Document<Private> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }
}

/// Private document implementations
impl Document<Private> {
    #[must_use] pub const fn new_private(title: String, content: String, author: String) -> Self {
        Self {
            title,
            content,
            author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn content_with_clearance(&self, has_clearance: bool) -> Option<&str> {
        if has_clearance {
            Some(&self.content)
        } else {
            None
        }
    }

    #[must_use] pub fn declassify(self) -> Document<Public> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn classify_as_secret(self) -> Document<Secret> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }
}

/// Secret document implementations
impl Document<Secret> {
    #[must_use] pub const fn new_secret(title: String, content: String, author: String) -> Self {
        Self {
            title,
            content,
            author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn content_with_secret_clearance(&self, has_secret_clearance: bool) -> Option<&str> {
        if has_secret_clearance {
            Some(&self.content)
        } else {
            None
        }
    }

    #[must_use] pub fn classify_as_top_secret(self) -> Document<TopSecret> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn downgrade_to_private(self) -> Document<Private> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }
}

/// Top Secret document implementations
impl Document<TopSecret> {
    #[must_use] pub const fn new_top_secret(title: String, content: String, author: String) -> Self {
        Self {
            title,
            content,
            author,
            _security: PhantomData,
        }
    }

    #[must_use] pub fn content_with_top_secret_clearance(&self, has_top_secret_clearance: bool) -> Option<&str> {
        if has_top_secret_clearance {
            Some(&self.content)
        } else {
            None
        }
    }

    #[must_use] pub fn emergency_declassify(self) -> Document<Public> {
        Document {
            title: self.title,
            content: self.content,
            author: self.author,
            _security: PhantomData,
        }
    }
}

/// Data processing states
#[derive(Debug, Clone, Copy)]
pub struct Raw;

#[derive(Debug, Clone, Copy)]
pub struct Validated;

#[derive(Debug, Clone, Copy)]
pub struct Processed;

/// Data container with processing state phantom type
#[derive(Debug, Clone)]
pub struct DataContainer<State> {
    data: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
    _state: PhantomData<State>,
}

/// Raw data implementations
impl DataContainer<Raw> {
    #[must_use] pub fn new(data: Vec<String>) -> Self {
        Self {
            data,
            metadata: std::collections::HashMap::new(),
            _state: PhantomData,
        }
    }

    pub fn validate(self) -> Result<DataContainer<Validated>, String> {
        // Validation logic
        for item in &self.data {
            if item.trim().is_empty() {
                return Err("Empty data item found".to_string());
            }
        }

        let mut metadata = self.metadata;
        metadata.insert("validation_status".to_string(), "passed".to_string());
        metadata.insert("validated_at".to_string(), "2024-01-01T00:00:00Z".to_string());

        Ok(DataContainer {
            data: self.data,
            metadata,
            _state: PhantomData,
        })
    }

    #[must_use] pub fn data(&self) -> &[String] {
        &self.data
    }

    #[must_use] pub const fn len(&self) -> usize {
        self.data.len()
    }
}

/// Validated data implementations
impl DataContainer<Validated> {
    #[must_use] pub fn process(self) -> DataContainer<Processed> {
        let processed_data: Vec<String> = self.data
            .into_iter()
            .map(|item| format!("PROCESSED: {}", item.to_uppercase()))
            .collect();

        let mut metadata = self.metadata;
        metadata.insert("processing_status".to_string(), "completed".to_string());
        metadata.insert("processed_at".to_string(), "2024-01-01T00:01:00Z".to_string());

        DataContainer {
            data: processed_data,
            metadata,
            _state: PhantomData,
        }
    }

    #[must_use] pub fn data(&self) -> &[String] {
        &self.data
    }

    #[must_use] pub fn validation_metadata(&self) -> Option<&String> {
        self.metadata.get("validation_status")
    }
}

/// Processed data implementations
impl DataContainer<Processed> {
    #[must_use] pub fn export(&self) -> String {
        format!(
            "Processed Data Export:\n{}\n\nMetadata: {:?}",
            self.data.join("\n"),
            self.metadata
        )
    }

    #[must_use] pub fn data(&self) -> &[String] {
        &self.data
    }

    #[must_use] pub fn processing_metadata(&self) -> Option<&String> {
        self.metadata.get("processing_status")
    }

    #[must_use] pub const fn get_metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }
}

/// Generic implementations for all states
impl<State> DataContainer<State> {
    #[must_use] pub const fn item_count(&self) -> usize {
        self.data.len()
    }

    #[must_use] pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Type-safe ID system
#[derive(Debug, Clone, Copy)]
pub struct User;

#[derive(Debug, Clone, Copy)]
pub struct Product;

#[derive(Debug, Clone, Copy)]
pub struct Order;

/// ID with phantom type for entity type
#[derive(Debug, Clone, Copy, Hash)]
pub struct Id<Entity> {
    value: u64,
    _entity: PhantomData<Entity>,
}

impl<Entity> PartialEq for Id<Entity> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Entity> Eq for Id<Entity> {}



impl<Entity> Id<Entity> {
    #[must_use] pub const fn new(value: u64) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    #[must_use] pub const fn value(&self) -> u64 {
        self.value
    }
}

impl<Entity> fmt::Display for Id<Entity> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID({})", self.value)
    }
}

/// Type aliases for specific entity IDs
pub type UserId = Id<User>;
pub type ProductId = Id<Product>;
pub type OrderId = Id<Order>;

/// Shopping cart with type-safe IDs
#[derive(Debug)]
pub struct ShoppingCart {
    user_id: UserId,
    items: Vec<(ProductId, u32)>, // (product_id, quantity)
}

impl ShoppingCart {
    #[must_use] pub const fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, product_id: ProductId, quantity: u32) {
        self.items.push((product_id, quantity));
    }

    #[must_use] pub const fn get_user_id(&self) -> UserId {
        self.user_id
    }

    #[must_use] pub fn get_items(&self) -> &[(ProductId, u32)] {
        &self.items
    }

    #[must_use] pub const fn item_count(&self) -> usize {
        self.items.len()
    }

    #[must_use] pub fn total_quantity(&self) -> u32 {
        self.items.iter().map(|(_, qty)| qty).sum()
    }
}

/// สาธิตการใช้งาน Phantom Types
pub fn demonstrate_phantom_types() {
    println!("👻 Phantom Types Examples:");
    
    // Length measurements with different unit systems
    println!("\n📏 Length Measurements:");
    
    let metric_length = Length::<Metric>::meters(100.0);
    let imperial_length = Length::<Imperial>::feet(328.0);
    
    println!("Metric: {metric_length}");
    println!("Imperial: {imperial_length}");
    
    // Convert between systems
    let converted_to_imperial = metric_length.to_imperial();
    let converted_to_metric = imperial_length.to_metric();
    
    println!("100m in imperial: {converted_to_imperial}");
    println!("328ft in metric: {converted_to_metric}");
    
    // Add lengths of the same unit (this compiles)
    let total_metric = metric_length + Length::<Metric>::meters(50.0);
    println!("Total metric: {total_metric}");
    
    // This would NOT compile (different unit types):
    // let invalid = metric_length + imperial_length; // Compilation error!
    
    // Money with different currencies
    println!("\n💰 Currency Operations:");
    
    let usd_amount = Money::<USD>::dollars(100.0);
    let eur_amount = Money::<EUR>::euros(85.0);
    let gbp_amount = Money::<GBP>::pounds(75.0);
    
    println!("USD: {usd_amount}");
    println!("EUR: {eur_amount}");
    println!("GBP: {gbp_amount}");
    
    // Currency conversions (with mock exchange rates)
    let usd_to_eur = usd_amount.to_eur(0.85);
    let eur_to_gbp = eur_amount.to_gbp(0.88);
    let gbp_to_usd = gbp_amount.to_usd(1.25);
    
    println!("$100 USD = {usd_to_eur}");
    println!("€85 EUR = {eur_to_gbp}");
    println!("£75 GBP = {gbp_to_usd}");
    
    // Add money of the same currency (this compiles)
    let total_usd = usd_amount + Money::<USD>::dollars(50.0);
    println!("Total USD: {total_usd}");
    
    // This would NOT compile (different currencies):
    // let invalid = usd_amount + eur_amount; // Compilation error!
    
    // Document security levels
    println!("\n🔒 Document Security:");
    
    let public_doc = Document::<Public>::new_public(
        "Public Announcement".to_string(),
        "This is public information available to everyone.".to_string(),
        "PR Team".to_string(),
    );
    
    let private_doc = Document::<Private>::new_private(
        "Internal Memo".to_string(),
        "This is internal company information.".to_string(),
        "Management".to_string(),
    );
    
    let secret_doc = Document::<Secret>::new_secret(
        "Classified Report".to_string(),
        "This contains sensitive information.".to_string(),
        "Intelligence".to_string(),
    );
    
    // Public document - anyone can read
    println!("Public doc: {}", public_doc.title());
    println!("Content: {}", public_doc.content());
    println!("Publish result: {}", public_doc.publish());
    
    // Private document - requires clearance
    println!("\nPrivate doc: {}", private_doc.title());
    match private_doc.content_with_clearance(true) {
        Some(content) => println!("Content (with clearance): {content}"),
        None => println!("Access denied - insufficient clearance"),
    }
    
    match private_doc.content_with_clearance(false) {
        Some(content) => println!("Content: {content}"),
        None => println!("Access denied - insufficient clearance"),
    }
    
    // Secret document - requires secret clearance
    println!("\nSecret doc: {}", secret_doc.title());
    match secret_doc.content_with_secret_clearance(true) {
        Some(content) => println!("Content (with secret clearance): {content}"),
        None => println!("Access denied - insufficient clearance"),
    }
    
    // Data processing pipeline
    println!("\n🔄 Data Processing Pipeline:");
    
    let raw_data = DataContainer::<Raw>::new(vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ]);
    
    println!("Raw data count: {}", raw_data.len());
    println!("Raw data: {:?}", raw_data.data());
    
    // Validate the data
    let validated_data = match raw_data.validate() {
        Ok(data) => {
            println!("✅ Data validation successful!");
            data
        }
        Err(e) => {
            println!("❌ Validation failed: {e}");
            return;
        }
    };
    
    println!("Validation status: {:?}", validated_data.validation_metadata());
    
    // Process the validated data
    let processed_data = validated_data.process();
    println!("Processing status: {:?}", processed_data.processing_metadata());
    println!("Processed data: {:?}", processed_data.data());
    
    // Export the processed data
    let export = processed_data.export();
    println!("\n📤 Export:\n{export}");
    
    // Type-safe ID system
    println!("\n🆔 Type-safe ID System:");
    
    let user_id = UserId::new(12345);
    let product_id1 = ProductId::new(67890);
    let product_id2 = ProductId::new(11111);
    let order_id = OrderId::new(99999);
    
    println!("User ID: {user_id}");
    println!("Product ID 1: {product_id1}");
    println!("Product ID 2: {product_id2}");
    println!("Order ID: {order_id}");
    
    // Create shopping cart
    let mut cart = ShoppingCart::new(user_id);
    cart.add_item(product_id1, 2);
    cart.add_item(product_id2, 1);
    
    println!("\n🛒 Shopping Cart:");
    println!("User: {}", cart.get_user_id());
    println!("Items: {:?}", cart.get_items());
    println!("Total items: {}", cart.item_count());
    println!("Total quantity: {}", cart.total_quantity());
    
    // This would NOT compile (wrong ID type):
    // cart.add_item(user_id, 1); // Compilation error!
    // cart.add_item(order_id, 1); // Compilation error!
    
    // Example of validation failure
    println!("\n❌ Validation Failure Example:");
    let invalid_data = DataContainer::<Raw>::new(vec![
        "valid".to_string(),
        String::new(), // Empty string will cause validation failure
        "also_valid".to_string(),
    ]);
    
    match invalid_data.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation failed: {e}"),
    }
    
    println!("\n✅ Phantom types demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversions() {
        let metric = Length::<Metric>::meters(100.0);
        let imperial = metric.to_imperial();
        
        assert!((imperial.to_feet() - 328.084).abs() < 0.1);
    }

    #[test]
    fn test_length_arithmetic() {
        let len1 = Length::<Metric>::meters(100.0);
        let len2 = Length::<Metric>::meters(50.0);
        let total = len1 + len2;
        
        assert_eq!(total.value(), 150.0);
    }

    #[test]
    fn test_money_operations() {
        let usd1 = Money::<USD>::dollars(100.0);
        let usd2 = Money::<USD>::dollars(50.0);
        let total = usd1 + usd2;
        
        assert_eq!(total.amount(), 150.0);
    }

    #[test]
    fn test_currency_conversion() {
        let usd = Money::<USD>::dollars(100.0);
        let eur = usd.to_eur(0.85);
        
        assert_eq!(eur.amount(), 85.0);
    }

    #[test]
    fn test_document_security() {
        let public_doc = Document::<Public>::new_public(
            "Test".to_string(),
            "Content".to_string(),
            "Author".to_string(),
        );
        
        assert_eq!(public_doc.content(), "Content");
        
        let private_doc = public_doc.classify_as_private();
        assert_eq!(private_doc.content_with_clearance(true), Some("Content"));
        assert_eq!(private_doc.content_with_clearance(false), None);
    }

    #[test]
    fn test_data_processing_pipeline() {
        let raw = DataContainer::<Raw>::new(vec!["test".to_string()]);
        let validated = raw.validate().unwrap();
        let processed = validated.process();
        
        assert_eq!(processed.data()[0], "PROCESSED: TEST");
        assert_eq!(processed.processing_metadata(), Some(&"completed".to_string()));
    }

    #[test]
    fn test_data_validation_failure() {
        let raw = DataContainer::<Raw>::new(vec![String::new()]);
        assert!(raw.validate().is_err());
    }

    #[test]
    fn test_type_safe_ids() {
        let user_id = UserId::new(123);
        let product_id = ProductId::new(456);
        
        let mut cart = ShoppingCart::new(user_id);
        cart.add_item(product_id, 2);
        
        assert_eq!(cart.get_user_id(), user_id);
        assert_eq!(cart.item_count(), 1);
        assert_eq!(cart.total_quantity(), 2);
    }

    #[test]
    fn test_id_equality() {
        let id1 = UserId::new(123);
        let id2 = UserId::new(123);
        let id3 = UserId::new(456);
        
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}