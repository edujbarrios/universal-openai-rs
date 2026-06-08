use reqwest::header::HeaderMap;
use serde_json::Value;
use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub status: reqwest::StatusCode,
    pub body: String,
    pub error_type: Option<String>,
    pub code: Option<String>,
    pub param: Option<String>,
    pub request_id: Option<String>,
}

impl ApiError {
    pub fn new(status: reqwest::StatusCode, body: impl Into<String>) -> Self {
        let mut error = Self {
            status,
            body: body.into(),
            error_type: None,
            code: None,
            param: None,
            request_id: None,
        };

        error.parse_openai_error_fields();
        error
    }

    pub fn from_parts(
        status: reqwest::StatusCode,
        headers: &HeaderMap,
        body: impl Into<String>,
    ) -> Self {
        let body = body.into();
        let mut error = Self {
            status,
            request_id: request_id(headers),
            body,
            error_type: None,
            code: None,
            param: None,
        };

        error.parse_openai_error_fields();
        error
    }

    fn parse_openai_error_fields(&mut self) {
        let Ok(value) = serde_json::from_str::<Value>(&self.body) else {
            return;
        };

        let error = value.get("error").unwrap_or(&value);
        self.error_type = string_field(error, "type");
        self.code = string_field(error, "code");
        self.param = string_field(error, "param");
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.status, self.body)?;

        if let Some(request_id) = &self.request_id {
            write!(formatter, " request_id={request_id}")?;
        }

        Ok(())
    }
}

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

    #[error("unknown tool: {0}")]
    UnknownTool(String),

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("provider returned an unsuccessful response: {0}")]
    Api(ApiError),
}

fn request_id(headers: &HeaderMap) -> Option<String> {
    [
        "x-request-id",
        "openai-request-id",
        "request-id",
        "x-correlation-id",
    ]
    .iter()
    .find_map(|name| {
        headers
            .get(*name)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned)
    })
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    match value.get(key)? {
        Value::String(value) => Some(value.clone()),
        Value::Null => None,
        value => Some(value.to_string()),
    }
}
