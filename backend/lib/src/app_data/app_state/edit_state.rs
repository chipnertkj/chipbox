pub mod audio_engine;

use self::audio_engine::AudioEngine;
use crate::common;
use common::{Project, Settings};

pub struct EditState {
    pub project: Project,
    pub audio_engine: AudioEngine,
}

impl EditState {
    /// # Errors
    /// Failure if encountered problems while creating the audio engine.
    pub fn create_project(
        settings: &Settings,
        name: String,
    ) -> Result<Self, audio_engine::Error> {
        let audio_engine = AudioEngine::from_settings(&settings.audio_engine)?;
        let editor = Self {
            audio_engine,
            project: Project::new(name),
        };
        Ok(editor)
    }

    pub fn play_stream(&mut self) -> Result<(), cpal::PlayStreamError> {
        self.audio_engine.play()
    }
}
