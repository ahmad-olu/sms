use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::{Decimal, RecordId};
use surrealdb_types::{Datetime, Decimal, RecordId, SurrealValue, Value};

use crate::models::AssessmentType;

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Assessment {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub class_id: RecordId,
    pub subject_id: RecordId,
    pub term_id: RecordId,
    pub assessment_type: AssessmentType,
    pub assessment_name: Option<String>,
    pub max_score: Decimal,
    pub assessment_date: Option<Value>, //Option<NaiveDate>,
    pub created_by: Option<RecordId>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct StudentScore {
    pub id: Option<RecordId>,
    pub assessment_id: RecordId,
    pub student_id: RecordId,
    pub score_obtained: Option<Decimal>,
    pub remarks: Option<String>,
    pub entered_by: Option<RecordId>,
    pub entered_at: Datetime,
    pub updated_at: Datetime,
    pub verified: bool,
    pub verified_by: Option<RecordId>,
    pub verified_at: Option<Datetime>,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct GradingScheme {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub scheme_name: Option<String>,
    pub min_score: Decimal,
    pub max_score: Decimal,
    pub grade: String,
    pub grade_point: Decimal,
    pub remark: Option<String>,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct ReportCard {
    pub id: Option<RecordId>,
    pub student_id: RecordId,
    pub class_id: RecordId,
    pub term_id: RecordId,
    pub total_score: Option<Decimal>,
    pub average_percentage: Option<Decimal>,
    pub class_position: Option<i32>,
    pub total_students_in_class: Option<i32>,
    pub class_average: Option<Decimal>,
    pub attendance_present: Option<i32>,
    pub attendance_absent: Option<i32>,
    pub attendance_late: Option<i32>,
    pub times_school_opened: Option<i32>,
    pub teacher_comment: Option<String>,
    pub principal_comment: Option<String>,
    pub next_term_begins: Option<Value>, //Option<NaiveDate>,
    pub generated_by: Option<RecordId>,
    pub generated_at: Datetime,
    pub published: bool,
    pub published_at: Option<Datetime>,
    pub pdf_url: Option<String>,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct ReportCardScore {
    pub id: Option<RecordId>,
    pub report_card_id: RecordId,
    pub subject_id: RecordId,
    pub ca_score: Option<Decimal>,
    pub exam_score: Option<Decimal>,
    pub total_score: Option<Decimal>,
    pub grade: Option<String>,
    pub subject_position: Option<i32>,
    pub subject_high_score: Option<Decimal>,
    pub subject_low_score: Option<Decimal>,
    pub subject_average: Option<Decimal>,
    pub remarks: Option<String>,
}
