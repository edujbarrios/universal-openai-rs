use crate::{
    AgentSpec, Agents, ApiError, ChatCompletionResponse, ChatRequestBuilder,
    CompletionRequestBuilder, CompletionResponse, Config, EmbeddingsRequestBuilder,
    EmbeddingsResponse, Error, FineTuning, ImagesRequestBuilder, Models, ModerationRequestBuilder,
    PromptBuilder, Provider, ResponseRequestBuilder, ResponsesResponse, Result,
};
#[cfg(feature = "audio")]
use crate::Audio;
#[cfg(feature = "files")]
use crate::{FileUploadBuilder, Files};
#[cfg(feature = "stream")]
use crate::{ChatStream, ChatStreamEvent, OpenAiSseDecoder, StreamDecoder};
use reqwest::{header::HeaderMap, RequestBuilder};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
        config.header_map()?;

        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = config.timeout() {
            builder = builder.timeout(timeout);
        }

        Ok(Self {
            config,
            http: builder.build()?,
        })
    }

    pub fn with_http_client(config: Config, http: reqwest::Client) -> Result<Self> {
        if config.api_key().trim().is_empty() {
            return Err(Error::InvalidConfig("API key cannot be empty".to_string()));
        }
        config.header_map()?;

        Ok(Self { config, http })
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

    pub fn completions(&self) -> CompletionRequestBuilder<'_> {
        CompletionRequestBuilder::new(self)
    }

    pub fn prompt(&self, prompt: impl Into<String>) -> PromptBuilder<'_> {
        PromptBuilder::new(self, prompt)
    }

    pub fn agents(&self) -> Agents<'_> {
        Agents::new(self)
    }

    pub fn agent(&self, name: impl Into<String>) -> AgentSpec {
        AgentSpec::new(name)
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

    pub fn images(&self) -> ImagesRequestBuilder<'_> {
        ImagesRequestBuilder::new(self)
    }

    pub fn moderations(&self) -> ModerationRequestBuilder<'_> {
        ModerationRequestBuilder::new(self)
    }

    pub fn responses(&self) -> ResponseRequestBuilder<'_> {
        ResponseRequestBuilder::new(self)
    }

    pub fn respond(&self, input: impl Into<String>) -> ResponseRequestBuilder<'_> {
        self.responses().input(input)
    }

    pub fn models(&self) -> Models<'_> {
        Models::new(self)
    }

    #[cfg(feature = "files")]
    pub fn files(&self) -> Files<'_> {
        Files::new(self)
    }

    #[cfg(feature = "files")]
    pub fn upload_file(&self, purpose: impl Into<String>) -> FileUploadBuilder<'_> {
        FileUploadBuilder::new(self, purpose)
    }

    #[cfg(feature = "audio")]
    pub fn audio(&self) -> Audio<'_> {
        Audio::new(self)
    }

    pub fn fine_tuning(&self) -> FineTuning<'_> {
        FineTuning::new(self)
    }

    pub async fn chat_text(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<ChatCompletionResponse> {
        self.chat().model(model).user(prompt).send().await
    }

    pub async fn complete_text(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<CompletionResponse> {
        self.completions().model(model).prompt(prompt).send().await
    }

    pub async fn list_models(&self) -> Result<crate::ListModelsResponse> {
        self.models().list().await
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

    #[cfg(feature = "structured-output")]
    pub async fn ask_structured<T>(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned + schemars::JsonSchema,
    {
        self.chat()
            .model(model)
            .user(prompt)
            .json_schema_auto::<T>()
            .send()
            .await?
            .json()
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

    pub async fn generate_image(
        &self,
        model: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<crate::ImageResponse> {
        self.images().model(model).prompt(prompt).generate().await
    }

    pub async fn moderate_text(
        &self,
        input: impl Into<String>,
    ) -> Result<crate::ModerationResponse> {
        self.moderations().input(input).send().await
    }

    #[cfg(feature = "audio")]
    pub async fn transcribe(
        &self,
        model: impl Into<String>,
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<crate::AudioResponse> {
        self.audio()
            .transcription()
            .model(model)
            .file(filename, bytes)
            .send()
            .await
    }

    #[cfg(feature = "audio")]
    pub async fn translate_audio(
        &self,
        model: impl Into<String>,
        filename: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<crate::AudioResponse> {
        self.audio()
            .translation()
            .model(model)
            .file(filename, bytes)
            .send()
            .await
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

    pub async fn get_compatible<R>(&self, path: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        self.get_json(path).await
    }

    pub async fn delete_compatible<R>(&self, path: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        self.delete_json(path).await
    }

    pub(crate) async fn get_json<R>(&self, path: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| self.authorized(self.http.get(&endpoint)))
            .await?;

        self.parse_response(response).await
    }

    #[cfg(feature = "files")]
    pub(crate) async fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| self.authorized(self.http.get(&endpoint)))
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(self.api_error(response).await);
        }

        Ok(response.bytes().await?.to_vec())
    }

    pub(crate) async fn delete_json<R>(&self, path: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| self.authorized(self.http.delete(&endpoint)))
            .await?;

        self.parse_response(response).await
    }

    #[cfg(any(feature = "audio", feature = "files"))]
    pub(crate) async fn post_multipart<R, F>(
        &self,
        path: &str,
        mut build_form: F,
    ) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
        F: FnMut() -> Result<reqwest::multipart::Form>,
    {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| {
                Ok(self
                    .authorized(self.http.post(&endpoint))?
                    .multipart(build_form()?))
            })
            .await?;

        self.parse_response(response).await
    }

    pub(crate) async fn post_json<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: serde::Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| Ok(self.authorized(self.http.post(&endpoint))?.json(body)))
            .await?;

        self.parse_response(response).await
    }

    async fn parse_response<R>(&self, response: reqwest::Response) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let status = response.status();
        if !status.is_success() {
            return Err(self.api_error(response).await);
        }

        Ok(response.json::<R>().await?)
    }

    #[cfg(feature = "stream")]
    pub(crate) async fn post_sse<T>(&self, path: &str, body: &T) -> Result<ChatStream>
    where
        T: serde::Serialize + ?Sized,
    {
        self.post_stream(path, body, OpenAiSseDecoder::new()).await
    }

    #[cfg(feature = "stream")]
    pub(crate) async fn post_stream<T, D>(
        &self,
        path: &str,
        body: &T,
        decoder: D,
    ) -> Result<ChatStream>
    where
        T: serde::Serialize + ?Sized,
        D: StreamDecoder<Event = ChatStreamEvent> + Send + 'static,
    {
        let endpoint = self.config.endpoint(path)?;
        let response = self
            .send_with_retries(|| Ok(self.authorized(self.http.post(&endpoint))?.json(body)))
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(self.api_error(response).await);
        }

        Ok(crate::streaming::decode_response_stream(response, decoder))
    }

    fn authorized(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        Ok(request
            .bearer_auth(self.config.api_key())
            .headers(self.config.header_map()?))
    }

    async fn send_with_retries<F>(&self, mut build: F) -> Result<reqwest::Response>
    where
        F: FnMut() -> Result<RequestBuilder>,
    {
        let mut attempt = 0;

        loop {
            let response = build()?.send().await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    }

                    if self.should_retry(response.status(), attempt) {
                        let headers = response.headers().clone();
                        attempt += 1;
                        self.sleep_before_retry(attempt, Some(&headers)).await;
                    } else {
                        return Ok(response);
                    }
                }
                Err(error) if self.should_retry_error(&error, attempt) => {
                    attempt += 1;
                    self.sleep_before_retry(attempt, None).await;
                }
                Err(error) => return Err(Error::Http(error)),
            }
        }
    }

    async fn api_error(&self, response: reqwest::Response) -> Error {
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text().await.unwrap_or_default();

        Error::Api(ApiError::from_parts(status, &headers, body))
    }

    fn should_retry(&self, status: reqwest::StatusCode, attempt: usize) -> bool {
        attempt < self.config.retry_config().max_retries
            && (status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error())
    }

    fn should_retry_error(&self, error: &reqwest::Error, attempt: usize) -> bool {
        attempt < self.config.retry_config().max_retries
            && (error.is_timeout() || error.is_connect())
    }

    async fn sleep_before_retry(&self, attempt: usize, headers: Option<&HeaderMap>) {
        let delay = self.retry_delay(attempt, headers);
        tokio::time::sleep(delay).await;
    }

    fn retry_delay(&self, attempt: usize, headers: Option<&HeaderMap>) -> Duration {
        let retry = self.config.retry_config();

        if retry.respect_retry_after {
            if let Some(delay) = headers.and_then(retry_after_delay) {
                return delay.min(retry.max_backoff);
            }
        }

        let multiplier = 2_u32.saturating_pow(attempt.saturating_sub(1) as u32);
        let mut delay = retry.initial_backoff.saturating_mul(multiplier);
        delay = delay.min(retry.max_backoff);

        if retry.jitter {
            delay = add_jitter(delay, retry.max_backoff);
        }

        delay
    }
}

fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get("retry-after")?.to_str().ok()?.trim();

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let retry_at = httpdate::parse_http_date(value).ok()?;
    retry_at.duration_since(SystemTime::now()).ok()
}

fn add_jitter(delay: Duration, max_backoff: Duration) -> Duration {
    if delay.is_zero() {
        return delay;
    }

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos())
        .unwrap_or(0);
    let jitter_nanos = (delay.as_nanos() / 4).min(u64::MAX as u128) as u64;

    if jitter_nanos == 0 {
        return delay;
    }

    let extra = Duration::from_nanos((nanos as u64) % jitter_nanos);
    delay.saturating_add(extra).min(max_backoff)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn retry_after_accepts_seconds() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("3"));

        assert_eq!(retry_after_delay(&headers), Some(Duration::from_secs(3)));
    }

    #[test]
    fn retry_after_accepts_http_date() {
        let retry_at = SystemTime::now() + Duration::from_secs(5);
        let mut headers = HeaderMap::new();
        headers.insert(
            "retry-after",
            HeaderValue::from_str(&httpdate::fmt_http_date(retry_at)).unwrap(),
        );

        assert!(retry_after_delay(&headers).is_some());
    }
}
