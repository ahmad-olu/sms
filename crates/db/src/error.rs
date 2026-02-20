use thiserror::Error;

pub type DbResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Student not found")]
    // NotFound,
    #[error("Database error: {0}")]
    SurrealDb(#[from] surrealdb::Error),
}
