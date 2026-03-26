use crate::domain::message::{Message, MessageRole};
use crate::domain::state::AppState;
use crate::errors::CoreResult;

pub fn append_user_message(state: &mut AppState, content: String) -> CoreResult<usize> {
    state.session.messages.push(Message {
        role: MessageRole::User,
        content,
    });

    Ok(state.session.messages.len())
}
