use agent_core::domain::tool::{ToolRequest, ToolResult, ToolSpec};
use agent_core::errors::{CoreError, CoreResult};
use agent_core::ports::tools::ToolPort;

#[derive(Debug, Default, Clone)]
pub struct FsToolAdapter;

impl FsToolAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ToolPort for FsToolAdapter {
    fn available_tools(&self) -> CoreResult<Vec<ToolSpec>> {
        Ok(vec![
            ToolSpec {
                name: "list_dir".to_string(),
                description: "List files and directories at a given path".to_string(),
            },
            ToolSpec {
                name: "read_file".to_string(),
                description: "Read a UTF-8 text file at a given path".to_string(),
            },
        ])
    }

    fn execute(&self, _request: ToolRequest) -> CoreResult<ToolResult> {
        Err(CoreError::NotImplemented)
    }
}
