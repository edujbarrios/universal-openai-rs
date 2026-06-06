use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .chat_text("gpt-4o-mini", "Write one sentence about Rust.")
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}
