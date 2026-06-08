use serde_json::json;
use universal_openai_rs::{Client, Config, Tool};

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
    assert_eq!(
        serialized["text"]["format"]["name"],
        "invoice_summary"
    );
}

