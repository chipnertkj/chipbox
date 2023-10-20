use crate::ConfiguredState;
use chipbox_common as common;
use common::project::ProjectMeta;

mod audio_engine;

pub struct Editor {
    pub settings: common::Settings,
    pub project: common::Project,
    pub project_meta_opt: Option<ProjectMeta>,
    audio_engine: audio_engine::AudioEngine,
}

impl ConfiguredState for Editor {
    fn settings(&self) -> &common::Settings {
        &self.settings
    }

    fn settings_mut(&mut self) -> &mut common::Settings {
        &mut self.settings
    }
}

impl Editor {
    pub fn from_settings(settings: common::Settings) -> Self {
        Self {
            audio_engine: audio_engine::AudioEngine::from_settings(
                &settings.audio_engine,
            )
            .expect("failed to create audio engine"),
            settings,
            project: common::Project::default(),
            project_meta_opt: None,
        }
    }
}
