use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let transcript = client
        .audio()
        .transcription()
        .model("whisper-1")
        .file("sample.mp3", Vec::<u8>::new())
        .send()
        .await?;

    println!("{}", transcript.text.unwrap_or_default());
    Ok(())
}

