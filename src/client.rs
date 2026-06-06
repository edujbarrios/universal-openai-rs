use crate::{ChatCompletionResponse, ChatRequestBuilder, Config, Error, Result};

#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    http: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        if config.api_key().trim().is_empty() {
            return Err(Error::InvalidConfig("API key cannot be empty".to_string()));
        }

        Ok(Self {
            config,
            http: reqwest::Client::new(),
        })
    }

    pub fn from_env() -> Result<Self> {
        Self::new(Config::from_env()?)
    }

    pub fn chat(&self) -> ChatRequestBuilder<'_> {
        ChatRequestBuilder::new(self)
    }

    pub async fn chat_text(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<ChatCompletionResponse> {
        self.chat().model(model).user(prompt).send().await
    }

    pub(crate) async fn post_json<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        let response = self
            .http
            .post(self.config.endpoint(path)?)
            .bearer_auth(self.config.api_key())
            .json(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Api { status, body });
        }

        Ok(response.json::<R>().await?)
    }
}
