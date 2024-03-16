use chipbox_common as common;

pub mod audio_engine;

pub struct Editor {
    pub project: common::Project,
    pub audio_engine: audio_engine::AudioEngine,
}

impl Editor {
    /// Creates an `Editor` with the supplied app `Settings`.
    /// # Errors
    /// Fails if unable to create the audio engine.
    pub fn create_project(
        settings: &common::Settings,
        name: String,
    ) -> Result<Self, audio_engine::Error> {
        let audio_engine =
            audio_engine::AudioEngine::from_settings(&settings.audio_engine)?;
        let editor = Self {
            audio_engine,
            project: common::Project::new(name),
        };
        Ok(editor)
    }

    pub fn play_stream(&mut self) -> Result<(), cpal::PlayStreamError> {
        self.audio_engine.play()
    }
}
