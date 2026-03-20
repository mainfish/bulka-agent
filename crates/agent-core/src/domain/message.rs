#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}
