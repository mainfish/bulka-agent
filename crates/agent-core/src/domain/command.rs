#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentCommand {
    Empty,
    Unknown(String),
    Prompt(String),
    Control(ControlCommand),
    Tool(ToolCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCommand {
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlCommand {
    ClearSession,
    CommandsList,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandDescription {
    pub command: &'static str,
    pub description: &'static str,
}

impl ControlCommand {
    pub const ALL: [Self; 3] = [Self::ClearSession, Self::CommandsList, Self::Exit];

    pub fn command(&self) -> &'static str {
        match self {
            Self::ClearSession => "/clear",
            Self::CommandsList => "/commands",
            Self::Exit => "/quit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ClearSession => "clear session",
            Self::CommandsList => "show available commands",
            Self::Exit => "quit agent-cli",
        }
    }

    pub fn descriptions() -> Vec<CommandDescription> {
        Self::ALL
            .into_iter()
            .map(|command| CommandDescription {
                command: command.command(),
                description: command.description(),
            })
            .collect()
    }
}

impl TryFrom<&str> for ControlCommand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "quit" => Ok(Self::Exit),
            "clear" => Ok(Self::ClearSession),
            "commands" => Ok(Self::CommandsList),
            _ => Err(()),
        }
    }
}
