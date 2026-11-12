use crate::{
    error::{AppError, AppResult},
    generator::DataObject,
    model::report_model::KeyFrequency,
};
use error_stack::{IntoReport, ResultExt, bail};
use sqlx::{
    PgPool, Postgres,
    pool::PoolConnection,
    postgres::{PgCopyIn, PgPoolCopyExt, PgPoolOptions},
};
use std::time::Duration;
use tokio::time::sleep;

/// Statement that opens the stream of a bulk load.
const COPY_STATEMENT: &str = "COPY data_objects (id, text) FROM STDIN";

/// Number of bytes buffered by a bulk load before they are sent to the database.
const FLUSH_BYTES: usize = 256 * 1024;

/// PostgreSQL Repository.
#[derive(Debug, Clone)]
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

    /// Closes the connection pool.
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Creates the table used to store generated data objects.
    pub async fn create_table(&self) -> AppResult<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS data_objects (\
                row_id BIGSERIAL PRIMARY KEY, \
                id BIGINT NOT NULL, \
                text CHAR(256) NOT NULL\
            )",
        )
        .execute(&self.pool)
        .await
        .change_context(AppError::DatabaseError(
            "failed to create data_objects table".into(),
        ))?;

        Ok(())
    }

    /// Returns the frequency histogram of the keys, ordered by key.
    pub async fn key_histogram(&self) -> AppResult<Vec<KeyFrequency>> {
        let rows: Vec<(i64, i64)> =
            sqlx::query_as("SELECT id, COUNT(*) FROM data_objects GROUP BY id ORDER BY id")
                .fetch_all(&self.pool)
                .await
                .change_context(AppError::DatabaseError(
                    "failed to compute key histogram".into(),
                ))?;

        Ok(rows
            .into_iter()
            .map(|(key, count)| KeyFrequency { key, count })
            .collect())
    }

    /// Returns up to `limit` rows after `after_row_id`.
    pub async fn fetch_batch(
        &self,
        after_row_id: i64,
        limit: i64,
    ) -> AppResult<Vec<(i64, DataObject)>> {
        let rows: Vec<(i64, i64, String)> = sqlx::query_as(
            "SELECT row_id, id, text FROM data_objects WHERE row_id > $1 ORDER BY row_id LIMIT $2",
        )
        .bind(after_row_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .change_context(AppError::DatabaseError(
            "failed to fetch data objects batch".into(),
        ))?;

        Ok(rows
            .into_iter()
            .map(|(row_id, id, text)| {
                (
                    row_id,
                    DataObject {
                        id: id as u64,
                        text,
                    },
                )
            })
            .collect())
    }

    /// Starts a bulk load. The data objects are streamed to the database with
    /// a single copy, which is committed only once, at the end.
    pub async fn bulk_load(&self) -> AppResult<BulkLoad> {
        Ok(BulkLoad::new(
            self.pool
                .copy_in_raw(COPY_STATEMENT)
                .await
                .change_context(AppError::DatabaseError("failed to begin bulk load".into()))?,
        ))
    }
}

/// A bulk load into the database. Dropping it without
/// calling commit discards everything added through it.
pub struct BulkLoad {
    copy: PgCopyIn<PoolConnection<Postgres>>,
    buffer: Vec<u8>,
}

impl BulkLoad {
    /// Creates a new bulk load.
    pub fn new(copy: PgCopyIn<PoolConnection<Postgres>>) -> Self {
        let buffer = Vec::new();
        Self { copy, buffer }
    }

    /// Adds a data object, sending the buffered ones once there are enough.
    pub async fn add(&mut self, object: DataObject) -> AppResult<()> {
        self.buffer.extend_from_slice(&object.into_row());
        if self.buffer.len() >= FLUSH_BYTES {
            self.flush().await?;
        }
        Ok(())
    }

    /// Sends the buffered data objects and commits the load.
    pub async fn commit(mut self) -> AppResult<()> {
        self.flush().await?;
        self.copy
            .finish()
            .await
            .change_context(AppError::DatabaseError("failed to commit bulk load".into()))?;
        Ok(())
    }

    /// Flushes the buffered data objects to the database.
    async fn flush(&mut self) -> AppResult<()> {
        self.copy
            .send(self.buffer.as_slice())
            .await
            .change_context(AppError::DatabaseError(
                "failed to send data objects to the database".into(),
            ))?;
        self.buffer.clear();
        Ok(())
    }
}
