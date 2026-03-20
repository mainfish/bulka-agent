use agent_core::domain::command::AgentCommand;
use agent_core::domain::state::AppState;
use agent_core::ports::llm::LlmPort;
use agent_core::usecases::init_state::init_state;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};
use agent_infra::llama::NullLlmAdapter;

pub fn run() {
    let mut state: AppState = init_state();
    let llm = NullLlmAdapter::new();
    let _llm_port: &dyn LlmPort = &llm;

    println!("agent-server");
    println!("server shell initialized");
    println!("llm adapter wired");

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
