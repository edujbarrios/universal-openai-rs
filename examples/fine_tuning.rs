use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let job = client
        .fine_tuning()
        .create()
        .model("gpt-4o-mini")
        .training_file("file-training")
        .send()
        .await?;

    println!("{}", job.id);
    Ok(())
}

