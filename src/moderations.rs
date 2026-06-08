use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationRequest {
    pub input: ModerationInput,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModerationInput {
    Text(String),
    Texts(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationResponse {
    pub id: Option<String>,
    pub model: Option<String>,
    pub results: Vec<ModerationResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: Value,
    pub category_scores: ModerationCategoryScores,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationCategoryScores {
    #[serde(flatten)]
    pub scores: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ModerationRequestBuilder<'a> {
    client: &'a Client,
    input: Option<ModerationInput>,
    model: Option<String>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> ModerationRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            input: None,
            model: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(ModerationInput::Text(input.into()));
        self
    }

    pub fn inputs<I, S>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.input = Some(ModerationInput::Texts(
            inputs.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<ModerationRequest> {
        let input = self
            .input
            .ok_or_else(|| Error::InvalidConfig("moderation input is required".to_string()))?;

        Ok(ModerationRequest {
            input,
            model: self.model,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<ModerationResponse> {
        let request = self.build()?;
        self.client.post_json("moderations", &request).await
    }
}

