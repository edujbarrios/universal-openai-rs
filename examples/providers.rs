use universal_openai_rs::{Client, Provider};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::for_provider("ollama", Provider::Ollama)?;
    let text = client
        .ask("llama3.2", "Say hello from a local model.")
        .await?;

    println!("{text}");
    Ok(())
}
