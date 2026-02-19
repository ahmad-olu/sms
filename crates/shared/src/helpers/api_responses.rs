use serde::{Deserialize, Serialize};
use surrealdb::types::Decimal;

use crate::models::{
    Parent, Student, User,
    academic::Class,
    assessments_nd_grading::{ReportCard, ReportCardScore},
    fee_management::{Invoice, InvoiceItem},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentWithParents {
    #[serde(flatten)]
    pub student: Student,
    pub parents: Vec<Parent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassWithTeacher {
    #[serde(flatten)]
    pub class: Class,
    pub teacher: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceWithItems {
    #[serde(flatten)]
    pub invoice: Invoice,
    pub items: Vec<InvoiceItem>,
    pub student: Student,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCardWithScores {
    #[serde(flatten)]
    pub report_card: ReportCard,
    pub scores: Vec<ReportCardScore>,
    pub student: Student,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub total_students: i64,
    pub total_teachers: i64,
    pub total_classes: i64,
    pub total_collected: Decimal,
    pub total_expected: Decimal,
    pub collection_rate: Decimal,
}
