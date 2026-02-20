use crate::error::DbResult;
use shared::{
    helpers::create_update_dtos::CreateSchoolDto,
    models::{School, SubscriptionStatus},
};
use surrealdb::{Surreal, engine::remote::ws::Client};

const SCHOOL_TABLE: &str = "schools";

pub struct SchoolQ;

impl SchoolQ {
    /// Create a school
    pub async fn create(
        &self,
        sdb: &Surreal<Client>,
        data: CreateSchoolDto,
    ) -> DbResult<Option<School>> {
        let school: Option<School> = sdb.create(SCHOOL_TABLE).content(data).await?;
        Ok(school)
    }

    /// Get school by ID
    pub async fn get_by_id(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
    ) -> DbResult<Option<School>> {
        let school: Option<School> = sdb
            .query("SELECT * FROM type::table($table) WHERE id = type::thing($table, $id)")
            .bind(("table", SCHOOL_TABLE))
            .bind(("id", school_id))
            .await?
            .take(0)?;
        Ok(school)
    }

    /// Update school name and address
    pub async fn update(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        name: String,
        address: Option<String>,
    ) -> DbResult<Option<School>> {
        let school: Option<School> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    school_name = $name,
                    school_address = $address,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", SCHOOL_TABLE))
            .bind(("id", school_id))
            .bind(("name", name))
            .bind(("address", address))
            .await?
            .take(0)?;
        Ok(school)
    }

    /// Get all active schools
    pub async fn get_active(&self, sdb: &Surreal<Client>) -> DbResult<Vec<School>> {
        let schools: Vec<School> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE subscription_status = 'active'
                "#,
            )
            .bind(("table", SCHOOL_TABLE))
            .await?
            .take(0)?;
        Ok(schools)
    }

    /// Get schools expiring in the next N days
    pub async fn get_expiring_soon(
        &self,
        sdb: &Surreal<Client>,
        days: u32,
    ) -> DbResult<Vec<School>> {
        let schools: Vec<School> = sdb
            .query(
                r#"
                SELECT * FROM type::table($table)
                WHERE subscription_expiry_date >= time::now()
                AND subscription_expiry_date <= time::now() + type::duration($days)
                ORDER BY subscription_expiry_date ASC
                "#,
            )
            .bind(("table", SCHOOL_TABLE))
            .bind(("days", format!("{}d", days)))
            .await?
            .take(0)?;
        Ok(schools)
    }

    /// Update subscription status
    pub async fn update_subscription_status(
        &self,
        sdb: &Surreal<Client>,
        school_id: String,
        status: SubscriptionStatus,
    ) -> DbResult<Option<School>> {
        let school: Option<School> = sdb
            .query(
                r#"
                UPDATE type::thing($table, $id) SET
                    subscription_status = $status,
                    updated_at = time::now()
                "#,
            )
            .bind(("table", SCHOOL_TABLE))
            .bind(("id", school_id))
            .bind(("status", status))
            .await?
            .take(0)?;
        Ok(school)
    }
}
