use crate::domain::{Entity, EntityId, Relationship, RelationshipId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub entities: Vec<Entity>,
    pub relationships: Vec<Relationship>,
}

impl Workspace {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: WorkspaceId(uuid::Uuid::new_v4().to_string()),
            name: name.into(),
            entities: Vec::new(),
            relationships: Vec::new(),
        }
    }

    pub fn rename(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(!name.trim().is_empty(), "workspace name cannot be empty");

        self.name = name;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, id: &EntityId) -> bool {
        let initial_len = self.entities.len();

        self.entities.retain(|e| &e.id != id);

        // Maintain aggregate consistency by removing connected relationships.
        self.relationships
            .retain(|r| &r.source != id && &r.target != id);

        initial_len != self.entities.len()
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        if self.relationships.iter().all(|r| r.id != relationship.id) {
            self.relationships.push(relationship);
        }
    }

    pub fn remove_relationship(&mut self, id: &RelationshipId) -> bool {
        let initial_len = self.relationships.len();

        self.relationships.retain(|r| &r.id != id);

        initial_len != self.relationships.len()
    }
}
