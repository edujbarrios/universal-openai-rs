use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let text = client
        .respond("Explain provider-agnostic APIs in one sentence.")
        .model("gpt-4o-mini")
        .run_text()
        .await?;

    println!("{text}");
    Ok(())
}
