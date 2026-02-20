use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
// use surrealdb::types::{RecordId, SurrealValue};
use surrealdb_types::{Datetime, RecordId, SurrealValue, Value};

use crate::models::{
    academic::{Class, Subject},
    fee_management::Invoice,
};

pub mod academic;
pub mod assessments_nd_grading;
pub mod attendance;
pub mod communication;
pub mod fee_management;
pub mod settings_and_configuration;
pub mod system_and_audit;

pub mod modules {
    pub use super::academic as acad;
    pub use super::assessments_nd_grading as grading;
    pub use super::attendance as attend;
    pub use super::communication as comm;
    pub use super::fee_management as fees;
    pub use super::settings_and_configuration as config;
    pub use super::system_and_audit as audit;
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Starter,
    Basic,
    Standard,
    Premium,
    Enterprise,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Trial,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    SuperAdmin,
    Admin,
    Teacher,
    Accountant,
    Parent,
    Student,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Suspended,
    Inactive,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StudentStatus {
    Active,
    Graduated,
    Withdrawn,
    Suspended,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    Father,
    Mother,
    Guardian,
    Other,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClassLevel {
    Nursery,
    Primary,
    JuniorSecondary,
    SeniorSecondary,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Core,
    Elective,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentType {
    Ca1,
    Ca2,
    Ca3,
    Exam,
    Test,
    Assignment,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    Excused,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    Tuition,
    Transport,
    Feeding,
    Development,
    Exam,
    Uniform,
    Books,
    Other,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeeFrequency {
    PerTerm,
    PerSession,
    OneTime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Unpaid,
    Partial,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Card,
    Paystack,
    Flutterwave,
    Ussd,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Successful,
    Failed,
    Refunded,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReminderType {
    Sms,
    Email,
    Push,
    Whatsapp,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Sent,
    Delivered,
    Failed,
    Bounced,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TargetAudience {
    All,
    Parents,
    Teachers,
    Students,
    SpecificClass,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RsvpResponse {
    Attending,
    NotAttending,
    Maybe,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Login,
    Logout,
    Create,
    Update,
    Delete,
    View,
    Export,
    Publish,
    Payment,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    FeeReminder,
    ResultPublished,
    AttendanceAlert,
    Announcement,
    Message,
    Event,
    System,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SmsType {
    FeeReminder,
    AttendanceAlert,
    Announcement,
    Event,
    ResultNotification,
    Other,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SettingType {
    Text,
    Number,
    Boolean,
    Json,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct School {
    pub id: Option<RecordId>,
    pub school_name: String,
    pub school_address: Option<String>,
    pub school_phone: Option<String>,
    pub school_email: Option<String>,
    pub school_logo_url: Option<String>,
    pub school_motto: Option<String>,
    pub total_students: i32,
    pub subscription_plan: SubscriptionPlan,
    pub subscription_expiry_date: Option<Datetime>,
    pub subscription_status: SubscriptionStatus,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub user_type: UserType,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub status: UserStatus,
    pub last_login: Option<Datetime>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub admission_number: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub date_of_birth: Value, //NaiveDate,
    pub gender: Gender,
    pub address: Option<String>,
    pub current_class_id: Option<RecordId>,
    pub admission_date: Value, //NaiveDate,
    pub status: StudentStatus,
    pub profile_photo_url: Option<String>,
    pub blood_group: Option<String>,
    pub genotype: Option<String>,
    pub medical_conditions: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct Parent {
    pub id: Option<RecordId>,
    pub user_id: RecordId,
    pub relationship_type: RelationshipType,
    pub occupation: Option<String>,
    pub home_address: Option<String>,
    pub office_address: Option<String>,
    pub emergency_contact: Option<String>,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, SurrealValue, Serialize, Deserialize)]
pub struct StudentParent {
    pub id: Option<RecordId>,
    #[serde(rename = "in")]
    pub student_id: RecordId,
    pub out: RecordId, // parent_id
    pub primary_contact: bool,
    pub created_at: Datetime,
}

pub trait SchoolScoped {
    fn school_id(&self) -> &RecordId;
}

pub trait Timestamped {
    fn created_at(&self) -> Datetime;
    fn updated_at(&self) -> Option<Datetime>;
}

pub trait SoftDeletable {
    fn is_deleted(&self) -> bool;
    fn deleted_at(&self) -> Option<Datetime>;
}

pub trait Auditable {
    fn created_by(&self) -> Option<&RecordId>;
    fn updated_by(&self) -> Option<&RecordId>;
}

impl SchoolScoped for School {
    fn school_id(&self) -> &RecordId {
        self.id.as_ref().unwrap()
    }
}

impl SchoolScoped for User {
    fn school_id(&self) -> &RecordId {
        &self.school_id
    }
}

impl SchoolScoped for Student {
    fn school_id(&self) -> &RecordId {
        &self.school_id
    }
}

impl SchoolScoped for Class {
    fn school_id(&self) -> &RecordId {
        &self.school_id
    }
}

impl SchoolScoped for Subject {
    fn school_id(&self) -> &RecordId {
        &self.school_id
    }
}

impl SchoolScoped for Invoice {
    fn school_id(&self) -> &RecordId {
        &self.school_id
    }
}

impl Timestamped for School {
    fn created_at(&self) -> Datetime {
        self.created_at
    }

    fn updated_at(&self) -> Option<Datetime> {
        Some(self.updated_at)
    }
}

impl Timestamped for User {
    fn created_at(&self) -> Datetime {
        self.created_at
    }

    fn updated_at(&self) -> Option<Datetime> {
        Some(self.updated_at)
    }
}

impl Timestamped for Student {
    fn created_at(&self) -> Datetime {
        self.created_at
    }

    fn updated_at(&self) -> Option<Datetime> {
        Some(self.updated_at)
    }
}
