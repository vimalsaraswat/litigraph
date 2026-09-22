use std::{io::Result, path::Path};

use crate::{
    domain::{Workspace, WorkspaceRepository},
    infrastructure::catalog::{WorkspaceCatalogStore, WorkspaceEntry},
};

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

    pub fn execute(&self, name: impl Into<String>, workspace_path: &Path) -> Result<Workspace> {
        let workspace = Workspace::new(name);

        self.repository.save(&workspace, workspace_path)?;

        let mut catalog = self.catalog_store.load()?;

        catalog.add(WorkspaceEntry::new(
            &workspace,
            workspace_path.to_path_buf(),
        ));

        self.catalog_store.save(&catalog)?;

        Ok(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[derive(Default, Clone)]
    struct MockWorkspaceRepository {
        workspaces: Arc<Mutex<HashMap<String, Workspace>>>,
    }

    impl WorkspaceRepository for MockWorkspaceRepository {
        fn save(&self, workspace: &Workspace, path: &Path) -> std::io::Result<()> {
            self.workspaces
                .lock()
                .unwrap()
                .insert(path.to_string_lossy().to_string(), workspace.clone());
            Ok(())
        }

        fn load(&self, path: &Path) -> std::io::Result<Workspace> {
            self.workspaces
                .lock()
                .unwrap()
                .get(&path.to_string_lossy().to_string())
                .cloned()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Not found"))
        }

        fn delete(&self, path: &Path) -> std::io::Result<()> {
            self.workspaces
                .lock()
                .unwrap()
                .remove(&path.to_string_lossy().to_string());
            Ok(())
        }
    }

    #[derive(Default, Clone)]
    struct MockCatalogStore {
        catalog: Arc<Mutex<crate::infrastructure::catalog::WorkspaceCatalog>>,
    }

    impl WorkspaceCatalogStore for MockCatalogStore {
        fn load(&self) -> std::io::Result<crate::infrastructure::catalog::WorkspaceCatalog> {
            Ok(self.catalog.lock().unwrap().clone())
        }

        fn save(
            &self,
            catalog: &crate::infrastructure::catalog::WorkspaceCatalog,
        ) -> std::io::Result<()> {
            *self.catalog.lock().unwrap() = catalog.clone();
            Ok(())
        }
    }

    #[test]
    fn test_create_workspace_use_case() {
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
}
