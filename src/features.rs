use std::{collections::HashSet, path::PathBuf};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use sqlx::Connection;

use crate::{
    derictive_constants::SqlDirective, errors::{MigrenError, Result}, util::{assert_migration_files_exists, create_migration_files}
};

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

/// Holds data about migration from path.
#[derive(Debug)]
pub struct MigrationToApply {
    pub id: u32,
    pub file: PathBuf,
}

/// Holds every migration. Root object for .migren.json file
#[derive(Deserialize, Serialize, Debug)]
pub struct MigrationsData {
    pub migrations: Vec<MigrationData>,
    pub migren_version: String,
    pub migrations_start_id: Option<u32>,
    pub migrations_counter: u32,
}

impl Default for MigrationsData {
    fn default() -> Self {
        Self {
            migrations: vec![MigrationData {
                files: MigrationFiles {
                    up_migration_file: PathBuf::new(),
                    down_migration_file: PathBuf::new(),
                },
                name: "initial".to_string(),
                id: 0,
                prev_migration_id: None,
                next_migration_id: None,
            }],
            migrations_start_id: None,
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
        self.migrations
            .iter_mut()
            .find(|migration| migration.id == id)
    }

    /// Create new migrations files + modify migrations_data
    pub fn new_migration(&mut self, migration_name: &str) -> Result<&MigrationData> {
        let migration_id = self.migrations_counter + 1;
        let last_migration_id = self
            .migration_by_id(self.migrations_counter)
            .map(|migration| migration.id);

        info!("New migration id is {migration_id}");
        info!("Found last migration: {last_migration_id:?}");
        let migration_files = create_migration_files(migration_id, migration_name)?;

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
        } else {
            self.migrations_start_id = Some(migration_id);
        }

        self.migrations_counter = migration_id;
        self.migrations.push(migration);

        Ok(self.migration_by_id(migration_id).unwrap())
    }

    pub fn build_migration_path_down(
        &self,
        start: &MigrationData,
        stop: &MigrationData,
    ) -> Result<Vec<MigrationToApply>> {
        let mut found_migrations =
            HashSet::<u32>::with_capacity((stop.id as usize).abs_diff(start.id as usize));

        let mut on_migration = start;
        let mut path = Vec::with_capacity((stop.id as usize).abs_diff(start.id as usize));

        while on_migration.id != stop.id {
            // Skipping 0 migration
            if on_migration.id != 0 {
                assert_migration_files_exists(on_migration)?;
                path.push(MigrationToApply {
                    id: on_migration.id,
                    file: on_migration.files.down_migration_file.clone(),
                });
            }

            if found_migrations.contains(&on_migration.id) {
                return Err(MigrenError::MigrationPathInvalid {
                    from: start.id,
                    to: stop.id,
                    comment: format!("Circular migration found: {on_migration:#?}"),
                });
            }
            found_migrations.insert(on_migration.id);

            let next_migration = on_migration
                .prev_migration_id
                .and_then(|next_id| self.migration_by_id(next_id));

            if next_migration.is_none() {
                return Err(MigrenError::MigrationPathInvalid {
                    from: start.id,
                    to: stop.id,
                    comment: format!(
                        "Next migration not found. Was on migration {on_migration:#?}"
                    ),
                });
            }
            on_migration = next_migration.unwrap();
        }
        Ok(path)
    }

    pub fn build_migration_path_up(
        &self,
        start: &MigrationData,
        stop: &MigrationData,
    ) -> Result<Vec<MigrationToApply>> {
        let mut found_migrations =
            HashSet::<u32>::with_capacity((stop.id as usize).abs_diff(start.id as usize));

        let mut on_migration = start;
        let mut path = Vec::with_capacity((stop.id as usize).abs_diff(start.id as usize));

        while on_migration.id != stop.id {
            if found_migrations.contains(&on_migration.id) {
                return Err(MigrenError::MigrationPathInvalid {
                    from: start.id,
                    to: stop.id,
                    comment: format!("Circular migration found: {on_migration:#?}"),
                });
            }
            found_migrations.insert(on_migration.id);

            let next_migration = on_migration
                .next_migration_id
                .and_then(|next_id| self.migration_by_id(next_id));

            if next_migration.is_none() {
                return Err(MigrenError::MigrationPathInvalid {
                    from: start.id,
                    to: stop.id,
                    comment: format!(
                        "Next migration not found. Was on migration {on_migration:#?}"
                    ),
                });
            }
            on_migration = next_migration.unwrap();
            // Skipping 0 migration
            if on_migration.id != 0 {
                assert_migration_files_exists(on_migration)?;
                path.push(MigrationToApply {
                    id: on_migration.id,
                    file: on_migration.files.up_migration_file.clone(),
                });
            }
        }
        Ok(path)
    }

    pub fn build_migration_path(&self, from: u32, to: u32) -> Result<Vec<MigrationToApply>> {
        let from_migration = self.migration_by_id(from);
        if from_migration.is_none() {
            return Err(MigrenError::MigrationPathInvalid {
                from,
                to,
                comment: "from migration does not exists".to_string(),
            });
        }

        let to_migration = self.migration_by_id(to);
        if to_migration.is_none() {
            return Err(MigrenError::MigrationPathInvalid {
                from,
                to,
                comment: "to migration does not exists".to_string(),
            });
        }
        let start = from_migration.unwrap();
        let stop = to_migration.unwrap();
        if start.id < stop.id {
            self.build_migration_path_up(start, stop)
        } else {
            self.build_migration_path_down(start, stop)
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct DatabaseMigrenData {
    pub migren_version: String,
    pub last_migration_applied: i32,
}

impl Default for DatabaseMigrenData {
    fn default() -> Self {
        Self {
            migren_version: env!("CARGO_PKG_VERSION").to_string(),
            last_migration_applied: 0,
        }
    }
}

pub trait DatabaseMigrationer {
    async fn migren_data(&mut self) -> Result<DatabaseMigrenData>;
    async fn set_migren_data(&mut self, data: DatabaseMigrenData) -> Result<()>;
    async fn to(&mut self, migrations_data: MigrationsData, migration_id: u32) -> Result<()>;
    async fn exec(&mut self, sql_query: &str)
    -> Result<<sqlx::Any as sqlx::Database>::QueryResult>;
}

impl DatabaseMigrationer for sqlx::AnyConnection {
    async fn migren_data(&mut self) -> Result<DatabaseMigrenData> {
        let mut migren_info =
            sqlx::query_as::<_, DatabaseMigrenData>("SELECT * FROM migren_data LIMIT 1")
                .fetch_all(&mut *self)
                .await?;

        if migren_info.len() == 0 {
            self.set_migren_data(DatabaseMigrenData::default()).await?;
            Ok(DatabaseMigrenData::default())
        } else {
            Ok(migren_info.pop().unwrap())
        }
    }

    async fn set_migren_data(&mut self, data: DatabaseMigrenData) -> Result<()> {
        // Removing all saves
        sqlx::query("DELETE FROM migren_data")
            .execute(&mut *self)
            .await?;
        debug!("Removed all rows from migren_data");

        sqlx::query(
            "INSERT INTO migren_data (migren_version, last_migration_applied) VALUES ($1, $2);",
        )
        .bind(data.migren_version)
        .bind(data.last_migration_applied)
        .execute(&mut *self)
        .await?;
        debug!("Saved new row into migren_data");

        Ok(())
    }

    async fn to(&mut self, migrations_data: MigrationsData, migration_id: u32) -> Result<()> {
        let migren_data = self.migren_data().await?;

        if migren_data.last_migration_applied == migration_id as i32 {
            info!("Database is already at migration {migration_id}");
            return Ok(());
        }

        let start_id = migren_data.last_migration_applied as u32;
        let migration_path = migrations_data.build_migration_path(start_id, migration_id)?;

        let mut tx = self.begin().await?;
        debug!("Begin transaction...");

        debug!("{migration_path:#?}");

        for migration in migration_path.into_iter() {
            let sql_code = std::fs::read_to_string(&migration.file)?;

            // TODO: MIG-23 - move usage of this derictive into distinct place + maybe add a couple
            // extra derictives.
            let mut statement_buffer = String::new();
            for line in sql_code.lines() {
                if let Some(derictive) = SqlDirective::match_str(line) {
                    debug!("Found directive: {derictive:?}");
                    match derictive {
                        SqlDirective::Split => {
                            sqlx::query(&statement_buffer).execute(&mut *tx).await?;
                            statement_buffer = String::new();
                        }
                    }
                }

                statement_buffer.push_str(line);
                statement_buffer.push_str("\n");
            }
            
            sqlx::query(&statement_buffer).execute(&mut *tx).await?;

            let semicolons_count = sql_code
                .as_bytes()
                .iter()
                .filter(|x| **x == ';' as u8)
                .count();
            if semicolons_count > 1 {
                warn!(
                    "Multiple semicolons found in the same query! Suggest using only one command in file, because it can lead to some problems... Semicolons count: {semicolons_count}"
                );
            }

            info!("Applied file {:?}", &migration.file);
        }

        sqlx::query("UPDATE migren_data SET last_migration_applied = $1")
            .bind(migration_id as i32)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        info!("Transaction completed");

        Ok(())
    }

    async fn exec(
        &mut self,
        sql_query: &str,
    ) -> Result<<sqlx::Any as sqlx::Database>::QueryResult> {
        Ok(sqlx::query(sql_query).execute(&mut *self).await?)
    }
}
