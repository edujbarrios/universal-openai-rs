use std::time::Duration;

use serde::Deserialize;
use universal_openai_rs::{
    ApiError, ChatChoice, ChatCompletionResponse, ChatMessage, Client, Config, EmbeddingData,
    EmbeddingsResponse, Error, Provider, ResponsesResponse, RetryConfig,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Profile {
    title: String,
}

#[test]
fn provider_presets_expose_openai_compatible_base_urls() {
    assert_eq!(Provider::OpenAI.base_url(), "https://api.openai.com/v1");
    assert_eq!(Provider::Ollama.base_url(), "http://localhost:11434/v1");
    assert_eq!(
        Provider::Custom("https://api.example.com/v1".to_string()).base_url(),
        "https://api.example.com/v1"
    );
}

#[test]
fn config_trims_base_url_whitespace_and_trailing_slashes() {
    let config = Config::new("test-key").with_base_url(" https://api.example.com/v1/// ");

    assert_eq!(config.base_url(), "https://api.example.com/v1");
}

#[test]
fn config_stores_production_http_headers() {
    let config = Config::new("test-key")
        .with_user_agent("universal-openai-rs-test/0.1")
        .with_organization("org-test")
        .with_project("proj-test")
        .with_header("x-provider-routing", "fast");

    assert_eq!(config.user_agent(), Some("universal-openai-rs-test/0.1"));
    assert_eq!(config.organization(), Some("org-test"));
    assert_eq!(config.project(), Some("proj-test"));
    assert_eq!(config.headers().len(), 1);
}

#[test]
fn config_debug_redacts_api_key_and_header_values() {
    let config = Config::new("sk-secret-test-key")
        .with_base_url("https://api.example.com/v1")
        .with_header("x-provider-secret", "secret-header-value")
        .with_default_model("gpt-4o-mini");

    let debug = format!("{config:?}");

    assert!(debug.contains("api_key: \"<redacted>\""));
    assert!(debug.contains("https://api.example.com/v1"));
    assert!(debug.contains("x-provider-secret"));
    assert!(!debug.contains("sk-secret-test-key"));
    assert!(!debug.contains("secret-header-value"));
}

#[test]
fn config_accepts_full_retry_configuration() {
    let retry = RetryConfig {
        max_retries: 5,
        initial_backoff: Duration::from_millis(250),
        max_backoff: Duration::from_secs(10),
        jitter: false,
        respect_retry_after: true,
    };
    let config = Config::new("test-key").with_retry_config(retry.clone());

    assert_eq!(config.retry_config(), &retry);
    assert_eq!(config.max_retries(), 5);
}

#[test]
fn max_retries_keeps_backward_compatible_builder() {
    let config = Config::new("test-key").with_max_retries(4);

    assert_eq!(config.retry_config().max_retries, 4);
}

#[test]
fn custom_http_client_still_validates_config() {
    let error = Client::with_http_client(Config::new(" "), reqwest::Client::new()).unwrap_err();

    assert!(matches!(error, Error::InvalidConfig(_)));
}

#[test]
fn custom_http_client_rejects_invalid_headers() {
    let config = Config::new("test-key").with_header("bad header", "value");
    let error = Client::with_http_client(config, reqwest::Client::new()).unwrap_err();

    assert!(matches!(error, Error::InvalidConfig(_)));
}

#[test]
fn api_error_extracts_openai_compatible_fields_and_request_id() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "x-request-id",
        reqwest::header::HeaderValue::from_static("req_123"),
    );

    let error = ApiError::from_parts(
        reqwest::StatusCode::BAD_REQUEST,
        &headers,
        r#"{"error":{"type":"invalid_request_error","code":"bad_model","param":"model"}}"#,
    );

    assert_eq!(error.status, reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(error.error_type.as_deref(), Some("invalid_request_error"));
    assert_eq!(error.code.as_deref(), Some("bad_model"));
    assert_eq!(error.param.as_deref(), Some("model"));
    assert_eq!(error.request_id.as_deref(), Some("req_123"));
}

#[test]
fn api_error_keeps_raw_body_for_non_json_provider_errors() {
    let headers = reqwest::header::HeaderMap::new();
    let error = ApiError::from_parts(
        reqwest::StatusCode::BAD_GATEWAY,
        &headers,
        "upstream down",
    );

    assert_eq!(error.body, "upstream down");
    assert_eq!(error.error_type, None);
    assert_eq!(error.code, None);
    assert_eq!(error.param, None);
    assert_eq!(error.request_id, None);
}

#[test]
fn chat_response_can_parse_json_text() {
    let response = ChatCompletionResponse {
        id: None,
        object: None,
        created: None,
        model: None,
        choices: vec![ChatChoice {
            index: Some(0),
            message: ChatMessage::assistant(r#"{"title":"AI Engineer"}"#),
            finish_reason: Some("stop".to_string()),
            extra: serde_json::Map::new(),
        }],
        usage: None,
        extra: serde_json::Map::new(),
    };

    let profile: Profile = response.json().unwrap();
    assert_eq!(profile.title, "AI Engineer");
}

#[test]
fn responses_response_can_parse_output_text() {
    let response = ResponsesResponse {
        id: None,
        object: None,
        status: None,
        model: None,
        output: None,
        output_text: Some(r#"{"title":"AI Engineer"}"#.to_string()),
        extra: serde_json::Map::new(),
    };

    let profile: Profile = response.json().unwrap();
    assert_eq!(profile.title, "AI Engineer");
}

#[test]
fn responses_response_extracts_text_from_output_items() {
    let response = ResponsesResponse {
        id: None,
        object: None,
        status: None,
        model: None,
        output: Some(vec![serde_json::json!({
            "type": "message",
            "content": [{
                "type": "output_text",
                "text": "fallback text"
            }]
        })]),
        output_text: None,
        extra: serde_json::Map::new(),
    };

    assert_eq!(response.first_text(), Some("fallback text"));
    assert_eq!(response.text().unwrap(), "fallback text");
}

#[test]
fn embeddings_response_extracts_vectors() {
    let response = EmbeddingsResponse {
        object: None,
        data: vec![EmbeddingData {
            object: None,
            embedding: vec![0.1, 0.2, 0.3],
            index: Some(0),
            extra: serde_json::Map::new(),
        }],
        model: None,
        usage: None,
        extra: serde_json::Map::new(),
    };

    assert_eq!(response.first_vector().unwrap(), vec![0.1, 0.2, 0.3]);
}
