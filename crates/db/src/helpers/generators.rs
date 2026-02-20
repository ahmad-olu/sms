use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::error::DbResult;

/// Generate invoice number: INV-{YEAR}-{6 digit padded count}
pub fn generate_invoice_number(year: i32, count: u64) -> String {
    todo!()
}

/// Generate receipt number: RCP-{YEAR}-{6 digit padded count}
pub fn generate_receipt_number(year: i32, count: u64) -> String {
    todo!()
}

/// Generate admission number: {YEAR}/{3 digit padded count}
pub fn generate_admission_number(year: i32, count: u64) -> String {
    todo!()
}

/// Get next invoice sequence number for a school
pub async fn next_invoice_seq(sdb: &Surreal<Client>, school_id: String) -> DbResult<u64> {
    todo!()
}

/// Get next receipt sequence number for a school
pub async fn next_receipt_seq(sdb: &Surreal<Client>, school_id: String) -> DbResult<u64> {
    todo!()
}
