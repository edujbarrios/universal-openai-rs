use serde_json::json;
use universal_openai_rs::{Client, Config};

#[test]
fn builds_legacy_completion_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .completions()
        .model("gpt-3.5-turbo-instruct")
        .prompt("Complete this sentence")
        .temperature(0.3)
        .max_tokens(64)
        .build()
        .unwrap();

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "model": "gpt-3.5-turbo-instruct",
            "prompt": "Complete this sentence",
            "temperature": 0.3,
            "max_tokens": 64
        })
    );
}

#[test]
fn builds_image_generation_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .images()
        .model("gpt-image-1")
        .prompt("A clean Rust API diagram")
        .size("1024x1024")
        .b64_json()
        .build()
        .unwrap();

    assert_eq!(request.prompt, "A clean Rust API diagram");
    assert_eq!(request.response_format.as_deref(), Some("b64_json"));
}

#[test]
fn builds_moderation_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .moderations()
        .model("omni-moderation-latest")
        .input("Please classify this content.")
        .build()
        .unwrap();

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "model": "omni-moderation-latest",
            "input": "Please classify this content."
        })
    );
}

#[test]
fn builds_fine_tuning_job_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .fine_tuning()
        .create()
        .model("gpt-4o-mini")
        .training_file("file-train")
        .validation_file("file-valid")
        .suffix("universal-openai-rs")
        .hyperparameters(json!({"n_epochs": 2}))
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["model"], "gpt-4o-mini");
    assert_eq!(serialized["training_file"], "file-train");
    assert_eq!(serialized["hyperparameters"]["n_epochs"], 2);
}

