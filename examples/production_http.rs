use std::time::Duration;
use universal_openai_rs::{Client, Config, RetryConfig};

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let http = reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_keepalive(Duration::from_secs(30))
        .build()?;

    let config = Config::new("your-api-key")
        .with_base_url("https://api.example.com/v1")
        .with_user_agent("my-agent-service/0.1")
        .with_organization("org_123")
        .with_project("proj_123")
        .with_header("x-provider-routing", "fast")
        .with_retry_config(RetryConfig {
            max_retries: 5,
            initial_backoff: Duration::from_millis(250),
            max_backoff: Duration::from_secs(20),
            jitter: true,
            respect_retry_after: true,
        });

    let _client = Client::with_http_client(config, http)?;
    Ok(())
}
