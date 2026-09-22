use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::{WorkspaceCatalog, WorkspaceCatalogStore};

pub struct FileWorkspaceCatalogStore {
    path: PathBuf,
}

impl FileWorkspaceCatalogStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl WorkspaceCatalogStore for FileWorkspaceCatalogStore {
    fn load(&self) -> io::Result<WorkspaceCatalog> {
        if !self.path.exists() {
            return Ok(WorkspaceCatalog::default());
        }

        let json = fs::read_to_string(&self.path)?;

        let catalog = serde_json::from_str(&json).map_err(io::Error::other)?;

        Ok(catalog)
    }

    fn save(&self, catalog: &WorkspaceCatalog) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(catalog).map_err(io::Error::other)?;

        fs::write(&self.path, json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Workspace;
    use crate::infrastructure::catalog::WorkspaceEntry;

    #[test]
    fn test_file_store_non_existent_load() {
        let temp_dir =
            std::env::temp_dir().join(format!("litigraph_test_{}", uuid::Uuid::new_v4()));
        let store = FileWorkspaceCatalogStore::new(temp_dir.join("catalog.json"));

        let catalog = store.load().unwrap();
        assert!(catalog.is_empty());
    }

    #[test]
    fn test_file_store_save_and_load() {
        let temp_dir =
            std::env::temp_dir().join(format!("litigraph_test_{}", uuid::Uuid::new_v4()));
        let store_path = temp_dir.join("nested").join("catalog.json");
        let store = FileWorkspaceCatalogStore::new(&store_path);

        let ws = Workspace::new("Persisted Workspace");
        let entry = WorkspaceEntry::new(&ws, PathBuf::from("/some/path"));

        let mut catalog = WorkspaceCatalog::default();
        catalog.add(entry);

        store.save(&catalog).unwrap();
        assert!(store_path.exists());

        let loaded = store.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.entries()[0].name, "Persisted Workspace");

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }
}
