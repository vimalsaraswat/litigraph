#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelationshipId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RelationshipType {
    Supports,
    References,
    Contradicts,
    Represents,
    RelatedTo,
    Custom(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Relationship {
    pub id: RelationshipId,
    pub source: crate::domain::EntityId,
    pub target: crate::domain::EntityId,
    pub relationship_type: RelationshipType,
}
