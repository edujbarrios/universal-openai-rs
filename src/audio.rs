use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone)]
pub struct Audio<'a> {
    client: &'a Client,
}

impl<'a> Audio<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn transcription(&self) -> TranscriptionBuilder<'a> {
        TranscriptionBuilder::new(self.client, "audio/transcriptions")
    }

    pub fn translation(&self) -> TranslationBuilder<'a> {
        TranslationBuilder::new(self.client, "audio/translations")
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptionBuilder<'a> {
    client: &'a Client,
    endpoint: &'static str,
    model: Option<String>,
    filename: Option<String>,
    bytes: Option<Vec<u8>>,
    response_format: Option<String>,
    language: Option<String>,
    prompt: Option<String>,
    extra: Vec<(String, String)>,
}

pub type TranslationBuilder<'a> = TranscriptionBuilder<'a>;

impl<'a> TranscriptionBuilder<'a> {
    pub(crate) fn new(client: &'a Client, endpoint: &'static str) -> Self {
        Self {
            client,
            endpoint,
            model: None,
            filename: None,
            bytes: None,
            response_format: None,
            language: None,
            prompt: None,
            extra: Vec::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn file(mut self, filename: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        self.filename = Some(filename.into());
        self.bytes = Some(bytes.into());
        self
    }

    pub fn response_format(mut self, response_format: impl Into<String>) -> Self {
        self.response_format = Some(response_format.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn extra_text(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.push((key.into(), value.into()));
        self
    }

    pub async fn send(self) -> Result<AudioResponse> {
        let model = self
            .model
            .ok_or_else(|| Error::InvalidConfig("audio model is required".to_string()))?;
        let filename = self
            .filename
            .ok_or_else(|| Error::InvalidConfig("audio file name is required".to_string()))?;
        let bytes = self
            .bytes
            .ok_or_else(|| Error::InvalidConfig("audio bytes are required".to_string()))?;

        let part = reqwest::multipart::Part::bytes(bytes).file_name(filename);
        let mut form = reqwest::multipart::Form::new()
            .text("model", model)
            .part("file", part);

        if let Some(response_format) = self.response_format {
            form = form.text("response_format", response_format);
        }
        if let Some(language) = self.language {
            form = form.text("language", language);
        }
        if let Some(prompt) = self.prompt {
            form = form.text("prompt", prompt);
        }
        for (key, value) in self.extra {
            form = form.text(key, value);
        }

        self.client.post_multipart(self.endpoint, form).await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioResponse {
    pub text: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
