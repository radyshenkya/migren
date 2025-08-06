use thiserror::Error;

pub type Result<T> = std::result::Result<T, MigrenError>;

#[derive(Error, Debug)]
pub enum MigrenError {
    #[error("Failed to load env variables")]
    Envy(#[from]envy::Error),
    #[error("Failed to parse CLI arguments")]
    Clap(#[from]clap::Error),
    #[error(transparent)]
    Io(#[from]std::io::Error),
}
