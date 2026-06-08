use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let text = client.ask_default("Write one sentence about Rust.").await?;

    println!("{text}");
    Ok(())
}

