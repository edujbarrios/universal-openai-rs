use serde_json::json;
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .chat()
        .model("gpt-4o-mini")
        .user("Return a compact profile for an AI engineer.")
        .json_schema(
            "engineer_profile",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string"},
                    "strengths": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["title", "strengths"]
            }),
        )
        .send()
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}

