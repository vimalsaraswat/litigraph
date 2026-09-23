use std::{
    fmt, io,
    path::{Path, PathBuf},
};

use crate::domain::{
    catalog::{WorkspaceCatalogStore, WorkspaceEntry},
    workspace::{Workspace, WorkspaceRepository},
};

#[derive(Debug)]
pub enum CreateWorkspaceError {
    EmptyName,
    AlreadyExists(PathBuf),
    Repository(io::Error),
    Catalog(io::Error),
}

impl fmt::Display for CreateWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Workspace name cannot be empty"),
            Self::AlreadyExists(path) => {
                write!(f, "Workspace already exists at path: {}", path.display())
            }
            Self::Repository(err) => write!(f, "Repository error: {}", err),
            Self::Catalog(err) => write!(f, "Catalog error: {}", err),
        }
    }
}

impl std::error::Error for CreateWorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Repository(err) | Self::Catalog(err) => Some(err),
            _ => None,
        }
    }
}

impl From<CreateWorkspaceError> for io::Error {
    fn from(err: CreateWorkspaceError) -> Self {
        match err {
            CreateWorkspaceError::EmptyName => {
                io::Error::new(io::ErrorKind::InvalidInput, err.to_string())
            }
            CreateWorkspaceError::AlreadyExists(_) => {
                io::Error::new(io::ErrorKind::AlreadyExists, err.to_string())
            }
            CreateWorkspaceError::Repository(e) | CreateWorkspaceError::Catalog(e) => e,
        }
    }
}

pub struct CreateWorkspace<R, C>
where
    R: WorkspaceRepository,
    C: WorkspaceCatalogStore,
{
    repository: R,
    catalog_store: C,
}

impl<R, C> CreateWorkspace<R, C>
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

    pub fn execute(
        &self,
        name: impl Into<String>,
        workspace_path: &Path,
    ) -> Result<Workspace, CreateWorkspaceError> {
        let name = name.into();
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err(CreateWorkspaceError::EmptyName);
        }

        if self.repository.exists(workspace_path) {
            return Err(CreateWorkspaceError::AlreadyExists(
                workspace_path.to_path_buf(),
            ));
        }

        let workspace = Workspace::new(trimmed_name);

        self.repository
            .save(&workspace, workspace_path)
            .map_err(CreateWorkspaceError::Repository)?;

        let mut catalog = self
            .catalog_store
            .load()
            .map_err(CreateWorkspaceError::Catalog)?;

        catalog.add(WorkspaceEntry::new(
            &workspace,
            workspace_path.to_path_buf(),
        ));

        self.catalog_store
            .save(&catalog)
            .map_err(CreateWorkspaceError::Catalog)?;

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

        fn exists(&self, path: &Path) -> bool {
            self.workspaces.lock().unwrap().contains_key(path)
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
    fn test_create_workspace_success() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let use_case = CreateWorkspace::new(repo.clone(), catalog_store.clone());

        let path = Path::new("/tmp/test_ws");
        let ws = use_case.execute("My Test WS", path).unwrap();

        assert_eq!(ws.name, "My Test WS");
        assert!(repo.load(path).is_ok());

        let catalog = catalog_store.load().unwrap();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog.entries()[0].id, ws.id);
    }

    #[test]
    fn test_create_workspace_rejects_empty_name() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let use_case = CreateWorkspace::new(repo, catalog_store);

        let path = Path::new("/tmp/test_ws");
        assert!(matches!(
            use_case.execute("   ", path),
            Err(CreateWorkspaceError::EmptyName)
        ));
    }

    #[test]
    fn test_create_workspace_rejects_if_already_exists() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let use_case = CreateWorkspace::new(repo.clone(), catalog_store.clone());

        let path = Path::new("/tmp/test_ws");
        use_case.execute("First WS", path).unwrap();

        let result = use_case.execute("Duplicate WS", path);
        assert!(matches!(
            result,
            Err(CreateWorkspaceError::AlreadyExists(_))
        ));
    }
}
