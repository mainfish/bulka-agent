use crate::domain::command::AgentCommand;
use crate::domain::session::Session;
use crate::errors::{CoreError, CoreResult};

pub fn run_turn(_session: &mut Session, _command: AgentCommand) -> CoreResult<()> {
    Err(CoreError::NotImplemented)
}
