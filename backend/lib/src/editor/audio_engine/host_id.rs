use chipbox_common as common;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostId(cpal::HostId);

impl From<HostId> for cpal::HostId {
    fn from(val: HostId) -> Self {
        let HostId(val) = val;
        val
    }
}

#[derive(Debug)]
pub enum ParseError {
    Invalid(String),
    WrongPlatform(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Invalid(s) => {
                write!(f, "`{s}` is not convertible to a valid host id")
            }
            ParseError::WrongPlatform(s) => {
                write!(
                    f,
                    "host `{s}` is not supported on this platform ({platform})",
                    platform = std::env::consts::OS
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for HostId {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const WASAPI: &str = "wasapi";
        const ASIO: &str = "asio";
        const JACK: &str = "jack";
        const ALSA: &str = "alsa";
        const CORE_AUDIO: &str = "coreaudio";
        const OBOE: &str = "oboe";
        const EMSCRIPTEN: &str = "emscripten";
        const WEBAUDIO: &str = "webaudio";
        const NULL: &str = "null";
        const SUPPORTED_BACKENDS: [&str; 9] = [
            WASAPI, ASIO, JACK, ALSA, CORE_AUDIO, OBOE, EMSCRIPTEN, WEBAUDIO,
            NULL,
        ];

        match s.to_lowercase().as_str() {
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
                    Err(ParseError::WrongPlatform(s.into()))
                } else {
                    Err(ParseError::Invalid(s.into()))
                }
            }
        }
    }
}

static DEFAULT_HOST_ID: once_cell::sync::Lazy<HostId> =
    once_cell::sync::Lazy::new(|| HostId(cpal::default_host().id()));

impl TryFrom<&common::settings::audio_engine::SelectedHost> for HostId {
    type Error = ParseError;

    fn try_from(
        value: &common::settings::audio_engine::SelectedHost,
    ) -> Result<Self, Self::Error> {
        match value {
            common::settings::audio_engine::SelectedHost::Default => {
                Ok(*DEFAULT_HOST_ID)
            }
            common::settings::audio_engine::SelectedHost::Named(name) => {
                Self::from_str(name)
            }
        }
    }
}
