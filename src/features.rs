use std::fs;
use std::path::PathBuf;

use log::info;

use crate::errors::Result;
use crate::schemas::MigrationsData;

pub fn load_migrations_data(migrations_file: &PathBuf) -> Result<MigrationsData> {
    if !fs::exists(migrations_file)? {
        info!(target: "load_migrations_data", "File {:?} does not exist. Creating one", migrations_file);

        let migrations_data = MigrationsData {
            migrations: Vec::new(),
            migren_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let json_str = serde_json::to_string(&migrations_data)?;

        fs::write(migrations_file, &json_str)?;
    }

    let migrations_data: MigrationsData =
        serde_json::from_str(&fs::read_to_string(migrations_file)?)?;
    Ok(migrations_data)
}
