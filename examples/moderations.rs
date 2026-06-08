use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .moderations()
        .input("Classify this text.")
        .send()
        .await?;

    println!("results: {}", response.results.len());
    Ok(())
}
