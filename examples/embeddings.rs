use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let vector = client
        .embed("text-embedding-3-small", "Rust makes API clients reliable.")
        .await?;

    println!("dimensions: {}", vector.len());
    Ok(())
}
