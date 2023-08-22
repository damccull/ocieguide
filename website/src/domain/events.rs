use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use super::User;

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

pub enum EquipmentProposalOperation {
    Add,
    Edit,
    Retire,
    Approve,
    Reject,
}

pub struct EquipmentProposalEvent {
    event_id: Uuid,
    aggregate_id: Uuid,
    timestamp: DateTime<Utc>,
    operation: EquipmentProposalOperation,
    payload: String,
    user: User,
}
