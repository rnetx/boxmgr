use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use sea_orm::{ConnectionTrait, Schema};

#[derive(Debug, Clone)]
pub(crate) struct Database {
    connection: sea_orm::DatabaseConnection,
}

impl Database {
    pub(crate) async fn new(url: &str) -> Result<Self, sea_orm::DbErr> {
        let mut options = sea_orm::ConnectOptions::new(url);
        options
            .max_connections(32)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Trace)
            .set_schema_search_path("my_schema");
        let conn = sea_orm::Database::connect(options).await?;

        let s = Self { connection: conn };

        s.initialize()
            .await
            .map_err(|e| sea_orm::DbErr::Custom(e))?;

        Ok(s)
    }

    pub(crate) async fn close(self) -> Result<(), sea_orm::DbErr> {
        self.connection.close().await
    }

    async fn initialize(&self) -> Result<(), String> {
        let builder = self.connection.get_database_backend();
        let schema = Schema::new(builder.clone());
        // Config
        let mut config_stmt_builder = schema.create_table_from_entity(super::ConfigEntity);
        config_stmt_builder.if_not_exists();
        let config_stmt = builder.build(&config_stmt_builder);
        // Script
        let mut script_stmt_builder = schema.create_table_from_entity(super::ScriptEntity);
        script_stmt_builder.if_not_exists();
        let script_stmt = builder.build(&script_stmt_builder);
        // Kv
        let mut kv_stmt_builder = schema.create_table_from_entity(super::KvEntity);
        kv_stmt_builder.if_not_exists();
        let kv_stmt = builder.build(&kv_stmt_builder);
        //
        let (config_result, script_result, kv_result) = tokio::join!(
            self.connection.execute(config_stmt),
            self.connection.execute(kv_stmt),
            self.connection.execute(script_stmt),
        );
        match (&config_result, &script_result, &kv_result) {
            (Ok(_), Ok(_), Ok(_)) => Ok(()),
            _ => {
                let mut s = format!("failed to create table: ");
                if let Err(e) = config_result {
                    s.push_str(&format!("config table: {}, ", e))
                }
                if let Err(e) = script_result {
                    s.push_str(&format!("script table: {}, ", e))
                }
                if let Err(e) = kv_result {
                    s.push_str(&format!("kv table: {}, ", e))
                }
                s.pop();
                s.pop();
                Err(s)
            }
        }
    }
}

impl Deref for Database {
    type Target = sea_orm::DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection
    }
}
