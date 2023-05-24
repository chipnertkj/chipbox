use std::future;
use std::ops::DerefMut;

use crate::config::ConfigTrait as _;
use async_std::task;
use itertools::Itertools;
use winit::{dpi, event, event_loop, window};

mod audio_engine;
mod config;
mod renderer;
mod ui;

pub struct Chipbox {
    windows: Vec<window::Window>,
    ui_layout: ui::UiLayout,
    ui_layout_config: ui::UiLayoutConfig,
    audio_engine: audio_engine::AudioEngine,
}

impl Chipbox {
    pub fn new<T>(event_loop: &event_loop::EventLoop<T>) -> Self {
        let audio_engine_future =
            audio_engine::AudioEngine::enabled_load_config();
        let ui_layout_config_future =
            ui::UiLayoutConfig::load_or_default_tracing();

        let (audio_engine, ui_layout_config) = task::block_on(async {
            future::join!(audio_engine_future, ui_layout_config_future).await
        });

        let ui_layout = ui_layout_config.selected_or_default();

        // We add one since `UiLayout` also has a `main_os_window`.
        let titles = Self::construct_titles(
            ui_layout
                .other_os_windows
                .len()
                + 1,
        );

        let windows = ui_layout
            .os_windows()
            .collect_vec()
            .iter()
            .enumerate()
            .map(|(i, x)| x.to_window(event_loop, &titles[i]))
            .collect_vec();

        // Will not panic, as `UiLayout::os_windows()` always returns at least one window.
        windows[0].focus_window();

        Self {
            windows,
            ui_layout,
            ui_layout_config,
            audio_engine,
        }
    }

    fn construct_titles(count: usize) -> Vec<String> {
        const APP_NAME: &str = "Chipbox";
        const SEPARATOR: &str = "|";

        let title_base = match option_env!("CARGO_PKG_VERSION") {
            Some(version) => format!("{APP_NAME} {version}"),
            None => APP_NAME.into(),
        };

        (0..count)
            .map(|index| match index {
                0 => std::format!("{title_base} {SEPARATOR} Main Window"),
                _ => {
                    // We add one since we display window indices with 1-based numbering.
                    let index = index + 1;
                    std::format!("{title_base} {SEPARATOR} Window {index}")
                }
            })
            .collect_vec()
    }

    fn window_index(&self, window_id: window::WindowId) -> usize {
        self.windows
            .iter()
            .position(|x| x.id() == window_id)
            .expect("expected event loop to contain only chipbox-owned windows")
    }

    fn window(&self, window_id: window::WindowId) -> &window::Window {
        self.windows
            .iter()
            .find(|x| x.id() == window_id)
            .expect("expected event loop to contain only chipbox-owned windows")
    }

    fn window_mut(
        &mut self,
        window_id: window::WindowId,
    ) -> &mut window::Window {
        self.windows
            .iter_mut()
            .find(|x| x.id() == window_id)
            .expect("expected event loop to contain only chipbox-owned windows")
    }

    fn os_window_mut(
        &mut self,
        window_id: window::WindowId,
    ) -> &mut ui::OsWindow {
        let mut os_windows = self
            .ui_layout
            .os_windows_mut()
            .collect_vec();
        todo!("figure out how to not tick the borrow checker off...");
        let index = self.window_index(window_id);
        os_windows
            .get_mut(index)
            .expect("expected event loop to contain only chipbox-owned windows")
    }

    pub fn run<T>(mut self, event_loop: event_loop::EventLoop<T>) -> ! {
        event_loop.run(move |event, _target, control_flow| match event {
            event::Event::WindowEvent { window_id, event } => match event {
                event::WindowEvent::CloseRequested => {
                    self.on_close_requested(window_id, control_flow);
                }
                event::WindowEvent::Resized(physical_size) => {
                    self.on_resized(window_id, physical_size);
                }
                event::WindowEvent::Moved(outer_position) => {
                    // It's not mentioned in the documentation, but `WindowEvent::Moved::0`
                    // is actually the outer position of the window, at least on Windows (platform).
                    self.on_moved(window_id, outer_position);
                }
                _ => {}
            },
            event::Event::RedrawRequested(window_id) => {
                self.on_redraw_requested(window_id)
            }
            event::Event::LoopDestroyed => self.on_loop_destroyed(),
            _ => {}
        });
    }

    fn on_loop_destroyed(&mut self) {}

    fn on_redraw_requested(&mut self, window_id: window::WindowId) {
        let _window = self.window(window_id);
    }

    fn on_resized(
        &mut self,
        window_id: window::WindowId,
        physical_size: dpi::PhysicalSize<u32>,
    ) {
    }

    fn on_moved(
        &mut self,
        window_id: window::WindowId,
        outer_position: dpi::PhysicalPosition<i32>,
    ) {
        let os_window = self.os_window_mut(window_id);
        os_window.outer_position_opt = Some(outer_position)
    }

    fn on_close_requested(
        &mut self,
        window_id: window::WindowId,
        control_flow: &mut event_loop::ControlFlow,
    ) {
        tracing::info!("User requested window close.");

        match self.window_index(window_id) {
            0 => {
                tracing::info!("Close requested on Main Window. Exiting...");

                self.ui_layout_config
                    .set_preserved_layout_if_enabled(self.ui_layout.clone());

                let audio_engine_future = self
                    .audio_engine
                    .save_config();
                let ui_layout_future = self
                    .ui_layout_config
                    .save_tracing();

                task::block_on(async {
                    future::join!(audio_engine_future, ui_layout_future).await
                });

                *control_flow = event_loop::ControlFlow::Exit;
            }
            index => {
                // We add one since we display window indices with 1-based numbering.
                let index = index + 1;
                tracing::info!(
                    "Close requested on Window {index}. Closing window..."
                );
            }
        }
    }
}
