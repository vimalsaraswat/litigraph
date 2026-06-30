#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Property {
    pub key: String,
    pub value: String,
}
