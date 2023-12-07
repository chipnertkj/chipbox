use super::themes::ThemeSelector;
use super::Theme;
use std::collections::HashMap;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default,
)]
pub struct UserThemes {
    inner: HashMap<String, Theme>,
}

impl UserThemes {
    pub fn get(&self, selector: &ThemeSelector) -> Option<&Theme> {
        match selector {
            ThemeSelector::Default(selector) => Some(super::get(selector)),
            ThemeSelector::Custom(name) => self.inner.get(name),
        }
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Theme> {
        self.inner.get_mut(name)
    }

    pub fn insert(&mut self, name: String, theme: Theme) {
        self.inner.insert(name, theme);
    }

    pub fn remove(&mut self, name: &str) {
        self.inner.remove(name);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Theme> {
        self.inner.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<String, Theme> {
        self.inner.iter_mut()
    }
}

impl IntoIterator for UserThemes {
    type Item = (String, Theme);
    type IntoIter = std::collections::hash_map::IntoIter<String, Theme>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
