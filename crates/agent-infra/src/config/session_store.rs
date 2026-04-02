use std::env;

use agent_core::errors::{CoreError, CoreResult};

use crate::db::PostgresConfig;

#[derive(Debug, Clone)]
pub enum SessionStoreConfig {
    Null,
    Postgres(PostgresConfig),
}

impl SessionStoreConfig {
    pub fn from_env() -> CoreResult<Self> {
        let kind = env::var("SESSION_STORE_KIND")
            .unwrap_or_else(|_| "null".to_string())
            .trim()
            .to_ascii_lowercase();

        match kind.as_str() {
            "null" => Ok(Self::Null),
            "postgres" => {
                let database_url = env::var("DATABASE_URL").map_err(|_| {
                    CoreError::invalid_config(
                        "DATABASE_URL is required when SESSION_STORE_KIND=postgres",
                    )
                })?;

                let database_url = database_url.trim().to_string();

                if database_url.is_empty() {
                    return Err(CoreError::invalid_config(
                        "DATABASE_URL must not be empty when SESSION_STORE_KIND=postgres",
                    ));
                }

                Ok(Self::Postgres(PostgresConfig::new(database_url)))
            }
            other => Err(CoreError::invalid_config(format!(
                "unsupported SESSION_STORE_KIND: {other}"
            ))),
        }
    }

    pub fn null() -> Self {
        Self::Null
    }

    pub fn postgres(database_url: impl Into<String>) -> Self {
        Self::Postgres(PostgresConfig::new(database_url))
    }
}
