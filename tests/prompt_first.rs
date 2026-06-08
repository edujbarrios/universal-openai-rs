use serde_json::json;
use universal_openai_rs::{Client, Config, Tool};

#[test]
fn prompt_builder_uses_default_model() {
    let client = Client::new(Config::new("test-key").with_default_model("gpt-4o-mini")).unwrap();

    let request = client
        .prompt("Explain OpenAI-compatible APIs.")
        .system("Be concise.")
        .temperature(0.2)
        .into_chat()
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(request.model, "gpt-4o-mini");
    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.temperature, Some(0.2));
}

#[test]
fn prompt_builder_can_attach_schema_and_tools() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .prompt("Should I bring an umbrella?")
        .model("gpt-4o-mini")
        .tool(Tool::function(
            "get_weather",
            "Get weather for a city.",
            json!({
                "type": "object",
                "properties": {
                    "city": {"type": "string"}
                },
                "required": ["city"]
            }),
        ))
        .json_schema(
            "packing_advice",
            json!({
                "type": "object",
                "properties": {
                    "bring_umbrella": {"type": "boolean"}
                },
                "required": ["bring_umbrella"]
            }),
        )
        .into_chat()
        .unwrap()
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["tools"][0]["function"]["name"], "get_weather");
    assert_eq!(
        serialized["response_format"]["json_schema"]["name"],
        "packing_advice"
    );
}

