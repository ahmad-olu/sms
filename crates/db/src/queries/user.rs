use crate::error::DbResult;
use shared::models::{User, UserStatus, UserType};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::Value;

const USER_TABLE: &str = "users";

pub struct UserQ;

impl UserQ {
    /// Create a user (password hashing should be done before calling this)
    pub async fn create(&self, sdb: &Surreal<Client>, data: User) -> DbResult<Option<User>> {
        let user: Option<User> = sdb.create(USER_TABLE).content(data).await?;
        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(
        &self,
        sdb: &Surreal<Client>,
        email: String,
    ) -> DbResult<Option<User>> {
        let user: Option<User> = sdb
            .query("SELECT * FROM type::table($table) WHERE email = $email LIMIT 1")
            .bind(("table", USER_TABLE))
            .bind(("email", email))
            .await?
            .take(0)?;
        Ok(user)
    }

    /// Find user by phone number
    pub async fn find_by_phone(
        &self,
        sdb: &Surreal<Client>,
        phone: String,
    ) -> DbResult<Option<User>> {
        let user: Option<User> = sdb
            .query("SELECT * FROM type::table($table) WHERE phone_number = $phone LIMIT 1")
            .bind(("table", USER_TABLE))
            .bind(("phone", phone))
            .await?
            .take(0)?;
        Ok(user)
    }

    /// Get all active teachers in a school
    pub async fn get_teachers(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Vec<User>> {
        let users: Vec<User> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND user_type = 'teacher'
                AND status = 'active'
                "#,
            )
            .bind(("table", USER_TABLE))
            .bind(("school_id", school_id))
            .await?
            .take(0)?;
        Ok(users)
    }

    /// Get all users of a specific type in a school
    pub async fn get_by_type(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        user_type: UserType,
    ) -> DbResult<Vec<User>> {
        let users: Vec<User> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE school_id = type::thing('schools', $school_id)
                AND user_type = $user_type
                "#,
            )
            .bind(("table", USER_TABLE))
            .bind(("school_id", school_id))
            .bind(("user_type", user_type))
            .await?
            .take(0)?;
        Ok(users)
    }

    /// Update last login timestamp
    pub async fn update_last_login(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
    ) -> DbResult<Option<User>> {
        let user: Option<User> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    last_login = time::now(),
                    updated_at = time::now()
                "#,
            )
            .bind(("table", USER_TABLE))
            .bind(("id", user_id))
            .await?
            .take(0)?;
        Ok(user)
    }

    /// Update user status (active, suspended, inactive)
    pub async fn update_status(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
        status: UserStatus,
    ) -> DbResult<Option<User>> {
        let user: Option<User> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    status = $status,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", USER_TABLE))
            .bind(("id", user_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(user)
    }

    /// Get user with school info (fetch)
    pub async fn get_with_school(
        &self,
        sdb: &Surreal<Client>,
        user_id: String,
    ) -> DbResult<Option<Value>> {
        let result: Option<Value> = sdb
            .query(
                r#"
                SELECT *, school_id.* FROM type::thing($table, $id)
                "#,
            )
            .bind(("table", USER_TABLE))
            .bind(("id", user_id))
            .await?
            .take(0)?;
        Ok(result)
    }
}
