mod fullscreen;
mod monitor_handle_serialized;
mod video_mode;

use crate::config::SerializedItemTrait as _;
use winit::{event_loop, window};

pub use fullscreen::Mode;
pub use monitor_handle_serialized::MonitorHandleSerialized;
pub use video_mode::VideoModeSerialized;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OsWindow {
    pub logical_size: winit::dpi::LogicalSize<f32>,
    pub outer_position_opt: Option<winit::dpi::PhysicalPosition<i32>>,
    pub fullscreen: Option<Mode>,
}

impl OsWindow {
    pub fn to_window<T>(
        &self,
        event_loop: &event_loop::EventLoop<T>,
        title: &str,
    ) -> window::Window {
        let size = self.logical_size;
        let fullscreen = self.deserialize_fullscreen(event_loop);
        tracing::info!("Building...");
        let builder = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_fullscreen(fullscreen);
        let window = builder
            .build(event_loop)
            .unwrap();
        tracing::info!("Ok!");

        if let Some(position) = self.outer_position_opt {
            window.set_outer_position(position);
        }
        window
    }

    fn deserialize_fullscreen<T>(
        &self,
        event_loop: &event_loop::EventLoop<T>,
    ) -> Option<window::Fullscreen> {
        match &self.fullscreen {
            Some(super::Mode::Borderless {
                monitor_handle_serialized,
            }) => match monitor_handle_serialized.deserialize(event_loop) {
                Ok(handle) => {
                    tracing::info!("Handle ok!");
                    Some(window::Fullscreen::Borderless(Some(handle)))
                }
                Err(e) => {
                    tracing::error!("{e}");
                    None
                }
            },
            Some(super::Mode::Fullscreen {
                monitor_handle_serialized,
                video_mode_serialized,
            }) => {
                let handle_result =
                    monitor_handle_serialized.deserialize(event_loop);
                match handle_result {
                    Ok(handle) => {
                        tracing::info!("Handle ok!");
                        match video_mode_serialized.deserialize(&handle) {
                            Ok(video_mode) => {
                                tracing::info!("Video mode ok!");
                                Some(window::Fullscreen::Exclusive(video_mode))
                            }
                            Err(e) => {
                                tracing::error!("{e}");
                                None
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("{e}");
                        None
                    }
                }
            }
            _ => None,
        }
    }
}

impl Default for OsWindow {
    fn default() -> Self {
        Self {
            outer_position_opt: None,
            logical_size: winit::dpi::LogicalSize::new(1280., 720.),
            fullscreen: None,
        }
    }
}
