use agent_core::domain::command::AgentCommand;
use agent_core::ports::llm::LlmPort;
use agent_core::ports::storage::SessionStore;
use agent_core::ports::tools::ToolPort;
use agent_core::usecases::load_state::load_state_from_store;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};
use agent_infra::llama::NullLlmAdapter;
use agent_infra::storage::NullSessionStore;
use agent_infra::tools::NullToolAdapter;

pub fn run() {
    let llm = NullLlmAdapter::new();
    let tools = NullToolAdapter::new();
    let store = NullSessionStore::new();

    let _llm_port: &dyn LlmPort = &llm;
    let _tool_port: &dyn ToolPort = &tools;
    let _session_store: &dyn SessionStore = &store;

    let mut state = match load_state_from_store(&store) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("failed to load state: {err}");
            return;
        }
    };

    println!("agent-server");
    println!("server shell initialized");
    println!("llm adapter wired");
    println!("tool adapter wired");
    println!("session store wired");

    match run_turn(
        &mut state,
        AgentCommand::UserPrompt("server boot".to_string()),
    ) {
        Ok(TurnOutcome::Exiting) => {
            println!("server requested exit");
        }
        Ok(TurnOutcome::SessionCleared) => {
            println!("session cleared");
            println!("messages: {}", state.session.messages.len());
        }
        Ok(TurnOutcome::UserMessageAdded {
            content,
            total_messages,
        }) => {
            println!("boot message: {content}");
            println!("messages: {total_messages}");
        }
        Ok(TurnOutcome::UnknownCommand(command)) => {
            println!("unknown command: {command}");
        }
        Err(err) => {
            eprintln!("server shell error: {err}");
        }
    }
}
