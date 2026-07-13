mod entity;
mod relationship;
mod repository;
mod value_objects;

pub use entity::*;
pub use relationship::*;
pub use repository::*;
pub use value_objects::*;

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
        self.name = name.into();
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, id: &EntityId) {
        self.entities.retain(|e| &e.id != id);

        self.relationships
            .retain(|r| &r.source != id && &r.target != id);
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    pub fn remove_relationship(&mut self, id: &RelationshipId) {
        self.relationships.retain(|r| &r.id != id);
    }
}
