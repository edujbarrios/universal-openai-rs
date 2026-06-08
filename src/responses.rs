use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result, Tool};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: ResponseInput,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseInput {
    Text(String),
    Items(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsesResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub status: Option<String>,
    pub model: Option<String>,
    pub output: Option<Vec<Value>>,
    pub output_text: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ResponseRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    input: Option<ResponseInput>,
    instructions: Option<String>,
    temperature: Option<f32>,
    max_output_tokens: Option<u32>,
    tools: Option<Vec<Tool>>,
    text: Option<Value>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> ResponseRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            input: None,
            instructions: None,
            temperature: None,
            max_output_tokens: None,
            tools: None,
            text: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = Some(ResponseInput::Text(input.into()));
        self
    }

    pub fn input_items(mut self, items: Vec<Value>) -> Self {
        self.input = Some(ResponseInput::Items(items));
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_output_tokens(mut self, max_output_tokens: u32) -> Self {
        self.max_output_tokens = Some(max_output_tokens);
        self
    }

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tool);
        self
    }

    pub fn json_schema(mut self, name: impl Into<String>, schema: Value) -> Self {
        self.text = Some(serde_json::json!({
            "format": {
                "type": "json_schema",
                "name": name.into(),
                "schema": schema
            }
        }));
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<ResponsesRequest> {
        let model = self
            .model
            .filter(|model| !model.trim().is_empty())
            .ok_or_else(|| Error::InvalidConfig("response model is required".to_string()))?;

        let input = self
            .input
            .ok_or_else(|| Error::InvalidConfig("response input is required".to_string()))?;

        Ok(ResponsesRequest {
            model,
            input,
            instructions: self.instructions,
            temperature: self.temperature,
            max_output_tokens: self.max_output_tokens,
            tools: self.tools,
            text: self.text,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<ResponsesResponse> {
        let request = self.build()?;
        self.client.post_json("responses", &request).await
    }
}

impl ResponsesResponse {
    pub fn text(self) -> Result<String> {
        self.output_text.ok_or(Error::MissingText)
    }

    pub fn json<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = self.output_text.as_deref().ok_or(Error::MissingText)?;
        Ok(serde_json::from_str(text)?)
    }
}
