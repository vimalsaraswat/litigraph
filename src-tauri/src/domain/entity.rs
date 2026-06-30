use crate::domain::{Position, Property};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EntityId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntityType {
    Person,
    Evidence,
    Event,
    Argument,
    Court,
    Note,
    Custom(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub title: String,
    pub entity_type: EntityType,
    pub properties: Vec<Property>,
    pub position: Position,
}

impl Entity {
    pub fn new(title: impl Into<String>, entity_type: EntityType, position: Position) -> Self {
        Self {
            id: EntityId(uuid::Uuid::new_v4().to_string()),
            title: title.into(),
            entity_type,
            properties: Vec::new(),
            position,
        }
    }

    pub fn rename(&mut self, title: impl Into<String>) {
        let title = title.into();
        self.title = title;
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    pub fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    pub fn remove_property(&mut self, key: &str) {
        self.properties.retain(|p| p.key != key);
    }

    pub fn property(&self, key: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.key == key)
    }
}
