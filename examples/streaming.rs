use futures_util::StreamExt;
use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let mut stream = client
        .chat()
        .model("gpt-4o-mini")
        .user("Write a short Rust haiku.")
        .stream_text_chunks()
        .await?;

    while let Some(chunk) = stream.next().await {
        print!("{}", chunk?);
    }

    Ok(())
}
