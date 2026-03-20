use crate::domain::command::AgentCommand;
use crate::domain::message::{Message, MessageRole};
use crate::domain::session::Session;
use crate::errors::CoreResult;

pub fn run_turn(session: &mut Session, command: AgentCommand) -> CoreResult<()> {
    match command {
        AgentCommand::Exit => {}
        AgentCommand::ClearSession => {
            session.messages.clear();
        }
        AgentCommand::UserPrompt(prompt) => {
            session.messages.push(Message {
                role: MessageRole::User,
                content: prompt,
            });
        }
    }

    Ok(())
}
