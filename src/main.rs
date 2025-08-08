mod cli_args;
mod commands;
mod database;
mod env_args;
mod errors;
mod features;
mod util;

use clap::Parser;
use log::error;
use util::create_dir_if_not_exists;

async fn run_migren() -> errors::Result<()> {
    let cli = cli_args::CliArgs::parse();
    let env_args = envy::from_env::<env_args::EnvArgs>()?;

    create_dir_if_not_exists(&cli.directory)?;
    std::env::set_current_dir(&cli.directory)?;

    match &cli.command {
        cli_args::Command::To { migration_id } => {
                commands::to(&cli, &env_args, *migration_id).await
            }
        cli_args::Command::Top => commands::top(&cli, &env_args).await,
        cli_args::Command::New { name } => commands::new(&cli, &env_args, name),
        cli_args::Command::Status => commands::status(&cli, &env_args).await,
    }?;

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
