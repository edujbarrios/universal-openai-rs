use crate::{Error, Result};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone)]
pub struct Config {
    api_key: String,
    base_url: String,
    timeout: Option<Duration>,
    max_retries: usize,
    default_model: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
    OpenAI,
    OpenRouter,
    Groq,
    Together,
    Ollama,
    Custom(String),
}

impl Provider {
    pub fn base_url(&self) -> &str {
        match self {
            Self::OpenAI => "https://api.openai.com/v1",
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::Together => "https://api.together.xyz/v1",
            Self::Ollama => "http://localhost:11434/v1",
            Self::Custom(base_url) => base_url,
        }
    }
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Some(Duration::from_secs(60)),
            max_retries: 2,
            default_model: None,
        }
    }

    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| Error::MissingEnv("OPENAI_API_KEY"))?;
        let base_url =
            std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        let default_model = std::env::var("OPENAI_MODEL").ok();

        let mut config = Self::new(api_key).with_base_url(base_url);
        if let Some(default_model) = default_model {
            config = config.with_default_model(default_model);
        }

        Ok(config)
    }

    pub fn for_provider(api_key: impl Into<String>, provider: Provider) -> Self {
        Self::new(api_key).with_base_url(provider.base_url())
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    pub fn max_retries(&self) -> usize {
        self.max_retries
    }

    pub fn default_model(&self) -> Option<&str> {
        self.default_model.as_deref()
    }

    pub(crate) fn endpoint(&self, path: &str) -> Result<String> {
        if self.base_url.is_empty() {
            return Err(Error::InvalidConfig("base URL cannot be empty".to_string()));
        }

        Ok(format!("{}/{}", self.base_url, path.trim_start_matches('/')))
    }
}
