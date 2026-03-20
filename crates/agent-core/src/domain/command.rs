#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    UserPrompt(String),
    ClearSession,
    Exit,
}
