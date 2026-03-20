use crate::domain::message::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub messages: Vec<Message>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}
