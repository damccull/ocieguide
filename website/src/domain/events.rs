use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use super::User;

pub struct EquipmentEventData {
    event_id: Uuid,
    aggregate_id: Uuid,
    timestamp: DateTime<Utc>,
    payload: String,
    user: User,
}

pub enum EquipmentEvent {
    Added(EquipmentEventData),
    Edited(EquipmentEventData),
    Retired(EquipmentEventData),
}

pub enum EquipmentProposalEvent {
    Added(EquipmentEventData),
    Edited(EquipmentEventData),
    Retired(EquipmentEventData),
    Approved(EquipmentEventData),
    Rejected(EquipmentEventData),
}
