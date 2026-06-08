use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AiTool, ChatMessage, Client, DynAiTool, Error, FunctionAiTool, Result, Tool, ToolRegistry,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub name: String,
    pub model: Option<String>,
    pub instructions: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub tools: Vec<Tool>,
    #[serde(skip)]
    pub tool_registry: Option<ToolRegistry>,
}

impl PartialEq for AgentSpec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.model == other.model
            && self.instructions == other.instructions
            && self.temperature == other.temperature
            && self.max_tokens == other.max_tokens
            && self.tools == other.tools
    }
}

impl AgentSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            model: None,
            instructions: None,
            temperature: None,
            max_tokens: None,
            tools: Vec::new(),
            tool_registry: None,
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn ai_tool<T>(mut self, tool: T) -> Self
    where
        T: AiTool,
    {
        self.tools.push(tool.definition());
        self.tool_registry
            .get_or_insert_with(ToolRegistry::new)
            .insert(tool);
        self
    }

    pub fn tool_fn<Args, Output, F, Fut>(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Value,
        call: F,
    ) -> Self
    where
        Args: serde::de::DeserializeOwned + Send + Sync + 'static,
        Output: Serialize + Send + Sync + 'static,
        F: Fn(Args) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Output>> + Send + 'static,
    {
        let tool = FunctionAiTool::new(name, description, parameters, call);
        self.tools.push(tool.definition());
        self.tool_registry
            .get_or_insert_with(ToolRegistry::new)
            .insert_dyn(tool);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentRun {
    pub agent: String,
    pub task: String,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentChainRun {
    pub initial_task: String,
    pub steps: Vec<AgentRun>,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct Agents<'a> {
    client: &'a Client,
    default_model: Option<String>,
    specs: BTreeMap<String, AgentSpec>,
}

impl<'a> Agents<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            default_model: client.default_model().map(ToOwned::to_owned),
            specs: BTreeMap::new(),
        }
    }

    pub fn default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }

    pub fn add(mut self, spec: AgentSpec) -> Self {
        self.specs.insert(spec.name.clone(), spec);
        self
    }

    pub fn simple(mut self, name: impl Into<String>, instructions: impl Into<String>) -> Self {
        let spec = AgentSpec::new(name).instructions(instructions);
        self.specs.insert(spec.name.clone(), spec);
        self
    }

    pub fn get(&self, name: &str) -> Option<&AgentSpec> {
        self.specs.get(name)
    }

    pub async fn run(&self, name: &str, task: impl Into<String>) -> Result<AgentRun> {
        let task = task.into();
        let spec = self
            .specs
            .get(name)
            .cloned()
            .unwrap_or_else(|| AgentSpec::new(name));
        self.run_spec(spec, task).await
    }

    pub async fn sequence<I, S>(&self, agents: I, task: impl Into<String>) -> Result<AgentChainRun>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let initial_task = task.into();
        let mut next_task = initial_task.clone();
        let mut steps = Vec::new();

        for agent in agents {
            let agent_name = agent.into();
            let run = self.run(&agent_name, next_task).await?;
            next_task = run.output.clone();
            steps.push(run);
        }

        Ok(AgentChainRun {
            initial_task,
            output: next_task,
            steps,
        })
    }

    pub async fn run_spec(&self, spec: AgentSpec, task: impl Into<String>) -> Result<AgentRun> {
        let task = task.into();
        let model = spec
            .model
            .clone()
            .or_else(|| self.default_model.clone())
            .ok_or_else(|| Error::InvalidConfig("agent model is required".to_string()))?;

        let mut prompt = self.client.prompt(task.clone()).model(model.clone());

        if let Some(instructions) = spec.instructions.clone() {
            prompt = prompt.system(instructions);
        }
        if let Some(temperature) = spec.temperature {
            prompt = prompt.temperature(temperature);
        }
        if let Some(max_tokens) = spec.max_tokens {
            prompt = prompt.max_tokens(max_tokens);
        }
        for tool in spec.tools.clone() {
            prompt = prompt.tool(tool);
        }

        let output = if let Some(registry) = spec.tool_registry.clone() {
            let first = prompt.into_chat()?.send().await?;
            let tool_calls = first.tool_calls().to_vec();

            if tool_calls.is_empty() {
                first.text()?
            } else {
                let executions = registry.call_all(&tool_calls).await?;
                let mut followup = self.client.chat().model(model);

                if let Some(instructions) = spec.instructions.clone() {
                    followup = followup.system(instructions);
                }
                followup = followup
                    .message(ChatMessage::user(task.clone()))
                    .message(ChatMessage::assistant_tool_calls(tool_calls));
                if let Some(temperature) = spec.temperature {
                    followup = followup.temperature(temperature);
                }
                if let Some(max_tokens) = spec.max_tokens {
                    followup = followup.max_tokens(max_tokens);
                }
                for tool in spec.tools.clone() {
                    followup = followup.tool(tool);
                }
                for execution in executions {
                    followup = followup.tool_execution(execution);
                }

                followup.send().await?.text()?
            }
        } else {
            prompt.run_text().await?
        };

        Ok(AgentRun {
            agent: spec.name,
            task,
            output,
        })
    }

    pub async fn agent1(&self, task: impl Into<String>) -> Result<AgentRun> {
        self.run("agent1", task).await
    }

    pub async fn agent2(&self, task: impl Into<String>) -> Result<AgentRun> {
        self.run("agent2", task).await
    }

    pub async fn agent3(&self, task: impl Into<String>) -> Result<AgentRun> {
        self.run("agent3", task).await
    }
}
