use universal_openai_rs::{Client, ResponseContentPart};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .responses()
        .model("gpt-4o-mini")
        .user_parts(vec![
            ResponseContentPart::text("Describe this image in one sentence."),
            ResponseContentPart::image_url("https://example.com/image.png"),
        ])
        .send()
        .await?;

    println!("{}", response.text()?);
    Ok(())
}
