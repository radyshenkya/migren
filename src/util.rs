use std::{fs, path::PathBuf};

use log::info;

use crate::{
    errors::{MigrenError, Result},
    features::{MigrationData, MigrationFiles, MigrationsData},
};

pub const MIGRATIONS_FILE_NAME: &str = ".migren.json";

/// Returns default migration directory - current working directory.
pub fn default_migrations_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn create_dir_if_not_exists(path: &PathBuf) -> Result<()> {
    if !std::fs::exists(path)? {
        log::info!(target: "create_dir_if_not_exists", "Directory {:?} does not exists. Creating it", path);
        std::fs::create_dir(path)?;
    }

    Ok(())
}

/// Loads migration data from file
pub fn load_migrations_data(migrations_file: &PathBuf) -> Result<MigrationsData> {
    if !fs::exists(migrations_file)? {
        info!(target: "load_migrations_data", "File {:?} does not exist. Creating one", migrations_file);

        let migrations_data = MigrationsData::default();
        let json_str = serde_json::to_string(&migrations_data)?;

        fs::write(migrations_file, &json_str)?;
    }

    let migrations_data: MigrationsData =
        serde_json::from_str(&fs::read_to_string(migrations_file)?)?;
    Ok(migrations_data)
}

/// Create files for migration
pub fn create_migration_files(migration_id: u32, migration_name: &str) -> Result<MigrationFiles> {
    info!("Creating migration files for {migration_name}.");
    let up_migration_file = PathBuf::from(format!("{migration_id}_{migration_name}_up.sql"));

    let down_migration_file = PathBuf::from(format!("{migration_id}_{migration_name}_down.sql"));

    fs::write(
        &up_migration_file,
        format!("-- {migration_id} - {migration_name} up query"),
    )?;
    info!("Wrote {up_migration_file:?}");

    fs::write(
        &down_migration_file,
        format!("-- {migration_id} - {migration_name} down query"),
    )?;
    info!("Wrote {down_migration_file:?}");

    Ok(MigrationFiles {
        up_migration_file: PathBuf::from(up_migration_file),
        down_migration_file: PathBuf::from(down_migration_file),
    })
}

pub fn assert_migration_files_exists(migration_data: &MigrationData) -> Result<()> {
    if !fs::exists(&migration_data.files.up_migration_file)?
        || !fs::exists(&migration_data.files.down_migration_file)?
    {
        return Err(MigrenError::MigrationFilesDoesNotExsists(
            migration_data.clone(),
        ));
    }

    Ok(())
}
