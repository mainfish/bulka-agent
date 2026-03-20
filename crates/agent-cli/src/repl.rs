use agent_core::domain::command::AgentCommand;
use agent_core::usecases::init_state::init_state;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};

use crate::commands::parse_command;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");

    let mut state = init_state();

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
