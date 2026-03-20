use crate::domain::message::Message;
use crate::errors::CoreResult;

pub trait LlmPort {
    fn complete(&self, history: &[Message]) -> CoreResult<Message>;
}
