use serde::Deserialize;
use serde_json::json;
use universal_openai_rs::{AgentSpec, Client, Config};

#[test]
fn builds_agent_specs() {
    let spec = AgentSpec::new("researcher")
        .model("gpt-4o-mini")
        .instructions("Find concise facts.")
        .temperature(0.2)
        .max_tokens(256);

    assert_eq!(spec.name, "researcher");
    assert_eq!(spec.model.as_deref(), Some("gpt-4o-mini"));
    assert_eq!(spec.instructions.as_deref(), Some("Find concise facts."));
}

#[derive(Debug, Deserialize)]
struct SearchArgs {
    query: String,
}

#[test]
fn agent_spec_registers_executable_tools() {
    let spec = AgentSpec::new("researcher").tool_fn(
        "search_docs",
        "Search docs.",
        json!({
            "type": "object",
            "properties": {"query": {"type": "string"}},
            "required": ["query"]
        }),
        |args: SearchArgs| async move { Ok(json!({ "answer": args.query })) },
    );

    assert_eq!(spec.tools[0].function.name, "search_docs");
    assert!(spec
        .tool_registry
        .as_ref()
        .unwrap()
        .get("search_docs")
        .is_some());
}

#[test]
fn agent_chain_run_records_steps_and_output() {
    let run = universal_openai_rs::AgentChainRun {
        initial_task: "start".to_string(),
        steps: vec![
            universal_openai_rs::AgentRun {
                agent: "agent1".to_string(),
                task: "start".to_string(),
                output: "draft".to_string(),
            },
            universal_openai_rs::AgentRun {
                agent: "agent2".to_string(),
                task: "draft".to_string(),
                output: "final".to_string(),
            },
        ],
        output: "final".to_string(),
    };

    assert_eq!(run.steps.len(), 2);
    assert_eq!(run.output, "final");
}


#[test]
fn agents_registry_stores_named_specs() {
    let client = Client::new(Config::new("test-key").with_default_model("gpt-4o-mini")).unwrap();
    let agents = client
        .agents()
        .simple("agent1", "Answer as a practical engineer.")
        .add(
            client
                .agent("agent2")
                .instructions("Review the answer critically."),
        );

    assert_eq!(
        agents.get("agent1").unwrap().instructions.as_deref(),
        Some("Answer as a practical engineer.")
    );
    assert_eq!(
        agents.get("agent2").unwrap().instructions.as_deref(),
        Some("Review the answer critically.")
    );
}
