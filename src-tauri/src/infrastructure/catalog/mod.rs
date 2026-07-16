pub mod file_store;

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{Workspace, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
}
