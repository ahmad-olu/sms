use shared::models::{Parent, Student};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::error::DbResult;

const TABLE_NAME: &str = "parents";
const TABLE_NAME_2: &str = "student_parents";
const TABLE_NAME_3: &str = "students";

pub struct ParentQ;

impl ParentQ {
    pub async fn create_parents(
        self,
        sdb: &Surreal<Client>,
        data: Parent,
    ) -> DbResult<Option<Parent>> {
        let parent: Option<Parent> = sdb.create(TABLE_NAME).content(data).await?;
        Ok(parent)
    }

    /// Link parent to student
    pub async fn link_parent_to_student(
        self,
        sdb: &Surreal<Client>,
        student_id: String,
        parent_id: String,
    ) -> DbResult<Option<Parent>> {
        let parent = sdb
            .query(
                r#"
                RELATE $student_id->type::table($table2)->$parent_id CONTENT {
                primary_contact: true
            };
            "#,
            )
            .bind(("student_id", student_id))
            .bind(("parent_id", parent_id))
            .bind(("table2", TABLE_NAME_2))
            .await?
            .take::<Option<Parent>>(0)?;
        Ok(parent)
    }

    /// Get all children for a parent
    pub async fn get_all_children_for_parent(
        self,
        sdb: &Surreal<Client>,
        parent_id: String,
    ) -> DbResult<Vec<Parent>> {
        let parent = sdb
            .query(
                r#"
                SELECT *, ->type::table($table2)->students.* AS children
                FROM type::table($table)
                WHERE id = $parent_id;
            };
            "#,
            )
            .bind(("parent_id", parent_id))
            .bind(("table", TABLE_NAME))
            .bind(("table2", TABLE_NAME_2))
            .await?
            .take::<Vec<Parent>>(0)?;
        Ok(parent)
    }

    /// Get all parents for a student
    pub async fn get_all_parent_for_student(
        self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Vec<Student>> {
        let parent = sdb
            .query(
                r#"
                SELECT *, <-type::table($table2)<-parents.* AS parents
                FROM type::table($table3)
                WHERE id = $student_id;
            };
            "#,
            )
            .bind(("student_id", student_id))
            .bind(("table3", TABLE_NAME_3))
            .bind(("table2", TABLE_NAME_2))
            .await?
            .take::<Vec<Student>>(0)?;
        Ok(parent)
    }

    //Get primary contact for student
    pub async fn get_student_pri_contact(
        self,
        sdb: &Surreal<Client>,
        student_id: String,
    ) -> DbResult<Vec<Student>> {
        let parent = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE in = $student_id
                AND primary_contact = true;
            };
            "#,
            )
            .bind(("student_id", student_id))
            .bind(("table", TABLE_NAME_2))
            .await?
            .take::<Vec<Student>>(0)?;
        Ok(parent)
    }
}
