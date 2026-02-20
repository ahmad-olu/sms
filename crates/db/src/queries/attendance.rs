use crate::error::DbResult;
use shared::{
    helpers::create_update_dtos::MarkAttendanceDto,
    models::{
        AttendanceStatus,
        attendance::{Attendance, AttendanceSummary},
    },
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::Value;

const ATTENDANCE_TABLE: &str = "attendance";
const ATTENDANCE_SUMMARY_TABLE: &str = "attendance_summary";

pub struct AttendanceQ;

impl AttendanceQ {
    /// Mark attendance for a student
    pub async fn mark(
        &self,
        sdb: &Surreal<Client>,
        data: MarkAttendanceDto,
    ) -> DbResult<Option<Attendance>> {
        let attendance: Option<Attendance> = sdb.create(ATTENDANCE_TABLE).content(data).await?;
        Ok(attendance)
    }

    /// Get attendance for a student on a specific date
    pub async fn get_by_student_and_date(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        date: String, // "YYYY-MM-DD"
    ) -> DbResult<Option<Attendance>> {
        let attendance: Option<Attendance> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND date = type::datetime($date)
                LIMIT 1
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("student_id", student_id))
            .bind(("date", date))
            .await?
            .take(0)?;
        Ok(attendance)
    }

    /// Get attendance for a class on a specific date (with student details)
    pub async fn get_by_class_and_date(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
        date: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *, student_id.* AS student
                FROM type::table($table)
                WHERE class_id = type::thing('classes', $class_id)
                AND date = type::datetime($date)
                ORDER BY status, student_id.last_name
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("class_id", class_id))
            .bind(("date", date))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get attendance history for a student (most recent first)
    pub async fn get_history(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<Attendance>> {
        let limit = limit.unwrap_or(30);
        let attendance: Vec<Attendance> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                ORDER BY date DESC
                LIMIT $limit
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("student_id", student_id))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(attendance)
    }

    /// Count attendance by status for a student in a term
    pub async fn count_by_status(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_start_date: String,
        term_end_date: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT status, count() AS count
                FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND date >= type::datetime($term_start_date)
                AND date <= type::datetime($term_end_date)
                GROUP BY status
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_start_date", term_start_date))
            .bind(("term_end_date", term_end_date))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get absent students for a class today
    pub async fn get_absent_today(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *, student_id.* AS student
                FROM type::table($table)
                WHERE class_id = type::thing('classes', $class_id)
                AND date = time::today()
                AND status = 'absent'
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("class_id", class_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Update an attendance record
    pub async fn update_status(
        &self,
        sdb: &Surreal<Client>,
        attendance_id: String,
        status: AttendanceStatus,
        reason: Option<String>,
    ) -> DbResult<Option<Attendance>> {
        let attendance: Option<Attendance> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    status = $status,
                    reason = $reason,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", ATTENDANCE_TABLE))
            .bind(("id", attendance_id))
            .bind(("status", status))
            .bind(("reason", reason))
            .await?
            .take(0)?;
        Ok(attendance)
    }
}

pub struct AttendanceSummaryQ;

impl AttendanceSummaryQ {
    /// Upsert attendance summary for a student in a term
    pub async fn upsert(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_id: String,
        total_present: i32,
        total_absent: i32,
        total_late: i32,
        total_excused: i32,
        attendance_percentage: f64,
    ) -> DbResult<Option<AttendanceSummary>> {
        let result: Option<AttendanceSummary> = sdb
            .query(
                r#"
                UPDATE type::table($table) CONTENT {
                    student_id: type::thing('students', $student_id),
                    term_id: type::thing('terms', $term_id),
                    total_present: $total_present,
                    total_absent: $total_absent,
                    total_late: $total_late,
                    total_excused: $total_excused,
                    attendance_percentage: $attendance_percentage,
                    updated_at: time::now()
                }
                WHERE student_id = type::thing('students', $student_id)
                AND term_id = type::thing('terms', $term_id)
                "#,
            )
            .bind(("table", ATTENDANCE_SUMMARY_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_id", term_id))
            .bind(("total_present", total_present))
            .bind(("total_absent", total_absent))
            .bind(("total_late", total_late))
            .bind(("total_excused", total_excused))
            .bind(("attendance_percentage", attendance_percentage))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get attendance summary for a student in a term
    pub async fn get(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        term_id: String,
    ) -> DbResult<Option<AttendanceSummary>> {
        let result: Option<AttendanceSummary> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE student_id = type::thing('students', $student_id)
                AND term_id = type::thing('terms', $term_id)
                LIMIT 1
                "#,
            )
            .bind(("table", ATTENDANCE_SUMMARY_TABLE))
            .bind(("student_id", student_id))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get attendance rate by class for a term
    pub async fn get_rate_by_class(
        &self,
        sdb: &Surreal<Client>,
        term_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT
                    student_id.current_class_id.class_name AS class_name,
                    math::mean(attendance_percentage) AS average_attendance
                FROM type::table($table)
                WHERE term_id = type::thing('terms', $term_id)
                GROUP BY student_id.current_class_id
                ORDER BY average_attendance DESC
                "#,
            )
            .bind(("table", ATTENDANCE_SUMMARY_TABLE))
            .bind(("term_id", term_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}
