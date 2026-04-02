use agent_core::errors::{CoreError, CoreResult};

use super::postgres::PostgresDatabase;

pub fn run(_db: &PostgresDatabase) -> CoreResult<()> {
    Err(CoreError::NotImplemented)
}
