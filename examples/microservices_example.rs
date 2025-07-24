//! Microservices Architecture Example
//! ตัวอย่างการสร้าง microservices ด้วย Rust
//!
//! การรันตัวอย่างนี้:
//! ```bash
//! cargo run --example microservices_example
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

/// Event ที่ใช้ในระบบ Event-Driven Architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    UserCreated {
        user_id: Uuid,
        name: String,
        email: String,
    },
    UserUpdated {
        user_id: Uuid,
        name: Option<String>,
        email: Option<String>,
    },
    UserDeleted {
        user_id: Uuid,
    },
    OrderCreated {
        order_id: Uuid,
        user_id: Uuid,
        amount: f64,
    },
    OrderCompleted {
        order_id: Uuid,
    },
    PaymentProcessed {
        order_id: Uuid,
        amount: f64,
    },
    NotificationSent {
        user_id: Uuid,
        message: String,
    },
}

/// Message Bus สำหรับการสื่อสารระหว่าง services
#[derive(Debug, Clone)]
pub struct MessageBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<tokio::sync::mpsc::UnboundedSender<Event>>>>>,
}

impl Default for MessageBus {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageBus {
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// สมัครรับ events ของ topic ที่กำหนด
    pub fn subscribe(&self, topic: &str, sender: tokio::sync::mpsc::UnboundedSender<Event>) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers
            .entry(topic.to_string())
            .or_default()
            .push(sender);
        println!("📡 Subscribed to topic: {topic}");
    }

    /// ส่ง event ไปยัง subscribers ทั้งหมดของ topic
    pub async fn publish(&self, topic: &str, event: Event) {
        let subscribers = {
            let subs = self.subscribers.lock().unwrap();
            subs.get(topic).cloned().unwrap_or_default()
        };

        println!("📤 Publishing event to topic '{topic}': {event:?}");

        for sender in subscribers {
            if let Err(e) = sender.send(event.clone()) {
                eprintln!("❌ Failed to send event: {e}");
            }
        }
    }
}

/// User Service - จัดการข้อมูลผู้ใช้
#[derive(Debug)]
pub struct UserService {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
    message_bus: MessageBus,
    event_receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserService {
    #[must_use]
    pub fn new(message_bus: MessageBus) -> (Self, tokio::sync::mpsc::UnboundedSender<Event>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let service = Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            message_bus,
            event_receiver: receiver,
        };

        (service, sender)
    }

    pub async fn create_user(&self, name: String, email: String) -> Result<Uuid> {
        let user = User {
            id: Uuid::new_v4(),
            name: name.clone(),
            email: email.clone(),
            created_at: chrono::Utc::now(),
        };

        let user_id = user.id;

        {
            let mut users = self.users.lock().unwrap();
            users.insert(user_id, user);
        }

        // ส่ง event
        self.message_bus
            .publish(
                "user_events",
                Event::UserCreated {
                    user_id,
                    name,
                    email,
                },
            )
            .await;

        println!("👤 Created user: {user_id}");
        Ok(user_id)
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(user_id).cloned()
    }

    pub async fn update_user(
        &self,
        user_id: &Uuid,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<()> {
        {
            let mut users = self.users.lock().unwrap();
            let user = users
                .get_mut(user_id)
                .ok_or_else(|| anyhow::anyhow!("User not found: {}", user_id))?;

            if let Some(new_name) = &name {
                user.name = new_name.clone();
            }

            if let Some(new_email) = &email {
                user.email = new_email.clone();
            }
        }

        // ส่ง event
        self.message_bus
            .publish(
                "user_events",
                Event::UserUpdated {
                    user_id: *user_id,
                    name,
                    email,
                },
            )
            .await;

        println!("👤 Updated user: {user_id}");
        Ok(())
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        {
            let mut users = self.users.lock().unwrap();
            users
                .remove(user_id)
                .ok_or_else(|| anyhow::anyhow!("User not found: {}", user_id))?;
        }

        // ส่ง event
        self.message_bus
            .publish("user_events", Event::UserDeleted { user_id: *user_id })
            .await;

        println!("👤 Deleted user: {user_id}");
        Ok(())
    }

    /// รัน event loop สำหรับ service
    pub async fn run(&mut self) {
        println!("🚀 User Service started");

        while let Some(event) = self.event_receiver.recv().await {
            if let Event::OrderCreated { user_id, .. } = event {
                println!("👤 User Service: Received order created for user {user_id}");
                // อัปเดตสถิติผู้ใช้หรือทำงานอื่นๆ
            }
        }
    }
}

/// Order Service - จัดการคำสั่งซื้อ
#[derive(Debug)]
pub struct OrderService {
    orders: Arc<Mutex<HashMap<Uuid, Order>>>,
    message_bus: MessageBus,
    event_receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub status: OrderStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled,
}

impl OrderService {
    #[must_use]
    pub fn new(message_bus: MessageBus) -> (Self, tokio::sync::mpsc::UnboundedSender<Event>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let service = Self {
            orders: Arc::new(Mutex::new(HashMap::new())),
            message_bus,
            event_receiver: receiver,
        };

        (service, sender)
    }

    pub async fn create_order(&self, user_id: Uuid, amount: f64) -> Result<Uuid> {
        let order = Order {
            id: Uuid::new_v4(),
            user_id,
            amount,
            status: OrderStatus::Pending,
            created_at: chrono::Utc::now(),
        };

        let order_id = order.id;

        {
            let mut orders = self.orders.lock().unwrap();
            orders.insert(order_id, order);
        }

        // ส่ง event
        self.message_bus
            .publish(
                "order_events",
                Event::OrderCreated {
                    order_id,
                    user_id,
                    amount,
                },
            )
            .await;

        println!("🛒 Created order: {order_id} for user: {user_id}");
        Ok(order_id)
    }

    pub async fn complete_order(&self, order_id: &Uuid) -> Result<()> {
        {
            let mut orders = self.orders.lock().unwrap();
            let order = orders
                .get_mut(order_id)
                .ok_or_else(|| anyhow::anyhow!("Order not found: {}", order_id))?;

            order.status = OrderStatus::Completed;
        }

        // ส่ง event
        self.message_bus
            .publish(
                "order_events",
                Event::OrderCompleted {
                    order_id: *order_id,
                },
            )
            .await;

        println!("🛒 Completed order: {order_id}");
        Ok(())
    }

    pub async fn get_order(&self, order_id: &Uuid) -> Option<Order> {
        let orders = self.orders.lock().unwrap();
        orders.get(order_id).cloned()
    }

    /// รัน event loop สำหรับ service
    pub async fn run(&mut self) {
        println!("🚀 Order Service started");

        while let Some(event) = self.event_receiver.recv().await {
            match event {
                Event::PaymentProcessed { order_id, .. } => {
                    println!("🛒 Order Service: Payment processed for order {order_id}");
                    // อัปเดตสถานะคำสั่งซื้อ
                    if let Err(e) = self.complete_order(&order_id).await {
                        eprintln!("❌ Failed to complete order: {e}");
                    }
                }
                Event::UserDeleted { user_id } => {
                    println!("🛒 Order Service: User {user_id} deleted, handling orders");
                    // จัดการคำสั่งซื้อของผู้ใช้ที่ถูกลบ
                }
                _ => {}
            }
        }
    }
}

/// Payment Service - จัดการการชำระเงิน
#[derive(Debug)]
pub struct PaymentService {
    payments: Arc<Mutex<HashMap<Uuid, Payment>>>,
    message_bus: MessageBus,
    event_receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: f64,
    pub status: PaymentStatus,
    pub processed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl PaymentService {
    #[must_use]
    pub fn new(message_bus: MessageBus) -> (Self, tokio::sync::mpsc::UnboundedSender<Event>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let service = Self {
            payments: Arc::new(Mutex::new(HashMap::new())),
            message_bus,
            event_receiver: receiver,
        };

        (service, sender)
    }

    pub async fn process_payment(&self, order_id: Uuid, amount: f64) -> Result<Uuid> {
        // จำลองการประมวลผลการชำระเงิน
        println!("💳 Processing payment for order: {order_id} amount: ${amount:.2}");

        // จำลองเวลาในการประมวลผล
        sleep(Duration::from_millis(100)).await;

        let payment = Payment {
            id: Uuid::new_v4(),
            order_id,
            amount,
            status: PaymentStatus::Completed,
            processed_at: chrono::Utc::now(),
        };

        let payment_id = payment.id;

        {
            let mut payments = self.payments.lock().unwrap();
            payments.insert(payment_id, payment);
        }

        // ส่ง event
        self.message_bus
            .publish(
                "payment_events",
                Event::PaymentProcessed { order_id, amount },
            )
            .await;

        println!("💳 Payment processed: {payment_id} for order: {order_id}");
        Ok(payment_id)
    }

    /// รัน event loop สำหรับ service
    pub async fn run(&mut self) {
        println!("🚀 Payment Service started");

        while let Some(event) = self.event_receiver.recv().await {
            if let Event::OrderCreated {
                order_id, amount, ..
            } = event
            {
                println!("💳 Payment Service: Processing payment for order {order_id}");
                if let Err(e) = self.process_payment(order_id, amount).await {
                    eprintln!("❌ Failed to process payment: {e}");
                }
            }
        }
    }
}

/// Notification Service - จัดการการส่งการแจ้งเตือน
#[derive(Debug)]
pub struct NotificationService {
    message_bus: MessageBus,
    event_receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl NotificationService {
    #[must_use]
    pub fn new(message_bus: MessageBus) -> (Self, tokio::sync::mpsc::UnboundedSender<Event>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let service = Self {
            message_bus,
            event_receiver: receiver,
        };

        (service, sender)
    }

    pub async fn send_notification(&self, user_id: Uuid, message: String) -> Result<()> {
        // จำลองการส่งการแจ้งเตือน (email, SMS, push notification)
        println!("📧 Sending notification to user {user_id}: {message}");

        // จำลองเวลาในการส่ง
        sleep(Duration::from_millis(50)).await;

        // ส่ง event
        self.message_bus
            .publish(
                "notification_events",
                Event::NotificationSent { user_id, message },
            )
            .await;

        Ok(())
    }

    /// รัน event loop สำหรับ service
    pub async fn run(&mut self) {
        println!("🚀 Notification Service started");

        while let Some(event) = self.event_receiver.recv().await {
            match event {
                Event::UserCreated { user_id, name, .. } => {
                    let message = format!("Welcome {name}! Your account has been created.");
                    if let Err(e) = self.send_notification(user_id, message).await {
                        eprintln!("❌ Failed to send welcome notification: {e}");
                    }
                }
                Event::OrderCompleted { order_id } => {
                    // ในระบบจริงต้องดึงข้อมูล user_id จาก order
                    let message = format!("Your order {order_id} has been completed!");
                    println!("📧 Would send notification: {message}");
                }
                Event::PaymentProcessed { order_id, amount } => {
                    let message =
                        format!("Payment of ${amount:.2} for order {order_id} has been processed.");
                    println!("📧 Would send notification: {message}");
                }
                _ => {}
            }
        }
    }
}

/// Service Registry - จัดการ service discovery
#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Arc<Mutex<HashMap<String, ServiceInfo>>>,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check_url: String,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_service(&self, service_info: ServiceInfo) {
        let mut services = self.services.lock().unwrap();
        services.insert(service_info.name.clone(), service_info.clone());
        println!(
            "🔧 Registered service: {} at {}:{}",
            service_info.name, service_info.host, service_info.port
        );
    }

    #[must_use]
    pub fn discover_service(&self, service_name: &str) -> Option<ServiceInfo> {
        let services = self.services.lock().unwrap();
        services.get(service_name).cloned()
    }

    #[must_use]
    pub fn list_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.lock().unwrap();
        services.values().cloned().collect()
    }

    pub async fn health_check(&self) -> HashMap<String, bool> {
        let services = {
            let services = self.services.lock().unwrap();
            services.clone()
        };

        let mut health_status = HashMap::new();

        for (name, service) in services {
            // จำลองการตรวจสอบสุขภาพของ service
            let is_healthy = chrono::Utc::now()
                .signed_duration_since(service.last_heartbeat)
                .num_seconds()
                < 30;
            health_status.insert(name, is_healthy);
        }

        health_status
    }
}

/// Circuit Breaker Pattern สำหรับ fault tolerance
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_count: Arc<Mutex<u32>>,
    failure_threshold: u32,
    timeout: Duration,
    last_failure_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    state: Arc<Mutex<CircuitBreakerState>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitBreakerState {
    Closed,   // ปกติ
    Open,     // มีปัญหา ไม่ให้ผ่าน
    HalfOpen, // ทดสอบว่าหายดีแล้วหรือยัง
}

impl CircuitBreaker {
    #[must_use]
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_count: Arc::new(Mutex::new(0)),
            failure_threshold,
            timeout,
            last_failure_time: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(CircuitBreakerState::Closed)),
        }
    }

    pub async fn call<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
        E: From<anyhow::Error>,
    {
        // ตรวจสอบสถานะ circuit breaker
        {
            let state = self.state.lock().unwrap();
            if *state == CircuitBreakerState::Open {
                let last_failure = self.last_failure_time.lock().unwrap();
                if let Some(last_time) = *last_failure {
                    if chrono::Utc::now().signed_duration_since(last_time)
                        > chrono::Duration::from_std(self.timeout).unwrap()
                    {
                        // เปลี่ยนเป็น HalfOpen
                        drop(state);
                        *self.state.lock().unwrap() = CircuitBreakerState::HalfOpen;
                        println!("🔄 Circuit breaker changed to HalfOpen");
                    } else {
                        return Err(anyhow::anyhow!("Circuit breaker is open").into());
                    }
                }
            }
        }

        // เรียกใช้ operation
        match operation.await {
            Ok(result) => {
                // สำเร็จ - reset failure count
                *self.failure_count.lock().unwrap() = 0;
                *self.state.lock().unwrap() = CircuitBreakerState::Closed;
                Ok(result)
            }
            Err(error) => {
                // ล้มเหลว - เพิ่ม failure count
                let mut count = self.failure_count.lock().unwrap();
                *count += 1;

                if *count >= self.failure_threshold {
                    *self.state.lock().unwrap() = CircuitBreakerState::Open;
                    *self.last_failure_time.lock().unwrap() = Some(chrono::Utc::now());
                    println!("⚠️ Circuit breaker opened due to {count} failures");
                }

                Err(error)
            }
        }
    }

    #[must_use]
    pub fn get_state(&self) -> CircuitBreakerState {
        self.state.lock().unwrap().clone()
    }
}

/// ตัวอย่างการใช้งาน Microservices
#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Microservices Architecture Example");
    println!("=====================================\n");

    // สร้าง Message Bus
    let message_bus = MessageBus::new();

    // สร้าง Service Registry
    let service_registry = ServiceRegistry::new();

    // ลงทะเบียน services
    service_registry.register_service(ServiceInfo {
        name: "user-service".to_string(),
        host: "localhost".to_string(),
        port: 8001,
        health_check_url: "http://localhost:8001/health".to_string(),
        last_heartbeat: chrono::Utc::now(),
    });

    service_registry.register_service(ServiceInfo {
        name: "order-service".to_string(),
        host: "localhost".to_string(),
        port: 8002,
        health_check_url: "http://localhost:8002/health".to_string(),
        last_heartbeat: chrono::Utc::now(),
    });

    // สร้าง services
    let (mut user_service, _user_sender) = UserService::new(message_bus.clone());
    let (mut order_service, order_sender) = OrderService::new(message_bus.clone());
    let (mut payment_service, payment_sender) = PaymentService::new(message_bus.clone());
    let (mut notification_service, notification_sender) =
        NotificationService::new(message_bus.clone());

    // Subscribe services ไปยัง events ที่สนใจ
    message_bus.subscribe("user_events", notification_sender.clone());
    message_bus.subscribe("order_events", payment_sender.clone());
    message_bus.subscribe("order_events", order_sender.clone());
    message_bus.subscribe("payment_events", order_sender.clone());
    message_bus.subscribe("user_events", order_sender.clone());

    // เริ่มรัน services ใน background
    let user_handle = tokio::spawn(async move {
        user_service.run().await;
    });

    let order_handle = tokio::spawn(async move {
        order_service.run().await;
    });

    let payment_handle = tokio::spawn(async move {
        payment_service.run().await;
    });

    let notification_handle = tokio::spawn(async move {
        notification_service.run().await;
    });

    // รอให้ services เริ่มทำงาน
    sleep(Duration::from_millis(100)).await;

    // สร้าง Circuit Breaker สำหรับการเรียก external services
    let circuit_breaker = CircuitBreaker::new(3, Duration::from_secs(5));

    println!("\n🎬 Starting microservices demo...");

    // สร้าง User Service instance สำหรับการทดสอบ
    let user_service_test = UserService::new(message_bus.clone()).0;
    let order_service_test = OrderService::new(message_bus.clone()).0;

    // ตัวอย่างการใช้งาน
    println!("\n👤 Creating users...");
    let user1_id = user_service_test
        .create_user("Alice Johnson".to_string(), "alice@example.com".to_string())
        .await?;

    let user2_id = user_service_test
        .create_user("Bob Smith".to_string(), "bob@example.com".to_string())
        .await?;

    // รอให้ events ถูกประมวลผล
    sleep(Duration::from_millis(200)).await;

    println!("\n🛒 Creating orders...");
    let _order1_id = order_service_test.create_order(user1_id, 99.99).await?;
    let _order2_id = order_service_test.create_order(user2_id, 149.50).await?;

    // รอให้ payment processing เสร็จ
    sleep(Duration::from_millis(300)).await;

    println!("\n👤 Updating user...");
    user_service_test
        .update_user(&user1_id, Some("Alice Johnson-Smith".to_string()), None)
        .await?;

    sleep(Duration::from_millis(100)).await;

    // ทดสอบ Circuit Breaker
    println!("\n🔧 Testing Circuit Breaker...");

    // จำลองการเรียก external service ที่อาจล้มเหลว
    for i in 1..=5 {
        let result = circuit_breaker
            .call(async {
                if i <= 3 {
                    // จำลองการล้มเหลว
                    Err(anyhow::anyhow!("Simulated failure {}", i))
                } else {
                    // จำลองการสำเร็จ
                    Ok(format!("Success on attempt {i}"))
                }
            })
            .await;

        match result {
            Ok(msg) => println!("✅ {msg}"),
            Err(e) => println!("❌ {e}"),
        }

        println!(
            "   Circuit breaker state: {:?}",
            circuit_breaker.get_state()
        );
        sleep(Duration::from_millis(100)).await;
    }

    // ทดสอบ Service Discovery
    println!("\n🔍 Service Discovery...");
    if let Some(user_service_info) = service_registry.discover_service("user-service") {
        println!(
            "Found user service at: {}:{}",
            user_service_info.host, user_service_info.port
        );
    }

    println!("\n📊 All registered services:");
    for service in service_registry.list_services() {
        println!("  - {} at {}:{}", service.name, service.host, service.port);
    }

    // Health Check
    println!("\n🏥 Health Check...");
    let health_status = service_registry.health_check().await;
    for (service, is_healthy) in health_status {
        let status = if is_healthy {
            "✅ Healthy"
        } else {
            "❌ Unhealthy"
        };
        println!("  - {service}: {status}");
    }

    println!("\n🎉 Microservices demo completed!");
    println!("\n📝 Key concepts demonstrated:");
    println!("  - Event-Driven Architecture");
    println!("  - Message Bus / Event Bus");
    println!("  - Service Discovery");
    println!("  - Circuit Breaker Pattern");
    println!("  - Asynchronous Communication");
    println!("  - Fault Tolerance");
    println!("  - Microservices Orchestration");

    // ปิด services
    user_handle.abort();
    order_handle.abort();
    payment_handle.abort();
    notification_handle.abort();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_message_bus() {
        let message_bus = MessageBus::new();
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

        message_bus.subscribe("test_topic", sender);

        let event = Event::UserCreated {
            user_id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        message_bus.publish("test_topic", event.clone()).await;

        let received_event = timeout(Duration::from_millis(100), receiver.recv())
            .await
            .unwrap()
            .unwrap();

        match (event, received_event) {
            (Event::UserCreated { user_id: id1, .. }, Event::UserCreated { user_id: id2, .. }) => {
                assert_eq!(id1, id2);
            }
            _ => panic!("Event types don't match"),
        }
    }

    #[tokio::test]
    async fn test_user_service() {
        let message_bus = MessageBus::new();
        let (user_service, _) = UserService::new(message_bus);

        let user_id = user_service
            .create_user("Test User".to_string(), "test@example.com".to_string())
            .await
            .unwrap();

        let user = user_service.get_user(&user_id).await.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let circuit_breaker = CircuitBreaker::new(2, Duration::from_millis(100));

        // ทดสอบการล้มเหลว
        for _ in 0..2 {
            let result = circuit_breaker
                .call(async { Err::<(), anyhow::Error>(anyhow::anyhow!("Test failure")) })
                .await;
            assert!(result.is_err());
        }

        // ตอนนี้ circuit breaker ควรเปิด
        assert_eq!(circuit_breaker.get_state(), CircuitBreakerState::Open);

        // การเรียกครั้งต่อไปควรถูกปฏิเสธ
        let result = circuit_breaker
            .call(async { Ok::<(), anyhow::Error>(()) })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_service_registry() {
        let registry = ServiceRegistry::new();

        let service_info = ServiceInfo {
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            health_check_url: "http://localhost:8080/health".to_string(),
            last_heartbeat: chrono::Utc::now(),
        };

        registry.register_service(service_info.clone());

        let discovered = registry.discover_service("test-service").unwrap();
        assert_eq!(discovered.name, service_info.name);
        assert_eq!(discovered.port, service_info.port);

        let services = registry.list_services();
        assert_eq!(services.len(), 1);
    }

    #[tokio::test]
    async fn test_order_service() {
        let message_bus = MessageBus::new();
        let (order_service, _) = OrderService::new(message_bus);

        let user_id = Uuid::new_v4();
        let order_id = order_service.create_order(user_id, 99.99).await.unwrap();

        let order = order_service.get_order(&order_id).await.unwrap();
        assert_eq!(order.user_id, user_id);
        assert_eq!(order.amount, 99.99);
        assert!(matches!(order.status, OrderStatus::Pending));

        order_service.complete_order(&order_id).await.unwrap();

        let updated_order = order_service.get_order(&order_id).await.unwrap();
        assert!(matches!(updated_order.status, OrderStatus::Completed));
    }
}
