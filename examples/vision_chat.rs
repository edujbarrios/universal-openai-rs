use universal_openai_rs::{ChatContentPart, Client};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .chat()
        .model("gpt-4o-mini")
        .user_parts(vec![
            ChatContentPart::text("Describe this image in one sentence."),
            // You can also pass a base64 data URL, for example:
            // data:image/png;base64,...
            ChatContentPart::image_url("https://example.com/image.png"),
        ])
        .send()
        .await?;

    println!("{}", response.first_text().unwrap_or_default());
    Ok(())
}
