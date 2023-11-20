use crate::ConfiguredState;
use chipbox_common as common;
use common::project::ProjectMeta;

pub mod audio_engine;

#[derive(Debug)]
pub enum Error {
    AudioEngine(audio_engine::Error),
    PlayStream {
        editor: Box<Editor>,
        e: cpal::PlayStreamError,
    },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AudioEngine(e) => write!(f, "unable to start editor: {e}"),
            Self::PlayStream { e, .. } => {
                write!(f, "unable to play stream: {e}")
            }
        }
    }
}

pub struct Editor {
    pub settings: common::Settings,
    pub project: common::Project,
    pub project_meta_opt: Option<ProjectMeta>,
    pub audio_engine: audio_engine::AudioEngine,
}

impl std::fmt::Debug for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Editor")
            .finish()
    }
}

impl Editor {
    /// Creates an `Editor` instance from the given app `Settings`.
    pub fn from_settings(settings: common::Settings) -> Result<Self, Error> {
        let audio_engine =
            audio_engine::AudioEngine::from_settings(&settings.audio_engine)
                .map_err(Error::AudioEngine)?;
        let mut editor = Self {
            audio_engine,
            settings,
            project: common::Project::default(),
            project_meta_opt: None,
        };
        let result = editor.audio_engine.play();
        match result {
            Ok(_) => Ok(editor),
            Err(e) => Err(Error::PlayStream {
                editor: Box::new(editor),
                e,
            }),
        }
    }
}

impl TryFrom<common::Settings> for Editor {
    type Error = Error;
    fn try_from(settings: common::Settings) -> Result<Self, Self::Error> {
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
