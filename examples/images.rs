use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let image = client
        .images()
        .model("gpt-image-1")
        .prompt("A clean Rust API diagram")
        .size("1024x1024")
        .b64_json()
        .generate()
        .await?;

    println!("images: {}", image.data.len());
    Ok(())
}
