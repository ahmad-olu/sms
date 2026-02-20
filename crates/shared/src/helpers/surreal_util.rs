use chrono::{DateTime, NaiveDate, Utc};
use surrealdb_types::Datetime;

pub fn to_surreal_datetime(date: NaiveDate) -> surrealdb_types::Value {
    let datetime: DateTime<Utc> = date.and_hms_opt(0, 0, 0).unwrap().and_utc();

    surrealdb_types::Value::Datetime(Datetime::from(datetime))
}
