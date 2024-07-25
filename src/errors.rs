use anyhow::Error;

#[derive(thiserror::Error, Debug)]
pub enum AptosClientError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("io error {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("not found")]
    NotFound,

    #[error("internal error {0}")]
    InternalError(#[from] Error),
}