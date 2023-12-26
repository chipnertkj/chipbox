#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq,
)]
pub struct ModLayer {
    points: Vec<ModPoint>,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq,
)]
pub struct ModPoint {
    pub x: u32,
    pub y: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ModLayerMeta {
    pub name: String,
}
