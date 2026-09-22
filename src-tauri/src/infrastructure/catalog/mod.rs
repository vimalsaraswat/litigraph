pub mod file_store;

use std::{io, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{Workspace, WorkspaceId};

pub trait WorkspaceCatalogStore {
    fn load(&self) -> io::Result<WorkspaceCatalog>;
    fn save(&self, catalog: &WorkspaceCatalog) -> io::Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceEntry {
    pub id: WorkspaceId,
    pub name: String,
    pub path: PathBuf,
    pub last_opened: DateTime<Utc>,
}

impl WorkspaceEntry {
    pub fn new(workspace: &Workspace, path: PathBuf) -> Self {
        Self {
            id: workspace.id.clone(),
            name: workspace.name.clone(),
            path,
            last_opened: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceCatalog {
    workspaces: Vec<WorkspaceEntry>,
}

impl WorkspaceCatalog {
    pub fn add(&mut self, workspace: WorkspaceEntry) {
        self.remove(&workspace.id);
        self.workspaces.insert(0, workspace);
    }

    pub fn remove(&mut self, id: &WorkspaceId) {
        self.workspaces.retain(|w| &w.id != id);
    }

    pub fn find(&self, id: &WorkspaceId) -> Option<&WorkspaceEntry> {
        self.workspaces.iter().find(|w| &w.id == id)
    }

    pub fn mark_opened(&mut self, id: &WorkspaceId) {
        if let Some(index) = self.workspaces.iter().position(|w| &w.id == id) {
            let mut workspace = self.workspaces.remove(index);
            workspace.last_opened = Utc::now();
            self.workspaces.insert(0, workspace);
        }
    }

    pub fn entries(&self) -> &[WorkspaceEntry] {
        &self.workspaces
    }

    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }

    pub fn clear(&mut self) {
        self.workspaces.clear();
    }
}

impl IntoIterator for WorkspaceCatalog {
    type Item = WorkspaceEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.workspaces.into_iter()
    }
}

impl<'a> IntoIterator for &'a WorkspaceCatalog {
    type Item = &'a WorkspaceEntry;
    type IntoIter = std::slice::Iter<'a, WorkspaceEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.workspaces.iter()
    }
}

impl<'a> IntoIterator for &'a mut WorkspaceCatalog {
    type Item = &'a mut WorkspaceEntry;
    type IntoIter = std::slice::IterMut<'a, WorkspaceEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.workspaces.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Workspace;

    #[test]
    fn test_catalog_add_and_deduplicate() {
        let mut catalog = WorkspaceCatalog::default();
        assert!(catalog.is_empty());

        let ws1 = Workspace::new("Project A");
        let ws2 = Workspace::new("Project B");

        let entry1 = WorkspaceEntry::new(&ws1, PathBuf::from("/path/a"));
        let entry2 = WorkspaceEntry::new(&ws2, PathBuf::from("/path/b"));

        catalog.add(entry1.clone());
        catalog.add(entry2.clone());

        assert_eq!(catalog.len(), 2);
        assert_eq!(catalog.entries()[0].id, ws2.id);
        assert_eq!(catalog.entries()[1].id, ws1.id);

        // Re-adding ws1 should move it to the front
        catalog.add(entry1.clone());
        assert_eq!(catalog.len(), 2);
        assert_eq!(catalog.entries()[0].id, ws1.id);
    }

    #[test]
    fn test_catalog_remove_and_find() {
        let mut catalog = WorkspaceCatalog::default();
        let ws = Workspace::new("Test Project");
        let entry = WorkspaceEntry::new(&ws, PathBuf::from("/path/test"));

        catalog.add(entry.clone());
        assert!(catalog.find(&ws.id).is_some());

        catalog.remove(&ws.id);
        assert!(catalog.find(&ws.id).is_none());
        assert!(catalog.is_empty());
    }

    #[test]
    fn test_catalog_mark_opened() {
        let mut catalog = WorkspaceCatalog::default();
        let ws1 = Workspace::new("One");
        let ws2 = Workspace::new("Two");

        catalog.add(WorkspaceEntry::new(&ws1, PathBuf::from("/1")));
        catalog.add(WorkspaceEntry::new(&ws2, PathBuf::from("/2")));

        // Currently ws2 is first, ws1 is second
        assert_eq!(catalog.entries()[0].id, ws2.id);

        catalog.mark_opened(&ws1.id);
        assert_eq!(catalog.entries()[0].id, ws1.id);
    }
}
