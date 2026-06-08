use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .complete_text("gpt-3.5-turbo-instruct", "Complete this sentence:")
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}

