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
    CommandsList,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCommand {
    NotImplemented,
}
