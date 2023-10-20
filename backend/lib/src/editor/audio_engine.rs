use chipbox_common as common;
use common::audio_engine::Settings;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HostId(cpal::HostId);

impl From<HostId> for cpal::HostId {
    fn from(val: HostId) -> Self {
        let HostId(val) = val;
        val
    }
}

#[derive(Debug)]
pub enum HostIdStrError {
    Invalid(String),
    WrongPlatform(String),
}

impl std::fmt::Display for HostIdStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostIdStrError::Invalid(s) => {
                write!(f, "`{s}` is not a valid host id")
            }
            HostIdStrError::WrongPlatform(s) => {
                write!(
                    f,
                    "host `{s}` is not supported on this platform ({platform})",
                    platform = std::env::consts::OS
                )
            }
        }
    }
}

impl std::error::Error for HostIdStrError {}

impl FromStr for HostId {
    type Err = HostIdStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const WASAPI: &str = "WASAPI";
        const ASIO: &str = "ASIO";
        const JACK: &str = "JACK";
        const ALSA: &str = "ALSA";
        const CORE_AUDIO: &str = "CoreAudio";
        const OBOE: &str = "Oboe";
        const EMSCRIPTEN: &str = "Emscripten";
        const WEBAUDIO: &str = "WebAudio";
        const NULL: &str = "Null";
        const SUPPORTED_BACKENDS: [&str; 9] = [
            WASAPI, ASIO, JACK, ALSA, CORE_AUDIO, OBOE, EMSCRIPTEN, WEBAUDIO,
            NULL,
        ];

        match s {
            #[cfg(target_os = "windows")]
            WASAPI => Ok(Self(cpal::HostId::Wasapi)),
            #[cfg(target_os = "windows")]
            ASIO => Ok(Self(cpal::HostId::Asio)),
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            ))]
            JACK => Ok(Self(cpal::HostId::Jack)),
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd"
            ))]
            ALSA => Ok(Self(cpal::HostId::Alsa)),
            #[cfg(target_os = "macos")]
            CORE_AUDIO => Ok(Self(cpal::HostId::CoreAudio)),
            #[cfg(target_os = "android")]
            OBOE => Ok(Self(cpal::HostId::Oboe)),
            #[cfg(target_os = "emscripten")]
            EMSCRIPTEN => Ok(Self(cpal::HostId::Emscripten)),
            #[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
            WEBAUDIO => Ok(Self(cpal::HostId::WebAudio)),
            #[cfg(not(any(
                windows,
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "emscripten",
                target_os = "android",
                all(target_arch = "wasm32", feature = "wasm-bindgen"),
            )))]
            NULL => Ok(Self(cpal::HostId::Null)),
            _ => {
                if SUPPORTED_BACKENDS.contains(&s) {
                    Err(HostIdStrError::WrongPlatform(s.into()))
                } else {
                    Err(HostIdStrError::Invalid(s.into()))
                }
            }
        }
    }
}

pub struct AudioEngine {
    host_id: HostId,
    host: cpal::Host,
}

impl AudioEngine {
    pub fn from_settings(settings: &Settings) -> Result<Self, HostIdStrError> {
        let Settings {
            host_id: host_id_str,
        } = settings;
        let host_id = HostId::from_str(host_id_str)?;
        let host =
            cpal::host_from_id(host_id.into()).expect("unable to get host");
        Ok(Self { host_id, host })
    }
}
