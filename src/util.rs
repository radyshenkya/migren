use std::path::PathBuf;

pub const MIGRATIONS_FILE_NAME: &str = ".migren.json";

/// Returns default migration directory - current working directory.
pub fn default_migrations_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn create_dir_if_not_exists(path: &PathBuf) -> crate::errors::Result<()>{
    if !std::fs::exists(path)? {
        log::info!(target: "create_dir_if_not_exists", "Directory {:?} does not exists. Creating it", path);
        std::fs::create_dir(path)?;
    }

    Ok(())
}
