use std::fs;
use std::path::PathBuf;

use log::info;

use crate::database::connect;
use crate::env_args::EnvArgs;
use crate::errors::Result;
use crate::features::DatabaseMigrationer;
use crate::util::{MIGRATIONS_FILE_NAME, load_migrations_data};

pub fn new(name: &str) -> Result<()> {
    let migration_data_file_path = PathBuf::from(MIGRATIONS_FILE_NAME);

    let mut migration_data = load_migrations_data(&migration_data_file_path)?;

    info!("Creating new migration {name}");
    migration_data.new_migration(name)?;

    let json_migrations_data = serde_json::to_string(&migration_data)?;

    fs::write(&migration_data_file_path, json_migrations_data)?;
    info!("Saved migrations data to {migration_data_file_path:?}");
    Ok(())
}

pub async fn to(migration_id: u32) -> Result<()> {
    let env = envy::from_env::<EnvArgs>()?;
    let mut db_connection = connect(&env.database_url).await?;

    let migration_data_file_path = PathBuf::from(MIGRATIONS_FILE_NAME);
    let migrations_data = load_migrations_data(&migration_data_file_path)?;

    db_connection.to(migrations_data, migration_id).await?;

    Ok(())
}

pub async fn top() -> Result<()> {
    let env = envy::from_env::<EnvArgs>()?;
    let mut db_connection = connect(&env.database_url).await?;

    let migration_data_file_path = PathBuf::from(MIGRATIONS_FILE_NAME);
    let migrations_data = load_migrations_data(&migration_data_file_path)?;
    let to_migration = migrations_data.migrations_counter;

    db_connection.to(migrations_data, to_migration).await?;

    Ok(())
}

pub async fn status() -> Result<()> {
    let env = envy::from_env::<EnvArgs>()?;
    let mut db_connection = connect(&env.database_url).await?;

    let migration_data_file_path = PathBuf::from(MIGRATIONS_FILE_NAME);
    let migrations_data = load_migrations_data(&migration_data_file_path)?;
    let migren_data = db_connection.migren_data().await?;

    info!("Migrations info:");
    info!(
        "Migrations counter is: {}",
        migrations_data.migrations_counter
    );
    info!("Migren version: {}", migrations_data.migren_version);

    info!("Database info:");
    info!(
        "Database is at migration: {} - info about migration: {:#?}",
        migren_data.last_migration_applied,
        migrations_data.migration_by_id(migren_data.last_migration_applied as u32),
    );
    info!("Migren version: {}", migren_data.migren_version);

    Ok(())
}

pub async fn exec(sql_file: &PathBuf) -> Result<()> {
    let env = envy::from_env::<EnvArgs>()?;
    let mut db_connection = connect(&env.database_url).await?;
    let sql_query = fs::read_to_string(sql_file)?;
    let res = db_connection.exec(&sql_query).await?;

    info!("Execution result: {:#?}", res);

    Ok(())
}
