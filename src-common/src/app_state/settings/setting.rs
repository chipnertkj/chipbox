#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Setting<T> {
    name: &'static str,
    pub value: T,
}

impl<T> Setting<T> {
    pub fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
}
