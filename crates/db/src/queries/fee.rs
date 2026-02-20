use crate::error::DbResult;
use shared::models::{
    InvoiceStatus,
    fee_management::{FeeStructure, Invoice, InvoiceItem, Payment, PaymentReminder},
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::Value;

const FEE_STRUCTURE_TABLE: &str = "fee_structures";
const INVOICE_TABLE: &str = "invoices";
const INVOICE_ITEM_TABLE: &str = "invoice_items";
const PAYMENT_TABLE: &str = "payments";
const PAYMENT_REMINDER_TABLE: &str = "payment_reminders";

pub struct FeeStructureQ;

impl FeeStructureQ {
    /// Create a fee structure
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: FeeStructure,
    ) -> DbResult<Option<FeeStructure>> {
        let fee: Option<FeeStructure> = sdb.create(FEE_STRUCTURE_TABLE).content(data).await?;
        Ok(fee)
    }

    /// Get fee structures for a class level in a session
    pub async fn get_by_level_and_session(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        level: String,
        session_id: String,
    ) -> DbResult<Vec<FeeStructure>> {
        let fees: Vec<FeeStructure> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND (class_level = $level OR class_level = 'all')
                AND session_id = type::thing('academic_sessions', $session_id)
                "#,
            )
            .bind(("table", FEE_STRUCTURE_TABLE))
            .bind(("school_id", school_id))
            .bind(("level", level))
            .bind(("session_id", session_id))
            .await?
            .take(0)?;
        Ok(fees)
    }

    /// Get all fee structures for a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<FeeStructure>> {
        let fees: Vec<FeeStructure> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY fee_type, class_level
                "#,
            )
            .bind(("table", FEE_STRUCTURE_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(fees)
    }
}

pub struct InvoiceQ;

impl InvoiceQ {
    /// Create an invoice
    pub async fn create(&self, sdb: &Surreal<Client>, data: Invoice) -> DbResult<Option<Invoice>> {
        let invoice: Option<Invoice> = sdb.create(INVOICE_TABLE).content(data).await?;
        Ok(invoice)
    }

    /// Add an item to an invoice
    pub async fn add_item(
        &self,
        sdb: &Surreal<Client>,
        data: InvoiceItem,
    ) -> DbResult<Option<InvoiceItem>> {
        let item: Option<InvoiceItem> = sdb.create(INVOICE_ITEM_TABLE).content(data).await?;
        Ok(item)
    }

    /// Get invoice for a student in a term
    pub async fn get_by_student_term(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_id: String,
    ) -> DbResult<Option<Invoice>> {
        let invoice: Option<Invoice> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND term_id = type::thing('terms', $term_id)
                LIMIT 1
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(invoice)
    }

    /// Get invoice with items
    pub async fn get_with_items(
        &self,
        sdb: &Surreal<Client>,
        invoice_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *,
                    student_id.*,
                    (SELECT * FROM type::table($item_table)
                     WHERE invoice_id = $parent.id) AS items
                FROM type::thing($table, $id)
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("item_table", INVOICE_ITEM_TABLE))
            .bind(("id", invoice_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get invoices by status for a school
    pub async fn get_by_status(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        status: InvoiceStatus,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *, student_id.* AS student
                FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND status = $status
                ORDER BY due_date
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("school_id", school_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get all invoices for a student
    pub async fn get_by_student(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Vec<Invoice>> {
        let invoices: Vec<Invoice> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                ORDER BY created_at DESC
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("student_id", student_id))
            .await?
            .take(0)?;
        Ok(invoices)
    }

    /// Update invoice balances after payment
    pub async fn apply_payment(
        &self,
        sdb: &Surreal<Client>,
        invoice_id: String,
        payment_amount: f64,
    ) -> DbResult<Option<Invoice>> {
        let invoice: Option<Invoice> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    amount_paid = amount_paid + $payment_amount,
                    balance = balance - $payment_amount,
                    status = IF((balance - $payment_amount) <= 0, 'paid', IF(amount_paid > 0, 'partial', status)),
                    updated_at = time::now()
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("id", invoice_id))
            .bind(("payment_amount", payment_amount))
            .await?
            .take(0)?;
        Ok(invoice)
    }

    /// Get fee collection summary for a term
    pub async fn get_collection_summary(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        term_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT
                    count() AS total_invoices,
                    math::sum(total_amount) AS total_expected,
                    math::sum(amount_paid) AS total_collected,
                    math::sum(balance) AS total_outstanding
                FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND term_id = type::thing('terms', $term_id)
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("school_id", school_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get fee collection grouped by class
    pub async fn get_collection_by_class(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        term_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT
                    student_id.current_class_id.class_name AS class_name,
                    count() AS total_invoices,
                    math::sum(amount_paid) AS total_collected,
                    math::sum(balance) AS total_outstanding
                FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND term_id = type::thing('terms', $term_id)
                GROUP BY student_id.current_class_id
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("school_id", school_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Cancel an invoice
    pub async fn cancel(
        &self,
        sdb: &Surreal<Client>,
        invoice_id: String,
    ) -> DbResult<Option<Invoice>> {
        let invoice: Option<Invoice> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    status = 'cancelled',
                    updated_at = time::now()
                "#,
            )
            .bind(("table", INVOICE_TABLE))
            .bind(("id", invoice_id))
            .await?
            .take(0)?;
        Ok(invoice)
    }
}

pub struct PaymentQ;

impl PaymentQ {
    /// Record a payment
    pub async fn record(&self, sdb: &Surreal<Client>, data: Payment) -> DbResult<Option<Payment>> {
        let payment: Option<Payment> = sdb.create(PAYMENT_TABLE).content(data).await?;
        Ok(payment)
    }

    /// Get payment history for a student
    pub async fn get_by_student(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Vec<Payment>> {
        let payments: Vec<Payment> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                ORDER BY payment_date DESC
                "#,
            )
            .bind(("table", PAYMENT_TABLE))
            .bind(("student_id", student_id))
            .await?
            .take(0)?;
        Ok(payments)
    }

    /// Get payments for an invoice
    pub async fn get_by_invoice(
        &self,
        sdb: &Surreal<Client>,
        invoice_id: String,
    ) -> DbResult<Vec<Payment>> {
        let payments: Vec<Payment> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE invoice_id = type::thing('invoices', $invoice_id)
                ORDER BY payment_date DESC
                "#,
            )
            .bind(("table", PAYMENT_TABLE))
            .bind(("invoice_id", invoice_id))
            .await?
            .take(0)?;
        Ok(payments)
    }

    /// Get payment by receipt number
    pub async fn get_by_receipt(
        &self,
        sdb: &Surreal<Client>,
        receipt_number: String,
    ) -> DbResult<Option<Payment>> {
        let payment: Option<Payment> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE receipt_number = $receipt_number
                LIMIT 1
                "#,
            )
            .bind(("table", PAYMENT_TABLE))
            .bind(("receipt_number", receipt_number))
            .await?
            .take(0)?;
        Ok(payment)
    }
}

pub struct PaymentReminderQ;

impl PaymentReminderQ {
    /// Create a payment reminder log
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: PaymentReminder,
    ) -> DbResult<Option<PaymentReminder>> {
        let reminder: Option<PaymentReminder> =
            sdb.create(PAYMENT_REMINDER_TABLE).content(data).await?;
        Ok(reminder)
    }

    /// Get reminders for an invoice
    pub async fn get_by_invoice(
        &self,
        sdb: &Surreal<Client>,
        invoice_id: String,
    ) -> DbResult<Vec<PaymentReminder>> {
        let reminders: Vec<PaymentReminder> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE invoice_id = type::thing('invoices', $invoice_id)
                ORDER BY sent_at DESC
                "#,
            )
            .bind(("table", PAYMENT_REMINDER_TABLE))
            .bind(("invoice_id", invoice_id))
            .await?
            .take(0)?;
        Ok(reminders)
    }

    // Get all invoices with outstanding balance older than N days (for overdue marking)
    pub async fn get_overdue_candidates(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        days_overdue: u32,
    ) -> DbResult<Vec<Invoice>> {
        todo!()
    }

    // Mark overdue invoices in bulk
    pub async fn mark_overdue(&self, sdb: &Surreal<Client>, school_id: String) -> DbResult<u64> {
        todo!()
    } // returns count updated

    // Get payment stats for a date range
    pub async fn get_payment_stats(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        from: Value, //NaiveDate,
        to: Value,   // NaiveDate
    ) -> DbResult<Value> {
        todo!()
    }
}
