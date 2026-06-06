use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing environment variable {0}")]
    MissingEnv(&'static str),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("provider returned an unsuccessful response: {status} {body}")]
    Api { status: reqwest::StatusCode, body: String },
}

