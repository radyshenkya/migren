use thiserror::Error;

use crate::features::MigrationData;

pub type Result<T> = std::result::Result<T, MigrenError>;

#[derive(Error, Debug)]
pub enum MigrenError {
    #[error("Failed to load env variables")]
    Envy(#[from] envy::Error),
    #[error("Failed to parse CLI arguments")]
    Clap(#[from] clap::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("Migration path from {from} to {to} is invalid. {comment}")]
    MigrationPathInvalid { from: u32, to: u32, comment: String },
    #[error("Migration files does not exists: {0:#?}")]
    MigrationFilesDoesNotExsists(MigrationData),
}
