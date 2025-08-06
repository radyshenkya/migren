mod cli_args;
mod commands;
mod env_args;

use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = cli_args::CliArgs::parse();

    let env_args = envy::from_env::<env_args::EnvArgs>()
        .expect("Failed to parse environment variables");
}
