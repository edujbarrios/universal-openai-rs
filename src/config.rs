use crate::{Error, Result};

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone)]
pub struct Config {
    api_key: String,
    base_url: String,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| Error::MissingEnv("OPENAI_API_KEY"))?;
        let base_url =
            std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Self::new(api_key).with_base_url(base_url))
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn endpoint(&self, path: &str) -> Result<String> {
        if self.base_url.is_empty() {
            return Err(Error::InvalidConfig("base URL cannot be empty".to_string()));
        }

        Ok(format!("{}/{}", self.base_url, path.trim_start_matches('/')))
    }
}

