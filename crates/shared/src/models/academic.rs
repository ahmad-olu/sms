use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::RecordId;
use surrealdb_types::{Datetime, RecordId, SurrealValue, Value};

use crate::models::{ClassLevel, SubjectType};

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct AcademicSession {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub session_name: String,
    pub start_date: Value, // NaiveDate,
    pub end_date: Value,   // NaiveDate,
    pub is_current: bool,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Term {
    pub id: Option<RecordId>,
    pub session_id: RecordId,
    pub term_number: i32,
    pub term_name: Option<String>,
    pub start_date: Value, // NaiveDate,
    pub end_date: Value,   // NaiveDate,
    pub is_current: bool,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Class {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub class_name: String,
    pub class_level: ClassLevel,
    pub class_teacher_id: Option<RecordId>,
    pub capacity: Option<i32>,
    pub current_enrollment: i32,
    pub session_id: Option<RecordId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Subject {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub subject_name: String,
    pub subject_code: Option<String>,
    pub subject_type: SubjectType,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct ClassSubject {
    pub id: Option<RecordId>,
    pub class_id: RecordId,
    pub subject_id: RecordId,
    pub teacher_id: Option<RecordId>,
    pub created_at: Datetime,
}
