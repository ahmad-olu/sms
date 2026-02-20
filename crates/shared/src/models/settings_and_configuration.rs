use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::RecordId;
use surrealdb_types::{Datetime, Decimal, RecordId, SurrealValue, Value};

use crate::models::SettingType;

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct SchoolSetting {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub setting_type: SettingType,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct ReportCardTemplate {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub template_name: String,
    pub template_html: Option<String>,
    pub is_default: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
