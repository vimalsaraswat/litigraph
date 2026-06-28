#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub entities: Vec<crate::domain::Entity>,
    pub relationships: Vec<crate::domain::Relationship>,
}
