use crate::domain::command::AgentCommand;
use crate::domain::state::AppState;
use crate::errors::CoreResult;

use super::append_user_message::append_user_message;
use super::clear_session::clear_session;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    Exiting,
    SessionCleared,
    UserMessageAdded {
        content: String,
        total_messages: usize,
    },
    UnknownCommand(String),
}

pub fn run_turn(state: &mut AppState, command: AgentCommand) -> CoreResult<TurnOutcome> {
    let outcome = match command {
        AgentCommand::Exit => TurnOutcome::Exiting,
        AgentCommand::ClearSession => {
            clear_session(state)?;
            TurnOutcome::SessionCleared
        }
        AgentCommand::UserPrompt(prompt) => {
            let total_messages = append_user_message(state, prompt.clone())?;

            TurnOutcome::UserMessageAdded {
                content: prompt,
                total_messages,
            }
        }
        AgentCommand::Unknown(command) => TurnOutcome::UnknownCommand(command),
    };

    Ok(outcome)
}
