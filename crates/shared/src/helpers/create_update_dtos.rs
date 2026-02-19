use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::types::{Decimal, RecordId};

use crate::models::{AttendanceStatus, Gender, PaymentMethod, SubscriptionPlan, UserType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSchoolDto {
    pub school_name: String,
    pub school_address: Option<String>,
    pub school_phone: Option<String>,
    pub school_email: Option<String>,
    pub subscription_plan: SubscriptionPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub school_id: RecordId,
    pub user_type: UserType,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String, // Plain password, will be hashed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStudentDto {
    pub school_id: RecordId,
    pub admission_number: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Gender,
    pub address: Option<String>,
    pub current_class_id: Option<RecordId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkAttendanceDto {
    pub student_id: RecordId,
    pub class_id: RecordId,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub arrival_time: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterScoreDto {
    pub assessment_id: RecordId,
    pub student_id: RecordId,
    pub score_obtained: Decimal,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPaymentDto {
    pub invoice_id: RecordId,
    pub student_id: RecordId,
    pub amount_paid: Decimal,
    pub payment_method: PaymentMethod,
    pub payment_reference: Option<String>,
    pub paid_by: Option<String>,
}
