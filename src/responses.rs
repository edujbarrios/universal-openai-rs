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
    Items(Vec<ResponseInputItem>),
    RawItems(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseInputItem {
    #[serde(rename = "type")]
    pub kind: String,
    pub role: String,
    pub content: Vec<ResponseContentPart>,
}

impl ResponseInputItem {
    pub fn message(role: impl Into<String>, content: Vec<ResponseContentPart>) -> Self {
        Self {
            kind: "message".to_string(),
            role: role.into(),
            content,
        }
    }

    pub fn user(content: Vec<ResponseContentPart>) -> Self {
        Self::message("user", content)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseContentPart {
    InputText { text: String },
    InputImage { image_url: String },
}

impl ResponseContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::InputText { text: text.into() }
    }

    pub fn image_url(image_url: impl Into<String>) -> Self {
        Self::InputImage {
            image_url: image_url.into(),
        }
    }
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
        self.input = Some(ResponseInput::RawItems(items));
        self
    }

    pub fn input_messages(mut self, items: Vec<ResponseInputItem>) -> Self {
        self.input = Some(ResponseInput::Items(items));
        self
    }

    pub fn user_parts(mut self, parts: Vec<ResponseContentPart>) -> Self {
        self.input = Some(ResponseInput::Items(vec![ResponseInputItem::user(parts)]));
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
    pub fn first_text(&self) -> Option<&str> {
        self.output_text
            .as_deref()
            .or_else(|| self.output.as_deref().and_then(extract_text_from_output))
    }

    pub fn text(self) -> Result<String> {
        if let Some(output_text) = self.output_text {
            return Ok(output_text);
        }

        self.output
            .as_deref()
            .and_then(extract_text_from_output)
            .map(ToOwned::to_owned)
            .ok_or(Error::MissingText)
    }

    pub fn json<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = self.first_text().ok_or(Error::MissingText)?;
        Ok(serde_json::from_str(text)?)
    }
}

fn extract_text_from_output(output: &[Value]) -> Option<&str> {
    for item in output {
        if let Some(text) = item.get("text").and_then(Value::as_str) {
            return Some(text);
        }

        let Some(content) = item.get("content").and_then(Value::as_array) else {
            continue;
        };

        for part in content {
            if let Some(text) = part.get("text").and_then(Value::as_str) {
                return Some(text);
            }
        }
    }

    None
}
