use crate::ConfiguredState;
use chipbox_common as common;
use common::project::ProjectMeta;

pub mod audio_engine;

pub struct Editor {
    pub settings: common::Settings,
    pub project: common::Project,
    pub project_meta_opt: Option<ProjectMeta>,
    pub audio_engine: audio_engine::AudioEngine,
}

impl Editor {
    /// Creates an `Editor` instance from the given app `Settings`.
    pub fn from_settings(settings: common::Settings) -> Self {
        let mut audio_engine =
            audio_engine::AudioEngine::from_settings(&settings.audio_engine)
                .expect("failed to create audio engine");
        audio_engine.play().unwrap();
        Self {
            audio_engine,
            settings,
            project: common::Project::default(),
            project_meta_opt: None,
        }
    }
}

impl From<common::Settings> for Editor {
    fn from(settings: common::Settings) -> Self {
        Self::from_settings(settings)
    }
}

impl ConfiguredState for Editor {
    fn settings(&self) -> &common::Settings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut common::Settings {
        &mut self.settings
    }
}
