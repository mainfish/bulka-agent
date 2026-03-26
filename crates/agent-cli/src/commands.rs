use agent_core::domain::command::AgentCommand;

pub fn parse_command(input: &str) -> AgentCommand {
    let trimmed = input.trim();

    match trimmed {
        "/exit" => AgentCommand::Exit,
        "/clear" => AgentCommand::ClearSession,
        "/commands" => AgentCommand::ComandsList,
        "" => AgentCommand::Unknown(String::new()),
        _ if trimmed.starts_with('/') => AgentCommand::Unknown(trimmed.to_string()),
        _ => AgentCommand::UserPrompt(trimmed.to_string()),
    }
}
