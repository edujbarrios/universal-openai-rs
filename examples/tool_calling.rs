use serde::{Deserialize, Serialize};
use serde_json::json;
use universal_openai_rs::{Client, Tool, ToolCall, ToolCallFunction, ToolRegistry};

#[derive(Debug, Deserialize)]
struct WeatherArgs {
    city: String,
}

#[derive(Debug, Serialize)]
struct WeatherOutput {
    forecast: String,
}

#[tokio::main]
async fn main() -> universal_openai_rs::Result<()> {
    let client = Client::from_env()?;
    let response = client
        .chat()
        .model("gpt-4o-mini")
        .user("What should I pack for Madrid today?")
        .tool(Tool::function(
            "get_weather",
            "Get weather for a city.",
            json!({
                "type": "object",
                "properties": {
                    "city": {"type": "string"}
                },
                "required": ["city"]
            }),
        ))
        .send()
        .await?;

    println!("{response:#?}");

    let registry = ToolRegistry::new().with_fn(
        "get_weather",
        "Get weather for a city.",
        json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            },
            "required": ["city"]
        }),
        |args: WeatherArgs| async move {
            Ok(WeatherOutput {
                forecast: format!("sunny in {}", args.city),
            })
        },
    );
    let tool_output = registry
        .call(&ToolCall {
            id: "call_example".to_string(),
            kind: "function".to_string(),
            function: ToolCallFunction {
                name: "get_weather".to_string(),
                arguments: r#"{"city":"Madrid"}"#.to_string(),
            },
        })
        .await?;

    println!("{}", tool_output.output);
    Ok(())
}
