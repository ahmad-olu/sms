use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::{Decimal, RecordId};
use surrealdb_types::{Datetime, Decimal, RecordId, SurrealValue, Value};

use crate::models::{ActionType, DeliveryStatus, NotificationType, SmsType};

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub user_id: Option<RecordId>,
    pub action: ActionType,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Notification {
    pub id: Option<RecordId>,
    pub user_id: RecordId,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub link_url: Option<String>,
    pub read: bool,
    pub read_at: Option<Datetime>,
    pub sent_at: Datetime,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct SmsLog {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub recipient_phone: String,
    pub recipient_user_id: Option<RecordId>,
    pub message: String,
    pub sms_type: SmsType,
    pub provider: Option<String>,
    pub provider_message_id: Option<String>,
    pub status: DeliveryStatus,
    pub cost: Option<Decimal>,
    pub sent_at: Option<Datetime>,
    pub delivered_at: Option<Datetime>,
    pub created_at: Datetime,
}
