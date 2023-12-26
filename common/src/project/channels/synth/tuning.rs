#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// A tuning system.
/// Defines the available pitches and their corresponding frequencies.
pub struct Tuning {
    pub pitches: Vec<Pitch>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// A named pitch value in the tuning system.
pub struct Pitch {
    pub name: String,
    pub frequency: f64,
}
