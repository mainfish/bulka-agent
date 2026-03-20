use crate::domain::command::AgentCommand;
use crate::domain::message::{Message, MessageRole};
use crate::domain::state::AppState;
use crate::errors::CoreResult;

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
            state.session.messages.clear();
            TurnOutcome::SessionCleared
        }
        AgentCommand::UserPrompt(prompt) => {
            state.session.messages.push(Message {
                role: MessageRole::User,
                content: prompt.clone(),
            });

            TurnOutcome::UserMessageAdded {
                content: prompt,
                total_messages: state.session.messages.len(),
            }
        }
        AgentCommand::Unknown(command) => TurnOutcome::UnknownCommand(command),
    };

    Ok(outcome)
}
