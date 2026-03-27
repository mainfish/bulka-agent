use crate::domain::command::{AgentCommand, ControlCommand, ToolCommand};
use crate::domain::state::AppState;
use crate::errors::CoreResult;

use super::append_user_message::append_user_message;
use super::clear_session::clear_session;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOutcome {
    Empty,
    Unknown(String),
    Prompt(PromptOutcome),
    Control(ControlOutcome),
    Tool(ToolOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptOutcome {
    UserMessageAdded {
        content: String,
        total_messages: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlOutcome {
    SessionCleared,
    CommandsList(Vec<CommandDescription>),
    Exiting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandDescription {
    pub command: String,
    pub description: String,
}

impl ControlCommand {
    pub fn descriptions() -> Vec<CommandDescription> {
        vec![
            CommandDescription {
                command: "/clear".to_string(),
                description: "clear session".to_string(),
            },
            CommandDescription {
                command: "/commands".to_string(),
                description: "show available commands".to_string(),
            },
            CommandDescription {
                command: "/quit".to_string(),
                description: "quit agent-cli".to_string(),
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolOutcome {
    NotImplemented,
}

pub fn process_command(state: &mut AppState, command: AgentCommand) -> CoreResult<CommandOutcome> {
    let outcome = match command {
        AgentCommand::Empty => CommandOutcome::Empty,
        AgentCommand::Unknown(command) => CommandOutcome::Unknown(command),
        AgentCommand::Prompt(prompt) => {
            let total_messages = append_user_message(state, prompt.clone())?;

            CommandOutcome::Prompt(PromptOutcome::UserMessageAdded {
                content: prompt,
                total_messages,
            })
        }
        AgentCommand::Control(control) => match control {
            ControlCommand::ClearSession => {
                clear_session(state)?;
                CommandOutcome::Control(ControlOutcome::SessionCleared)
            }
            ControlCommand::ShowCommands => CommandOutcome::Control(ControlOutcome::CommandsList(
                ControlCommand::descriptions(),
            )),
            ControlCommand::Exit => CommandOutcome::Control(ControlOutcome::Exiting),
        },
        AgentCommand::Tool(tool) => match tool {
            ToolCommand::NotImplemented => CommandOutcome::Tool(ToolOutcome::NotImplemented),
        },
    };

    Ok(outcome)
}
