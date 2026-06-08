use serde_json::Value;

use crate::{ChatRequestBuilder, Client, Error, Result, Tool};

#[derive(Debug, Clone)]
pub struct PromptBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    system: Option<String>,
    prompt: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    tools: Vec<Tool>,
    json_object: bool,
    json_schema: Option<(String, Value)>,
}

impl<'a> PromptBuilder<'a> {
    pub(crate) fn new(client: &'a Client, prompt: impl Into<String>) -> Self {
        Self {
            client,
            model: None,
            system: None,
            prompt: prompt.into(),
            temperature: None,
            max_tokens: None,
            tools: Vec::new(),
            json_object: false,
            json_schema: None,
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
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

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn json_object(mut self) -> Self {
        self.json_object = true;
        self
    }

    pub fn json_schema(mut self, name: impl Into<String>, schema: Value) -> Self {
        self.json_schema = Some((name.into(), schema));
        self
    }

    pub fn into_chat(self) -> Result<ChatRequestBuilder<'a>> {
        let model = self
            .model
            .or_else(|| self.client.default_model().map(ToOwned::to_owned))
            .ok_or_else(|| Error::InvalidConfig("prompt model is required".to_string()))?;

        let mut chat = self.client.chat().model(model);

        if let Some(system) = self.system {
            chat = chat.system(system);
        }

        chat = chat.user(self.prompt);

        if let Some(temperature) = self.temperature {
            chat = chat.temperature(temperature);
        }

        if let Some(max_tokens) = self.max_tokens {
            chat = chat.max_tokens(max_tokens);
        }

        for tool in self.tools {
            chat = chat.tool(tool);
        }

        if let Some((name, schema)) = self.json_schema {
            chat = chat.json_schema(name, schema);
        } else if self.json_object {
            chat = chat.json_object();
        }

        Ok(chat)
    }

    pub async fn run_text(self) -> Result<String> {
        self.into_chat()?.send().await?.text()
    }

    pub async fn run_json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.json_object().into_chat()?.send().await?;
        response.json()
    }

    pub async fn stream_text(self) -> Result<String> {
        self.into_chat()?.stream_text().await
    }
}

