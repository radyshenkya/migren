use std::path::PathBuf;

use clap::{Parser, Subcommand};
use migren::default_migrations_dir;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Move to selected migration (can be used as rollback as well)
    To { migration_id: u32 },
    /// Move to last added migration
    Top,
    /// Create new migration
    New { name: String },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value = default_migrations_dir().into_os_string())]
    pub directory: PathBuf, #[command(subcommand)]
    pub command: Command,
}
