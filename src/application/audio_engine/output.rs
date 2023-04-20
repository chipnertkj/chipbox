use cpal::traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _};

pub struct Output {
    device_opt: Option<cpal::Device>,
    stream_opt: Option<cpal::Stream>,
    use_default_device: bool,
    use_default_stream: bool,
}

impl Output {
    pub fn new() -> Self {
        Output {
            device_opt: None,
            stream_opt: None,
            use_default_device: false,
            use_default_stream: false,
        }
    }

    fn build_stream<T, D, E>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        data_callback: D,
        error_callback: E,
    ) -> Result<cpal::Stream, cpal::BuildStreamError>
    where
        T: cpal::SizedSample,
        D: FnMut(&mut [T], &cpal::OutputCallbackInfo) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        device.build_output_stream(config, data_callback, error_callback, None)
    }

    pub fn play(&self) -> Result<(), cpal::PlayStreamError> {
        self.stream_opt
            .as_ref()
            .expect("expected self.stream_opt to be Some(_)")
            .play()
    }

    pub fn pause(&self) -> Result<(), cpal::PauseStreamError> {
        self.stream_opt
            .as_ref()
            .expect("expected self.stream_opt to be Some(_)")
            .pause()
    }

    pub fn recreate(&mut self, host: &cpal::Host) {
        todo!("recreate device");
        if self.device_opt.is_some() {
            self.recreate_stream()
        }
    }

    pub fn recreate_stream(&mut self) {
        let device = self
            .device_opt
            .as_ref()
            .expect("expected self.device_opt to be Some(_)");
        match self.use_default_stream {
            true => match device.default_output_config() {
                Ok(supported_config) => {
                    let stream_result = Self::build_stream(
                            &device,
                            &supported_config.config(),
                            |_: &mut [f32], _: &cpal::OutputCallbackInfo| {},
                            |_| {},
                        );
                        match stream_result {
                            Ok(stream) => {
                                todo!("log, update member var etc.")
                            }
                        }
                    }
                }
            },
        }
    }

    pub fn default(host: &cpal::Host) -> Self {
        match host.default_output_device() {
            Some(device) => {
                match device.default_output_config() {
                    Ok(supported_config) => {
                        let stream_result = Self::build_stream(
                            &device,
                            &supported_config.config(),
                            |_: &mut [f32], _: &cpal::OutputCallbackInfo| {},
                            |_| {},
                        );
                        match stream_result {
                            Ok(stream) => {
                                tracing::info!("Successfully opened a default output stream.");
                                Self {
                                    device_opt: Some(device),
                                    stream_opt: Some(stream),
                                    use_default_device: true,
                                    use_default_stream: true,
                                }
                            }
                            Err(cpal::BuildStreamError::DeviceNotAvailable) => {
                                tracing::error!("Device was disconnected during stream construction.");
                                tracing::warn!("Constructing null output.");
                                Self::new()
                            }
                            Err(cpal::BuildStreamError::StreamConfigNotSupported) => {
                                tracing::error!("Default config not supported by host.");
                                tracing::warn!("Constructing output without stream.");
                                Self {
                                    device_opt: Some(device),
                                    stream_opt: None,
                                    use_default_device: true,
                                    use_default_stream: false,
                                }
                            }
                            Err(e) => {
                                tracing::error!("Unable to construct output stream: {e}");
                                tracing::warn!("Constructing null output.");
                                Self::new()
                            }
                        }
                    }
                    Err(cpal::DefaultStreamConfigError::DeviceNotAvailable) => {
                        tracing::error!("Device was disconnected during default stream config detection.");
                        tracing::warn!("Constructing null output.");
                        Self::new()
                    }
                    Err(e) => {
                        tracing::error!(
                            "Unable to retrieve default stream config: {e}"
                        );
                        tracing::warn!("Constructing null output.");
                        Self::new()
                    }
                }
            }
            None => {
                tracing::warn!("Default output device not set in host.");
                tracing::warn!("Constructing null output.");
                Self::new()
            }
        }
    }
}
