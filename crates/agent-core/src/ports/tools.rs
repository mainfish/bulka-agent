use crate::domain::tool::{ToolRequest, ToolResult, ToolSpec};
use crate::errors::CoreResult;

pub trait ToolPort {
    fn available_tools(&self) -> CoreResult<Vec<ToolSpec>>;
    fn execute(&self, request: ToolRequest) -> CoreResult<ToolResult>;
}
