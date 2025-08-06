mod cli_args;
mod commands;
mod env_args;
mod errors;
mod schemas;

use clap::Parser;
use log::{error, debug};

async fn run_migren() -> errors::Result<()> {
    let cli = cli_args::CliArgs::parse();
    debug!(target: "main", "Hello world! {:?}", cli);

    let env_args = envy::from_env::<env_args::EnvArgs>()?;

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
