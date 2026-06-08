use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Client, Error, Result};

#[derive(Debug, Clone)]
pub struct FineTuning<'a> {
    client: &'a Client,
}

impl<'a> FineTuning<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self) -> FineTuningJobRequestBuilder<'a> {
        FineTuningJobRequestBuilder::new(self.client)
    }

    pub async fn list(&self) -> Result<ListFineTuningJobsResponse> {
        self.client.get_json("fine_tuning/jobs").await
    }

    pub async fn retrieve(&self, job_id: &str) -> Result<FineTuningJob> {
        self.client
            .get_json(&format!("fine_tuning/jobs/{job_id}"))
            .await
    }

    pub async fn cancel(&self, job_id: &str) -> Result<FineTuningJob> {
        self.client
            .post_json(
                &format!("fine_tuning/jobs/{job_id}/cancel"),
                &serde_json::json!({}),
            )
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobRequest {
    pub model: String,
    pub training_file: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<Value>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
#[derive(Debug, Clone)]
pub struct FineTuningJobRequestBuilder<'a> {
    client: &'a Client,
    model: Option<String>,
    training_file: Option<String>,
    validation_file: Option<String>,
    suffix: Option<String>,
    hyperparameters: Option<Value>,
    extra: serde_json::Map<String, Value>,
}

impl<'a> FineTuningJobRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            model: None,
            training_file: None,
            validation_file: None,
            suffix: None,
            hyperparameters: None,
            extra: serde_json::Map::new(),
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn training_file(mut self, training_file: impl Into<String>) -> Self {
        self.training_file = Some(training_file.into());
        self
    }

    pub fn validation_file(mut self, validation_file: impl Into<String>) -> Self {
        self.validation_file = Some(validation_file.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn hyperparameters(mut self, hyperparameters: impl Into<Value>) -> Self {
        self.hyperparameters = Some(hyperparameters.into());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Result<FineTuningJobRequest> {
        let model = self
            .model
            .ok_or_else(|| Error::InvalidConfig("fine-tuning model is required".to_string()))?;
        let training_file = self.training_file.ok_or_else(|| {
            Error::InvalidConfig("fine-tuning training file is required".to_string())
        })?;

        Ok(FineTuningJobRequest {
            model,
            training_file,
            validation_file: self.validation_file,
            suffix: self.suffix,
            hyperparameters: self.hyperparameters,
            extra: self.extra,
        })
    }

    pub async fn send(self) -> Result<FineTuningJob> {
        let request = self.build()?;
        self.client.post_json("fine_tuning/jobs", &request).await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListFineTuningJobsResponse {
    pub object: Option<String>,
    pub data: Vec<FineTuningJob>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJob {
    pub id: String,
    pub object: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<u64>,
    pub finished_at: Option<u64>,
    pub fine_tuned_model: Option<String>,
    pub training_file: Option<String>,
    pub validation_file: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
