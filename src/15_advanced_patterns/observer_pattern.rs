//! Observer Pattern Implementation
//!
//! การใช้งาน Observer Pattern ใน Rust
//! ครอบคลุม Event System, Publisher-Subscriber, และ Reactive Programming

use std::collections::HashMap;
// use std::sync::{Arc, Mutex, Weak};
// use std::rc::{Rc, Weak as RcWeak};

/// Observer trait สำหรับรับการแจ้งเตือน
pub trait Observer<T> {
    fn update(&mut self, data: &T);
    fn id(&self) -> usize;
}

/// Subject trait สำหรับจัดการ observers
pub trait Subject<T> {
    fn attach(&mut self, observer: Box<dyn Observer<T>>);
    fn detach(&mut self, observer_id: usize);
    fn notify(&mut self, data: &T);
}

/// Simple Observable implementation
pub struct SimpleObservable<T> {
    observers: Vec<Box<dyn Observer<T>>>,
    data: Option<T>,
}

impl<T: Clone> Default for SimpleObservable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> SimpleObservable<T> {
    #[must_use] pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            data: None,
        }
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data.clone());
        self.notify(&data);
    }

    pub const fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

impl<T: Clone> Subject<T> for SimpleObservable<T> {
    fn attach(&mut self, observer: Box<dyn Observer<T>>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer_id: usize) {
        self.observers.retain(|obs| obs.id() != observer_id);
    }

    fn notify(&mut self, data: &T) {
        for observer in &mut self.observers {
            observer.update(data);
        }
    }
}

/// Event system implementation
#[derive(Debug, Clone)]
pub enum Event {
    UserLogin { user_id: u32, username: String },
    UserLogout { user_id: u32 },
    MessageSent { from: u32, to: u32, content: String },
    SystemAlert { level: String, message: String },
}

/// Event listener trait
pub trait EventListener {
    fn handle_event(&mut self, event: &Event);
    fn listener_id(&self) -> usize;
}

/// Event dispatcher
pub struct EventDispatcher {
    listeners: HashMap<String, Vec<Box<dyn EventListener>>>,
    next_id: usize,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    #[must_use] pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn subscribe<L: EventListener + 'static>(&mut self, event_type: &str, listener: L) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.listeners
            .entry(event_type.to_string())
            .or_default()
            .push(Box::new(listener));
        
        id
    }

    pub fn unsubscribe(&mut self, event_type: &str, listener_id: usize) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            listeners.retain(|listener| listener.listener_id() != listener_id);
        }
    }

    pub fn dispatch(&mut self, event_type: &str, event: &Event) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            for listener in listeners {
                listener.handle_event(event);
            }
        }
    }

    pub fn dispatch_event(&mut self, event: &Event) {
        let event_type = match event {
            Event::UserLogin { .. } => "user_login",
            Event::UserLogout { .. } => "user_logout",
            Event::MessageSent { .. } => "message_sent",
            Event::SystemAlert { .. } => "system_alert",
        };
        
        self.dispatch(event_type, event);
    }
}

/// Logger listener
#[derive(Debug)]
pub struct LoggerListener {
    id: usize,
    name: String,
}

impl LoggerListener {
    #[must_use] pub const fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}

impl EventListener for LoggerListener {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::UserLogin { user_id, username } => {
                println!("[{}] 📝 User {} ({}) logged in", self.name, username, user_id);
            }
            Event::UserLogout { user_id } => {
                println!("[{}] 📝 User {} logged out", self.name, user_id);
            }
            Event::MessageSent { from, to, content } => {
                println!("[{}] 📝 Message from {} to {}: {}", self.name, from, to, content);
            }
            Event::SystemAlert { level, message } => {
                println!("[{}] 📝 System Alert [{}]: {}", self.name, level, message);
            }
        }
    }

    fn listener_id(&self) -> usize {
        self.id
    }
}

/// Analytics listener
#[derive(Debug)]
pub struct AnalyticsListener {
    id: usize,
    login_count: u32,
    message_count: u32,
    alert_count: u32,
}

impl AnalyticsListener {
    #[must_use] pub const fn new(id: usize) -> Self {
        Self {
            id,
            login_count: 0,
            message_count: 0,
            alert_count: 0,
        }
    }

    #[must_use] pub const fn get_stats(&self) -> (u32, u32, u32) {
        (self.login_count, self.message_count, self.alert_count)
    }
}

impl EventListener for AnalyticsListener {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::UserLogin { .. } => {
                self.login_count += 1;
                println!("📊 Analytics: Login count = {}", self.login_count);
            }
            Event::MessageSent { .. } => {
                self.message_count += 1;
                println!("📊 Analytics: Message count = {}", self.message_count);
            }
            Event::SystemAlert { .. } => {
                self.alert_count += 1;
                println!("📊 Analytics: Alert count = {}", self.alert_count);
            }
            _ => {}
        }
    }

    fn listener_id(&self) -> usize {
        self.id
    }
}

/// Temperature sensor example
#[derive(Debug, Clone)]
pub struct Temperature {
    pub celsius: f64,
    pub timestamp: u64,
}

/// Temperature observer
#[derive(Debug)]
pub struct TemperatureDisplay {
    id: usize,
    name: String,
    last_temperature: Option<Temperature>,
}

impl TemperatureDisplay {
    #[must_use] pub const fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            last_temperature: None,
        }
    }

    #[must_use] pub const fn get_last_temperature(&self) -> Option<&Temperature> {
        self.last_temperature.as_ref()
    }
}

impl Observer<Temperature> for TemperatureDisplay {
    fn update(&mut self, temperature: &Temperature) {
        self.last_temperature = Some(temperature.clone());
        println!(
            "🌡️  [{}] Temperature: {:.1}°C at timestamp {}",
            self.name, temperature.celsius, temperature.timestamp
        );
        
        // Alert for extreme temperatures
        if temperature.celsius > 35.0 {
            println!("🔥 [{}] HIGH TEMPERATURE ALERT!", self.name);
        } else if temperature.celsius < 0.0 {
            println!("🧊 [{}] FREEZING TEMPERATURE ALERT!", self.name);
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}

/// Stock price example
#[derive(Debug, Clone)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub timestamp: u64,
}

/// Stock observer
#[derive(Debug)]
pub struct StockTracker {
    id: usize,
    name: String,
    threshold: f64,
    positions: HashMap<String, f64>,
}

impl StockTracker {
    #[must_use] pub fn new(id: usize, name: String, threshold: f64) -> Self {
        Self {
            id,
            name,
            threshold,
            positions: HashMap::new(),
        }
    }

    pub fn add_position(&mut self, symbol: String, shares: f64) {
        self.positions.insert(symbol, shares);
    }
}

impl Observer<StockPrice> for StockTracker {
    fn update(&mut self, stock: &StockPrice) {
        println!(
            "📈 [{}] {} = ${:.2} ({}${:.2})",
            self.name,
            stock.symbol,
            stock.price,
            if stock.change >= 0.0 { "+" } else { "" },
            stock.change
        );
        
        // Check if we have a position in this stock
        if let Some(&shares) = self.positions.get(&stock.symbol) {
            let value = shares * stock.price;
            println!("💰 [{}] Position value: ${:.2}", self.name, value);
        }
        
        // Alert for significant changes
        if stock.change.abs() > self.threshold {
            println!(
                "🚨 [{}] SIGNIFICANT CHANGE ALERT for {}: {}${:.2}",
                self.name, stock.symbol,
                if stock.change >= 0.0 { "+" } else { "" },
                stock.change
            );
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}

/// สาธิตการใช้งาน Observer Pattern
pub fn demonstrate_observer() {
    println!("👁️  Observer Pattern Examples:");
    
    // Event System Example
    println!("\n🎯 Event System:");
    let mut dispatcher = EventDispatcher::new();
    
    // Subscribe listeners
    let logger = LoggerListener::new(1, "MainLogger".to_string());
    let analytics = AnalyticsListener::new(2);
    
    dispatcher.subscribe("user_login", logger);
    dispatcher.subscribe("user_login", analytics);
    
    let logger2 = LoggerListener::new(3, "SecurityLogger".to_string());
    dispatcher.subscribe("system_alert", logger2);
    
    // Dispatch events
    let events = vec![
        Event::UserLogin {
            user_id: 1,
            username: "alice".to_string(),
        },
        Event::UserLogin {
            user_id: 2,
            username: "bob".to_string(),
        },
        Event::MessageSent {
            from: 1,
            to: 2,
            content: "Hello Bob!".to_string(),
        },
        Event::SystemAlert {
            level: "WARNING".to_string(),
            message: "High CPU usage detected".to_string(),
        },
    ];
    
    for event in events {
        dispatcher.dispatch_event(&event);
    }
    
    // Temperature Monitoring Example
    println!("\n🌡️  Temperature Monitoring:");
    let mut temp_sensor = SimpleObservable::new();
    
    let display1 = TemperatureDisplay::new(1, "Kitchen".to_string());
    let display2 = TemperatureDisplay::new(2, "Living Room".to_string());
    
    temp_sensor.attach(Box::new(display1));
    temp_sensor.attach(Box::new(display2));
    
    // Simulate temperature readings
    let temperatures = vec![
        Temperature { celsius: 22.5, timestamp: 1000 },
        Temperature { celsius: 25.0, timestamp: 1001 },
        Temperature { celsius: 38.5, timestamp: 1002 }, // High temp
        Temperature { celsius: -2.0, timestamp: 1003 }, // Low temp
        Temperature { celsius: 20.0, timestamp: 1004 },
    ];
    
    for temp in temperatures {
        temp_sensor.set_data(temp);
    }
    
    // Stock Price Monitoring Example
    println!("\n📈 Stock Price Monitoring:");
    let mut stock_feed = SimpleObservable::new();
    
    let mut trader1 = StockTracker::new(1, "Trader1".to_string(), 5.0);
    trader1.add_position("AAPL".to_string(), 100.0);
    
    let trader2 = StockTracker::new(2, "Trader2".to_string(), 10.0);
    
    stock_feed.attach(Box::new(trader1));
    stock_feed.attach(Box::new(trader2));
    
    // Simulate stock price updates
    let stock_updates = vec![
        StockPrice {
            symbol: "AAPL".to_string(),
            price: 150.00,
            change: 2.50,
            timestamp: 2000,
        },
        StockPrice {
            symbol: "GOOGL".to_string(),
            price: 2800.00,
            change: -15.75,
            timestamp: 2001,
        },
        StockPrice {
            symbol: "AAPL".to_string(),
            price: 145.25,
            change: -4.75,
            timestamp: 2002,
        },
    ];
    
    for stock in stock_updates {
        stock_feed.set_data(stock);
    }
    
    println!("\n✅ Observer pattern demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_observable() {
        let mut observable = SimpleObservable::new();
        let display = TemperatureDisplay::new(1, "Test".to_string());
        
        observable.attach(Box::new(display));
        
        let temp = Temperature { celsius: 25.0, timestamp: 1000 };
        observable.set_data(temp);
        
        assert_eq!(observable.get_data().unwrap().celsius, 25.0);
    }

    #[test]
    fn test_event_dispatcher() {
        let mut dispatcher = EventDispatcher::new();
        let logger = LoggerListener::new(1, "Test".to_string());
        
        dispatcher.subscribe("user_login", logger);
        
        let event = Event::UserLogin {
            user_id: 1,
            username: "test".to_string(),
        };
        
        dispatcher.dispatch_event(&event);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_analytics_listener() {
        let mut analytics = AnalyticsListener::new(1);
        
        let login_event = Event::UserLogin {
            user_id: 1,
            username: "test".to_string(),
        };
        
        analytics.handle_event(&login_event);
        analytics.handle_event(&login_event);
        
        let (login_count, _, _) = analytics.get_stats();
        assert_eq!(login_count, 2);
    }
}