use super::{Workspace, WorkspaceId};

pub trait WorkspaceRepository {
    fn save(&self, workspace: &Workspace);

    fn load(&self, id: &WorkspaceId) -> Option<Workspace>;

    fn delete(&self, id: &WorkspaceId);

    fn list(&self) -> Vec<Workspace>;
}
