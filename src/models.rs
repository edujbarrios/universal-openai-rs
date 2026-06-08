use serde::{Deserialize, Serialize};

use crate::{Client, Result};

#[derive(Debug, Clone)]
pub struct Models<'a> {
    client: &'a Client,
}

impl<'a> Models<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<ListModelsResponse> {
        self.client.get_json("models").await
    }

    pub async fn retrieve(&self, model: &str) -> Result<Model> {
        self.client.get_json(&format!("models/{model}")).await
    }

    pub async fn delete(&self, model: &str) -> Result<DeletedModel> {
        self.client.delete_json(&format!("models/{model}")).await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub object: Option<String>,
    pub data: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub owned_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletedModel {
    pub id: String,
    pub object: Option<String>,
    pub deleted: bool,
}
