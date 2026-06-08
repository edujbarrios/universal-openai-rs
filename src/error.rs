use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing environment variable {0}")]
    MissingEnv(&'static str),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("response did not contain text output")]
    MissingText,

    #[error("response did not contain an embedding vector")]
    MissingEmbedding,

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("provider returned an unsuccessful response: {status} {body}")]
    Api { status: reqwest::StatusCode, body: String },
}
