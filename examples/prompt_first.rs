use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let text = client
        .prompt("Explain why Rust is useful for AI API clients.")
        .model("gpt-4o-mini")
        .system("Answer in one practical sentence.")
        .run_text()
        .await?;

    println!("{text}");
    Ok(())
}

