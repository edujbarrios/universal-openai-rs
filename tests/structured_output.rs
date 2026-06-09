#![cfg(feature = "structured-output")]

use schemars::JsonSchema;
use serde::Deserialize;
use universal_openai_rs::{Client, Config};

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(dead_code)]
struct Profile {
    title: String,
    strengths: Vec<String>,
}

#[test]
fn chat_builder_generates_schema_from_type() {
    let client = Client::new(Config::new("test-key")).unwrap();
    let request = client
        .chat()
        .model("gpt-4o-mini")
        .user("Return a profile.")
        .json_schema_auto::<Profile>()
        .build()
        .unwrap();
    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(
        serialized["response_format"]["json_schema"]["name"],
        "Profile"
    );
    assert_eq!(
        serialized["response_format"]["json_schema"]["schema"]["title"],
        "Profile"
    );
}

#[test]
fn responses_builder_generates_schema_from_type() {
    let client = Client::new(Config::new("test-key")).unwrap();
    let request = client
        .respond("Return a profile.")
        .model("gpt-4.1-mini")
        .json_schema_auto::<Profile>()
        .build()
        .unwrap();
    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["text"]["format"]["name"], "Profile");
    assert_eq!(serialized["text"]["format"]["schema"]["title"], "Profile");
}
