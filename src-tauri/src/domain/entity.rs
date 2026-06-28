#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub properties: Vec<crate::domain::Property>,
    pub position: crate::domain::Position,
}
