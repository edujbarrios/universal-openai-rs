mod chat;
mod client;
mod config;
mod embeddings;
mod error;
mod responses;

pub use chat::{
    ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
    ChatRequestBuilder, ChatRole, ChatStream, ChatStreamChoice, ChatStreamDelta,
    ChatStreamEvent, ChatStreamToolCall, ChatStreamToolCallFunction, FunctionTool,
    Tool, ToolCall, ToolCallFunction, Usage,
};
pub use client::Client;
pub use config::{Config, Provider};
pub use embeddings::{
    EmbeddingData, EmbeddingInput, EmbeddingUsage, EmbeddingsRequest,
    EmbeddingsRequestBuilder, EmbeddingsResponse,
};
pub use error::{Error, Result};
pub use responses::{
    ResponseInput, ResponseRequestBuilder, ResponsesRequest, ResponsesResponse,
};

pub mod prelude {
    pub use crate::{
        ChatMessage, Client, Config, Error, Provider, ResponseInput, Result, Tool,
    };
}
