use crate::{
    ChatCompletionResponse, ChatRequestBuilder, ChatStream, ChatStreamEvent, Config,
    EmbeddingsRequestBuilder, EmbeddingsResponse, Error, Provider, ResponseRequestBuilder,
    ResponsesResponse, Result, PromptBuilder,
};
use futures_util::StreamExt;
use std::time::Duration;

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

        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = config.timeout() {
            builder = builder.timeout(timeout);
        }

        Ok(Self {
            config,
            http: builder.build()?,
        })
    }

    pub fn from_env() -> Result<Self> {
        Self::new(Config::from_env()?)
    }

    pub fn openai(api_key: impl Into<String>) -> Result<Self> {
        Self::new(Config::for_provider(api_key, Provider::OpenAI))
    }

    pub fn compatible(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        Self::new(Config::for_provider(
            api_key,
            Provider::Custom(base_url.into()),
        ))
    }

    pub fn for_provider(api_key: impl Into<String>, provider: Provider) -> Result<Self> {
        Self::new(Config::for_provider(api_key, provider))
    }

    pub fn chat(&self) -> ChatRequestBuilder<'_> {
        ChatRequestBuilder::new(self)
    }

    pub fn prompt(&self, prompt: impl Into<String>) -> PromptBuilder<'_> {
        PromptBuilder::new(self, prompt)
    }

    pub fn chat_default(&self) -> Result<ChatRequestBuilder<'_>> {
        let model = self
            .config
            .default_model()
            .ok_or_else(|| Error::InvalidConfig("default model is not configured".to_string()))?;

        Ok(self.chat().model(model))
    }

    pub fn embeddings(&self) -> EmbeddingsRequestBuilder<'_> {
        EmbeddingsRequestBuilder::new(self)
    }

    pub fn responses(&self) -> ResponseRequestBuilder<'_> {
        ResponseRequestBuilder::new(self)
    }

    pub async fn chat_text(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<ChatCompletionResponse> {
        self.chat().model(model).user(prompt).send().await
    }

    pub async fn ask(&self, model: impl Into<String>, prompt: impl Into<String>) -> Result<String> {
        self.chat_text(model, prompt)
            .await?
            .first_text()
            .map(ToOwned::to_owned)
            .ok_or(Error::MissingText)
    }

    pub async fn ask_default(&self, prompt: impl Into<String>) -> Result<String> {
        self.chat_default()?.user(prompt).send().await?.text()
    }

    pub(crate) fn default_model(&self) -> Option<&str> {
        self.config.default_model()
    }

    pub async fn ask_json<T>(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = self
            .chat()
            .model(model)
            .user(prompt)
            .json_object()
            .send()
            .await?
            .first_text()
            .map(ToOwned::to_owned)
            .ok_or(Error::MissingText)?;

        Ok(serde_json::from_str(&text)?)
    }

    pub async fn embed_text(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<EmbeddingsResponse> {
        self.embeddings().model(model).input(input).send().await
    }

    pub async fn embed(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<Vec<f32>> {
        let response = self.embed_text(model, input).await?;
        response.first_vector().ok_or(Error::MissingEmbedding)
    }

    pub async fn embed_many<I, S>(
        &self,
        model: impl Into<String>,
        inputs: I,
    ) -> Result<Vec<Vec<f32>>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Ok(self
            .embeddings()
            .model(model)
            .inputs(inputs)
            .send()
            .await?
            .vectors())
    }

    pub async fn respond_text(
        &self,
        model: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<ResponsesResponse> {
        self.responses().model(model).input(input).send().await
    }

    pub async fn send_compatible<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        self.post_json(path, body).await
    }

    pub(crate) async fn post_json<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        let endpoint = self.config.endpoint(path)?;
        let mut attempt = 0;

        loop {
            let response = self
                .http
                .post(&endpoint)
                .bearer_auth(self.config.api_key())
                .json(body)
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => {
                    return Ok(response.json::<R>().await?);
                }
                Ok(response) if self.should_retry(response.status(), attempt) => {
                    attempt += 1;
                    self.sleep_before_retry(attempt).await;
                }
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::Api { status, body });
                }
                Err(error) if self.should_retry_error(&error, attempt) => {
                    attempt += 1;
                    self.sleep_before_retry(attempt).await;
                }
                Err(error) => return Err(Error::Http(error)),
            }
        }
    }

    pub(crate) async fn post_sse<T>(&self, path: &str, body: &T) -> Result<ChatStream>
    where
        T: serde::Serialize + ?Sized,
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

        let mut bytes = response.bytes_stream();
        let stream = async_stream::try_stream! {
            let mut buffer = String::new();

            while let Some(chunk) = bytes.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));

                while let Some(newline) = buffer.find('\n') {
                    let line: String = buffer.drain(..=newline).collect();
                    let line = line.trim();

                    if line.is_empty() || line.starts_with("event:") {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data:") {
                        let data = data.trim();
                        if data == "[DONE]" {
                            return;
                        }

                        yield serde_json::from_str::<ChatStreamEvent>(data)?;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    fn should_retry(&self, status: reqwest::StatusCode, attempt: usize) -> bool {
        attempt < self.config.max_retries()
            && (status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error())
    }

    fn should_retry_error(&self, error: &reqwest::Error, attempt: usize) -> bool {
        attempt < self.config.max_retries() && (error.is_timeout() || error.is_connect())
    }

    async fn sleep_before_retry(&self, attempt: usize) {
        let millis = 100_u64.saturating_mul(2_u64.saturating_pow(attempt as u32));
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }
}
