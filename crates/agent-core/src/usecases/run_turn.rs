use crate::domain::command::AgentCommand;
use crate::domain::message::{Message, MessageRole};
use crate::domain::session::Session;
use crate::errors::CoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    Exiting,
    SessionCleared,
    UserMessageAdded {
        content: String,
        total_messages: usize,
    },
}

pub fn run_turn(session: &mut Session, command: AgentCommand) -> CoreResult<TurnOutcome> {
    let outcome = match command {
        AgentCommand::Exit => TurnOutcome::Exiting,
        AgentCommand::ClearSession => {
            session.messages.clear();
            TurnOutcome::SessionCleared
        }
        AgentCommand::UserPrompt(prompt) => {
            session.messages.push(Message {
                role: MessageRole::User,
                content: prompt.clone(),
            });

            TurnOutcome::UserMessageAdded {
                content: prompt,
                total_messages: session.messages.len(),
            }
        }
    };

    Ok(outcome)
}
