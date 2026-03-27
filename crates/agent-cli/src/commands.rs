use agent_core::domain::command::{AgentCommand, ControlCommand};

pub fn parse_command(input: &str) -> AgentCommand {
    let trimmed = input.trim();

    match trimmed {
        "" => AgentCommand::Empty,
        _ if trimmed.starts_with('/') => parse_slash_command(trimmed),
        _ => AgentCommand::Prompt(trimmed.to_string()),
    }
}

fn parse_slash_command(trimmed: &str) -> AgentCommand {
    match trimmed.get(1..) {
        Some(command) => match command {
            "quit" => AgentCommand::Control(ControlCommand::Exit),
            "clear" => AgentCommand::Control(ControlCommand::ClearSession),
            "commands" => AgentCommand::Control(ControlCommand::CommandsList),
            _ => AgentCommand::Unknown(trimmed.to_string()),
        },
        None => AgentCommand::Empty,
    }
}
