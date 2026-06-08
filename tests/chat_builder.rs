use serde_json::json;
use universal_openai_rs::{
    ChatContent, ChatContentPart, ChatMessage, ChatRole, Client, Config, Error,
};

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
        content: ChatContent::text("Done."),
        tool_call_id: None,
        tool_calls: None,
    };

    assert_eq!(
        serde_json::to_value(message).unwrap(),
        json!({"role": "assistant", "content": "Done."})
    );
}

#[test]
fn builds_multimodal_chat_message() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .chat()
        .model("gpt-4o-mini")
        .user_parts(vec![
            ChatContentPart::text("Describe this image."),
            ChatContentPart::image_url("https://example.com/image.png"),
        ])
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["messages"][0]["content"][0]["type"], "text");
    assert_eq!(serialized["messages"][0]["content"][1]["type"], "image_url");
}

#[test]
fn builds_structured_output_and_tools_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let weather_tool = universal_openai_rs::Tool::function(
        "get_weather",
        "Get the current weather for a city.",
        json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            },
            "required": ["city"]
        }),
    );

    let request = client
        .chat()
        .model("gpt-4o-mini")
        .user("What is the weather in Madrid?")
        .tool(weather_tool)
        .json_schema(
            "weather_answer",
            json!({
                "type": "object",
                "properties": {
                    "city": {"type": "string"},
                    "summary": {"type": "string"}
                },
                "required": ["city", "summary"]
            }),
        )
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["tools"][0]["type"], "function");
    assert_eq!(
        serialized["response_format"]["json_schema"]["name"],
        "weather_answer"
    );
}
