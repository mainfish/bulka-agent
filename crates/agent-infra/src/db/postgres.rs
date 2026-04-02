use agent_core::errors::{CoreError, CoreResult};

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
}

impl PostgresConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresDatabase {
    config: PostgresConfig,
}

impl PostgresDatabase {
    pub fn new(config: PostgresConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &PostgresConfig {
        &self.config
    }

    pub fn database_url(&self) -> &str {
        &self.config.database_url
    }
}

pub fn create_database(config: PostgresConfig) -> CoreResult<PostgresDatabase> {
    let database_url = config.database_url.trim();

    if database_url.is_empty() {
        return Err(CoreError::invalid_config(
            "postgres database_url must not be empty",
        ));
    }

    Ok(PostgresDatabase::new(config))
}
