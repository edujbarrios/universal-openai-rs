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

#[test]
fn agents_registry_stores_named_specs() {
    let client = Client::new(Config::new("test-key").with_default_model("gpt-4o-mini")).unwrap();
    let agents = client
        .agents()
        .simple("agent1", "Answer as a practical engineer.")
        .add(client.agent("agent2").instructions("Review the answer critically."));

    assert_eq!(
        agents.get("agent1").unwrap().instructions.as_deref(),
        Some("Answer as a practical engineer.")
    );
    assert_eq!(
        agents.get("agent2").unwrap().instructions.as_deref(),
        Some("Review the answer critically.")
    );
}

