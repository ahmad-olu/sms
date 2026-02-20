use crate::error::DbResult;
use shared::models::academic::{AcademicSession, Class, ClassSubject, Subject, Term};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::Value;

const ACAD_SESSION_TABLE: &str = "academic_sessions";
const TERMS_TABLE: &str = "terms";
const CLASS_TABLE: &str = "classes";
const SUBJECT_TABLE: &str = "subjects";
const CLASS_SUBJECT_TABLE: &str = "class_subjects";

pub struct AcademicSessionQ;

impl AcademicSessionQ {
    /// Create academic session
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: AcademicSession,
    ) -> DbResult<Option<AcademicSession>> {
        let session: Option<AcademicSession> = sdb.create(ACAD_SESSION_TABLE).content(data).await?;
        Ok(session)
    }

    /// Get current session for a school
    pub async fn get_current(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Option<AcademicSession>> {
        let session: Option<AcademicSession> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND is_current = true
                LIMIT 1
                "#,
            )
            .bind(("table", ACAD_SESSION_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(session)
    }

    /// Get all sessions for a school
    pub async fn get_all_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<AcademicSession>> {
        let sessions: Vec<AcademicSession> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY start_date DESC
                "#,
            )
            .bind(("table", ACAD_SESSION_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(sessions)
    }

    /// Set a session as current (and unset others)
    pub async fn set_current(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        session_id: String,
    ) -> DbResult<()> {
        sdb.query(
            r#"
            BEGIN TRANSACTION;
            UPDATE type::table($table) SET is_current = false
                WHERE school_id = type::thing('schools', $school_id);
            UPDATE type::thing($table, $session_id) SET is_current = true;
            COMMIT TRANSACTION;
            "#,
        )
        .bind(("table", ACAD_SESSION_TABLE))
        .bind(("school_id", school_id))
        .bind(("session_id", session_id))
        .await?;
        Ok(())
    }
}

pub struct TermQ;

impl TermQ {
    /// Create a term
    pub async fn create(&self, sdb: &Surreal<Client>, data: Term) -> DbResult<Option<Term>> {
        let term: Option<Term> = sdb.create(TERMS_TABLE).content(data).await?;
        Ok(term)
    }

    /// Get current term for a session
    pub async fn get_current(
        &self,
        sdb: &Surreal<Client>,
        session_id: String,
    ) -> DbResult<Option<Term>> {
        let term: Option<Term> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE session_id = type::thing('academic_sessions', $session_id)
                AND is_current = true
                LIMIT 1
                "#,
            )
            .bind(("table", TERMS_TABLE))
            .bind(("session_id", session_id))
            .await?
            .take(0)?;
        Ok(term)
    }

    /// Get all terms for a session
    pub async fn get_by_session(
        &self,
        sdb: &Surreal<Client>,
        session_id: String,
    ) -> DbResult<Vec<Term>> {
        let terms: Vec<Term> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE session_id = type::thing('academic_sessions', $session_id)
                ORDER BY term_number
                "#,
            )
            .bind(("table", TERMS_TABLE))
            .bind(("session_id", session_id))
            .await?
            .take(0)?;
        Ok(terms)
    }

    /// Set a term as current
    pub async fn set_current(
        &self,
        sdb: &Surreal<Client>,
        session_id: String,
        term_id: String,
    ) -> DbResult<()> {
        sdb.query(
            r#"
            BEGIN TRANSACTION;
            UPDATE type::table($table) SET is_current = false
                WHERE session_id = type::thing('academic_sessions', $session_id);
            UPDATE type::thing($table, $term_id) SET is_current = true;
            COMMIT TRANSACTION;
            "#,
        )
        .bind(("table", TERMS_TABLE))
        .bind(("session_id", session_id))
        .bind(("term_id", term_id))
        .await?;
        Ok(())
    }
}

pub struct ClassQ;

impl ClassQ {
    /// Create a class
    pub async fn create(&self, sdb: &Surreal<Client>, data: Class) -> DbResult<Option<Class>> {
        let class: Option<Class> = sdb.create(CLASS_TABLE).content(data).await?;
        Ok(class)
    }

    /// Get all classes in a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Class>> {
        let classes: Vec<Class> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY class_level, class_name
                "#,
            )
            .bind(("table", CLASS_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(classes)
    }

    /// Get classes taught by a teacher (as class teacher)
    pub async fn get_by_class_teacher(
        &self,
        sdb: &Surreal<Client>,
        teacher_id: String,
    ) -> DbResult<Vec<Class>> {
        let classes: Vec<Class> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE class_teacher_id = type::thing('users', $teacher_id)
                "#,
            )
            .bind(("table", CLASS_TABLE))
            .bind(("teacher_id", teacher_id))
            .await?
            .take(0)?;
        Ok(classes)
    }

    /// Update class teacher
    pub async fn update_teacher(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
        teacher_id: String,
    ) -> DbResult<Option<Class>> {
        let class: Option<Class> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    class_teacher_id = type::thing('users', $teacher_id),
                    updated_at = time::now()
                "#,
            )
            .bind(("table", CLASS_TABLE))
            .bind(("id", class_id))
            .bind(("teacher_id", teacher_id))
            .await?
            .take(0)?;
        Ok(class)
    }

    /// Increment enrollment count
    pub async fn increment_enrollment(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
    ) -> DbResult<Option<Class>> {
        let class: Option<Class> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    current_enrollment = current_enrollment + 1,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", CLASS_TABLE))
            .bind(("id", class_id))
            .await?
            .take(0)?;
        Ok(class)
    }
}

pub struct SubjectQ;

impl SubjectQ {
    /// Create a subject
    pub async fn create(&self, sdb: &Surreal<Client>, data: Subject) -> DbResult<Option<Subject>> {
        let subject: Option<Subject> = sdb.create(SUBJECT_TABLE).content(data).await?;
        Ok(subject)
    }

    /// Get all subjects in a school
    pub async fn get_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Subject>> {
        let subjects: Vec<Subject> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                ORDER BY subject_name
                "#,
            )
            .bind(("table", SUBJECT_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(subjects)
    }
}

pub struct ClassSubjectQ;

impl ClassSubjectQ {
    /// Assign subject to class with an optional teacher
    pub async fn assign(
        &self,
        sdb: &Surreal<Client>,
        data: ClassSubject,
    ) -> DbResult<Option<ClassSubject>> {
        let cs: Option<ClassSubject> = sdb.create(CLASS_SUBJECT_TABLE).content(data).await?;
        Ok(cs)
    }

    /// Get all subjects taught in a class (with subject and teacher details)
    pub async fn get_by_class(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *,
                    subject_id.* AS subject,
                    teacher_id.* AS teacher
                FROM type::table($table)
                WHERE class_id = type::thing('classes', $class_id)
                "#,
            )
            .bind(("table", CLASS_SUBJECT_TABLE))
            .bind(("class_id", class_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get all classes and subjects taught by a teacher
    pub async fn get_by_teacher(
        &self,
        sdb: &Surreal<Client>,
        teacher_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT *,
                    class_id.* AS class,
                    subject_id.* AS subject
                FROM type::table($table)
                WHERE teacher_id = type::thing('users', $teacher_id)
                "#,
            )
            .bind(("table", CLASS_SUBJECT_TABLE))
            .bind(("teacher_id", teacher_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Update the teacher assigned to a class-subject
    pub async fn update_teacher(
        &self,
        sdb: &Surreal<Client>,
        class_subject_id: String,
        teacher_id: String,
    ) -> DbResult<Option<ClassSubject>> {
        let cs: Option<ClassSubject> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    teacher_id = type::thing('users', $teacher_id)
                "#,
            )
            .bind(("table", CLASS_SUBJECT_TABLE))
            .bind(("id", class_subject_id))
            .bind(("teacher_id", teacher_id))
            .await?
            .take(0)?;
        Ok(cs)
    }
}
