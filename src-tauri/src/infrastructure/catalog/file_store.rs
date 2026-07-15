use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::WorkspaceCatalog;

pub struct FileWorkspaceCatalogStore {
    path: PathBuf,
}

impl FileWorkspaceCatalogStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> io::Result<WorkspaceCatalog> {
        if !self.path.exists() {
            return Ok(WorkspaceCatalog::default());
        }

        let json = fs::read_to_string(&self.path)?;

        let catalog = serde_json::from_str(&json).map_err(io::Error::other)?;

        Ok(catalog)
    }

    pub fn save(&self, catalog: &WorkspaceCatalog) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(catalog).map_err(io::Error::other)?;

        fs::write(&self.path, json)?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
