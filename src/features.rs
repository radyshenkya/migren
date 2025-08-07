use std::path::PathBuf;

use log::info;
use serde::{Deserialize, Serialize};

use crate::{errors::Result, util::create_migration_files};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MigrationFiles {
    /// Name of up migration file. Relative to migrations directory
    pub up_migration_file: PathBuf,
    /// Name of down migration file. Relative to migrations directory
    pub down_migration_file: PathBuf,
}

/// Single migration information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MigrationData {
    /// Migration files names
    pub files: MigrationFiles,
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
    pub migrations: Vec<MigrationData>,
    pub migren_version: String,
    pub migrations_counter: u32,
}

impl Default for MigrationsData {
    fn default() -> Self {
        Self {
            migrations: Vec::new(),
            migren_version: env!("CARGO_PKG_VERSION").to_string(),
            migrations_counter: 0,
        }
    }
}

impl MigrationsData {
    pub fn migration_by_id(&self, id: u32) -> Option<&MigrationData> {
        self.migrations.iter().find(|migration| migration.id == id)
    }

    pub fn migration_by_id_mut(&mut self, id: u32) -> Option<&mut MigrationData> {
        self.migrations.iter_mut().find(|migration| migration.id == id)
    }

    /// Create new migrations files + modify migrations_data
    pub fn new_migration(&mut self, working_directory: &PathBuf, migration_name: &str) -> Result<&MigrationData> {
        let migration_id = self.migrations_counter + 1;
        let last_migration_id = self
            .migration_by_id(self.migrations_counter)
            .map(|migration| migration.id);

        info!("New migration id is {migration_id}");
        info!("Found last migration: {last_migration_id:?}");
        let migration_files = create_migration_files(working_directory, migration_id, migration_name)?;

        let migration = MigrationData {
            files: migration_files,
            name: migration_name.to_string(),
            id: migration_id,
            prev_migration_id: last_migration_id,
            next_migration_id: Option::None,
        };

        info!("New migration data: {migration:#?}");

        if let Some(last_migration) = self.migration_by_id_mut(self.migrations_counter) {
            last_migration.next_migration_id = Some(migration_id);
            info!("Changed last migration refs: {last_migration:#?}");
        }

        self.migrations_counter = migration_id;
        self.migrations.push(migration);

        Ok(self.migration_by_id(migration_id).unwrap())
    }
}

