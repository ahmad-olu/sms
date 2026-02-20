use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::{Decimal, RecordId};
use surrealdb_types::{Datetime, Decimal, RecordId, SurrealValue, Value};

use crate::models::AttendanceStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub id: Option<RecordId>,
    pub student_id: RecordId,
    pub class_id: RecordId,
    pub date: Value, //NaiveDate,
    pub status: AttendanceStatus,
    pub arrival_time: Option<String>,
    pub reason: Option<String>,
    pub marked_by: Option<RecordId>,
    pub marked_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceSummary {
    pub id: Option<RecordId>,
    pub student_id: RecordId,
    pub term_id: RecordId,
    pub total_present: i32,
    pub total_absent: i32,
    pub total_late: i32,
    pub total_excused: i32,
    pub attendance_percentage: Decimal,
    pub updated_at: Datetime,
}
