use universal_openai_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    let agents = client
        .agents()
        .default_model("gpt-4o-mini")
        .simple("agent1", "Draft a concise technical answer.")
        .simple("agent2", "Review and improve the draft.");

    let draft = agents
        .agent1("Design a simple OpenAI-compatible Rust API call.")
        .await?;

    let review_task = format!(
        "Use this draft as context, then improve it for a Rust developer:\n\n{}",
        draft.output
    );

    let reviewed = agents.agent2(review_task).await?;

    println!("{}", reviewed.output);
    Ok(())
}
