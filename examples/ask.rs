use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let text = client
        .ask("gpt-4o-mini", "Write one sentence about Rust.")
        .await?;

    println!("{text}");
    Ok(())
}

