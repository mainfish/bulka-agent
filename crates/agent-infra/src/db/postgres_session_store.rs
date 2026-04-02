use agent_core::domain::session::Session;
use agent_core::errors::{CoreError, CoreResult};
use agent_core::ports::storage::SessionStore;

use super::postgres::PostgresDatabase;

#[derive(Debug, Clone)]
pub struct PostgresSessionStore {
    db: PostgresDatabase,
}

impl PostgresSessionStore {
    pub fn new(db: PostgresDatabase) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &PostgresDatabase {
        &self.db
    }

    pub fn load_session(&self) -> CoreResult<Session> {
        Err(CoreError::NotImplemented)
    }

    pub fn save_session(&self, _session: &Session) -> CoreResult<()> {
        Err(CoreError::NotImplemented)
    }
}

impl SessionStore for PostgresSessionStore {
    fn load(&self) -> CoreResult<Session> {
        self.load_session()
    }

    fn save(&self, session: &Session) -> CoreResult<()> {
        self.save_session(session)
    }
}
