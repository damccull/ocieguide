use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

pub enum EquipmentOperation {
    Add,
    Edit,
    Retire,
}

pub struct EquipmentEvent {
    event_id: Uuid,
    aggregate_id: Uuid,
    timestamp: DateTime<Utc>,
    operation: EquipmentOperation,
    payload: String,
    user: User,
}

pub struct EquipmentSuggestion {
    event_id: Uuid,
    aggregate_id: Uuid,
    timestamp: DateTime<Utc>,
    operation: EquipmentOperation,
    payload: String,
    user: User,
}
