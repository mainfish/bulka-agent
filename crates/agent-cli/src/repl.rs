use agent_core::domain::command::AgentCommand;
use agent_core::usecases::list_tools::list_tools;
use agent_core::usecases::load_state::load_state_from_store;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};
use agent_infra::storage::NullSessionStore;
use agent_infra::tools::NullToolAdapter;

use crate::commands::parse_command;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");
    println!("type /tools to list available tools");

    let store = NullSessionStore::new();
    let tools = NullToolAdapter::new();
    let mut state = load_state_from_store(&store)?;

    loop {
        let command = read_command()?;
        let outcome = run_turn(&mut state, command)?;

        match outcome {
            TurnOutcome::Exiting => {
                println!("Bye ✋🏼");
                break;
            }
            TurnOutcome::SessionCleared => {
                println!("session cleared");
                println!("messages: {}", state.session.messages.len());
            }
            TurnOutcome::ToolsRequested => {
                let specs = list_tools(&tools)?;

                if specs.is_empty() {
                    println!("tools: no tools available");
                } else {
                    println!("tools:");
                    for spec in specs {
                        println!("- {}: {}", spec.name, spec.description);
                    }
                }
            }
            TurnOutcome::UserMessageAdded {
                content,
                total_messages,
            } => {
                println!("user: {content}");
                println!("messages: {total_messages}");
            }
            TurnOutcome::UnknownCommand(command) => {
                if !command.is_empty() {
                    println!("unknown command: {command}");
                }
            }
        }
    }

    Ok(())
}

fn read_command() -> Result<AgentCommand, Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(parse_command(&input))
}
