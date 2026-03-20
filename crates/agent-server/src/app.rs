use agent_core::domain::command::AgentCommand;
use agent_core::domain::state::AppState;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};

pub fn run() {
    let mut state = AppState::new();

    println!("agent-server");
    println!("server shell initialized");

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
        Err(err) => {
            eprintln!("server shell error: {err}");
        }
    }
}
