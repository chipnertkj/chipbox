use chipbox_common as common;

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
    pub project: common::Project,
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
    pub fn create_project(
        settings: &common::Settings,
        name: String,
    ) -> Result<Self, Error> {
        let audio_engine =
            audio_engine::AudioEngine::from_settings(&settings.audio_engine)
                .map_err(Error::AudioEngine)?;
        let mut editor = Self {
            audio_engine,
            project: common::Project::new(name),
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
