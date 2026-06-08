use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let file = client
        .upload_file("fine-tune")
        .bytes("training.jsonl", br#"{"messages":[]}"#.to_vec())
        .send()
        .await?;

    println!("{}", file.id);
    Ok(())
}

