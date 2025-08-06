mod cli_args;
mod commands;
mod env_args;
mod errors;
mod schemas;
mod util;

use clap::Parser;
use log::{error};
use util::create_dir_if_not_exists;

async fn run_migren() -> errors::Result<()> {
    let cli = cli_args::CliArgs::parse();
    let env_args = envy::from_env::<env_args::EnvArgs>()?;

    create_dir_if_not_exists(&cli.directory)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let res: errors::Result<()> = run_migren().await;

    match res {
        Ok(_) => {}
        Err(err) => error!(target: "main", "Program failed: {:#?}", err),
    }
}
