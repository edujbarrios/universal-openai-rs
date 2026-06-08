//! Simple, well-structured API calls for OpenAI-compatible providers.
//!
//! `universal-openai-rs` keeps OpenAI-compatible request and response shapes
//! available while adding a smaller Rust-friendly layer for common workflows.
//!
//! The crate is designed around three levels:
//!
//! - Intent-first helpers such as [`Client::ask`], [`Client::prompt`], and
//!   [`Client::embed`].
//! - Spec-shaped builders such as [`Client::chat`], [`Client::responses`], and
//!   [`Client::embeddings`].
//! - Raw compatibility escape hatches through [`Client::send_compatible`],
//!   [`Client::get_compatible`], and [`Client::delete_compatible`].
//!
//! ```no_run
//! use universal_openai_rs::Client;
//!
//! # async fn run() -> universal_openai_rs::Result<()> {
//! let client = Client::from_env()?;
//! let text = client.ask("gpt-4o-mini", "Explain Rust in one sentence.").await?;
//! println!("{text}");
//! # Ok(())
//! # }
//! ```
//!
//! For provider-agnostic usage, configure a preset provider or any custom
//! OpenAI-compatible base URL.
//!
//! ```no_run
//! use universal_openai_rs::{Client, Provider};
//!
//! # async fn run() -> universal_openai_rs::Result<()> {
//! let client = Client::for_provider("ollama", Provider::Ollama)?;
//! let text = client.ask("llama3.2", "Say hello from a local model.").await?;
//! # Ok(())
//! # }
//! ```

mod chat;
mod client;
mod config;
mod completions;
mod embeddings;
mod error;
mod files;
mod fine_tuning;
mod images;
mod models;
mod moderations;
mod audio;
mod agents;
mod prompt;
mod responses;

pub use agents::{AgentChainRun, AgentRun, AgentSpec, Agents};
pub use chat::{
    ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatContent,
    ChatContentPart, ChatMessage, ChatRequestBuilder, ChatRole, ChatStream,
    ChatStreamChoice, ChatStreamDelta, ChatStreamEvent, ChatStreamToolCall,
    ChatStreamToolCallFunction, FunctionTool, ImageUrl, Tool, ToolCall,
    ToolCallFunction, Usage,
};
pub use client::Client;
pub use config::{Config, Provider};
pub use completions::{
    CompletionChoice, CompletionPrompt, CompletionRequest, CompletionRequestBuilder,
    CompletionResponse,
};
pub use embeddings::{
    EmbeddingData, EmbeddingInput, EmbeddingUsage, EmbeddingsRequest,
    EmbeddingsRequestBuilder, EmbeddingsResponse,
};
pub use error::{Error, Result};
pub use files::{
    DeletedFile, FileObject, FileUploadBuilder, Files, ListFilesResponse, UploadedFile,
};
pub use fine_tuning::{
    FineTuning, FineTuningJob, FineTuningJobRequest, FineTuningJobRequestBuilder,
    ListFineTuningJobsResponse,
};
pub use images::{
    ImageData, ImageGenerationRequest, ImageResponse, ImagesRequestBuilder,
};
pub use models::{DeletedModel, ListModelsResponse, Model, Models};
pub use moderations::{
    ModerationCategoryScores, ModerationInput, ModerationRequest,
    ModerationRequestBuilder, ModerationResponse, ModerationResult,
};
pub use audio::{
    Audio, AudioResponse, TranscriptionBuilder, TranslationBuilder,
};
pub use prompt::PromptBuilder;
pub use responses::{
    ResponseContentPart, ResponseInput, ResponseInputItem, ResponseRequestBuilder,
    ResponsesRequest, ResponsesResponse,
};

pub mod prelude {
    pub use crate::{
        AgentChainRun, AgentRun, AgentSpec, Agents, ChatContentPart, ChatMessage, Client,
        Config, Error, PromptBuilder, Provider, ResponseContentPart,
        ResponseInput, ResponseInputItem, Result, Tool,
    };
}
