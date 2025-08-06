use std::path::PathBuf;

/// Returns default migration directory - current working directory.
pub fn default_migrations_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}
