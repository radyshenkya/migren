use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Single migration information
#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationData {
    /// Name of up migration file. Relative to migrations directory
    pub up_migration_file: PathBuf,
    /// Name of down migration file. Relative to migrations directory
    pub down_migration_file: PathBuf,
    /// Migration name
    pub name: String,
    /// Migration id
    pub id: u32,
    /// Previous migration
    pub prev_migration_id: Option<u32>,
    /// Next migration
    pub next_migration_id: Option<u32>,
}

/// Holds every migration. Root object for .migren.json file
#[derive(Deserialize, Serialize, Debug)]
pub struct MigrationsData {
    migrations: Vec<MigrationData>
}
