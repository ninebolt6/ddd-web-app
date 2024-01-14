use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Error as DbError, PgPool};

pub async fn connect_test_db() -> Result<PgPool, DbError> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(8))
        .connect("postgresql://testUser:testUserPassword@localhost:55432/postgres")
        .await
}
