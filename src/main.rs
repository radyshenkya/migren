mod cli_args;
mod commands;
mod env_args;
mod errors;
mod features;
mod schemas;
mod util;

use clap::Parser;
use log::error;
use util::create_dir_if_not_exists;

use crate::{features::load_migrations_data, util::MIGRATIONS_FILE_NAME};

async fn run_migren() -> errors::Result<()> {
    let cli = cli_args::CliArgs::parse();
    let env_args = envy::from_env::<env_args::EnvArgs>()?;

    create_dir_if_not_exists(&cli.directory)?;

    let mut migration_data_file_path = cli.directory.clone();
    migration_data_file_path.push(MIGRATIONS_FILE_NAME);
    let migration_data_file_path = migration_data_file_path;

    let migration_data = load_migrations_data(&migration_data_file_path)?;
    log::info!(target: "run_migren", "Migration_data: {:?}", migration_data);

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let res: errors::Result<()> = run_migren().await;

    match res {
        Ok(_) => {}
        Err(err) => error!(target: "main", "Program failed: {:?}", err),
    }
}
