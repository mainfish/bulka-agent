use agent_core::errors::CoreResult;
use agent_core::ports::storage::SessionStore;

use crate::config::SessionStoreConfig;
use crate::db::{PostgresSessionStore, create_database};
use crate::storage::NullSessionStore;

pub struct SessionStoreFactory;

impl SessionStoreFactory {
    pub fn build(config: SessionStoreConfig) -> CoreResult<Box<dyn SessionStore>> {
        match config {
            SessionStoreConfig::Null => Ok(Box::new(NullSessionStore::new())),
            SessionStoreConfig::Postgres(postgres_config) => {
                let db = create_database(postgres_config)?;
                Ok(Box::new(PostgresSessionStore::new(db)))
            }
        }
    }

    pub fn from_env() -> CoreResult<Box<dyn SessionStore>> {
        let config = SessionStoreConfig::from_env()?;
        Self::build(config)
    }
}
