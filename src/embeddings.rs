use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingsRequest {
    pub model: String,
    pub input: EmbeddingInput,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Text(String),
    Texts(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: Option<String>,
    pub data: Vec<EmbeddingData>,
    pub model: Option<String>,
    pub usage: Option<EmbeddingUsage>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

impl EmbeddingsResponse {
    pub fn first_vector(&self) -> Option<Vec<f32>> {
        self.data.first().map(|item| item.embedding.clone())
    }

    pub fn vectors(self) -> Vec<Vec<f32>> {
        self.data.into_iter().map(|item| item.embedding).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub object: Option<String>,
    pub embedding: Vec<f32>,
    pub index: Option<u32>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct EmbeddingsRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    input: Option<EmbeddingInput>,
    dimensions: Option<u32>,
    encoding_format: Option<String>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> EmbeddingsRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            input: None,
            dimensions: None,
            encoding_format: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(EmbeddingInput::Text(input.into()));
        self
    }

    pub fn inputs<I, S>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.input = Some(EmbeddingInput::Texts(
            inputs.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn encoding_format(mut self, encoding_format: impl Into<String>) -> Self {
        self.encoding_format = Some(encoding_format.into());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<EmbeddingsRequest> {
        let model = self
            .model
            .filter(|model| !model.trim().is_empty())
            .ok_or_else(|| Error::InvalidConfig("embedding model is required".to_string()))?;

        let input = self
            .input
            .ok_or_else(|| Error::InvalidConfig("embedding input is required".to_string()))?;

        Ok(EmbeddingsRequest {
            model,
            input,
            dimensions: self.dimensions,
            encoding_format: self.encoding_format,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<EmbeddingsResponse> {
        let request = self.build()?;
        self.client.post_json("embeddings", &request).await
    }
}
