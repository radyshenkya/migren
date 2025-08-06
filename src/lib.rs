use std::path::PathBuf;

const MIGRATIONS_FILE_NAME: &str = ".migren.json";

/// Returns default migration directory - current working directory.
pub fn default_migrations_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}
