use serde_json::{json, Value};
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response: Value = client
        .send_compatible(
            "chat/completions",
            &json!({
                "model": "gpt-4o-mini",
                "messages": [{"role": "user", "content": "Hello"}]
            }),
        )
        .await?;

    println!("{response:#?}");
    Ok(())
}
