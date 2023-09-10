#[derive(
    serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Clone, Copy,
)]
#[serde(tag = "type")]
pub enum QueryingBackend {
    #[default]
    Requesting,
    ReadingSettings,
}
