use crate::error::DbResult;
use shared::{
    helpers::create_update_dtos::CreateStudentDto,
    models::{Parent, Student, StudentParent, StudentStatus},
};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::Value;

const STUDENT_TABLE: &str = "students";
const PARENT_TABLE: &str = "parents";
const STUDENT_PARENTS_TABLE: &str = "student_parents";

pub struct StudentQ;

impl StudentQ {
    /// Create a student
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: CreateStudentDto,
    ) -> DbResult<Option<Student>> {
        let student: Option<Student> = sdb.create(STUDENT_TABLE).content(data).await?;
        Ok(student)
    }

    /// Get student by admission number
    pub async fn get_by_admission_number(
        &self,
        sdb: &Surreal<Client>,
        admission_number: String,
    ) -> DbResult<Option<Student>> {
        let student: Option<Student> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE admission_number = $admission_number
                LIMIT 1
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("admission_number", admission_number))
            .await?
            .take(0)?;
        Ok(student)
    }

    /// Get all active students in a school
    pub async fn get_active_by_school(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Student>> {
        let students: Vec<Student> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND status = 'active'
                ORDER BY last_name, first_name
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(students)
    }

    /// Get students in a specific class
    pub async fn get_by_class(
        &self,
        sdb: &Surreal<Client>,
        class_id: String,
    ) -> DbResult<Vec<Student>> {
        let students: Vec<Student> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE current_class_id = type::thing('classes', $class_id)
                AND status = 'active'
                ORDER BY last_name, first_name
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("class_id", class_id))
            .await?
            .take(0)?;
        Ok(students)
    }

    /// Get student with class and parents info
    pub async fn get_with_parents(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *,
                    current_class_id.*,
                    <-student_parents<-parents.* AS parents
                FROM type::thing($table, $id)
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("id", student_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Search students by name
    pub async fn search(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        query: String,
        limit: Option<u32>,
    ) -> DbResult<Vec<Student>> {
        let limit = limit.unwrap_or(20);
        let students: Vec<Student> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND (
                    string::lowercase(first_name) CONTAINS string::lowercase($query)
                    OR string::lowercase(last_name) CONTAINS string::lowercase($query)
                )
                AND status = 'active'
                LIMIT $limit
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("school_id", school_id))
            .bind(("query", query))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(students)
    }

    /// Promote student to a new class
    pub async fn promote(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        new_class_id: String,
    ) -> DbResult<Option<Student>> {
        let student: Option<Student> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    current_class_id = type::thing('classes', $new_class_id),
                    updated_at = time::now()
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("id", student_id))
            .bind(("new_class_id", new_class_id))
            .await?
            .take(0)?;
        Ok(student)
    }

    /// Update student status
    pub async fn update_status(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        status: StudentStatus,
    ) -> DbResult<Option<Student>> {
        let student: Option<Student> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    status = $status,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("id", student_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(student)
    }

    /// Count students by class level for a school
    pub async fn count_by_class_level(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Value>> {
        let result: Vec<Value> = sdb
            .query(
                r#"
                SELECT class_level, count() AS student_count
                FROM (
                    SELECT *, current_class_id.class_level AS class_level
                    FROM type::table($table)
                    WHERE school_id = type::thing('schools', $school_id)
                    AND status = 'active'
                )
                GROUP BY class_level
                "#,
            )
            .bind(("table", STUDENT_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}

pub struct ParentQ;

impl ParentQ {
    /// Create a parent
    pub async fn create(&self, sdb: &Surreal<Client>, data: Parent) -> DbResult<Option<Parent>> {
        let parent: Option<Parent> = sdb.create(PARENT_TABLE).content(data).await?;
        Ok(parent)
    }

    /// Link parent to student using a graph relation
    pub async fn link_to_student(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
        parent_id: String,
        primary_contact: bool,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                RELATE type::thing('students', $student_id)
                    ->student_parents->
                    type::thing('parents', $parent_id)
                CONTENT {
                    primary_contact: $primary_contact,
                    created_at: time::now()
                }
                "#,
            )
            .bind(("student_id", student_id))
            .bind(("parent_id", parent_id))
            .bind(("primary_contact", primary_contact))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get all children for a parent
    pub async fn get_children(
        &self,
        sdb: &Surreal<Client>,
        parent_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *, ->student_parents->students.* AS children
                FROM type::thing($table, $id)
                "#,
            )
            .bind(("table", PARENT_TABLE))
            .bind(("id", parent_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get all parents for a student
    pub async fn get_for_student(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *, <-student_parents<-parents.* AS parents
                FROM type::thing('students', $student_id)
                "#,
            )
            .bind(("student_id", student_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    /// Get primary contact for a student
    pub async fn get_primary_contact(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Option<StudentParent>> {
        let result: Option<StudentParent> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE in = type::thing('students', $student_id)
                AND primary_contact = true
                LIMIT 1
                "#,
            )
            .bind(("table", STUDENT_PARENTS_TABLE))
            .bind(("student_id", student_id))
            .await?
            .take(0)?;
        Ok(result)
    }

    // Count all students by status for a school
    pub async fn count_by_status(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<(StudentStatus, u64)>> {
        todo!()
    }

    // Get students without a class assigned (unassigned)
    pub async fn get_unassigned(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<Student>> {
        todo!()
    }

    // Get student by ID (basic fetch)
    pub async fn get_by_id(
        &self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Option<Student>> {
        todo!()
    }
}
