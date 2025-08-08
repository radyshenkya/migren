use log::info;
use sqlx::{AnyConnection, Connection};

use crate::errors::Result;

const TABLE_CREATE: &str = "
CREATE TABLE IF NOT EXISTS migren_data (
    migren_version TEXT,
    last_migration_applied INTEGER
);
";

pub async fn connect(url: &str) -> Result<AnyConnection> {
    sqlx::any::install_default_drivers();
    let mut conn = sqlx::AnyConnection::connect(url).await?;
    info!("Connected to DB");

    sqlx::query(TABLE_CREATE).execute(&mut conn).await?;
    info!("Creating migren_data table if does not exists yet...");

    Ok(conn)
}
