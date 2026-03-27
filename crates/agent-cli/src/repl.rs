use agent_core::domain::command::AgentCommand;
use agent_core::usecases::load_state::load_state_from_store;
use agent_core::usecases::process_command::{
    CommandOutcome, ControlOutcome, PromptOutcome, ToolOutcome, process_command,
};
use agent_infra::storage::NullSessionStore;

use crate::commands::parse_command;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let store = NullSessionStore::new();
    let mut state = load_state_from_store(&store)?;

    print_start_description();

    loop {
        let command = read_command()?;
        let outcome = process_command(&mut state, command)?;

        match &outcome {
            CommandOutcome::Empty => {}
            CommandOutcome::Unknown(command) => {
                if !command.is_empty() {
                    println!("unknown command: {command}");
                }
            }
            CommandOutcome::Prompt(PromptOutcome::UserMessageAdded {
                content,
                total_messages,
            }) => {
                println!("user: {content}");
                println!("messages: {total_messages}");
            }
            CommandOutcome::Control(control_outcome) => match control_outcome {
                ControlOutcome::Exiting => {
                    println!("Bye ✋🏼");
                    break;
                }
                ControlOutcome::SessionCleared => {
                    println!("session cleared");
                }
                ControlOutcome::CommandsList(commands_list) => {
                    println!("commands:");
                    for description in commands_list {
                        println!("- {}: {}", description.command, description.description);
                    }
                }
            },
            CommandOutcome::Tool(tool_outcome) => match tool_outcome {
                ToolOutcome::NotImplemented => {
                    println!("tools are not implemented yet");
                }
            },
        }
    }

    Ok(())
}

fn read_command() -> Result<AgentCommand, Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(parse_command(&input))
}

fn print_start_description() {
    println!("Hello ✋🏼 This is the agent-cli 🤖");
    println!("type /quit to quit");
    println!("type /commands to see list available commands");
}
