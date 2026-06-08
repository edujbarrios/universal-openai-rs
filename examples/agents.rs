use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    let agents = client
        .agents()
        .default_model("gpt-4o-mini")
        .simple("agent1", "Answer as a concise Rust AI engineer.")
        .simple("agent2", "Review the answer and suggest one improvement.");

    let run = agents
        .sequence(
            ["agent1", "agent2"],
            "Design a simple OpenAI-compatible Rust API call.",
        )
        .await?;

    println!("{}", run.output);
    Ok(())
}
