//! State Pattern Implementation
//!
//! State Pattern ช่วยให้ object สามารถเปลี่ยน behavior ได้เมื่อ internal state เปลี่ยน
//! ใน Rust เราใช้ enum และ trait เพื่อ implement pattern นี้อย่างมีประสิทธิภาพ

use std::fmt;

/// State ของ Traffic Light
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrafficLightState {
    Red { remaining_seconds: u32 },
    Yellow { remaining_seconds: u32 },
    Green { remaining_seconds: u32 },
    Flashing,
    Maintenance,
}

impl TrafficLightState {
    /// เปลี่ยนไปยัง state ถัดไป
    #[must_use]
    pub const fn next_state(self) -> Self {
        match self {
            Self::Red { remaining_seconds } => {
                if remaining_seconds > 0 {
                    Self::Red { remaining_seconds: remaining_seconds - 1 }
                } else {
                    Self::Green { remaining_seconds: 30 }
                }
            }
            Self::Yellow { remaining_seconds } => {
                if remaining_seconds > 0 {
                    Self::Yellow { remaining_seconds: remaining_seconds - 1 }
                } else {
                    Self::Red { remaining_seconds: 45 }
                }
            }
            Self::Green { remaining_seconds } => {
                if remaining_seconds > 0 {
                    Self::Green { remaining_seconds: remaining_seconds - 1 }
                } else {
                    Self::Yellow { remaining_seconds: 5 }
                }
            }
            Self::Flashing => Self::Flashing, // ยังคง flashing
            Self::Maintenance => Self::Maintenance, // ยังคง maintenance
        }
    }

    /// ตรวจสอบว่าสามารถข้ามได้หรือไม่
    #[must_use]
    pub const fn can_cross(&self) -> bool {
        matches!(self, Self::Green { .. })
    }

    /// ตรวจสอบว่าต้องหยุดหรือไม่
    #[must_use]
    pub const fn must_stop(&self) -> bool {
        matches!(self, Self::Red { .. })
    }

    /// ตรวจสอบว่าต้องระวังหรือไม่
    #[must_use]
    pub const fn should_be_careful(&self) -> bool {
        matches!(self, Self::Yellow { .. } | Self::Flashing)
    }

    /// เข้าสู่โหมด maintenance
    #[must_use]
    pub const fn enter_maintenance(self) -> Self {
        Self::Maintenance
    }

    /// ออกจากโหมด maintenance
    #[must_use]
    pub const fn exit_maintenance(self) -> Self {
        Self::Red { remaining_seconds: 45 }
    }

    /// เข้าสู่โหมด flashing (emergency)
    #[must_use]
    pub const fn enter_flashing(self) -> Self {
        Self::Flashing
    }

    /// ออกจากโหมด flashing
    #[must_use]
    pub const fn exit_flashing(self) -> Self {
        Self::Red { remaining_seconds: 45 }
    }
}

impl fmt::Display for TrafficLightState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red { remaining_seconds } => write!(f, "🔴 RED ({remaining_seconds}s)"),
            Self::Yellow { remaining_seconds } => write!(f, "🟡 YELLOW ({remaining_seconds}s)"),
            Self::Green { remaining_seconds } => write!(f, "🟢 GREEN ({remaining_seconds}s)"),
            Self::Flashing => write!(f, "🟡 FLASHING"),
            Self::Maintenance => write!(f, "🔧 MAINTENANCE"),
        }
    }
}

/// Traffic Light Controller
#[derive(Debug)]
pub struct TrafficLight {
    state: TrafficLightState,
    intersection_id: String,
}

impl TrafficLight {
    /// สร้าง traffic light ใหม่
    #[must_use]
    pub const fn new(intersection_id: String) -> Self {
        Self {
            state: TrafficLightState::Red { remaining_seconds: 45 },
            intersection_id,
        }
    }

    /// อัปเดต state (เรียกทุกวินาที)
    pub fn tick(&mut self) {
        self.state = self.state.clone().next_state();
    }

    /// ดู state ปัจจุบัน
    #[must_use]
    pub const fn current_state(&self) -> &TrafficLightState {
        &self.state
    }

    /// ตรวจสอบว่าสามารถข้ามได้หรือไม่
    #[must_use]
    pub const fn can_cross(&self) -> bool {
        self.state.can_cross()
    }

    /// เข้าสู่โหมด maintenance
    pub fn enter_maintenance(&mut self) {
        self.state = self.state.clone().enter_maintenance();
    }

    /// ออกจากโหมด maintenance
    pub fn exit_maintenance(&mut self) {
        self.state = self.state.clone().exit_maintenance();
    }

    /// เข้าสู่โหมด emergency (flashing)
    pub fn emergency_mode(&mut self) {
        self.state = self.state.clone().enter_flashing();
    }

    /// ออกจากโหมด emergency
    pub fn normal_mode(&mut self) {
        self.state = self.state.clone().exit_flashing();
    }

    /// แสดงสถานะปัจจุบัน
    pub fn display_status(&self) {
        println!("🚦 Intersection {}: {}", self.intersection_id, self.state);
        
        match &self.state {
            TrafficLightState::Red { .. } => println!("   ⛔ STOP - Do not cross"),
            TrafficLightState::Yellow { .. } => println!("   ⚠️ CAUTION - Prepare to stop"),
            TrafficLightState::Green { .. } => println!("   ✅ GO - Safe to cross"),
            TrafficLightState::Flashing => println!("   🚨 EMERGENCY - Treat as stop sign"),
            TrafficLightState::Maintenance => println!("   🔧 OUT OF SERVICE"),
        }
    }
}

/// State สำหรับ Document Workflow
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentState {
    Draft {
        author: String,
        last_modified: String,
    },
    UnderReview {
        reviewer: String,
        submitted_at: String,
    },
    Approved {
        approved_by: String,
        approved_at: String,
    },
    Published {
        published_at: String,
        version: u32,
    },
    Rejected {
        rejected_by: String,
        reason: String,
        rejected_at: String,
    },
    Archived {
        archived_at: String,
    },
}

impl DocumentState {
    /// ส่งเอกสารเพื่อ review
    #[must_use]
    pub fn submit_for_review(self, reviewer: String, timestamp: String) -> Result<Self, String> {
        match self {
            Self::Draft { .. } => Ok(Self::UnderReview {
                reviewer,
                submitted_at: timestamp,
            }),
            _ => Err("Can only submit draft documents for review".to_string()),
        }
    }

    /// อนุมัติเอกสาร
    #[must_use]
    pub fn approve(self, approver: String, timestamp: String) -> Result<Self, String> {
        match self {
            Self::UnderReview { .. } => Ok(Self::Approved {
                approved_by: approver,
                approved_at: timestamp,
            }),
            _ => Err("Can only approve documents under review".to_string()),
        }
    }

    /// ปฏิเสธเอกสาร
    #[must_use]
    pub fn reject(self, rejector: String, reason: String, timestamp: String) -> Result<Self, String> {
        match self {
            Self::UnderReview { .. } => Ok(Self::Rejected {
                rejected_by: rejector,
                reason,
                rejected_at: timestamp,
            }),
            _ => Err("Can only reject documents under review".to_string()),
        }
    }

    /// เผยแพร่เอกสาร
    #[must_use]
    pub fn publish(self, timestamp: String, version: u32) -> Result<Self, String> {
        match self {
            Self::Approved { .. } => Ok(Self::Published {
                published_at: timestamp,
                version,
            }),
            _ => Err("Can only publish approved documents".to_string()),
        }
    }

    /// เก็บเอกสารเข้าคลัง
    #[must_use]
    pub fn archive(self, timestamp: String) -> Result<Self, String> {
        match self {
            Self::Published { .. } | Self::Rejected { .. } => Ok(Self::Archived {
                archived_at: timestamp,
            }),
            _ => Err("Can only archive published or rejected documents".to_string()),
        }
    }

    /// แก้ไขเอกสารที่ถูกปฏิเสธ
    #[must_use]
    pub fn revise(self, author: String, timestamp: String) -> Result<Self, String> {
        match self {
            Self::Rejected { .. } => Ok(Self::Draft {
                author,
                last_modified: timestamp,
            }),
            _ => Err("Can only revise rejected documents".to_string()),
        }
    }
}

impl fmt::Display for DocumentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft { author, last_modified } => {
                write!(f, "📝 DRAFT (by {author}, modified: {last_modified})")
            }
            Self::UnderReview { reviewer, submitted_at } => {
                write!(f, "👀 UNDER REVIEW (by {reviewer}, submitted: {submitted_at})")
            }
            Self::Approved { approved_by, approved_at } => {
                write!(f, "✅ APPROVED (by {approved_by}, at: {approved_at})")
            }
            Self::Published { published_at, version } => {
                write!(f, "🌐 PUBLISHED (v{version}, at: {published_at})")
            }
            Self::Rejected { rejected_by, reason, rejected_at } => {
                write!(f, "❌ REJECTED (by {rejected_by}, reason: {reason}, at: {rejected_at})")
            }
            Self::Archived { archived_at } => {
                write!(f, "📦 ARCHIVED (at: {archived_at})")
            }
        }
    }
}

/// Document Workflow Manager
#[derive(Debug)]
pub struct Document {
    id: String,
    title: String,
    content: String,
    state: DocumentState,
}

impl Document {
    /// สร้างเอกสารใหม่
    #[must_use]
    pub fn new(id: String, title: String, content: String, author: String) -> Self {
        Self {
            id,
            title,
            content,
            state: DocumentState::Draft {
                author,
                last_modified: "2024-01-01T10:00:00Z".to_string(),
            },
        }
    }

    /// ส่งเพื่อ review
    pub fn submit_for_review(&mut self, reviewer: String) -> Result<(), String> {
        self.state = self.state.clone().submit_for_review(reviewer, "2024-01-01T11:00:00Z".to_string())?;
        Ok(())
    }

    /// อนุมัติเอกสาร
    pub fn approve(&mut self, approver: String) -> Result<(), String> {
        self.state = self.state.clone().approve(approver, "2024-01-01T12:00:00Z".to_string())?;
        Ok(())
    }

    /// ปฏิเสธเอกสาร
    pub fn reject(&mut self, rejector: String, reason: String) -> Result<(), String> {
        self.state = self.state.clone().reject(rejector, reason, "2024-01-01T12:30:00Z".to_string())?;
        Ok(())
    }

    /// เผยแพร่เอกสาร
    pub fn publish(&mut self, version: u32) -> Result<(), String> {
        self.state = self.state.clone().publish("2024-01-01T13:00:00Z".to_string(), version)?;
        Ok(())
    }

    /// แสดงสถานะเอกสาร
    pub fn display_status(&self) {
        println!("📄 Document: {} (ID: {})", self.title, self.id);
        println!("   State: {}", self.state);
    }
}

/// สาธิต State Pattern
pub fn demonstrate_state() {
    println!("🔄 State Pattern Examples:");
    
    // Traffic Light Example
    println!("\n🚦 Traffic Light State Machine:");
    let mut traffic_light = TrafficLight::new("Main-St-1st-Ave".to_string());
    
    // แสดงการเปลี่ยน state ตามเวลา
    for i in 1..=10 {
        println!("\nSecond {i}:");
        traffic_light.display_status();
        traffic_light.tick();
    }
    
    // Emergency mode
    println!("\n🚨 Emergency Mode:");
    traffic_light.emergency_mode();
    traffic_light.display_status();
    
    // Back to normal
    println!("\n🔄 Back to Normal:");
    traffic_light.normal_mode();
    traffic_light.display_status();
    
    // Document Workflow Example
    println!("\n\n📄 Document Workflow State Machine:");
    let mut doc = Document::new(
        "DOC-001".to_string(),
        "API Documentation".to_string(),
        "This is the API documentation content...".to_string(),
        "Alice".to_string(),
    );
    
    doc.display_status();
    
    // Submit for review
    println!("\n📤 Submitting for review...");
    if let Err(e) = doc.submit_for_review("Bob".to_string()) {
        println!("Error: {e}");
    } else {
        doc.display_status();
    }
    
    // Approve document
    println!("\n✅ Approving document...");
    if let Err(e) = doc.approve("Bob".to_string()) {
        println!("Error: {e}");
    } else {
        doc.display_status();
    }
    
    // Publish document
    println!("\n🌐 Publishing document...");
    if let Err(e) = doc.publish(1) {
        println!("Error: {e}");
    } else {
        doc.display_status();
    }
    
    // Try invalid transition
    println!("\n❌ Trying invalid transition (submit published doc for review)...");
    if let Err(e) = doc.submit_for_review("Charlie".to_string()) {
        println!("Error: {e}");
    }
    
    println!("\n💡 State Pattern Benefits:");
    println!("  • Clear state transitions");
    println!("  • Type-safe state management");
    println!("  • Prevents invalid operations");
    println!("  • Easy to extend with new states");
    println!("  • Compile-time validation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_transitions() {
        let mut light = TrafficLight::new("test".to_string());
        
        // Should start with Red
        assert!(matches!(light.current_state(), TrafficLightState::Red { .. }));
        assert!(!light.can_cross());
        
        // Tick through red countdown (45 seconds + 1 to transition)
        for _ in 0..46 {
            light.tick();
        }
        
        // Should now be Green
        assert!(matches!(light.current_state(), TrafficLightState::Green { .. }));
        assert!(light.can_cross());
    }

    #[test]
    fn test_document_workflow() {
        let mut doc = Document::new(
            "test".to_string(),
            "Test Doc".to_string(),
            "Content".to_string(),
            "Author".to_string(),
        );
        
        // Should start as Draft
        assert!(matches!(doc.state, DocumentState::Draft { .. }));
        
        // Submit for review
        assert!(doc.submit_for_review("Reviewer".to_string()).is_ok());
        assert!(matches!(doc.state, DocumentState::UnderReview { .. }));
        
        // Approve
        assert!(doc.approve("Approver".to_string()).is_ok());
        assert!(matches!(doc.state, DocumentState::Approved { .. }));
        
        // Publish
        assert!(doc.publish(1).is_ok());
        assert!(matches!(doc.state, DocumentState::Published { .. }));
    }

    #[test]
    fn test_invalid_transitions() {
        let mut doc = Document::new(
            "test".to_string(),
            "Test Doc".to_string(),
            "Content".to_string(),
            "Author".to_string(),
        );
        
        // Try to approve draft (should fail)
        assert!(doc.approve("Approver".to_string()).is_err());
        
        // Try to publish draft (should fail)
        assert!(doc.publish(1).is_err());
    }
}