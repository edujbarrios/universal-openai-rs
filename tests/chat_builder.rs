use serde_json::json;
use universal_openai::{ChatMessage, ChatRole, Client, Config, Error};

#[test]
fn builds_openai_compatible_chat_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .chat()
        .model("gpt-4o-mini")
        .system("Be concise.")
        .user("Say hello.")
        .temperature(0.1)
        .max_tokens(32)
        .extra("top_p", json!(0.9))
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(
        serialized,
        json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "system", "content": "Be concise."},
                {"role": "user", "content": "Say hello."}
            ],
            "temperature": 0.1,
            "max_tokens": 32,
            "top_p": 0.9
        })
    );
}

#[test]
fn rejects_missing_model() {
    let client = Client::new(Config::new("test-key")).unwrap();
    let error = client.chat().user("Hello").build().unwrap_err();

    assert!(matches!(error, Error::InvalidConfig(_)));
}

#[test]
fn rejects_missing_messages() {
    let client = Client::new(Config::new("test-key")).unwrap();
    let error = client.chat().model("gpt-4o-mini").build().unwrap_err();

    assert!(matches!(error, Error::InvalidConfig(_)));
}

#[test]
fn serializes_chat_message_roles() {
    let message = ChatMessage {
        role: ChatRole::Assistant,
        content: "Done.".to_string(),
    };

    assert_eq!(
        serde_json::to_value(message).unwrap(),
        json!({"role": "assistant", "content": "Done."})
    );
}

