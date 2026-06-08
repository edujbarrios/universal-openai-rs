use serde::Deserialize;
use universal_openai_rs::Client;

#[derive(Debug, Deserialize)]
struct Profile {
    title: String,
    strengths: Vec<String>,
}

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let profile: Profile = client
        .ask_json(
            "gpt-4o-mini",
            "Return a compact profile for an AI engineer.",
        )
        .await?;

    println!("{}: {} strengths", profile.title, profile.strengths.len());
    Ok(())
}
