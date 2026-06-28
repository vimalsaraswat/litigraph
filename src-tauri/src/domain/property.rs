#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Property {
    pub key: String,
    pub value: String,
}
