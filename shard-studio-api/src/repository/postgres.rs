use crate::{
    error::{AppError, AppResult},
    generator::DataObject,
};
use error_stack::{IntoReport, ResultExt, bail};
use sqlx::{PgPool, QueryBuilder, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::time::sleep;

/// PostgreSQL Repository.
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    /// Connects to the PostgreSQL instance exposed on the given host port.
    pub async fn connect(host_port: u16) -> AppResult<Self> {
        let url = format!("postgres://username:password@localhost:{host_port}/postgres");

        let mut attempts_left = 50;
        loop {
            match PgPoolOptions::new().max_connections(5).connect(&url).await {
                Ok(pool) => return Ok(Self { pool }),
                Err(_) if attempts_left > 1 => {
                    attempts_left -= 1;
                    sleep(Duration::from_millis(100)).await;
                }
                Err(err) => {
                    bail!(err.into_report().change_context(AppError::DatabaseError(
                        "failed to connect to postgres".into()
                    )));
                }
            }
        }
    }

    /// Creates the table used to store generated data objects.
    pub async fn create_table(&self) -> AppResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS data_objects (\
                row_id BIGSERIAL PRIMARY KEY, \
                id BIGINT NOT NULL, \
                text TEXT NOT NULL\
            )",
        )
        .execute(&self.pool)
        .await
        .change_context(AppError::DatabaseError(
            "failed to create data_objects table".into(),
        ))?;

        Ok(())
    }

    /// Saves all of the given data objects to the database.
    pub async fn save_all(&self, items: &[DataObject]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new("INSERT INTO data_objects (id, text) ");
        query_builder.push_values(items, |mut builder, item| {
            builder.push_bind(item.id as i64).push_bind(&item.text);
        });

        query_builder
            .build()
            .execute(&self.pool)
            .await
            .change_context(AppError::DatabaseError(
                "failed to insert data objects batch".into(),
            ))?;

        Ok(())
    }
}
