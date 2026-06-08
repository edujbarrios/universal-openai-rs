use serde_json::json;
use universal_openai_rs::{Client, Tool};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .chat()
        .model("gpt-4o-mini")
        .user("What should I pack for Madrid today?")
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
        .send()
        .await?;

    println!("{response:#?}");
    Ok(())
}
