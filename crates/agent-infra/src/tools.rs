use agent_core::domain::tool::{ToolRequest, ToolResult, ToolSpec};
use agent_core::errors::{CoreError, CoreResult};
use agent_core::ports::tools::ToolPort;

#[derive(Debug, Default, Clone)]
pub struct NullToolAdapter;

impl NullToolAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ToolPort for NullToolAdapter {
    fn available_tools(&self) -> CoreResult<Vec<ToolSpec>> {
        Ok(Vec::new())
    }

    fn execute(&self, _request: ToolRequest) -> CoreResult<ToolResult> {
        Err(CoreError::NotImplemented)
    }
}
