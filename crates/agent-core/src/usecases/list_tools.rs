use crate::domain::tool::ToolSpec;
use crate::errors::CoreResult;
use crate::ports::tools::ToolPort;

pub fn list_tools(tool_port: &dyn ToolPort) -> CoreResult<Vec<ToolSpec>> {
    tool_port.available_tools()
}
