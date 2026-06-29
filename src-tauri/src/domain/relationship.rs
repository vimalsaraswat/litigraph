use crate::domain::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RelationshipId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub source: EntityId,
    pub target: EntityId,
    pub relationship_type: RelationshipType,
}

impl Relationship {
    pub fn new(source: EntityId, target: EntityId, relationship_type: RelationshipType) -> Self {
        Self {
            id: RelationshipId(uuid::Uuid::new_v4().to_string()),
            source,
            target,
            relationship_type,
        }
    }

    pub fn change_type(&mut self, relationship_type: RelationshipType) {
        self.relationship_type = relationship_type;
    }

    pub fn connects(&self, entity_id: &EntityId) -> bool {
        &self.source == entity_id || &self.target == entity_id
    }
}
