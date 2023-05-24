#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Mode {
    Borderless {
        monitor_handle_serialized: super::MonitorHandleSerialized,
    },
    Fullscreen {
        monitor_handle_serialized: super::MonitorHandleSerialized,
        video_mode_serialized: super::VideoModeSerialized,
    },
    Maximized,
    Minimized,
}
