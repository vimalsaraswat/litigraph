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

        fs::write(workspace_file, json)?;

        Ok(())
    }

    fn load(&self, path: &Path) -> io::Result<Workspace> {
        let workspace_file = Self::workspace_file(path);

        let json = fs::read_to_string(workspace_file)?;

        let workspace = serde_json::from_str(&json).map_err(io::Error::other)?;

        Ok(workspace)
    }

    fn delete(&self, path: &Path) -> io::Result<()> {
        if path.exists() {
            fs::remove_dir_all(path)?;
        }

        Ok(())
    }
}
