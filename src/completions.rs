use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: CompletionPrompt,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionPrompt {
    Text(String),
    Texts(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<CompletionChoice>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

impl CompletionResponse {
    pub fn first_text(&self) -> Option<&str> {
        self.choices.first().map(|choice| choice.text.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: Option<u32>,
    pub finish_reason: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct CompletionRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    prompt: Option<CompletionPrompt>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> CompletionRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            prompt: None,
            temperature: None,
            max_tokens: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(CompletionPrompt::Text(prompt.into()));
        self
    }

    pub fn prompts<I, S>(mut self, prompts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.prompt = Some(CompletionPrompt::Texts(
            prompts.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<CompletionRequest> {
        let model = self
            .model
            .filter(|model| !model.trim().is_empty())
            .ok_or_else(|| Error::InvalidConfig("completion model is required".to_string()))?;
        let prompt = self
            .prompt
            .ok_or_else(|| Error::InvalidConfig("completion prompt is required".to_string()))?;

        Ok(CompletionRequest {
            model,
            prompt,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<CompletionResponse> {
        let request = self.build()?;
        self.client.post_json("completions", &request).await
    }
}
