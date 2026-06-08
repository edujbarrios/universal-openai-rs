use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageResponse {
    pub created: Option<u64>,
    pub data: Vec<ImageData>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    pub url: Option<String>,
    pub b64_json: Option<String>,
    pub revised_prompt: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ImagesRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    prompt: Option<String>,
    n: Option<u32>,
    size: Option<String>,
    response_format: Option<String>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> ImagesRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            prompt: None,
            n: None,
            size: None,
            response_format: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn b64_json(mut self) -> Self {
        self.response_format = Some("b64_json".to_string());
        self
    }

    pub fn url(mut self) -> Self {
        self.response_format = Some("url".to_string());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<ImageGenerationRequest> {
        let prompt = self
            .prompt
            .filter(|prompt| !prompt.trim().is_empty())
            .ok_or_else(|| Error::InvalidConfig("image prompt is required".to_string()))?;

        Ok(ImageGenerationRequest {
            prompt,
            model: self.model,
            n: self.n,
            size: self.size,
            response_format: self.response_format,
            extra: self.extra,
        })
    }

    pub async fn generate(self) -> Result<ImageResponse> {
        let request = self.build()?;
        self.client.post_json("images/generations", &request).await
    }
}

