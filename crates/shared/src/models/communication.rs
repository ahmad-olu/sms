use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::RecordId;
use surrealdb_types::{Datetime, Decimal, RecordId, SurrealValue, Value};

use crate::models::{Priority, RsvpResponse, TargetAudience};

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Announcement {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub title: String,
    pub message: String,
    pub target_audience: TargetAudience,
    pub class_id: Option<RecordId>,
    pub priority: Priority,
    pub send_sms: bool,
    pub send_email: bool,
    pub send_push: bool,
    pub published: bool,
    pub published_at: Option<Datetime>,
    pub created_by: Option<RecordId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub sender_id: RecordId,
    pub recipient_id: RecordId,
    pub subject: Option<String>,
    pub message_body: String,
    pub read: bool,
    pub read_at: Option<Datetime>,
    pub replied: bool,
    pub parent_message_id: Option<RecordId>,
    pub sent_at: Datetime,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub event_name: String,
    pub event_description: Option<String>,
    pub event_date: Value, //NaiveDate,
    pub event_time: Option<String>,
    pub event_location: Option<String>,
    pub target_audience: TargetAudience,
    pub class_id: Option<RecordId>,
    pub requires_rsvp: bool,
    pub rsvp_deadline: Option<Value>, //Option<NaiveDate>,
    pub created_by: Option<RecordId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct EventRsvp {
    pub id: Option<RecordId>,
    pub event_id: RecordId,
    pub user_id: RecordId,
    pub student_id: Option<RecordId>,
    pub response: RsvpResponse,
    pub number_of_guests: i32,
    pub responded_at: Datetime,
    pub created_at: Datetime,
}
