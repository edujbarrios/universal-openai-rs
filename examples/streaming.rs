use futures_util::StreamExt;
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let mut stream = client
        .chat()
        .model("gpt-4o-mini")
        .user("Write a short Rust haiku.")
        .stream()
        .await?;

    while let Some(event) = stream.next().await {
        let event = event?;
        for choice in event.choices {
            if let Some(text) = choice.delta.content {
                print!("{text}");
            }
        }
    }

    Ok(())
}

