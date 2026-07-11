pub mod entity;
pub mod relationship;
pub mod workspace;

pub use entity::{Entity, EntityId, EntityType};
pub use relationship::{Relationship, RelationshipId, RelationshipType};
pub use workspace::{Workspace, WorkspaceId};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Property {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
