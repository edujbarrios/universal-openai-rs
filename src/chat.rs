use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ChatStream, Client, Error, Result, StreamDecoder, TextChunkStream};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: ChatContent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: ChatContent::text(content),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: ChatContent::text(content),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: ChatContent::text(content),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Tool,
            content: ChatContent::text(content),
            tool_call_id: Some(tool_call_id.into()),
            tool_calls: None,
        }
    }

    pub fn user_parts(parts: Vec<ChatContentPart>) -> Self {
        Self {
            role: ChatRole::User,
            content: ChatContent::Parts(parts),
            tool_call_id: None,
            tool_calls: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatContent {
    Null,
    Text(String),
    Parts(Vec<ChatContentPart>),
}

impl ChatContent {
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Null => None,
            Self::Text(text) => Some(text),
            Self::Parts(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

impl ChatContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    pub fn image_url(url: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: ImageUrl {
                url: url.into(),
                detail: None,
            },
        }
    }

    pub fn image_url_detail(url: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: ImageUrl {
                url: url.into(),
                detail: Some(detail.into()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionTool,
}

impl Tool {
    pub fn function(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Value,
    ) -> Self {
        Self {
            kind: "function".to_string(),
            function: FunctionTool {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<Usage>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamEvent {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<ChatStreamChoice>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamChoice {
    pub index: Option<u32>,
    pub delta: ChatStreamDelta,
    pub finish_reason: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamDelta {
    pub role: Option<ChatRole>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ChatStreamToolCall>>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamToolCall {
    pub index: Option<u32>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub function: Option<ChatStreamToolCallFunction>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStreamToolCallFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

impl ChatCompletionResponse {
    pub fn first_text(&self) -> Option<&str> {
        self.choices
            .first()
            .and_then(|choice| choice.message.content.as_text())
    }

    pub fn text(self) -> Result<String> {
        self.choices
            .into_iter()
            .next()
            .and_then(|choice| match choice.message.content {
                ChatContent::Null => None,
                ChatContent::Text(text) => Some(text),
                ChatContent::Parts(_) => None,
            })
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: Option<u32>,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ChatRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    stream: Option<bool>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<Value>,
    response_format: Option<Value>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> ChatRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
            stream: None,
            tools: None,
            tool_choice: None,
            response_format: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn message(mut self, message: ChatMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn system(self, content: impl Into<String>) -> Self {
        self.message(ChatMessage::system(content))
    }

    pub fn user(self, content: impl Into<String>) -> Self {
        self.message(ChatMessage::user(content))
    }

    pub fn assistant(self, content: impl Into<String>) -> Self {
        self.message(ChatMessage::assistant(content))
    }

    pub fn user_parts(self, parts: Vec<ChatContentPart>) -> Self {
        self.message(ChatMessage::user_parts(parts))
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tool);
        self
    }

    pub fn tool_choice(mut self, tool_choice: impl Into<Value>) -> Self {
        self.tool_choice = Some(tool_choice.into());
        self
    }

    pub fn json_object(mut self) -> Self {
        self.response_format = Some(serde_json::json!({ "type": "json_object" }));
        self
    }

    pub fn json_schema(mut self, name: impl Into<String>, schema: Value) -> Self {
        self.response_format = Some(serde_json::json!({
            "type": "json_schema",
            "json_schema": {
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

    pub fn build(self) -> Result<ChatCompletionRequest> {
        let model = self
            .model
            .filter(|model| !model.trim().is_empty())
            .ok_or_else(|| Error::InvalidConfig("chat model is required".to_string()))?;

        if self.messages.is_empty() {
            return Err(Error::InvalidConfig(
                "at least one chat message is required".to_string(),
            ));
        }

        Ok(ChatCompletionRequest {
            model,
            messages: self.messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            stream: self.stream,
            tools: self.tools,
            tool_choice: self.tool_choice,
            response_format: self.response_format,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<ChatCompletionResponse> {
        let request = self.build()?;
        self.client.post_json("chat/completions", &request).await
    }

    pub async fn stream_events(mut self) -> Result<ChatStream> {
        self.stream = Some(true);
        let request = self.build()?;
        self.client.post_sse("chat/completions", &request).await
    }

    pub async fn stream_events_with_decoder<D>(mut self, decoder: D) -> Result<ChatStream>
    where
        D: StreamDecoder<Event = ChatStreamEvent> + Send + 'static,
    {
        self.stream = Some(true);
        let request = self.build()?;
        self.client
            .post_stream("chat/completions", &request, decoder)
            .await
    }

    pub async fn stream(self) -> Result<ChatStream> {
        self.stream_events().await
    }

    pub async fn stream_text_chunks(self) -> Result<TextChunkStream> {
        let events = self.stream_events().await?;
        Ok(crate::streaming::text_chunks_from_events(events))
    }

    pub async fn stream_text(self) -> Result<String> {
        let mut stream = self.stream_text_chunks().await?;
        let mut output = String::new();

        while let Some(chunk) = stream.next().await {
            output.push_str(&chunk?);
        }

        Ok(output)
    }
}
