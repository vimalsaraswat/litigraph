use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::domain::{Workspace, WorkspaceRepository};

pub struct WorkspaceFileRepository;

impl WorkspaceFileRepository {
    fn workspace_file(path: &Path) -> PathBuf {
        path.join("workspace.json")
    }
}

impl WorkspaceRepository for WorkspaceFileRepository {
    fn save(&self, workspace: &Workspace, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)?;

        let workspace_file = Self::workspace_file(path);
        let json = serde_json::to_string_pretty(workspace).map_err(io::Error::other)?;

        let temp_file = path.join(format!(".workspace.json.tmp.{}", uuid::Uuid::new_v4()));
        if let Err(err) = fs::write(&temp_file, &json) {
            let _ = fs::remove_file(&temp_file);
            return Err(err);
        }

        if let Err(err) = fs::rename(&temp_file, &workspace_file) {
            let _ = fs::remove_file(&temp_file);
            return Err(err);
        }

        Ok(())
    }

    fn load(&self, path: &Path) -> io::Result<Workspace> {
        let workspace_file = Self::workspace_file(path);

        let json = fs::read_to_string(workspace_file)?;

        let workspace = serde_json::from_str(&json).map_err(io::Error::other)?;

        Ok(workspace)
    }

    fn delete(&self, path: &Path) -> io::Result<()> {
        let workspace_file = Self::workspace_file(path);

        match fs::remove_file(workspace_file) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn exists(&self, path: &Path) -> bool {
        Self::workspace_file(path).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_delete_only_removes_workspace_file_and_preserves_user_files() {
        let temp = TestDir::new();
        let repo = WorkspaceFileRepository;

        let workspace = Workspace::new("Test Project");
        repo.save(&workspace, &temp.path).unwrap();

        let user_file = temp.path.join("user_notes.md");
        fs::write(&user_file, "Important user data").unwrap();

        let workspace_file = temp.path.join("workspace.json");
        assert!(workspace_file.exists());
        assert!(user_file.exists());

        repo.delete(&temp.path).unwrap();

        assert!(!workspace_file.exists());
        assert!(temp.path.exists());
        assert!(user_file.exists());
        assert_eq!(
            fs::read_to_string(&user_file).unwrap(),
            "Important user data"
        );
    }

    #[test]
    fn test_delete_non_existent_file_is_idempotent() {
        let temp = TestDir::new();
        let repo = WorkspaceFileRepository;

        assert!(repo.delete(&temp.path).is_ok());

        let non_existent = temp.path.join("does_not_exist");
        assert!(repo.delete(&non_existent).is_ok());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp = TestDir::new();
        let repo = WorkspaceFileRepository;

        let workspace = Workspace::new("My Novel");
        assert!(!repo.exists(&temp.path));

        repo.save(&workspace, &temp.path).unwrap();
        assert!(repo.exists(&temp.path));

        let loaded = repo.load(&temp.path).unwrap();
        assert_eq!(loaded.id, workspace.id);
        assert_eq!(loaded.name, "My Novel");
    }
}
