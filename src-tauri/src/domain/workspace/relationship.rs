use super::{EntityId, RelationshipId, RelationshipType};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Relationship {
    pub id: RelationshipId,
    pub source: EntityId,
    pub target: EntityId,
    pub relationship_type: RelationshipType,
}

impl Relationship {
    pub fn new(source: EntityId, target: EntityId, relationship_type: impl Into<String>) -> Self {
        Self {
            id: RelationshipId(uuid::Uuid::new_v4().to_string()),
            source,
            target,
            relationship_type: RelationshipType(relationship_type.into()),
        }
    }

    pub fn change_type(&mut self, relationship_type: impl Into<String>) {
        self.relationship_type = RelationshipType(relationship_type.into());
    }

    pub fn connects(&self, entity_id: &EntityId) -> bool {
        &self.source == entity_id || &self.target == entity_id
    }
}
