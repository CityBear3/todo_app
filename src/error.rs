use thiserror::Error;

#[derive(Error, Debug)]
pub enum Myerror {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Faild to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Faild SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}
