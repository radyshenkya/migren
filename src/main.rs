mod cli_args;
mod commands;
mod database;
mod env_args;
mod errors;
mod features;
mod util;

use clap::Parser;
use dotenv::dotenv;
use log::error;
use util::create_dir_if_not_exists;

async fn run_migren() -> errors::Result<()> {
    let cli = cli_args::CliArgs::parse();

    create_dir_if_not_exists(&cli.directory)?;
    std::env::set_current_dir(&cli.directory)?;

    match &cli.command {
        cli_args::Command::To { migration_id } => {
                commands::to(*migration_id).await
            }
        cli_args::Command::Top => commands::top().await,
        cli_args::Command::New { name } => commands::new(name),
        cli_args::Command::Status => commands::status().await,
        cli_args::Command::Exec { sql_file } => commands::exec(sql_file).await,
    }?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let res: errors::Result<()> = run_migren().await;

    match res {
        Ok(_) => {}
        Err(err) => {
            error!("Program failed: {:?}", err);
            panic!();
        }
    }
}
