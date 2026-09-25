use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::domain::catalog::{WorkspaceCatalog, WorkspaceCatalogStore};

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
        let json = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Ok(WorkspaceCatalog::default());
            }
            Err(err) => return Err(err),
        };

        let catalog = serde_json::from_str(&json).map_err(io::Error::other)?;
        Ok(catalog)
    }

    fn save(&self, catalog: &WorkspaceCatalog) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(catalog).map_err(io::Error::other)?;

        let temp_path = match self.path.parent() {
            Some(parent) => parent.join(format!(
                ".{}.tmp.{}",
                self.path.file_name().unwrap_or_default().to_string_lossy(),
                uuid::Uuid::new_v4()
            )),
            None => PathBuf::from(format!(".catalog.tmp.{}", uuid::Uuid::new_v4())),
        };

        if let Err(err) = fs::write(&temp_path, &json) {
            let _ = fs::remove_file(&temp_path);
            return Err(err);
        }

        if let Err(err) = fs::rename(&temp_path, &self.path) {
            let _ = fs::remove_file(&temp_path);
            return Err(err);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{catalog::WorkspaceEntry, Workspace};

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let path =
                std::env::temp_dir().join(format!("litigraph_test_{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn test_file_store_non_existent_load_returns_default() {
        let temp = TestDir::new();
        let store = FileWorkspaceCatalogStore::new(temp.path.join("non_existent_catalog.json"));

        let catalog = store.load().unwrap();
        assert!(catalog.is_empty());
    }

    #[test]
    fn test_file_store_save_and_load_roundtrip() {
        let temp = TestDir::new();
        let store_path = temp.path.join("nested").join("catalog.json");
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
    }

    #[test]
    fn test_file_store_load_invalid_json_returns_error() {
        let temp = TestDir::new();
        let store_path = temp.path.join("invalid_catalog.json");
        fs::write(&store_path, "{ not valid json").unwrap();

        let store = FileWorkspaceCatalogStore::new(&store_path);
        let result = store.load();
        assert!(result.is_err());
    }
}
