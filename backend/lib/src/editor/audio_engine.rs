use self::device::Error;
use self::host_id::HostId;
use self::stream_config::{SampleFormat, StreamConfig};
use self::stream_handle::StreamHandle;
use chipbox_common as common;
use common::audio_engine::{SelectedDevice, Settings};
use cpal::traits::{DeviceTrait as _, HostTrait, StreamTrait as _};

mod device;
mod host_id;
mod stream_config;
mod stream_handle;

#[derive(Debug)]
pub enum ParseSettingsError {
    StreamConfigParse(stream_config::ParseError),
    HostIdParse(host_id::ParseError),
    InvalidStreamConfig(stream_config::Error),
}

pub struct AudioEngine {
    host: cpal::Host,
    output_device: cpal::Device,
    output_stream_handle: StreamHandle,

    host_id: HostId,
    selected_device: SelectedDevice,
    expected_output_stream_config: StreamConfig,
}

impl AudioEngine {
    pub fn from_settings(
        settings: &Settings,
    ) -> Result<Self, ParseSettingsError> {
        let host_id = HostId::try_from(&settings.host)
            .map_err(ParseSettingsError::HostIdParse)?;
        let host =
            cpal::host_from_id(host_id.into()).expect("unable to get host");

        let output_device = Self::output_device(&host, &settings.output_device)
            .map_err(|e| {
                ParseSettingsError::InvalidStreamConfig(
                    stream_config::Error::Device(e),
                )
            })?;
        let expected_output_stream_config =
            StreamConfig::from_settings(&settings.output_stream_config)
                .map_err(ParseSettingsError::StreamConfigParse)?;

        let output_stream = Self::create_output_stream(
            &output_device,
            &expected_output_stream_config,
        )
        .map_err(ParseSettingsError::InvalidStreamConfig)?;

        let output_stream_handle =
            stream_handle::STREAMS.with_borrow_mut(|streams| {
                streams.insert(output_stream)
                    .expect("unable to insert stream due to generational index overflow")
            });

        Ok(Self {
            host_id,
            host,
            output_device,
            expected_output_stream_config,
            output_stream_handle,
            selected_device: settings.output_device.clone(),
        })
    }

    pub fn play(&self) -> Result<(), cpal::PlayStreamError> {
        self.with_output_stream(|stream| stream.play())
    }

    pub fn pause(&self) -> Result<(), cpal::PauseStreamError> {
        self.with_output_stream(|stream| stream.pause())
    }

    fn with_output_stream<V>(&self, f: impl FnOnce(&cpal::Stream) -> V) -> V {
        stream_handle::STREAMS.with_borrow(|streams| {
            let stream = streams
                .get(&self.output_stream_handle)
                .expect("stream not found");
            f(stream)
        })
    }

    fn create_output_stream(
        output_device: &cpal::Device,
        expected_stream_config: &StreamConfig,
    ) -> Result<cpal::Stream, stream_config::Error> {
        let supported_config = match expected_stream_config {
            StreamConfig::Default => output_device
                .default_output_config()
                .map_err(|e| {
                    stream_config::Error::Device(device::Error::Other(
                        Box::new(e),
                    ))
                }),
            StreamConfig::Custom {
                sample_format,
                sample_rate,
                channels,
            } => {
                let SampleFormat(sample_format) = sample_format;
                let supported_configs = output_device
                    .supported_output_configs()
                    .map_err(|x| {
                        stream_config::Error::Device(Error::Disconnected(
                            Box::new(x),
                        ))
                    })?;
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
                Ok(supported_config)
            }
        }?;

        let sample_format = supported_config.sample_format();
        let config = supported_config.config();

        let stream = output_device
            .build_output_stream_raw(
                &config,
                sample_format,
                |_, _| {},
                |_| {},
                None,
            )
            .map_err(|x| stream_config::Error::Other(Box::new(x)))?;

        Ok(stream)
    }

    fn output_device(
        host: &cpal::Host,
        device_settings: &SelectedDevice,
    ) -> Result<cpal::Device, Error> {
        match &device_settings {
            SelectedDevice::Default => host
                .default_output_device()
                .ok_or(Error::NoDefault),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_default_stream() {
        let settings = Settings::default();
        let audio_engine = AudioEngine::from_settings(&settings)
            .expect("unable to parse default config");
        println!("created audio engine");
        audio_engine
            .play()
            .expect("unable to start audio engine with default config");
        println!("audio engine running");
        audio_engine
            .pause()
            .expect("unable to stop audio engine");
        println!("audio engine stopped");
    }
}
