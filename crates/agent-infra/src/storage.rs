use agent_core::domain::session::Session;
use agent_core::errors::{CoreError, CoreResult};
use agent_core::ports::storage::SessionStore;

#[derive(Debug, Default, Clone)]
pub struct NullSessionStore;

impl NullSessionStore {
    pub fn new() -> Self {
        Self
    }
}

impl SessionStore for NullSessionStore {
    fn load(&self) -> CoreResult<Session> {
        Ok(Session::new())
    }

    fn save(&self, _session: &Session) -> CoreResult<()> {
        Err(CoreError::NotImplemented)
    }
}
