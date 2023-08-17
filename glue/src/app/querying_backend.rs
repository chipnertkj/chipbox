#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(tag = "type")]
pub enum QueryingBackend {
    #[default]
    Requesting,
    ReadingSettings,
}
