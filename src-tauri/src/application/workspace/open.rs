use std::{
    fmt, io,
    path::{Path, PathBuf},
};

use crate::domain::{
    catalog::{WorkspaceCatalogStore, WorkspaceEntry},
    workspace::{Workspace, WorkspaceRepository},
};

#[derive(Debug)]
pub enum OpenWorkspaceError {
    NotFound(PathBuf),
    Repository(io::Error),
    Catalog(io::Error),
}

impl fmt::Display for OpenWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => {
                write!(f, "Workspace not found at path: {}", path.display())
            }
            Self::Repository(err) => write!(f, "Repository error: {}", err),
            Self::Catalog(err) => write!(f, "Catalog error: {}", err),
        }
    }
}

impl std::error::Error for OpenWorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Repository(err) | Self::Catalog(err) => Some(err),
            _ => None,
        }
    }
}

impl From<OpenWorkspaceError> for io::Error {
    fn from(err: OpenWorkspaceError) -> Self {
        match err {
            OpenWorkspaceError::NotFound(_) => {
                io::Error::new(io::ErrorKind::NotFound, err.to_string())
            }
            OpenWorkspaceError::Repository(e) | OpenWorkspaceError::Catalog(e) => e,
        }
    }
}

pub struct OpenWorkspace<R, C>
where
    R: WorkspaceRepository,
    C: WorkspaceCatalogStore,
{
    repository: R,
    catalog_store: C,
}

impl<R, C> OpenWorkspace<R, C>
where
    R: WorkspaceRepository,
    C: WorkspaceCatalogStore,
{
    pub fn new(repository: R, catalog_store: C) -> Self {
        Self {
            repository,
            catalog_store,
        }
    }

    pub fn execute(&self, workspace_path: &Path) -> Result<Workspace, OpenWorkspaceError> {
        let workspace = self
            .repository
            .load(workspace_path)
            .map_err(|err| match err.kind() {
                io::ErrorKind::NotFound => {
                    OpenWorkspaceError::NotFound(workspace_path.to_path_buf())
                }
                _ => OpenWorkspaceError::Repository(err),
            })?;

        let mut catalog = self
            .catalog_store
            .load()
            .map_err(OpenWorkspaceError::Catalog)?;

        catalog.add(WorkspaceEntry::new(
            &workspace,
            workspace_path.to_path_buf(),
        ));

        self.catalog_store
            .save(&catalog)
            .map_err(OpenWorkspaceError::Catalog)?;

        Ok(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::catalog::WorkspaceCatalog;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[derive(Default, Clone)]
    struct MockWorkspaceRepository {
        workspaces: Arc<Mutex<HashMap<PathBuf, Workspace>>>,
    }

    impl WorkspaceRepository for MockWorkspaceRepository {
        fn save(&self, workspace: &Workspace, path: &Path) -> io::Result<()> {
            self.workspaces
                .lock()
                .unwrap()
                .insert(path.to_path_buf(), workspace.clone());
            Ok(())
        }

        fn load(&self, path: &Path) -> io::Result<Workspace> {
            self.workspaces
                .lock()
                .unwrap()
                .get(path)
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Not found"))
        }

        fn delete(&self, path: &Path) -> io::Result<()> {
            self.workspaces.lock().unwrap().remove(path);
            Ok(())
        }
    }

    #[derive(Default, Clone)]
    struct MockCatalogStore {
        catalog: Arc<Mutex<WorkspaceCatalog>>,
    }

    impl WorkspaceCatalogStore for MockCatalogStore {
        fn load(&self) -> io::Result<WorkspaceCatalog> {
            Ok(self.catalog.lock().unwrap().clone())
        }

        fn save(&self, catalog: &WorkspaceCatalog) -> io::Result<()> {
            *self.catalog.lock().unwrap() = catalog.clone();
            Ok(())
        }
    }

    #[test]
    fn test_open_workspace_success_and_updates_catalog() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let path = Path::new("/tmp/existing_ws");

        let ws = Workspace::new("Existing Project");
        repo.save(&ws, path).unwrap();

        let use_case = OpenWorkspace::new(repo, catalog_store.clone());
        let opened = use_case.execute(path).unwrap();

        assert_eq!(opened.id, ws.id);
        let catalog = catalog_store.load().unwrap();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog.entries()[0].id, ws.id);
    }

    #[test]
    fn test_open_workspace_not_found() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let use_case = OpenWorkspace::new(repo, catalog_store);

        let path = Path::new("/tmp/does_not_exist");
        assert!(matches!(
            use_case.execute(path),
            Err(OpenWorkspaceError::NotFound(_))
        ));
    }
}
