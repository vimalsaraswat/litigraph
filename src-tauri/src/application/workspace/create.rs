use std::{io::Result, path::Path};

use crate::{
    domain::{Workspace, WorkspaceRepository},
    infrastructure::catalog::{file_store::FileWorkspaceCatalogStore, WorkspaceEntry},
};

pub struct CreateWorkspace<R>
where
    R: WorkspaceRepository,
{
    repository: R,
    catalog_store: FileWorkspaceCatalogStore,
}

impl<R> CreateWorkspace<R>
where
    R: WorkspaceRepository,
{
    pub fn new(repository: R, catalog_store: FileWorkspaceCatalogStore) -> Self {
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
