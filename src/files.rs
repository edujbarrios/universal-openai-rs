use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone)]
pub struct Files<'a> {
    client: &'a Client,
}

impl<'a> Files<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<ListFilesResponse> {
        self.client.get_json("files").await
    }

    pub async fn retrieve(&self, file_id: &str) -> Result<FileObject> {
        self.client.get_json(&format!("files/{file_id}")).await
    }

    pub async fn delete(&self, file_id: &str) -> Result<DeletedFile> {
        self.client.delete_json(&format!("files/{file_id}")).await
    }
}

#[derive(Debug, Clone)]
pub struct FileUploadBuilder<'a> {
    client: &'a Client,
    purpose: String,
    filename: Option<String>,
    bytes: Option<Vec<u8>>,
    extra: Vec<(String, String)>,
}

impl<'a> FileUploadBuilder<'a> {
    pub(crate) fn new(client: &'a Client, purpose: impl Into<String>) -> Self {
        Self {
            client,
            purpose: purpose.into(),
            filename: None,
            bytes: None,
            extra: Vec::new(),
        }
    }

    pub fn bytes(mut self, filename: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Self {
        self.filename = Some(filename.into());
        self.bytes = Some(bytes.into());
        self
    }

    pub fn extra_text(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.push((key.into(), value.into()));
        self
    }

    pub async fn send(self) -> Result<UploadedFile> {
        let filename = self
            .filename
            .ok_or_else(|| Error::InvalidConfig("file name is required".to_string()))?;
        let bytes = self
            .bytes
            .ok_or_else(|| Error::InvalidConfig("file bytes are required".to_string()))?;

        let part = reqwest::multipart::Part::bytes(bytes).file_name(filename);
        let mut form = reqwest::multipart::Form::new()
            .text("purpose", self.purpose)
            .part("file", part);

        for (key, value) in self.extra {
            form = form.text(key, value);
        }

        self.client.post_multipart("files", form).await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListFilesResponse {
    pub object: Option<String>,
    pub data: Vec<FileObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileObject {
    pub id: String,
    pub object: Option<String>,
    pub bytes: Option<u64>,
    pub created_at: Option<u64>,
    pub filename: Option<String>,
    pub purpose: Option<String>,
    pub status: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub type UploadedFile = FileObject;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletedFile {
    pub id: String,
    pub object: Option<String>,
    pub deleted: bool,
}

