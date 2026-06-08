use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    let agents = client
        .agents()
        .default_model("gpt-4o-mini")
        .simple("agent1", "Answer as a concise Rust AI engineer.")
        .simple("agent2", "Review the answer and suggest one improvement.");

    let first = agents
        .agent1("Design a simple OpenAI-compatible Rust API call.")
        .await?;
    let second = agents.agent2(first.output).await?;

    println!("{}", second.output);
    Ok(())
}

