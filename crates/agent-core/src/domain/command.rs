#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    ClearSession,
    ComandsList,
    Exit,
    UserPrompt(String),
    Unknown(String),
}
