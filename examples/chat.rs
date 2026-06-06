use universal_openai::Client;

#[tokio::main]
async fn main() -> universal_openai::Result<()> {
    let client = Client::from_env()?;

    let response = client
        .chat()
        .model("gpt-4o-mini")
        .system("You answer with concise, practical Rust advice.")
        .user("Explain ownership in one sentence.")
        .temperature(0.2)
        .send()
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}

