use thiserror::Error;

pub type DbResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Student not found")]
    // NotFound,
    #[error("Database error: {0}")]
    SurrealDb(#[from] surrealdb::Error),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Duplicate record: {0}")]
    Duplicate(String),
}
