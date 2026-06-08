use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .respond_text(
            "gpt-4o-mini",
            "Explain provider-agnostic APIs in one sentence.",
        )
        .await?;

    println!("{}", response.text()?);
    Ok(())
}
