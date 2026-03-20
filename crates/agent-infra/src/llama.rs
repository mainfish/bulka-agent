use agent_core::domain::message::Message;
use agent_core::errors::{CoreError, CoreResult};
use agent_core::ports::llm::LlmPort;

#[derive(Debug, Default, Clone)]
pub struct NullLlmAdapter;

impl NullLlmAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl LlmPort for NullLlmAdapter {
    fn complete(&self, _history: &[Message]) -> CoreResult<Message> {
        Err(CoreError::NotImplemented)
    }
}
