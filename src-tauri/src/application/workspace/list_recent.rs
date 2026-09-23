use std::io;

use crate::domain::catalog::{WorkspaceCatalogStore, WorkspaceEntry};

pub struct ListRecentWorkspaces<C>
where
    C: WorkspaceCatalogStore,
{
    catalog_store: C,
}

impl<C> ListRecentWorkspaces<C>
where
    C: WorkspaceCatalogStore,
{
    pub fn new(catalog_store: C) -> Self {
        Self { catalog_store }
    }

    pub fn execute(&self) -> io::Result<Vec<WorkspaceEntry>> {
        let catalog = self.catalog_store.load()?;
        Ok(catalog.entries().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{catalog::WorkspaceCatalog, Workspace};
    use std::{
        path::PathBuf,
        sync::{Arc, Mutex},
    };

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
    fn test_list_recent_workspaces() {
        let catalog_store = MockCatalogStore::default();
        let ws = Workspace::new("Recent One");
        let entry = WorkspaceEntry::new(&ws, PathBuf::from("/recent/1"));

        let mut catalog = WorkspaceCatalog::default();
        catalog.add(entry.clone());
        catalog_store.save(&catalog).unwrap();

        let use_case = ListRecentWorkspaces::new(catalog_store);
        let list = use_case.execute().unwrap();

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, ws.id);
    }
}
