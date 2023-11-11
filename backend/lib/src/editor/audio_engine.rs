use self::device::Error;
use self::error::{ResetStreamError, SettingsError};
use self::host_id::HostId;
use self::stream_config::{SampleFormat, StreamConfig};
use self::stream_handle::StreamHandle;
use chipbox_common as common;
use common::audio_engine::{SelectedDevice, Settings};
use cpal::traits::{DeviceTrait as _, HostTrait, StreamTrait as _};

mod device;
mod error;
mod host_id;
mod stream_config;
mod stream_handle;

/// Represents a configuration of the AudioEngine.
pub struct AudioEngineConfig {
    pub host_id: HostId,
    pub selected_device: SelectedDevice,
    pub output_stream: StreamConfig,
    pub playing: bool,
}

pub struct AudioEngine {
    host: cpal::Host,
    output_device: cpal::Device,
    output_stream_handle: StreamHandle,
    config: AudioEngineConfig,
}

impl AudioEngine {
    /// Creates an `AudioEngine` instance from the given audio engine `Settings`.
    pub fn from_settings(settings: &Settings) -> Result<Self, SettingsError> {
        // Read HostId and open Host.
        let host_id = HostId::try_from(&settings.host)
            .map_err(SettingsError::HostIdParse)?;
        let host =
            cpal::host_from_id(host_id.into()).expect("unable to get host");

        // Open output device.
        let output_device = Self::output_device(&host, &settings.output_device)
            .map_err(|e| {
                SettingsError::InvalidStreamConfig(
                    stream_config::Error::Device(e),
                )
            })?;

        // Create output stream.
        let output_stream_config =
            StreamConfig::from_settings(&settings.output_stream_config)
                .map_err(SettingsError::StreamConfigParse)?;
        let output_stream =
            Self::create_output_stream(&output_device, &output_stream_config)
                .map_err(SettingsError::InvalidStreamConfig)?;
        let output_stream_handle = Self::add_thread_local_stream(output_stream);

        // Prepare engine config.
        let config = AudioEngineConfig {
            host_id,
            selected_device: settings.output_device.clone(),
            output_stream: output_stream_config,
            playing: false,
        };

        // Construct.
        tracing::info!("created audio engine from settings: `{:?}`", settings);
        Ok(Self {
            host,
            output_device,
            output_stream_handle,
            config,
        })
    }

    /// Plays the output stream.
    pub fn play(&mut self) -> Result<(), cpal::PlayStreamError> {
        tracing::info!("starting audio engine");
        let result = self.with_output_stream(|stream| stream.play());
        if result.is_ok() {
            // Update current config.
            self.config.playing = true;
        }
        result
    }

    /// Pauses the output stream.
    pub fn pause(&mut self) -> Result<(), cpal::PauseStreamError> {
        tracing::info!("pausing audio engine");
        let result = self.with_output_stream(|stream| stream.pause());
        if result.is_ok() {
            // Update current config.
            self.config.playing = false;
        }
        result
    }

    /// Resets the output device and stream.
    pub fn reset_output_device(&mut self) -> Result<(), ResetStreamError> {
        tracing::info!("resetting output device");
        // Reset device.
        self.output_device =
            Self::output_device(&self.host, &self.config.selected_device)
                .expect("unable to reset output device");
        // Reset stream.
        self.reset_output_stream()
    }

    /// Resets the output stream.
    pub fn reset_output_stream(&mut self) -> Result<(), ResetStreamError> {
        tracing::info!("resetting output stream");
        // This replaces the stream handle with a new one.
        // The handle removes the old stream from the thread-local
        // storage on drop.
        self.output_stream_handle = Self::add_thread_local_stream(
            Self::create_output_stream(
                &self.output_device,
                &self.config.output_stream,
            )
            .map_err(ResetStreamError::Config)?,
        );
        // Play if previously configured to.
        if self.config.playing {
            self.play()
                .map_err(ResetStreamError::Play)?;
        }
        Ok(())
    }

    // Helper function for adding a stream to the thread-local storage.
    fn add_thread_local_stream(stream: cpal::Stream) -> StreamHandle {
        stream_handle::with_streams_mut(|streams| {
            streams.insert(stream)
            // Unlikely (idx len is usize^2), but panic on overflow.
            .expect(
                "unable to insert stream due to generational index overflow",
            )
        })
    }

    // Helper function for accessing the output stream from thread-local storage.
    fn with_output_stream<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&cpal::Stream) -> T,
    {
        stream_handle::with_streams(|streams| {
            let stream = streams
                .get(&self.output_stream_handle)
                .expect("stream not found");
            f(stream)
        })
    }

    // Helper function for creating an output stream based on the given `StreamConfig`.
    fn create_output_stream(
        output_device: &cpal::Device,
        expected_stream_config: &StreamConfig,
    ) -> Result<cpal::Stream, stream_config::Error> {
        // Get a supported config.
        let supported_config = match expected_stream_config {
            // Get default output device config.
            StreamConfig::Default => output_device
                .default_output_config()
                .map_err(|e| {
                    stream_config::Error::Device(device::Error::Other(
                        Box::new(e),
                    ))
                }),
            // Read custom output stream config.
            StreamConfig::Custom {
                sample_format,
                sample_rate,
                channels,
            } => {
                let SampleFormat(sample_format) = sample_format;
                // Retrieve all supported configs.
                let supported_configs = output_device
                    .supported_output_configs()
                    .map_err(|x| {
                        stream_config::Error::Device(Error::Disconnected(
                            Box::new(x),
                        ))
                    })?;
                // Find a config that matches the requested parameters.
                let supported_config = supported_configs
                    .into_iter()
                    .find(|x| {
                        x.channels() == *channels
                            && x.min_sample_rate() <= *sample_rate
                            && x.max_sample_rate() >= *sample_rate
                            && x.sample_format() == *sample_format
                    })
                    .ok_or(stream_config::Error::NoMatchingConfig)?
                    .with_sample_rate(*sample_rate);
                // Ok!
                Ok(supported_config)
            }
        }?;
        // Build output stream.
        let stream = output_device
            .build_output_stream_raw(
                &supported_config.config(),
                supported_config.sample_format(),
                |_, _| {},
                |_| {},
                None,
            )
            .map_err(|x| stream_config::Error::Other(Box::new(x)))?;
        // Ok!
        Ok(stream)
    }

    // Helper function for opening an output device based on the given device selector.
    fn output_device(
        host: &cpal::Host,
        device_selection: &SelectedDevice,
    ) -> Result<cpal::Device, Error> {
        match &device_selection {
            // Open default output device.
            SelectedDevice::Default => host
                .default_output_device()
                .ok_or(Error::NoDefault),
            // Open output device with matching name.
            SelectedDevice::Named(name) => host
                .output_devices()
                .map_err(|x| Error::Other(Box::new(x)))?
                .try_find(|d| {
                    Ok(&d
                        .name()
                        .map_err(|x| Error::Other(Box::new(x)))?
                        == name)
                })?
                .ok_or(Error::NoMatch),
        }
    }
}

impl TryFrom<&Settings> for AudioEngine {
    type Error = SettingsError;
    fn try_from(value: &Settings) -> Result<Self, Self::Error> {
        Self::from_settings(value)
    }
}
