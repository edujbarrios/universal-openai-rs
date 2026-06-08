use universal_openai_rs::{Client, Config};

#[test]
fn default_model_preloads_chat_builder() {
    let client = Client::new(Config::new("test-key").with_default_model("gpt-4o-mini")).unwrap();

    let request = client
        .chat_default()
        .unwrap()
        .user("Hello")
        .build()
        .unwrap();

    assert_eq!(request.model, "gpt-4o-mini");
}
