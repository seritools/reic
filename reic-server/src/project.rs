use std::path::PathBuf;

use sled::Db;

#[derive(Debug)]
struct PersistedProject {
    db_path: PathBuf,
    db: Db,
}

impl PersistedProject {}
