use std::fs;

use log::info;

use crate::cli_args::CliArgs;
use crate::env_args::EnvArgs;
use crate::errors::Result;
use crate::util::{MIGRATIONS_FILE_NAME, load_migrations_data};

pub fn new(cli: &CliArgs, env: &EnvArgs, name: &str) -> Result<()> {
    let mut migration_data_file_path = cli.directory.clone();
    migration_data_file_path.push(MIGRATIONS_FILE_NAME);
    let migration_data_file_path = migration_data_file_path;

    let mut migration_data = load_migrations_data(&migration_data_file_path)?;

    info!("Creating new migration {name}");
    let new_migration = migration_data.new_migration(&cli.directory, name)?;

    let json_migrations_data = serde_json::to_string(&migration_data)?;

    fs::write(&migration_data_file_path, json_migrations_data)?;
    info!("Saved migrations data to {migration_data_file_path:?}");
    Ok(())
}
