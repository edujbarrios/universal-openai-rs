use universal_openai_rs::Client;

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let models = client.models().list().await?;

    for model in models.data {
        println!("{}", model.id);
    }

    Ok(())
}

