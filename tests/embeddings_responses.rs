use serde_json::json;
use universal_openai_rs::{Client, Config, ResponseContentPart, Tool};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct InvoiceSummary {
    status: String,
    amount: f64,
}

#[test]
fn builds_embeddings_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .embeddings()
        .model("text-embedding-3-small")
        .inputs(["hello", "world"])
        .dimensions(512)
        .build()
        .unwrap();

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "model": "text-embedding-3-small",
            "input": ["hello", "world"],
            "dimensions": 512
        })
    );
}

#[test]
fn builds_multimodal_responses_request() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .responses()
        .model("gpt-4o-mini")
        .user_parts(vec![
            ResponseContentPart::text("Describe this image."),
            ResponseContentPart::image_url("https://example.com/image.png"),
        ])
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["input"][0]["type"], "message");
    assert_eq!(serialized["input"][0]["content"][0]["type"], "input_text");
    assert_eq!(serialized["input"][0]["content"][1]["type"], "input_image");
}

#[test]
fn builds_responses_request_with_schema_and_tool() {
    let client = Client::new(Config::new("test-key")).unwrap();

    let request = client
        .responses()
        .model("gpt-4o-mini")
        .instructions("Return concise JSON.")
        .input("Summarize the latest invoice.")
        .tool(Tool::function(
            "lookup_invoice",
            "Look up invoice metadata.",
            json!({
                "type": "object",
                "properties": {
                    "invoice_id": {"type": "string"}
                },
                "required": ["invoice_id"]
            }),
        ))
        .json_schema(
            "invoice_summary",
            json!({
                "type": "object",
                "properties": {
                    "status": {"type": "string"},
                    "amount": {"type": "number"}
                },
                "required": ["status", "amount"]
            }),
        )
        .build()
        .unwrap();

    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["model"], "gpt-4o-mini");
    assert_eq!(serialized["tools"][0]["function"]["name"], "lookup_invoice");
    assert_eq!(serialized["text"]["format"]["name"], "invoice_summary");
}

#[test]
fn builds_responses_first_request_from_client_respond() {
    let client = Client::new(Config::new("test-key")).unwrap();
    let request = client
        .respond("Summarize the latest invoice.")
        .model("gpt-4.1-mini")
        .instructions("Return concise JSON.")
        .json_schema_for::<InvoiceSummary>(json!({
            "type": "object",
            "properties": {
                "status": {"type": "string"},
                "amount": {"type": "number"}
            },
            "required": ["status", "amount"]
        }))
        .build()
        .unwrap();
    let serialized = serde_json::to_value(request).unwrap();

    assert_eq!(serialized["model"], "gpt-4.1-mini");
    assert_eq!(serialized["input"], "Summarize the latest invoice.");
    assert_eq!(serialized["text"]["format"]["name"], "InvoiceSummary");
}

#[test]
fn responses_response_exposes_typed_output_items_and_function_calls() {
    let response = universal_openai_rs::ResponsesResponse {
        id: None,
        object: None,
        status: None,
        model: None,
        output: Some(vec![
            json!({
                "type": "message",
                "content": [{"type": "output_text", "text": "done"}]
            }),
            json!({
                "type": "function_call",
                "name": "lookup_invoice",
                "arguments": "{\"invoice_id\":\"inv_1\"}",
                "call_id": "call_1"
            }),
        ]),
        output_text: None,
        extra: serde_json::Map::new(),
    };

    let items = response.output_items().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].content.as_ref().unwrap()[0].text.as_deref(), Some("done"));

    let calls = response.function_calls().unwrap();
    assert_eq!(calls[0].name.as_deref(), Some("lookup_invoice"));
    assert_eq!(calls[0].call_id.as_deref(), Some("call_1"));
}
