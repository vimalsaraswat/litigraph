use std::{fmt, io, path::Path};

use crate::domain::{catalog::WorkspaceCatalogStore, workspace::WorkspaceRepository};

#[derive(Debug)]
pub enum DeleteWorkspaceError {
    Repository(io::Error),
    Catalog(io::Error),
}

impl fmt::Display for DeleteWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(err) => write!(f, "Repository error: {}", err),
            Self::Catalog(err) => write!(f, "Catalog error: {}", err),
        }
    }
}

impl std::error::Error for DeleteWorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Repository(err) | Self::Catalog(err) => Some(err),
        }
    }
}

impl From<DeleteWorkspaceError> for io::Error {
    fn from(err: DeleteWorkspaceError) -> Self {
        match err {
            DeleteWorkspaceError::Repository(e) | DeleteWorkspaceError::Catalog(e) => e,
        }
    }
}

pub struct DeleteWorkspace<R, C>
where
    R: WorkspaceRepository,
    C: WorkspaceCatalogStore,
{
    repository: R,
    catalog_store: C,
}

impl<R, C> DeleteWorkspace<R, C>
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

    pub fn execute(&self, workspace_path: &Path) -> Result<(), DeleteWorkspaceError> {
        self.repository
            .delete(workspace_path)
            .map_err(DeleteWorkspaceError::Repository)?;

        let mut catalog = self
            .catalog_store
            .load()
            .map_err(DeleteWorkspaceError::Catalog)?;

        catalog.remove_by_path(workspace_path);

        self.catalog_store
            .save(&catalog)
            .map_err(DeleteWorkspaceError::Catalog)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        catalog::{WorkspaceCatalog, WorkspaceEntry},
        Workspace,
    };
    use std::{
        collections::HashMap,
        path::PathBuf,
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
    fn test_delete_workspace_removes_from_repo_and_catalog() {
        let repo = MockWorkspaceRepository::default();
        let catalog_store = MockCatalogStore::default();
        let path = PathBuf::from("/tmp/delete_ws");

        let ws = Workspace::new("To Delete");
        repo.save(&ws, &path).unwrap();

        let mut catalog = WorkspaceCatalog::default();
        catalog.add(WorkspaceEntry::new(&ws, path.clone()));
        catalog_store.save(&catalog).unwrap();

        let use_case = DeleteWorkspace::new(repo.clone(), catalog_store.clone());
        use_case.execute(&path).unwrap();

        assert!(repo.load(&path).is_err());
        let updated_catalog = catalog_store.load().unwrap();
        assert_eq!(updated_catalog.len(), 0);
    }
}
