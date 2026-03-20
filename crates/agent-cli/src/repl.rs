use agent_core::domain::command::AgentCommand;
use agent_core::domain::state::AppState;
use agent_core::usecases::run_turn::{TurnOutcome, run_turn};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("agent-cli");
    println!("type /exit to quit");

    let mut state = AppState::new();

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
        }
    }

    Ok(())
}

fn read_command() -> Result<AgentCommand, Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let trimmed = input.trim();

    let command = match trimmed {
        "/exit" | ":q" => AgentCommand::Exit,
        "/clear" => AgentCommand::ClearSession,
        "" => return read_command(),
        _ => AgentCommand::UserPrompt(trimmed.to_string()),
    };

    Ok(command)
}
