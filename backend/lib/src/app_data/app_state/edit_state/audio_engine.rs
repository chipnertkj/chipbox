pub use self::error::{ResetDeviceError, ResetStreamError, SettingsError};

use self::buffer::{Buffer, BufferConfig, Consumer, MonoFrame, StereoFrame};
use self::host_id::HostId;
use self::stream_config::{SampleFormat, StreamConfig};
use self::stream_handle::StreamHandle;
use chipbox_common as common;
use common::audio_engine::{SelectedDevice, Settings};
use cpal::traits::{DeviceTrait as _, HostTrait, StreamTrait as _};
use cpal::Sample;
use rb::RbConsumer;

mod buffer;
mod device;
mod error;
mod host_id;
mod stream_config;

pub mod stream_handle;

/// Represents a configuration of the AudioEngine.
pub struct AudioEngineConfig {
    pub host_id: HostId,
    pub selected_device: SelectedDevice,
    pub output_stream: StreamConfig,
    pub output_buffer_config: BufferConfig,
    pub playing: bool,
}

pub struct AudioEngine {
    host: cpal::Host,
    output_device: cpal::Device,
    output_stream_handle: StreamHandle,
    config: AudioEngineConfig,
    output_buffer: Buffer,
}

#[derive(Debug)]
pub enum Error {
    Settings(SettingsError),
    HostUnavailable(cpal::HostUnavailable),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Settings(err) => Some(err),
            Self::HostUnavailable(err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Settings(err) => err.fmt(f),
            Self::HostUnavailable(err) => err.fmt(f),
        }
    }
}

impl AudioEngine {
    /// Creates an `AudioEngine` instance from the given audio engine `Settings`.
    pub fn from_settings(settings: &Settings) -> Result<Self, Error> {
        // Read HostId and open Host.
        let host_id = HostId::try_from(&settings.host)
            .map_err(|err| Error::Settings(SettingsError::HostIdParse(err)))?;
        let host = cpal::host_from_id(host_id.into())
            .map_err(Error::HostUnavailable)?;

        // Open output device.
        let output_device = Self::output_device(&host, &settings.output_device)
            .map_err(|err| {
                Error::Settings(SettingsError::InvalidStreamConfig(
                    stream_config::Error::Device(err),
                ))
            })?;

        // Get output stream config.
        let output_stream_config =
            StreamConfig::from_settings(&settings.output_stream_config)
                .map_err(|err| {
                    Error::Settings(SettingsError::StreamConfigParse(err))
                })?;
        let supported_output_stream_config =
            Self::supported_output_stream_config(
                &output_device,
                &output_stream_config,
            )
            .map_err(|err| {
                Error::Settings(SettingsError::InvalidStreamConfig(err))
            })?;

        // Calculate frame count in buffer.
        let cpal::SampleRate(sample_rate) =
            supported_output_stream_config.sample_rate();
        let buffer_duration = settings
            .output_stream_config
            .buffer_duration();
        // Convert duration to number of frames.
        let buffer_length = (buffer_duration.as_secs_f64() / sample_rate as f64)
            .ceil() as usize;
        // Prepare buffer config.
        let output_buffer_config =
            match supported_output_stream_config.channels() {
                MonoFrame::CHANNEL_COUNT => BufferConfig::Mono {
                    length: buffer_length,
                },
                StereoFrame::CHANNEL_COUNT => BufferConfig::Stereo {
                    length: buffer_length,
                },
                n => {
                    return Err(Error::Settings(
                        SettingsError::InvalidStreamConfig(
                            stream_config::Error::UnsupportedChannelCount(n),
                        ),
                    ))
                }
            };

        // Prepare engine config.
        let config = AudioEngineConfig {
            host_id,
            selected_device: settings.output_device.clone(),
            output_buffer_config,
            output_stream: output_stream_config,
            playing: false,
        };

        // Construct buffer.
        let mut output_buffer =
            Buffer::from_config(&config.output_buffer_config);
        let consumer = output_buffer.consumer();

        // Create output stream.
        let output_stream = Self::create_output_stream(
            &output_device,
            &supported_output_stream_config,
            consumer,
        )
        .map_err(|err| {
            Error::Settings(SettingsError::InvalidStreamConfig(err))
        })?;
        let output_stream_handle = Self::add_thread_local_stream(output_stream);

        // Construct.
        tracing::info!("created audio engine from settings: `{:?}`", settings);
        Ok(Self {
            host,
            output_device,
            output_stream_handle,
            output_buffer,
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
    pub fn reset_output_device(&mut self) -> Result<(), ResetDeviceError> {
        tracing::info!("resetting output device");
        // Reset device.
        self.output_device =
            Self::output_device(&self.host, &self.config.selected_device)
                .map_err(ResetDeviceError::Device)?;
        // Reset stream.
        self.reset_output_stream()
            .map_err(ResetDeviceError::Stream)?;
        Ok(())
    }

    /// Resets the output stream.
    pub fn reset_output_stream(&mut self) -> Result<(), ResetStreamError> {
        tracing::info!("resetting output stream");
        let supported_output_stream_config =
            Self::supported_output_stream_config(
                &self.output_device,
                &self.config.output_stream,
            )
            .map_err(ResetStreamError::Config)?;
        // This replaces the stream handle with a new one.
        // The handle removes the old stream from the thread-local
        // storage on drop.
        self.output_stream_handle = Self::add_thread_local_stream(
            Self::create_output_stream(
                &self.output_device,
                &supported_output_stream_config,
                self.output_buffer.consumer(),
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
            // Unlikely (max index is usize^2-1).
            // Panic on overflow.
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
                // Panic. Why are you accessing an invalid stream?
                .expect("stream not found");
            f(stream)
        })
    }

    fn supported_output_stream_config(
        output_device: &cpal::Device,
        expected_stream_config: &StreamConfig,
    ) -> Result<cpal::SupportedStreamConfig, stream_config::Error> {
        // Get a supported config.
        let supported_config = match expected_stream_config {
            // Get default output device config.
            StreamConfig::Default => output_device
                .default_output_config()
                .map_err(|err| {
                    stream_config::Error::Device(device::Error::Other(
                        Box::new(err),
                    ))
                })?,
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
                        stream_config::Error::Device(
                            device::Error::Disconnected(Box::new(x)),
                        )
                    })?;
                // Find a config that matches the requested parameters.
                supported_configs
                    .into_iter()
                    .find(|x| {
                        x.channels() == *channels
                            && x.min_sample_rate() <= *sample_rate
                            && x.max_sample_rate() >= *sample_rate
                            && x.sample_format() == *sample_format
                    })
                    .ok_or(stream_config::Error::NoMatchingConfig)?
                    .with_sample_rate(*sample_rate)
            }
        };
        // Check if the channel count is supported.
        let channel_count = supported_config.channels();
        if !Buffer::SUPPORTED_CHANNEL_COUNTS.contains(&channel_count) {
            Err(stream_config::Error::UnsupportedChannelCount(channel_count))
        } else {
            Ok(supported_config)
        }
    }

    // Helper function for creating an output stream based on the given `StreamConfig`.
    fn create_output_stream(
        output_device: &cpal::Device,
        supported_config: &cpal::SupportedStreamConfig,
        consumer: Consumer,
    ) -> Result<cpal::Stream, stream_config::Error> {
        // Check if the channel count is supported.
        let channel_count = supported_config.channels();
        if !Buffer::SUPPORTED_CHANNEL_COUNTS.contains(&channel_count) {
            Err(stream_config::Error::UnsupportedChannelCount(channel_count))
        } else {
            // Build output stream.
            let stream = Self::build_output_stream(
                output_device,
                supported_config,
                consumer,
            )
            .map_err(|x| stream_config::Error::Other(Box::new(x)))?;
            // Ok!
            tracing::info!(
                "created output stream with config: {:?}",
                supported_config
            );
            Ok(stream)
        }
    }

    fn build_output_stream(
        output_device: &cpal::Device,
        supported_config: &cpal::SupportedStreamConfig,
        mut consumer: Consumer,
    ) -> Result<cpal::Stream, cpal::BuildStreamError> {
        let config = supported_config.config();
        let sample_format = supported_config.sample_format();
        let channel_count = supported_config.channels();
        match sample_format {
            // Special case: f64 matches the memory layout of our buffer.
            cpal::SampleFormat::F64 => output_device.build_output_stream(
                &config,
                move |data: &mut [f64], _info| {
                    Self::output_callback_f64(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            // Rest of the cases has to convert samples manually.
            cpal::SampleFormat::F32 => output_device.build_output_stream(
                &config,
                move |data: &mut [f32], _info| {
                    Self::output_callback::<f32>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::I8 => output_device.build_output_stream(
                &config,
                move |data: &mut [i8], _info| {
                    Self::output_callback::<i8>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::I16 => output_device.build_output_stream(
                &config,
                move |data: &mut [i16], _info| {
                    Self::output_callback::<i16>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::I32 => output_device.build_output_stream(
                &config,
                move |data: &mut [i32], _info| {
                    Self::output_callback::<i32>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::I64 => output_device.build_output_stream(
                &config,
                move |data: &mut [i64], _info| {
                    Self::output_callback::<i64>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::U8 => output_device.build_output_stream(
                &config,
                move |data: &mut [u8], _info| {
                    Self::output_callback::<u8>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::U16 => output_device.build_output_stream(
                &config,
                move |data: &mut [u16], _info| {
                    Self::output_callback::<u16>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::U32 => output_device.build_output_stream(
                &config,
                move |data: &mut [u32], _info| {
                    Self::output_callback::<u32>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            cpal::SampleFormat::U64 => output_device.build_output_stream(
                &config,
                move |data: &mut [u64], _info| {
                    Self::output_callback::<u64>(
                        data,
                        &mut consumer,
                        channel_count,
                    )
                },
                Self::output_error_callback,
                None,
            ),
            _ => todo!("unsupported sample format: {:?}", sample_format),
        }
    }

    fn output_callback_f64(
        data: &mut [f64],
        consumer: &mut Consumer,
        channel_count: cpal::ChannelCount,
    ) {
        match consumer {
            Consumer::Mono(consumer)
                if channel_count == MonoFrame::CHANNEL_COUNT =>
            {
                // reinterpret data as MonoFrame slice, as memory layout matches
                let data = unsafe {
                    std::slice::from_raw_parts_mut(
                        data.as_ptr() as *mut MonoFrame,
                        data.len(),
                    )
                };
                let _result = consumer.read(data);
            }
            Consumer::Stereo(consumer)
                if channel_count == StereoFrame::CHANNEL_COUNT =>
            {
                // reinterpret data as StereoFrame slice, as memory layout matches
                let data = unsafe {
                    std::slice::from_raw_parts_mut(
                        data.as_ptr() as *mut StereoFrame,
                        data.len(),
                    )
                };
                let _result = consumer.read(data);
            }
            _ => unreachable!(),
        }
    }

    fn output_callback<T>(
        data: &mut [T],
        consumer: &mut Consumer,
        channel_count: cpal::ChannelCount,
    ) where
        T: cpal::Sample + cpal::FromSample<f64>,
    {
        match consumer {
            Consumer::Mono(consumer)
                if channel_count == MonoFrame::CHANNEL_COUNT =>
            {
                for sample in data {
                    let read_frame = Default::default();
                    let _result = consumer.read(&mut [read_frame]);
                    *sample = read_frame.center.to_sample();
                }
            }
            Consumer::Stereo(consumer)
                if channel_count == StereoFrame::CHANNEL_COUNT =>
            {
                for frame in data.chunks_exact_mut(2) {
                    let read_frame = Default::default();
                    let _result = consumer.read(&mut [read_frame]);
                    frame[0] = read_frame.left.to_sample();
                    frame[1] = read_frame.right.to_sample();
                }
            }
            _ => unreachable!(),
        }
    }

    fn output_error_callback(err: cpal::StreamError) {
        tracing::error!("output stream error: {}", err);
    }

    // Helper function for opening an output device based on the given device selector.
    fn output_device(
        host: &cpal::Host,
        device_selection: &SelectedDevice,
    ) -> Result<cpal::Device, device::Error> {
        match &device_selection {
            // Open default output device.
            SelectedDevice::Default => host
                .default_output_device()
                .ok_or(device::Error::NoDefault),
            // Open output device with matching name.
            // In case of multiple matches, the first one is selected.
            // TODO: Do any of the platforms allow for duplicate names?
            SelectedDevice::Named(name) => host
                .output_devices()
                .map_err(|x| device::Error::Other(Box::new(x)))?
                .try_find(|d| {
                    Ok(&d
                        .name()
                        .map_err(|x| device::Error::Other(Box::new(x)))?
                        == name)
                })?
                .ok_or(device::Error::NoMatch),
        }
    }
}

impl TryFrom<&Settings> for AudioEngine {
    type Error = Error;
    fn try_from(value: &Settings) -> Result<Self, Self::Error> {
        Self::from_settings(value)
    }
}
