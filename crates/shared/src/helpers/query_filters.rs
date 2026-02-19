use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

use crate::models::{AttendanceStatus, InvoiceStatus, StudentStatus};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentFilter {
    pub school_id: Option<RecordId>,
    pub class_id: Option<RecordId>,
    pub status: Option<StudentStatus>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceFilter {
    pub school_id: Option<RecordId>,
    pub student_id: Option<RecordId>,
    pub term_id: Option<RecordId>,
    pub status: Option<InvoiceStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceFilter {
    pub student_id: Option<RecordId>,
    pub class_id: Option<RecordId>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub status: Option<AttendanceStatus>,
}
