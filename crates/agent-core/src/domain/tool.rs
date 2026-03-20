#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest {
    pub name: String,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub name: String,
    pub output: String,
}
