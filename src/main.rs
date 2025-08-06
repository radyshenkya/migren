mod cli_args;
mod commands;
mod errors;
mod env_args;

use clap::Parser;
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();

    let res: errors::Result<()> = (|| {
        let cli = cli_args::CliArgs::parse();
        info!(target: "main", "Hello world! {:?}", cli);

        let env_args =
            envy::from_env::<env_args::EnvArgs>()?;

        Ok(())
    })();

    match res {
        Ok(_) => {},
        Err(err) => error!(target: "main", "Program failed: {:#?}", err),
    }
}
