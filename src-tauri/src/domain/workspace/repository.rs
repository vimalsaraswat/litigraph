use std::{io, path::Path};

use super::Workspace;

pub trait WorkspaceRepository {
    fn save(&self, workspace: &Workspace, path: &Path) -> io::Result<()>;

    fn load(&self, path: &Path) -> io::Result<Workspace>;

    fn delete(&self, path: &Path) -> io::Result<()>;
}
