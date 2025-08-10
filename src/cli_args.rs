use std::path::PathBuf;

use crate::util::default_migrations_dir;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Move to selected migration (can be used as rollback as well)
    To { migration_id: u32 },
    /// Move to last added migration
    Top,
    /// Status about DB and migrations
    Status,
    /// Execute .sql file for db
    Exec { sql_file: PathBuf },
    /// Create new migration
    New { name: String },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value = default_migrations_dir().into_os_string())]
    pub directory: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}
