use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::models::{School, SubscriptionPlan, SubscriptionStatus};
use surrealdb::{Surreal, engine::remote::ws::Client};
use surrealdb_types::{RecordId, SurrealValue, Value};

use crate::error::DbResult;

pub struct SchoolQ;

impl SchoolQ {
    pub async fn create_school(self, sdb: &Surreal<Client>, data: School) -> DbResult<School> {
        let school: Option<School> = sdb.create("schools").content(data).await?;
        todo!()
    }
}
