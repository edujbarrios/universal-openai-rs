use std::{collections::BTreeMap, future::Future, marker::PhantomData, sync::Arc};

use futures_util::future::{BoxFuture, FutureExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{Result, Tool, ToolCall};

pub trait AiTool: Send + Sync + 'static {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    type Args: DeserializeOwned + Send + 'static;
    type Output: Serialize + Send + 'static;

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "additionalProperties": true
        })
    }

    fn call(&self, args: Self::Args) -> BoxFuture<'_, Result<Self::Output>>;

    fn definition(&self) -> Tool {
        Tool::function(Self::NAME, Self::DESCRIPTION, self.parameters())
    }
}

pub trait DynAiTool: Send + Sync {
    fn name(&self) -> &str;
    fn definition(&self) -> Tool;
    fn call_json(&self, arguments: &str) -> BoxFuture<'_, Result<Value>>;
}

impl<T> DynAiTool for T
where
    T: AiTool,
{
    fn name(&self) -> &str {
        T::NAME
    }

    fn definition(&self) -> Tool {
        AiTool::definition(self)
    }

    fn call_json(&self, arguments: &str) -> BoxFuture<'_, Result<Value>> {
        let parsed = serde_json::from_str::<T::Args>(arguments);

        async move {
            let args = parsed?;
            let output = self.call(args).await?;
            Ok(serde_json::to_value(output)?)
        }
        .boxed()
    }
}

pub struct ToolRegistry {
    tools: BTreeMap<String, Arc<dyn DynAiTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
        }
    }

    pub fn with<T>(mut self, tool: T) -> Self
    where
        T: AiTool,
    {
        self.insert(tool);
        self
    }

    pub fn with_fn<Args, Output, F, Fut>(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Value,
        call: F,
    ) -> Self
    where
        Args: DeserializeOwned + Send + Sync + 'static,
        Output: Serialize + Send + Sync + 'static,
        F: Fn(Args) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Output>> + Send + 'static,
    {
        self.insert_dyn(FunctionAiTool::new(name, description, parameters, call));
        self
    }

    pub fn insert<T>(&mut self, tool: T)
    where
        T: AiTool,
    {
        self.insert_dyn(tool);
    }

    pub fn insert_dyn<T>(&mut self, tool: T)
    where
        T: DynAiTool + 'static,
    {
        self.tools.insert(tool.name().to_string(), Arc::new(tool));
    }

    pub fn definitions(&self) -> Vec<Tool> {
        self.tools.values().map(|tool| tool.definition()).collect()
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn DynAiTool>> {
        self.tools.get(name).cloned()
    }

    pub async fn call(&self, call: &ToolCall) -> Result<ToolExecution> {
        let tool = self
            .get(&call.function.name)
            .ok_or_else(|| crate::Error::UnknownTool(call.function.name.clone()))?;
        let output = tool.call_json(&call.function.arguments).await?;

        Ok(ToolExecution {
            tool_call_id: call.id.clone(),
            name: call.function.name.clone(),
            output,
        })
    }

    pub async fn call_all<'a, I>(&self, calls: I) -> Result<Vec<ToolExecution>>
    where
        I: IntoIterator<Item = &'a ToolCall>,
    {
        let mut outputs = Vec::new();

        for call in calls {
            outputs.push(self.call(call).await?);
        }

        Ok(outputs)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FunctionAiTool<Args, Output, F> {
    name: String,
    description: String,
    parameters: Value,
    call: F,
    _args: PhantomData<fn(Args) -> Output>,
}

impl<Args, Output, F> FunctionAiTool<Args, Output, F> {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Value,
        call: F,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
            call,
            _args: PhantomData,
        }
    }
}

impl<Args, Output, F, Fut> DynAiTool for FunctionAiTool<Args, Output, F>
where
    Args: DeserializeOwned + Send + Sync + 'static,
    Output: Serialize + Send + Sync + 'static,
    F: Fn(Args) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Output>> + Send + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn definition(&self) -> Tool {
        Tool::function(
            self.name.clone(),
            self.description.clone(),
            self.parameters.clone(),
        )
    }

    fn call_json(&self, arguments: &str) -> BoxFuture<'_, Result<Value>> {
        let parsed = serde_json::from_str::<Args>(arguments);

        async move {
            let args = parsed?;
            let output = (self.call)(args).await?;
            Ok(serde_json::to_value(output)?)
        }
        .boxed()
    }
}

impl Clone for ToolRegistry {
    fn clone(&self) -> Self {
        Self {
            tools: self.tools.clone(),
        }
    }
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ToolRegistry")
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolExecution {
    pub tool_call_id: String,
    pub name: String,
    pub output: Value,
}

impl ToolExecution {
    pub fn message(&self) -> crate::ChatMessage {
        crate::ChatMessage::tool(self.tool_call_id.clone(), self.output.to_string())
    }
}
