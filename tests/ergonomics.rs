use serde::Deserialize;
use universal_openai_rs::{
    ChatChoice, ChatCompletionResponse, ChatMessage, EmbeddingData, EmbeddingsResponse,
    Provider, ResponsesResponse,
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
