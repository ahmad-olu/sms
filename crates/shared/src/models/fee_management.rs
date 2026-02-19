use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{Decimal, RecordId};

use crate::models::{
    DeliveryStatus, FeeFrequency, FeeType, InvoiceStatus, PaymentMethod, PaymentStatus,
    ReminderType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    pub id: Option<RecordId>,
    pub school_id: RecordId,
    pub fee_name: String,
    pub fee_type: FeeType,
    pub class_level: String, // Can be ClassLevel or "all"
    pub amount: Decimal,
    pub frequency: FeeFrequency,
    pub session_id: Option<RecordId>,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Option<RecordId>,
    pub student_id: RecordId,
    pub school_id: RecordId,
    pub term_id: RecordId,
    pub invoice_number: String,
    pub total_amount: Decimal,
    pub amount_paid: Decimal,
    pub balance: Decimal,
    pub discount_amount: Decimal,
    pub discount_reason: Option<String>,
    pub due_date: NaiveDate,
    pub status: InvoiceStatus,
    pub generated_by: Option<RecordId>,
    pub generated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Option<RecordId>,
    pub invoice_id: RecordId,
    pub fee_structure_id: Option<RecordId>,
    pub description: Option<String>,
    pub amount: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Option<RecordId>,
    pub invoice_id: RecordId,
    pub student_id: RecordId,
    pub receipt_number: String,
    pub amount_paid: Decimal,
    pub payment_method: PaymentMethod,
    pub payment_reference: Option<String>,
    pub payment_date: DateTime<Utc>,
    pub paid_by: Option<String>,
    pub received_by: Option<RecordId>,
    pub transaction_fee: Decimal,
    pub net_amount: Option<Decimal>,
    pub status: PaymentStatus,
    pub receipt_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReminder {
    pub id: Option<RecordId>,
    pub invoice_id: RecordId,
    pub parent_id: RecordId,
    pub reminder_type: ReminderType,
    pub message: String,
    pub sent_at: DateTime<Utc>,
    pub delivery_status: DeliveryStatus,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
