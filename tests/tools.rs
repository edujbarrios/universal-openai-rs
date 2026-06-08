use futures_util::future::{BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use universal_openai_rs::{AiTool, Error, ToolCall, ToolCallFunction, ToolRegistry};

#[derive(Debug, Deserialize)]
struct WeatherArgs {
    city: String,
}

#[derive(Debug, Serialize)]
struct WeatherOutput {
    forecast: String,
}

struct WeatherTool;

impl AiTool for WeatherTool {
    const NAME: &'static str = "weather_tool";
    const DESCRIPTION: &'static str = "Get weather for a city.";

    type Args = WeatherArgs;
    type Output = WeatherOutput;

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            },
            "required": ["city"]
        })
    }

    fn call(&self, args: Self::Args) -> BoxFuture<'_, universal_openai_rs::Result<Self::Output>> {
        async move {
            Ok(WeatherOutput {
                forecast: format!("cloudy in {}", args.city),
            })
        }
        .boxed()
    }
}

#[tokio::test]
async fn registry_executes_typed_function_tools() {
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
    let call = ToolCall {
        id: "call_1".to_string(),
        kind: "function".to_string(),
        function: ToolCallFunction {
            name: "get_weather".to_string(),
            arguments: r#"{"city":"Madrid"}"#.to_string(),
        },
    };

    let output = registry.call(&call).await.unwrap();

    assert_eq!(output.tool_call_id, "call_1");
    assert_eq!(output.name, "get_weather");
    assert_eq!(output.output["forecast"], "sunny in Madrid");
    assert_eq!(output.message().tool_call_id.as_deref(), Some("call_1"));
}

#[tokio::test]
async fn registry_executes_ai_tool_trait_implementations() {
    let registry = ToolRegistry::new().with(WeatherTool);
    let call = ToolCall {
        id: "call_1".to_string(),
        kind: "function".to_string(),
        function: ToolCallFunction {
            name: "weather_tool".to_string(),
            arguments: r#"{"city":"Madrid"}"#.to_string(),
        },
    };

    let output = registry.call(&call).await.unwrap();

    assert_eq!(output.output["forecast"], "cloudy in Madrid");
    assert_eq!(registry.definitions()[0].function.name, "weather_tool");
}

#[tokio::test]
async fn registry_reports_unknown_tools() {
    let registry = ToolRegistry::new();
    let call = ToolCall {
        id: "call_1".to_string(),
        kind: "function".to_string(),
        function: ToolCallFunction {
            name: "missing".to_string(),
            arguments: "{}".to_string(),
        },
    };
    let error = registry.call(&call).await.unwrap_err();

    assert!(matches!(error, Error::UnknownTool(name) if name == "missing"));
}
