pub struct WorkspaceApplication;

impl WorkspaceApplication {
    pub fn create(name: impl Into<String>) -> Workspace {
        Workspace::new(name)
    }

    pub fn rename(workspace: &mut Workspace, name: impl Into<String>) {
        workspace.rename(name);
    }
}
