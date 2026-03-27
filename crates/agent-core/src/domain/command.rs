#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    Empty,
    Unknown(String),
    Prompt(String),
    Control(ControlCommand),
    Tool(ToolCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlCommand {
    ClearSession,
    ShowCommands,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCommand {
    NotImplemented,
}
