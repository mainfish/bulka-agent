use agent_core::domain::command::AgentCommand;
use agent_core::usecases::load_state::load_state_from_store;
use agent_core::usecases::process_command::{
    CommandOutcome, ControlOutcome, PromptOutcome, ToolOutcome, run_turn,
};
use agent_infra::storage::NullSessionStore;

use crate::commands::parse_command;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let store = NullSessionStore::new();
    let mut state = load_state_from_store(&store)?;

    print_start_description();

    loop {
        let command = read_command()?;
        let outcome = run_turn(&mut state, command)?;

        match &outcome {
            CommandOutcome::Empty => {}
            CommandOutcome::Control(ControlOutcome::Exiting) => {
                println!("{}", outcome_text(&outcome));
                break;
            }
            CommandOutcome::Unknown(_)
            | CommandOutcome::Prompt(_)
            | CommandOutcome::Control(ControlOutcome::SessionCleared)
            | CommandOutcome::Control(ControlOutcome::CommandsList)
            | CommandOutcome::Tool(ToolOutcome::NotImplemented) => {
                let text = outcome_text(&outcome);
                if !text.is_empty() {
                    println!("{}", text);
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

fn print_start_description() {
    println!("Hello ✋🏼 This is the agent-cli 🤖");
    println!("type /quit to quit");
    println!("type /commands to see list available commands");
}

fn outcome_text(outcome: &CommandOutcome) -> String {
    match outcome {
        CommandOutcome::Empty => String::new(),
        CommandOutcome::Unknown(command) => {
            if command.is_empty() {
                String::new()
            } else {
                format!("unknown command: {command}")
            }
        }
        CommandOutcome::Prompt(prompt_outcome) => match prompt_outcome {
            PromptOutcome::UserMessageAdded {
                content,
                total_messages,
            } => format!("user: {content}\nmessages: {total_messages}"),
        },
        CommandOutcome::Control(control_outcome) => match control_outcome {
            ControlOutcome::SessionCleared => "session cleared".to_string(),
            ControlOutcome::CommandsList => "commands:\n- /clear\n- /commands\n- /quit".to_string(),
            ControlOutcome::Exiting => "Bye ✋🏼".to_string(),
        },
        CommandOutcome::Tool(tool_outcome) => match tool_outcome {
            ToolOutcome::NotImplemented => "tools are not implemented yet".to_string(),
        },
    }
}
